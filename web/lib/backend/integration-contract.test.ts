import { describe, expect, it } from "vitest";
import { z } from "zod";

import {
	acceptedRequestId,
	backendContextHeaders,
	createCursorPageSchema,
	normalizeBackendError,
	parseRetryAfterMs,
	retryDelayMs,
	shouldRetryBackendRequest,
	validateUploadDescriptor,
} from "./integration-contract";

describe("backend integration helpers", () => {
	it("normalizes problem details and carries request correlation", () => {
		expect(
			normalizeBackendError(
				{ detail: "temporarily unavailable", code: "upstream_down" },
				503,
				"req-1",
			),
		).toEqual({
			error: "temporarily unavailable",
			code: "upstream_down",
			requestId: "req-1",
			retryable: true,
		});
		expect(
			normalizeBackendError(
				{ error: "conflict", code: "conflict", retryable: false },
				409,
				"response-header-1",
			),
		).toEqual({
			error: "conflict",
			code: "conflict",
			requestId: "response-header-1",
			retryable: false,
		});
	});

	it("builds authoritative BFF context headers", () => {
		const headers = backendContextHeaders({
			requestId: "req-1",
			userId: "user-1",
			organizationId: "org-1",
			organizationRole: "member",
			sessionId: "session-1",
		});
		expect(Object.fromEntries(headers)).toMatchObject({
			"x-request-id": "req-1",
			"x-user-id": "user-1",
			"x-organization-id": "org-1",
			"x-organization-role": "member",
			"x-session-id": "session-1",
		});
		expect(acceptedRequestId("bad id", "fallback-1")).toBe("fallback-1");
	});

	it("retries only safe requests and honors Retry-After as a floor", () => {
		expect(shouldRetryBackendRequest({ attempt: 1, method: "GET", status: 503 })).toBe(true);
		expect(shouldRetryBackendRequest({ attempt: 1, method: "POST", status: 503 })).toBe(false);
		expect(
			shouldRetryBackendRequest({
				attempt: 1,
				method: "POST",
				status: 503,
				idempotencyKey: "request-123",
			}),
		).toBe(true);
		expect(parseRetryAfterMs("2", 0)).toBe(2_000);
		expect(retryDelayMs(1, { baseDelayMs: 250 }, 2_000, () => 0)).toBe(2_000);
	});

	it("validates cursor pages and rejects expired upload descriptors", () => {
		const page = createCursorPageSchema(z.object({ id: z.string() }));
		expect(
			page.parse({ items: [{ id: "widget-1" }], page: { nextCursor: null, hasMore: false } }),
		).toEqual({ items: [{ id: "widget-1" }], page: { nextCursor: null, hasMore: false } });
		expect(() =>
			validateUploadDescriptor(
				{
					uploadId: "upload-1",
					url: "https://uploads.example.test/object",
					method: "PUT",
					headers: {},
					expiresAt: "2026-01-01T00:00:00.000Z",
				},
				Date.parse("2026-02-01T00:00:00.000Z"),
			),
		).toThrow(/expired/i);
	});
});
