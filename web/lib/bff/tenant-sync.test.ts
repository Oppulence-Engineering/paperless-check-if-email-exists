import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("server-only", () => ({}));
vi.mock("@/lib/auth/config", () => ({
	backendApiURL: (path: string) => new URL(`http://backend.test${path}`),
}));
vi.mock("@/lib/billing/workspace-plan", () => ({ workspacePlan: vi.fn() }));

const { tenantIdForOrganization, syncTenantPlan } = await import("./tenant-sync");
const { workspacePlan } = await import("@/lib/billing/workspace-plan");

afterEach(() => {
	vi.unstubAllEnvs();
	vi.unstubAllGlobals();
});

describe("tenant plan sync", () => {
	it("derives the same tenant id as Postgres uuid_generate_v5(uuid_ns_oid(), id)", () => {
		expect(tenantIdForOrganization("org_Q3vX9kTz1")).toBe("17c52724-178e-583e-b114-bafd58987eb8");
		expect(tenantIdForOrganization("7d3f2c1e-org")).toBe("b1bc787c-a9e7-5882-9265-654dd02d060b");
	});

	it("pushes the plan and its allowance, and keeps an Enterprise limit", async () => {
		vi.stubEnv("RCH__HEADER_SECRET", "test-secret");
		const fetch = vi.fn(async () => new Response("{}", { status: 200 }));
		vi.stubGlobal("fetch", fetch);

		vi.mocked(workspacePlan).mockResolvedValueOnce({ plan: "free", status: null });
		await syncTenantPlan("org_Q3vX9kTz1");
		const [url, init] = fetch.mock.calls[0] as unknown as [URL, RequestInit];
		expect(url.pathname).toBe("/v1/admin/tenants/17c52724-178e-583e-b114-bafd58987eb8");
		expect(init.method).toBe("PUT");
		expect(JSON.parse(String(init.body))).toEqual({ plan_tier: "free", monthly_email_limit: 100 });

		vi.mocked(workspacePlan).mockResolvedValueOnce({ plan: "enterprise", status: "active" });
		await syncTenantPlan("org_Q3vX9kTz1");
		const [, enterprise] = fetch.mock.calls[1] as unknown as [URL, RequestInit];
		expect(JSON.parse(String(enterprise.body))).toEqual({ plan_tier: "enterprise" });
	});

	it("fails the webhook when the backend rejects the update, so Stripe retries", async () => {
		vi.stubEnv("RCH__HEADER_SECRET", "test-secret");
		vi.stubGlobal(
			"fetch",
			vi.fn(async () => new Response("", { status: 500 })),
		);
		vi.mocked(workspacePlan).mockResolvedValueOnce({ plan: "starter", status: "active" });
		await expect(syncTenantPlan("org_Q3vX9kTz1")).rejects.toThrow("500");
	});
});
