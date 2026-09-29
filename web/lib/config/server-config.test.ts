import { afterEach, describe, expect, it } from "vitest";

import {
	assertServerConfig,
	resetServerConfigCache,
	serverConfig,
	serverConfigProblems,
} from "./server-config";

const saved = { ...process.env };

afterEach(() => {
	process.env = { ...saved };
	resetServerConfigCache();
});

describe("serverConfig", () => {
	it("reads the environment once and caches it", () => {
		process.env.BRAND_NAME = "Acme";
		resetServerConfigCache();
		expect(serverConfig().brand.name).toBe("Acme");

		process.env.BRAND_NAME = "Changed";
		expect(serverConfig().brand.name).toBe("Acme");
	});

	it("falls back to defaults a developer can boot with", () => {
		resetServerConfigCache();
		const config = serverConfig();
		expect(config.realtime.backendConnectionPath).toBe("/v1/realtime/connection");
		expect(config.realtime.handshakeTimeoutMs).toBe(10_000);
	});

	it("splits comma-separated lists and drops the blanks", () => {
		process.env.REALTIME_ALLOWED_ORIGINS = "https://a.example, ,https://b.example";
		resetServerConfigCache();
		expect(serverConfig().realtime.allowedOrigins).toEqual([
			"https://a.example",
			"https://b.example",
		]);
	});
});

describe("serverConfigProblems", () => {
	it("finds nothing wrong with a development default", () => {
		resetServerConfigCache();
		expect(serverConfigProblems()).toEqual([]);
	});

	it("reports a malformed value instead of throwing", () => {
		process.env.BRAND_LOGO_URL = "not-a-url";
		resetServerConfigCache();
		const problems = serverConfigProblems();
		expect(problems.length).toBeGreaterThan(0);
		expect(problems.some((problem) => problem.variable.includes("logoUrl"))).toBe(true);
	});

	it("lists every missing production variable in one pass", () => {
		process.env.NODE_ENV = "production";
		for (const key of [
			"BETTER_AUTH_URL",
			"BETTER_AUTH_SECRET",
			"DATABASE_URL",
			"BACKEND_API_URL",
			"BACKEND_JWT_AUDIENCE",
			"SCIM_CREDENTIAL_HASH_SECRET",
			"RESEND_API_KEY",
			"RESEND_FROM",
		]) {
			delete process.env[key];
		}
		resetServerConfigCache();

		const problems = serverConfigProblems();
		expect(problems).toHaveLength(8);
		expect(() => assertServerConfig()).toThrow(/BETTER_AUTH_SECRET is required in production/);
	});
});
