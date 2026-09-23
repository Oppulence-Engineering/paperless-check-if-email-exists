import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	database: vi.fn(),
	isBackendReady: vi.fn(),
}));

vi.mock("@/lib/auth/database", () => ({ database: mocks.database }));
vi.mock("@/lib/bff/readiness", () => ({
	isBackendReady: mocks.isBackendReady,
}));

import { GET } from "@/app/readyz/route";

describe("GET /readyz", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		vi.stubEnv("BACKEND_API_URL", "https://api.example.test/v2");
		vi.stubEnv("BETTER_AUTH_URL", "https://ui.example.test");
		vi.stubEnv("BETTER_AUTH_SECRET", "a-production-secret-that-is-long-enough");
		vi.stubEnv("SCIM_CREDENTIAL_HASH_SECRET", "a-separate-scim-secret-that-is-long-enough");
		vi.stubEnv("DATABASE_URL", "postgres://auth.example.test/app");
		vi.stubEnv("RESEND_API_KEY", "re_test");
		vi.stubEnv("RESEND_FROM", "Auth <auth@example.test>");
		mocks.database.mockResolvedValue(true);
		mocks.isBackendReady.mockResolvedValue(true);
	});

	afterEach(() => vi.unstubAllEnvs());

	it("reports ready only after the database and backend checks pass", async () => {
		const response = await GET();
		expect(response.status).toBe(200);
		await expect(response.json()).resolves.toEqual({ status: "ready" });
		expect(response.headers.get("cache-control")).toBe("no-store");
		expect(mocks.database).toHaveBeenCalledOnce();
		expect(mocks.isBackendReady).toHaveBeenCalledWith(
			"https://api.example.test/v2",
			"/readyz",
			2_000,
		);
	});

	it("reports not ready when a dependency is unhealthy", async () => {
		mocks.database.mockResolvedValue(false);
		const response = await GET();
		expect(response.status).toBe(503);
		await expect(response.json()).resolves.toEqual({ status: "not_ready" });
	});

	it("reports not ready without exposing invalid configuration", async () => {
		vi.stubEnv("BACKEND_API_URL", "not-a-url");
		const response = await GET();
		expect(response.status).toBe(503);
		await expect(response.json()).resolves.toEqual({ status: "not_ready" });
	});
});
