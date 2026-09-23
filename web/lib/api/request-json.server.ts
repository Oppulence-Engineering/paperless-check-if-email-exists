import "server-only";

import { randomUUID } from "node:crypto";

import { headers } from "next/headers";
import { cache } from "react";

import { backendApiURL } from "@/lib/auth/config";
import { getAuthorizedSession, mintBackendToken } from "@/lib/auth/session";
import {
	DashboardRequestError,
	type RequestJsonFn,
	type RequestJsonInput,
} from "@/lib/api/request-json";
import { loadBackendCapabilities } from "@/lib/backend/capabilities";
import {
	EntityTagSchema,
	IdempotencyKeySchema,
	RequestIdSchema,
} from "@/lib/backend/integration-contract.schema";
import {
	backendContextHeaders,
	normalizeBackendError,
	parseRetryAfterMs,
	requestSignal,
	retryDelayMs,
	shouldRetryBackendRequest,
	waitForRetry,
} from "@/lib/backend/integration-contract";
import { isDevelopment } from "@/lib/environment";

async function readJsonBody(response: Response): Promise<unknown> {
	if (response.status === 204) return undefined;
	const text = await response.text();
	if (!text) return null;
	try {
		return JSON.parse(text);
	} catch {
		throw new DashboardRequestError("Backend returned malformed JSON", response.status);
	}
}

function upstreamHeaders(
	input: RequestJsonInput<unknown>,
	token: string,
	context: Headers,
): Headers {
	const result = new Headers(context);
	result.set("accept", "application/json");
	result.set("authorization", `Bearer ${token}`);
	if (input.body !== undefined) result.set("content-type", "application/json");
	if (input.idempotencyKey) {
		result.set("x-idempotency-key", IdempotencyKeySchema.parse(input.idempotencyKey));
	}
	if (input.ifMatch) result.set("if-match", EntityTagSchema.parse(input.ifMatch));
	if (input.requestId) result.set("x-request-id", RequestIdSchema.parse(input.requestId));
	return result;
}

const requestBackendIdentity = cache(async () => {
	const requestHeaders = await headers();
	const session = await getAuthorizedSession(requestHeaders);
	if (!session) {
		throw new DashboardRequestError("unauthenticated", 401, "unauthorized");
	}
	const token = await mintBackendToken(requestHeaders, session.membership.organizationId);
	return { session, token };
});

/**
 * Server prefetch transport. Mints a short-lived JWT and talks to the backend
 * directly, never by HTTP-looping through the BFF.
 */
export const requestUpstreamJson: RequestJsonFn = async <T>(
	input: RequestJsonInput<T>,
): Promise<T> => {
	const { session, token } = await requestBackendIdentity();

	const path = input.path.startsWith("/") ? input.path : `/${input.path}`;
	const capabilities = loadBackendCapabilities();
	const method = (input.method ?? "GET").toUpperCase();
	const requestId = input.requestId || randomUUID();
	const context = backendContextHeaders({
		requestId,
		userId: session.user.id,
		organizationId: session.membership.organizationId,
		organizationRole: session.membership.role,
		sessionId: session.session.id,
	});
	const fetchHeaders = upstreamHeaders(input, token, context);
	let attempt = 1;

	for (;;) {
		let response: Response;
		try {
			response = await fetch(backendApiURL(path), {
				method,
				cache: "no-store",
				signal: requestSignal(input.signal, input.timeoutMs ?? capabilities.http.requestTimeoutMs),
				body: input.body === undefined ? undefined : JSON.stringify(input.body),
				headers: fetchHeaders,
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

		const body = await readJsonBody(response);
		if (!response.ok) {
			const envelope = normalizeBackendError(
				body,
				response.status,
				response.headers.get("x-request-id") || requestId,
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
			throw new DashboardRequestError(envelope.error, response.status, envelope.code, {
				requestId: envelope.requestId,
				retryable: envelope.retryable,
			});
		}

		try {
			return input.schema.parse(body);
		} catch (error) {
			console.error(
				...(isDevelopment()
					? ([`Unexpected ${input.path} upstream response`, error] as const)
					: (["Backend response did not match its schema"] as const)),
			);
			throw new DashboardRequestError(
				`The ${input.path} response did not match what this app expects. The app and the API are probably running different versions.`,
				0,
				"schema_mismatch",
			);
		}
	}
};
