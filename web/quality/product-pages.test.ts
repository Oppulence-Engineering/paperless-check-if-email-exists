import { describe, expect, it } from "vitest";

import { getPlatformPage, platformPages } from "@/app/(marketing)/marketing-data";
import { footerGroups } from "@/app/(marketing)/site";

describe("product surfaces", () => {
	it("publishes the web workspace and the self-hosting page", () => {
		expect(platformPages.map((page) => page.slug)).toEqual(["web", "self-hosting"]);
	});

	it("keeps installer links off the product surfaces", () => {
		for (const page of platformPages) expect(getPlatformPage(page.slug)?.download).toBe(false);
	});

	it("gives every product page the copy the layout depends on", () => {
		for (const page of platformPages) {
			expect(page.title.length).toBeGreaterThan(0);
			expect(page.lede.length).toBeGreaterThan(0);
			expect(page.summary.length).toBeGreaterThan(0);
			expect(page.sections.length).toBeGreaterThanOrEqual(3);
			expect(page.specs.length).toBeGreaterThanOrEqual(2);
			expect(page.cta?.href.length).toBeGreaterThan(0);

			for (const section of page.sections) {
				expect(section.bullets.length).toBeGreaterThan(0);
				expect(section.screenshot).toBe("/marketing/email-check-preview.svg");
				expect(section.alt.length).toBeGreaterThan(0);
			}
		}
	});
});

describe("product navigation", () => {
	it("keeps every product page reachable from the footer", () => {
		const footerLinks = footerGroups.flatMap((group) => group.items.map((link) => link.href));

		for (const page of platformPages) {
			expect(footerLinks).toContain(`/${page.slug}`);
		}
	});
});
