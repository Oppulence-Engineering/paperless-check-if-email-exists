import { afterEach, describe, expect, it, vi } from "vitest";
import { NextRequest } from "next/server";

import { config, proxy } from "@/proxy";

describe("HTTPS proxy", () => {
	afterEach(() => vi.unstubAllEnvs());

	it("permanently redirects forwarded HTTP requests without losing the path or query", () => {
		const response = proxy(
			new NextRequest("http://oppulence.io/pricing?plan=team", {
				headers: { "x-forwarded-proto": "http" },
			}),
		);

		expect(response.status).toBe(308);
		expect(response.headers.get("location")).toBe("https://oppulence.io/pricing?plan=team");
	});

	it("limits the global matcher to requests explicitly forwarded over HTTP", () => {
		expect(config.matcher).toContainEqual(
			expect.objectContaining({
				has: [expect.objectContaining({ key: "x-forwarded-proto", value: "http" })],
			}),
		);
	});

	it("keeps the explicitly flagged loopback E2E server on HTTP", () => {
		vi.stubEnv("AUTH_E2E_MODE", "1");
		const response = proxy(
			new NextRequest("http://127.0.0.1:4317/", {
				headers: { "x-forwarded-proto": "http" },
			}),
		);

		expect(response.status).toBe(200);
		expect(response.headers.get("location")).toBeNull();
	});

	it("redirects from the container hostname to the configured public origin", () => {
		vi.stubEnv("AUTH_E2E_MODE", "1");
		vi.stubEnv("BETTER_AUTH_URL", "http://localhost:4300");
		const response = proxy(
			new NextRequest("http://container:3000/app/check", {
				headers: { "x-forwarded-proto": "http" },
			}),
		);

		expect(response.status).toBe(307);
		expect(response.headers.get("location")).toBe(
			"http://localhost:4300/sign-in?return_to=%2Fapp%2Fcheck",
		);
	});
});
