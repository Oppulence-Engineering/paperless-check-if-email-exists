import { z } from "zod";

import {
	BackendErrorEnvelopeSchema,
	BackendRequestContextSchema,
	CursorPageMetadataSchema,
	IdempotencyKeySchema,
	RequestIdSchema,
	RetryPolicySchema,
	UploadDescriptorSchema,
	type BackendErrorEnvelope,
	type BackendRequestContext,
	type RetryPolicyInput,
	type UploadDescriptor,
} from "./integration-contract.schema";

const BACKEND_REQUEST_HEADERS = {
	idempotencyKey: "x-idempotency-key",
	organizationId: "x-organization-id",
	organizationRole: "x-organization-role",
	requestId: "x-request-id",
	sessionId: "x-session-id",
	userId: "x-user-id",
} as const;

const LegacyErrorSchema = z
	.object({
		code: z.string().optional(),
		detail: z.string().optional(),
		error: z.string().optional(),
		message: z.string().optional(),
		requestId: z.string().optional(),
		title: z.string().optional(),
	})
	.passthrough();

/**
 * @oppulence-gen kind=lib
 * Pure helpers for the canonical frontend-to-backend integration contract.
 * Canonical backend request, response, pagination, retry, concurrency, and upload contracts.
 *
 * This module contains no React state and performs no network I/O. Owned by
 * `integration-contract.lit.ts`.
 */
export function createCursorPageSchema<T extends z.ZodType>(itemSchema: T) {
	return z.strictObject({
		items: z.array(itemSchema),
		page: CursorPageMetadataSchema,
	});
}

export function normalizeBackendError(
	body: unknown,
	status: number,
	fallbackRequestId?: string | null,
): BackendErrorEnvelope {
	const canonical = BackendErrorEnvelopeSchema.safeParse(body);
	if (canonical.success) {
		const requestId = RequestIdSchema.safeParse(canonical.data.requestId || fallbackRequestId);
		return {
			...canonical.data,
			...(requestId.success ? { requestId: requestId.data } : {}),
		};
	}

	const legacy = LegacyErrorSchema.safeParse(body);
	const value = legacy.success ? legacy.data : {};
	const requestId = RequestIdSchema.safeParse(value.requestId || fallbackRequestId);
	return BackendErrorEnvelopeSchema.parse({
		error:
			value.message ||
			value.error ||
			value.detail ||
			value.title ||
			`Request failed (${String(status)})`,
		code: value.code || `http_${String(status)}`,
		...(requestId.success ? { requestId: requestId.data } : {}),
		retryable: [408, 425, 429, 502, 503, 504].includes(status),
	});
}

export function backendContextHeaders(input: BackendRequestContext): Headers {
	const context = BackendRequestContextSchema.parse(input);
	return new Headers({
		[BACKEND_REQUEST_HEADERS.requestId]: context.requestId,
		[BACKEND_REQUEST_HEADERS.userId]: context.userId,
		[BACKEND_REQUEST_HEADERS.organizationId]: context.organizationId,
		[BACKEND_REQUEST_HEADERS.organizationRole]: context.organizationRole,
		[BACKEND_REQUEST_HEADERS.sessionId]: context.sessionId,
	});
}

export function acceptedRequestId(value: string | null | undefined, fallback: string): string {
	const parsed = RequestIdSchema.safeParse(value);
	return parsed.success ? parsed.data : RequestIdSchema.parse(fallback);
}

function isRetrySafeMethod(method: string, idempotencyKey?: string): boolean {
	const normalized = method.toUpperCase();
	if (["GET", "HEAD", "OPTIONS", "PUT", "DELETE"].includes(normalized)) return true;
	return Boolean(idempotencyKey && IdempotencyKeySchema.safeParse(idempotencyKey).success);
}

export function shouldRetryBackendRequest(input: {
	attempt: number;
	idempotencyKey?: string;
	method: string;
	policy?: RetryPolicyInput;
	status?: number;
}): boolean {
	const policy = RetryPolicySchema.parse(input.policy ?? {});
	if (input.attempt >= policy.maxAttempts) return false;
	if (!isRetrySafeMethod(input.method, input.idempotencyKey)) return false;
	return input.status === undefined || policy.retryStatuses.includes(input.status);
}

export function parseRetryAfterMs(value: string | null, now = Date.now()): number | null {
	if (!value) return null;
	const seconds = Number(value);
	if (Number.isFinite(seconds) && seconds >= 0) return seconds * 1_000;
	const date = Date.parse(value);
	return Number.isFinite(date) ? Math.max(0, date - now) : null;
}

export function retryDelayMs(
	attempt: number,
	policyInput?: RetryPolicyInput,
	retryAfterMs: number | null = null,
	random = Math.random,
): number {
	const policy = RetryPolicySchema.parse(policyInput ?? {});
	const exponential = Math.min(
		policy.baseDelayMs * 2 ** Math.max(0, attempt - 1),
		policy.maxDelayMs,
	);
	const jittered = Math.round(exponential * (0.8 + random() * 0.4));
	return Math.max(0, retryAfterMs ?? 0, jittered);
}

export function waitForRetry(delayMs: number, signal?: AbortSignal): Promise<void> {
	if (signal?.aborted) return Promise.reject(signal.reason);
	return new Promise((resolve, reject) => {
		const cleanup = () => signal?.removeEventListener("abort", abort);
		const timer = setTimeout(() => {
			cleanup();
			resolve();
		}, delayMs);
		const abort = () => {
			clearTimeout(timer);
			cleanup();
			reject(signal?.reason);
		};
		signal?.addEventListener("abort", abort, { once: true });
	});
}

export function requestSignal(signal: AbortSignal | undefined, timeoutMs: number): AbortSignal {
	const timeout = AbortSignal.timeout(timeoutMs);
	return signal ? AbortSignal.any([signal, timeout]) : timeout;
}

export function validateUploadDescriptor(value: unknown, now = Date.now()): UploadDescriptor {
	const descriptor = UploadDescriptorSchema.parse(value);
	if (Date.parse(descriptor.expiresAt) <= now) throw new Error("Upload descriptor has expired");
	return descriptor;
}
