import { afterEach, describe, expect, it, vi } from "vitest";

import { branding } from "./branding";

afterEach(() => vi.unstubAllEnvs());

describe("branding", () => {
	it("validates deployment branding", async () => {
		vi.stubEnv("BRAND_NAME", "Example");
		vi.stubEnv("BRAND_PRIMARY_COLOR", "not-a-color");
		await expect(branding()).rejects.toThrow();
	});
});
