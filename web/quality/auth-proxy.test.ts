import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { NextRequest } from "next/server";

const auth = vi.hoisted(() => ({
	getAuthorizedSession: vi.fn(),
	mintBackendToken: vi.fn(),
	deleteUserIdentity: vi.fn(),
}));

vi.mock("@/lib/auth/session", () => auth);

import { proxyBackendAPI } from "@/lib/auth/proxy";

const session = {
	user: { id: "user-1" },
	session: { id: "session-1" },
	membership: { organizationId: "org-1", role: "member" },
};

describe("backend proxy", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		vi.stubEnv("BACKEND_API_URL", "https://backend.example.test");
		vi.stubEnv("BETTER_AUTH_URL", "https://app.example.test");
		vi.stubEnv("TRUSTED_PUBLIC_ORIGINS", "https://app.example.test");
		auth.getAuthorizedSession.mockResolvedValue(session);
		auth.mintBackendToken.mockResolvedValue("server-jwt");
	});

	afterEach(() => {
		vi.unstubAllEnvs();
		vi.unstubAllGlobals();
	});

	it("rejects an anonymous request before contacting the backend", async () => {
		auth.getAuthorizedSession.mockResolvedValue(null);
		const upstream = vi.fn();
		vi.stubGlobal("fetch", upstream);
		const response = await proxyBackendAPI(
			new NextRequest("https://app.example.test/api/backend/v1/me"),
			["v1", "me"],
		);
		expect(response.status).toBe(401);
		expect(auth.mintBackendToken).not.toHaveBeenCalled();
		expect(upstream).not.toHaveBeenCalled();
	});

	it.each([
		["v0", "check_email"],
		["v1", "admin", "tenants"],
		["v1", "check-email-with-onboard"],
		["v1", "inbound", "providers"],
	])("keeps internal and legacy routes out of the generic BFF: %j", async (...path) => {
		const upstream = vi.fn();
		vi.stubGlobal("fetch", upstream);
		const response = await proxyBackendAPI(
			new NextRequest(`https://app.example.test/api/backend/${path.join("/")}`),
			path,
		);
		expect(response.status).toBe(404);
		expect(upstream).not.toHaveBeenCalled();
	});

	it("rejects a cross-origin mutation before checking the session", async () => {
		const response = await proxyBackendAPI(
			new NextRequest("https://app.example.test/api/backend/v1/lists", {
				method: "POST",
				headers: { origin: "https://attacker.example.test" },
			}),
			["v1", "lists"],
		);
		expect(response.status).toBe(403);
		expect(auth.getAuthorizedSession).not.toHaveBeenCalled();
	});

	it("replaces browser supplied identity and authorization headers", async () => {
		const upstream = vi.fn((...args: [RequestInfo | URL, RequestInit?]) => {
			expect(args.length).toBeGreaterThan(0);
			return Promise.resolve(Response.json({ id: "user-1" }));
		});
		vi.stubGlobal("fetch", upstream);
		const response = await proxyBackendAPI(
			new NextRequest("https://app.example.test/api/backend/v1/me", {
				headers: {
					authorization: "Bearer attacker",
					"x-organization-id": "org-attacker",
					"x-user-id": "user-attacker",
				},
			}),
			["v1", "me"],
		);
		expect(response.status).toBe(200);
		expect(upstream).toHaveBeenCalledTimes(1);
		const [url, options] = upstream.mock.calls[0];
		expect(url).toBeInstanceOf(URL);
		if (!(url instanceof URL)) throw new Error("Expected a backend URL");
		expect(url.href).toBe("https://backend.example.test/v1/me");
		const headers = new Headers(options?.headers);
		expect(headers.get("authorization")).toBe("Bearer server-jwt");
		expect(headers.get("x-organization-id")).toBe("org-1");
		expect(headers.get("x-user-id")).toBe("user-1");
	});

	it("keeps upstream failures from exposing backend details", async () => {
		vi.stubGlobal(
			"fetch",
			vi.fn(() => Promise.resolve(Response.json({ secret: "internal" }, { status: 500 }))),
		);
		const response = await proxyBackendAPI(
			new NextRequest("https://app.example.test/api/backend/v1/me"),
			["v1", "me"],
		);
		expect(response.status).toBe(500);
		expect(await response.json()).toMatchObject({ code: "upstream_error" });
	});
});
