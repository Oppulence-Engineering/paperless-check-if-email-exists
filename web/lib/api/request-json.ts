import { z } from "zod";

import {
	IdempotencyKeySchema,
	EntityTagSchema,
	RequestIdSchema,
	type RetryPolicyInput,
} from "@/lib/backend/integration-contract.schema";
import {
	normalizeBackendError,
	parseRetryAfterMs,
	retryDelayMs,
	shouldRetryBackendRequest,
	waitForRetry,
} from "@/lib/backend/integration-contract";
import {
	dashboardRequest,
	redirectBrowserIfUnauthorized,
	toDashboardAPIPath,
} from "@/lib/auth/dashboard-fetch";
import { isDevelopment } from "@/lib/environment";

export class BackendRequestError extends Error {
	status: number;
	code?: string;
	requestId?: string;
	retryable: boolean;

	constructor(
		message: string,
		status: number,
		code?: string,
		options: { requestId?: string; retryable?: boolean } = {},
	) {
		super(message);
		this.name = "BackendRequestError";
		this.status = status;
		this.code = code;
		this.requestId = options.requestId;
		this.retryable = options.retryable ?? false;
	}
}

/** @deprecated Use BackendRequestError in new backend-neutral modules. */
export { BackendRequestError as DashboardRequestError };

export type RequestJsonOptions = {
	idempotencyKey?: string;
	ifMatch?: string;
	requestId?: string;
	retryPolicy?: RetryPolicyInput;
	signal?: AbortSignal;
	timeoutMs?: number;
};

export type RequestJsonInput<T> = RequestJsonOptions & {
	/** Path relative to `BACKEND_API_URL`, for example `/relationship-sources/status`. */
	path: string;
	schema: z.ZodType<T>;
	method?: string;
	body?: unknown;
};

export type RequestJsonFn = <T>(input: RequestJsonInput<T>) => Promise<T>;

/** 404/501/502/503 mean the optional backend feature is absent or unavailable. */
export function isOptionalRequestFailure(error: unknown): boolean {
	return (
		error instanceof BackendRequestError &&
		(error.status === 404 || error.status === 501 || error.status === 502 || error.status === 503)
	);
}

async function readJsonBody(response: Response): Promise<unknown> {
	if (response.status === 204) return undefined;
	const text = await response.text();
	if (!text) return null;
	try {
		return JSON.parse(text);
	} catch {
		throw new BackendRequestError("Backend returned malformed JSON", response.status);
	}
}

function requestHeaders(input: RequestJsonOptions, hasBody: boolean): Headers {
	const headers = new Headers({ Accept: "application/json" });
	if (hasBody) headers.set("content-type", "application/json");
	if (input.idempotencyKey) {
		headers.set("x-idempotency-key", IdempotencyKeySchema.parse(input.idempotencyKey));
	}
	if (input.ifMatch) headers.set("if-match", EntityTagSchema.parse(input.ifMatch));
	if (input.requestId) headers.set("x-request-id", RequestIdSchema.parse(input.requestId));
	return headers;
}

/**
 * Same-origin BFF JSON call with runtime validation, bounded retries, caller
 * cancellation, idempotency, optimistic concurrency, and per-attempt timeouts.
 */
export async function requestJson<T>(input: RequestJsonInput<T>): Promise<T> {
	const method = (input.method ?? "GET").toUpperCase();
	const headers = requestHeaders(input, input.body !== undefined);
	let attempt = 1;

	for (;;) {
		let response: Response;
		try {
			response = await dashboardRequest(toDashboardAPIPath(input.path), {
				method,
				signal: input.signal,
				timeoutMs: input.timeoutMs,
				body: input.body === undefined ? undefined : JSON.stringify(input.body),
				headers,
			});
		} catch (error) {
			if (
				input.signal?.aborted ||
				!shouldRetryBackendRequest({
					attempt,
					idempotencyKey: input.idempotencyKey,
					method,
					policy: input.retryPolicy,
				})
			) {
				throw error;
			}
			await waitForRetry(retryDelayMs(attempt, input.retryPolicy), input.signal);
			attempt += 1;
			continue;
		}

		redirectBrowserIfUnauthorized(response.status);
		const body = await readJsonBody(response);
		if (!response.ok) {
			const envelope = normalizeBackendError(
				body,
				response.status,
				response.headers.get("x-request-id"),
			);
			if (
				shouldRetryBackendRequest({
					attempt,
					idempotencyKey: input.idempotencyKey,
					method,
					policy: input.retryPolicy,
					status: response.status,
				})
			) {
				await waitForRetry(
					retryDelayMs(
						attempt,
						input.retryPolicy,
						parseRetryAfterMs(response.headers.get("retry-after")),
					),
					input.signal,
				);
				attempt += 1;
				continue;
			}
			throw new BackendRequestError(envelope.error, response.status, envelope.code, {
				requestId: envelope.requestId,
				retryable: envelope.retryable,
			});
		}

		try {
			return input.schema.parse(body);
		} catch (error) {
			if (isDevelopment()) console.error(`Unexpected ${input.path} response`, error);
			throw new BackendRequestError(
				`The ${input.path} response did not match what this app expects. The app and the API are probably running different versions.`,
				0,
				"schema_mismatch",
			);
		}
	}
}
