import { beforeEach, describe, expect, it, vi } from "vitest";
import { z } from "zod";

import { dashboardRequest } from "@/lib/auth/dashboard-fetch";
import {
	DashboardRequestError,
	isOptionalRequestFailure,
	requestJson,
} from "@/lib/api/request-json";

vi.mock("@/lib/auth/dashboard-fetch", () => ({
	dashboardRequest: vi.fn(),
	toDashboardAPIPath: (path: string) => `/api/backend${path}`,
	redirectBrowserIfUnauthorized: () => undefined,
	loginURL: () => "/sign-in",
}));

const mockRequest = vi.mocked(dashboardRequest);

beforeEach(() => mockRequest.mockReset());

describe("requestJson", () => {
	it("validates a successful BFF body against the supplied contract", async () => {
		mockRequest.mockResolvedValueOnce(
			new Response(JSON.stringify({ sources: [] }), {
				status: 200,
				headers: { "Content-Type": "application/json" },
			}),
		);

		await expect(
			requestJson({
				path: "/relationship-sources/status",
				schema: z.object({ sources: z.array(z.unknown()) }),
			}),
		).resolves.toEqual({ sources: [] });
		expect(mockRequest.mock.calls[0]?.[0]).toBe("/api/backend/relationship-sources/status");
	});

	it("keeps Go problem-detail text on the thrown error", async () => {
		mockRequest.mockResolvedValueOnce(
			new Response(JSON.stringify({ detail: "invalid relationshipId" }), {
				status: 400,
				headers: { "Content-Type": "application/json" },
			}),
		);

		await expect(
			requestJson({
				path: "/relationships/graph",
				schema: z.object({}),
			}),
		).rejects.toMatchObject({
			name: "BackendRequestError",
			message: "invalid relationshipId",
			status: 400,
		} satisfies Partial<DashboardRequestError>);
	});

	it("treats missing or degraded routes as optional failures", () => {
		expect(isOptionalRequestFailure(new DashboardRequestError("gone", 404))).toBe(true);
		expect(isOptionalRequestFailure(new DashboardRequestError("unsupported", 501))).toBe(true);
		expect(isOptionalRequestFailure(new DashboardRequestError("down", 503))).toBe(true);
		expect(isOptionalRequestFailure(new DashboardRequestError("bad", 400))).toBe(false);
	});

	it("extracts the public error envelope on failure", async () => {
		mockRequest.mockResolvedValueOnce(
			new Response(JSON.stringify({ error: "unauthenticated", code: "unauthorized" }), {
				status: 401,
				headers: { "Content-Type": "application/json" },
			}),
		);

		await expect(
			requestJson({
				path: "/relationship-sources/status",
				schema: z.object({ sources: z.array(z.unknown()) }),
			}),
		).rejects.toMatchObject({
			name: "BackendRequestError",
			status: 401,
			code: "unauthorized",
		} satisfies Partial<DashboardRequestError>);
	});

	it("retries safe failures and preserves request contract headers", async () => {
		mockRequest
			.mockResolvedValueOnce(
				Response.json(
					{ error: "temporarily unavailable", code: "service_unavailable", retryable: true },
					{ status: 503 },
				),
			)
			.mockResolvedValueOnce(Response.json({ id: "widget-1" }));

		await expect(
			requestJson({
				path: "/widgets",
				method: "POST",
				body: { name: "Widget" },
				idempotencyKey: "create-widget-123",
				ifMatch: '"version-1"',
				requestId: "request-123",
				retryPolicy: { baseDelayMs: 0, maxDelayMs: 0 },
				schema: z.object({ id: z.string() }),
			}),
		).resolves.toEqual({ id: "widget-1" });

		expect(mockRequest).toHaveBeenCalledTimes(2);
		const options = mockRequest.mock.calls[0]?.[1];
		const headers = new Headers(options?.headers);
		expect(headers.get("x-idempotency-key")).toBe("create-widget-123");
		expect(headers.get("if-match")).toBe('"version-1"');
		expect(headers.get("x-request-id")).toBe("request-123");
		expect(options?.body).toBe(JSON.stringify({ name: "Widget" }));
	});

	it("does not retry an unsafe POST without an idempotency key", async () => {
		mockRequest.mockResolvedValue(
			Response.json(
				{ error: "temporarily unavailable", code: "service_unavailable" },
				{ status: 503 },
			),
		);

		await expect(
			requestJson({
				path: "/widgets",
				method: "POST",
				body: { name: "Widget" },
				retryPolicy: { baseDelayMs: 0, maxDelayMs: 0 },
				schema: z.object({ id: z.string() }),
			}),
		).rejects.toMatchObject({ status: 503, code: "service_unavailable" });
		expect(mockRequest).toHaveBeenCalledOnce();
	});
});
