import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const hero = readFileSync(
	new URL("../app/(marketing)/sim-landing/hero.tsx", import.meta.url),
	"utf8",
);
const navigation = readFileSync(
	new URL("../app/(marketing)/sim-landing/top-bar.tsx", import.meta.url),
	"utf8",
);
const config = readFileSync(new URL("../next.config.ts", import.meta.url), "utf8");

describe("public conversion", () => {
	it("shows an email verification offer with self-service entry points", () => {
		expect(hero).toContain("Check email addresses");
		expect(hero).toContain("before you send.");
		expect(navigation).toContain('href="/sign-up"');
		expect(navigation).toContain('href="/sign-in"');
	});

	it("keeps browser API requests on the same origin", () => {
		expect(config).toContain("connect-src 'self'");
		expect(config).not.toContain("api.workos.com");
	});
});
