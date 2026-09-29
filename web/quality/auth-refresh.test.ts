import { afterEach, describe, expect, it, vi } from "vitest";

import { backendApiURL, getAuthRuntimeConfig } from "@/lib/auth/config";

afterEach(() => vi.unstubAllEnvs());

describe("backend API URL", () => {
	it("keeps the configured base path and appends query parameters", () => {
		vi.stubEnv("BACKEND_API_URL", "https://api.example.test/root/v2");
		expect(backendApiURL("/widgets", new URLSearchParams({ page: "2" })).toString()).toBe(
			"https://api.example.test/root/v2/widgets?page=2",
		);
	});

	it.each(["widgets", "//evil.example/widgets"])("rejects unsafe path %s", (path) => {
		expect(() => backendApiURL(path)).toThrow("absolute path");
	});
});

describe("production authentication configuration", () => {
	function productionEnvironment() {
		vi.stubEnv("NODE_ENV", "production");
		vi.stubEnv("BACKEND_API_URL", "https://api.example.test/v1");
		vi.stubEnv("BACKEND_JWT_AUDIENCE", "example-api");
		vi.stubEnv("BETTER_AUTH_URL", "https://app.example.test");
		vi.stubEnv("BETTER_AUTH_SECRET", "test-value-".repeat(4));
		vi.stubEnv("SCIM_CREDENTIAL_HASH_SECRET", "other-test-value-".repeat(4));
		vi.stubEnv("DATABASE_URL", "postgres://app:unique-password@db.internal/check_email");
		vi.stubEnv("RESEND_API_KEY", "re_production_key");
		vi.stubEnv("RESEND_FROM", "Oppulence <noreply@example.test>");
	}

	it("rejects insecure public origins", () => {
		productionEnvironment();
		vi.stubEnv("BETTER_AUTH_URL", "http://app.example.test");

		expect(() => getAuthRuntimeConfig()).toThrow("BETTER_AUTH_URL must use HTTPS");
	});

	it("allows HTTP only for the explicit loopback E2E server", () => {
		productionEnvironment();
		vi.stubEnv("AUTH_E2E_MODE", "1");
		vi.stubEnv("BETTER_AUTH_URL", "http://127.0.0.1:4317");

		expect(getAuthRuntimeConfig().betterAuthUrl).toBe("http://127.0.0.1:4317");

		vi.stubEnv("BETTER_AUTH_URL", "http://app.example.test");
		expect(() => getAuthRuntimeConfig()).toThrow("BETTER_AUTH_URL must use HTTPS");
	});

	it("allows default database credentials only for the explicit loopback E2E server", () => {
		productionEnvironment();
		vi.stubEnv("AUTH_E2E_MODE", "1");
		vi.stubEnv("BETTER_AUTH_URL", "http://127.0.0.1:4317");
		vi.stubEnv("DATABASE_URL", "postgres://postgres:postgres@127.0.0.1:5432/check_email");

		expect(getAuthRuntimeConfig().databaseUrl).toContain("127.0.0.1");

		vi.stubEnv("DATABASE_URL", "postgres://postgres:postgres@db.internal/check_email");
		expect(() => getAuthRuntimeConfig()).toThrow(
			"DATABASE_URL must not use the default development credentials",
		);
	});

	it("rejects example secrets and default database credentials", () => {
		productionEnvironment();
		vi.stubEnv("BETTER_AUTH_SECRET", "replace-with-at-least-32-random-characters");
		vi.stubEnv("DATABASE_URL", "postgres://postgres:postgres@db.internal/check_email");

		expect(() => getAuthRuntimeConfig()).toThrow("BETTER_AUTH_SECRET must not use an example");
	});

	it("requires the server-side email provider key", () => {
		productionEnvironment();
		vi.stubEnv("RESEND_API_KEY", "");

		expect(() => getAuthRuntimeConfig()).toThrow("RESEND_API_KEY is required");
	});

	it("requires an independent SCIM credential hash secret", () => {
		productionEnvironment();
		vi.stubEnv("SCIM_CREDENTIAL_HASH_SECRET", "");

		expect(() => getAuthRuntimeConfig()).toThrow();
	});

	it.each(["BACKEND_API_URL", "BACKEND_JWT_AUDIENCE", "BETTER_AUTH_URL"])(
		"requires %s instead of silently using a production default",
		(name) => {
			productionEnvironment();
			vi.stubEnv(name, "");

			expect(() => getAuthRuntimeConfig()).toThrow();
		},
	);
});
