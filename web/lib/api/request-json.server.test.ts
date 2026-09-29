import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { z } from "zod";

const mocks = vi.hoisted(() => ({
	headers: vi.fn(),
	getAuthorizedSession: vi.fn(),
	mintBackendToken: vi.fn(),
	backendApiURL: vi.fn((path: string) => `https://backend.example.test/v1${path}`),
	loadBackendCapabilities: vi.fn(() => ({ http: { requestTimeoutMs: 5_000 } })),
}));

vi.mock("next/headers", () => ({ headers: mocks.headers }));
vi.mock("@/lib/auth/config", () => ({ backendApiURL: mocks.backendApiURL }));
vi.mock("@/lib/auth/session", () => ({
	getAuthorizedSession: mocks.getAuthorizedSession,
	mintBackendToken: mocks.mintBackendToken,
}));
vi.mock("@/lib/backend/capabilities", () => ({
	loadBackendCapabilities: mocks.loadBackendCapabilities,
}));
vi.mock("@/lib/environment", () => ({ isDevelopment: () => false }));

import { BackendRequestError } from "@/lib/api/request-json";
import { requestUpstreamJson } from "@/lib/api/request-json.server";

const session = {
	user: { id: "user-1" },
	session: { id: "session-1" },
	membership: { organizationId: "org-1", role: "member" },
};

function fetchCall(): [RequestInfo | URL, RequestInit | undefined] {
	const call = vi.mocked(fetch).mock.calls[0];
	if (!call) throw new Error("Expected a backend fetch");
	return call;
}

describe("requestUpstreamJson", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		mocks.headers.mockResolvedValue(new Headers({ cookie: "session=browser-only" }));
		mocks.getAuthorizedSession.mockResolvedValue(session);
		mocks.mintBackendToken.mockResolvedValue("backend-jwt");
	});

	afterEach(() => {
		vi.unstubAllGlobals();
	});

	it("rejects anonymous server prefetches before minting a backend token", async () => {
		mocks.getAuthorizedSession.mockResolvedValue(null);
		vi.stubGlobal("fetch", vi.fn());

		await expect(
			requestUpstreamJson({ path: "/widgets", schema: z.object({ id: z.string() }) }),
		).rejects.toMatchObject<BackendRequestError>({
			status: 401,
			code: "unauthorized",
		});
		expect(mocks.mintBackendToken).not.toHaveBeenCalled();
		expect(fetch).not.toHaveBeenCalled();
	});

	it("calls the backend directly with authoritative identity and request headers", async () => {
		vi.stubGlobal(
			"fetch",
			vi
				.fn()
				.mockResolvedValue(
					Response.json({ id: "widget-1" }, { headers: { "x-request-id": "request-1" } }),
				),
		);

		await expect(
			requestUpstreamJson({
				path: "widgets",
				method: "POST",
				body: { label: "Example" },
				schema: z.object({ id: z.string() }),
				idempotencyKey: "mutation-1",
				ifMatch: '"revision-1"',
				requestId: "request-1",
			}),
		).resolves.toEqual({ id: "widget-1" });

		const [url, init] = fetchCall();
		const requestHeaders = new Headers(init?.headers);
		expect(url).toBe("https://backend.example.test/v1/widgets");
		expect(init).toMatchObject({
			method: "POST",
			cache: "no-store",
			body: JSON.stringify({ label: "Example" }),
		});
		expect(requestHeaders.get("authorization")).toBe("Bearer backend-jwt");
		expect(requestHeaders.get("x-user-id")).toBe("user-1");
		expect(requestHeaders.get("x-organization-id")).toBe("org-1");
		expect(requestHeaders.get("x-organization-role")).toBe("member");
		expect(requestHeaders.get("x-session-id")).toBe("session-1");
		expect(requestHeaders.get("x-idempotency-key")).toBe("mutation-1");
		expect(requestHeaders.get("if-match")).toBe('"revision-1"');
		expect(requestHeaders.get("x-request-id")).toBe("request-1");
		expect(requestHeaders.has("cookie")).toBe(false);
		expect(mocks.mintBackendToken).toHaveBeenCalledWith(expect.any(Headers), "org-1");
	});

	it("normalizes backend errors with request metadata", async () => {
		vi.stubGlobal(
			"fetch",
			vi.fn().mockResolvedValue(
				Response.json(
					{
						error: "Widget conflict",
						code: "widget_conflict",
						retryable: false,
					},
					{ status: 409, headers: { "x-request-id": "backend-request-1" } },
				),
			),
		);

		await expect(
			requestUpstreamJson({
				path: "/widgets/widget-1",
				schema: z.object({ id: z.string() }),
				retryPolicy: { maxAttempts: 1 },
			}),
		).rejects.toMatchObject<BackendRequestError>({
			message: "Widget conflict",
			status: 409,
			code: "widget_conflict",
			requestId: "backend-request-1",
			retryable: false,
		});
	});
});
