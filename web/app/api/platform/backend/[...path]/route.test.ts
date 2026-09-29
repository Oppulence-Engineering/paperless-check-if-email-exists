import { NextRequest } from "next/server";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

const checks = vi.hoisted(() => ({
	session: vi.fn(),
	admin: vi.fn(),
	stepUp: vi.fn(),
	audit: vi.fn(),
}));

vi.mock("@/lib/auth/session", () => ({ getAuthorizedSession: checks.session }));
vi.mock("@/lib/admin/platform-admin", () => ({ isPlatformAdmin: checks.admin }));
vi.mock("@/lib/auth/step-up", () => ({ hasValidStepUp: checks.stepUp }));
vi.mock("@/lib/auth/identity-audit", () => ({ identityAudit: checks.audit }));

import { GET, POST } from "./route";

const context = (path: string[]) => ({ params: Promise.resolve({ path }) });

beforeEach(() => {
	vi.clearAllMocks();
	vi.stubEnv("BETTER_AUTH_URL", "https://app.example.test");
	vi.stubEnv("BACKEND_API_URL", "https://backend.example.test");
	vi.stubEnv("RCH__HEADER_SECRET", "server-only-secret");
	checks.session.mockResolvedValue({
		user: { id: "operator-1", email: "operator@example.test" },
		session: {},
	});
	checks.admin.mockReturnValue(true);
	checks.stepUp.mockReturnValue(true);
	checks.audit.mockResolvedValue(undefined);
});

afterEach(() => {
	vi.unstubAllEnvs();
	vi.unstubAllGlobals();
});

it("rejects undeclared routes and ordinary members before backend access", async () => {
	const fetch = vi.fn();
	vi.stubGlobal("fetch", fetch);
	expect(
		(
			await GET(
				new NextRequest("https://app.example.test/api/platform/backend/v1/admin/missing"),
				context(["v1", "admin", "missing"]),
			)
		).status,
	).toBe(404);
	checks.admin.mockReturnValue(false);
	expect(
		(
			await GET(
				new NextRequest("https://app.example.test/api/platform/backend/v1/admin/tenants"),
				context(["v1", "admin", "tenants"]),
			)
		).status,
	).toBe(404);
	expect(fetch).not.toHaveBeenCalled();
	expect(checks.audit).toHaveBeenCalledWith(expect.objectContaining({ result: "failure" }));
});

it("fails closed without an operator secret or a durable audit write", async () => {
	const fetch = vi.fn();
	vi.stubGlobal("fetch", fetch);
	const request = new NextRequest("https://app.example.test/api/platform/backend/v1/admin/tenants");
	vi.stubEnv("RCH__HEADER_SECRET", "  ");
	expect((await GET(request, context(["v1", "admin", "tenants"]))).status).toBe(503);
	vi.stubEnv("RCH__HEADER_SECRET", "server-only-secret");
	checks.audit.mockRejectedValueOnce(new Error("audit unavailable"));
	await expect(GET(request, context(["v1", "admin", "tenants"]))).rejects.toThrow(
		"audit unavailable",
	);
	expect(fetch).not.toHaveBeenCalled();
});

it("forwards an audited admin read with the secret only on the server", async () => {
	const fetch = vi.fn().mockResolvedValue(Response.json({ tenants: [] }));
	vi.stubGlobal("fetch", fetch);
	const response = await GET(
		new NextRequest("https://app.example.test/api/platform/backend/v1/admin/tenants?page=2", {
			headers: { "x-reacher-secret": "attacker-secret" },
		}),
		context(["v1", "admin", "tenants"]),
	);
	expect(response.status).toBe(200);
	expect(fetch.mock.calls[0][0].href).toBe("https://backend.example.test/v1/admin/tenants?page=2");
	expect(new Headers(fetch.mock.calls[0][1].headers).get("x-reacher-secret")).toBe(
		"server-only-secret",
	);
	expect(checks.audit).toHaveBeenCalledWith(
		expect.objectContaining({ actorId: "operator-1", action: "platform.backend.list_tenants" }),
	);
});

it("requires an operator reason and fresh step-up before a mutation", async () => {
	const fetch = vi.fn().mockResolvedValue(Response.json({ id: "tenant-1" }));
	vi.stubGlobal("fetch", fetch);
	const url = "https://app.example.test/api/platform/backend/v1/admin/tenants";
	const request = (reason?: string) =>
		new NextRequest(url, {
			method: "POST",
			headers: {
				origin: "https://app.example.test",
				"content-type": "application/json",
				...(reason ? { "x-admin-reason": reason } : {}),
			},
			body: JSON.stringify({ name: "Tenant" }),
		});
	expect((await POST(request(), context(["v1", "admin", "tenants"]))).status).toBe(400);
	checks.stepUp.mockReturnValue(false);
	expect(
		(await POST(request("Support request 123"), context(["v1", "admin", "tenants"]))).status,
	).toBe(403);
	expect(fetch).not.toHaveBeenCalled();
	checks.stepUp.mockReturnValue(true);
	expect(
		(await POST(request("Support request 123"), context(["v1", "admin", "tenants"]))).status,
	).toBe(200);
	expect(checks.audit).toHaveBeenCalledWith(
		expect.objectContaining({
			reason: "Support request 123",
			action: "platform.backend.create_tenant",
		}),
	);
	expect(fetch).toHaveBeenCalledTimes(1);
});
