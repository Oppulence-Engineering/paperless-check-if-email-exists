import { describe, expect, it } from "vitest";

import { getPlatformPage, platformPages, productLinks } from "@/app/(marketing)/marketing-data";

describe("Oppulence product surfaces", () => {
	it("publishes one page per way of running Oppulence, linked from the product nav", () => {
		expect(platformPages.map((page) => page.slug)).toEqual(["web", "desktop", "voice-app"]);

		for (const page of platformPages) {
			expect(productLinks).toContainEqual(expect.objectContaining({ href: `/${page.slug}` }));
		}
	});

	it("keeps installer links off the web product surfaces", () => {
		for (const page of platformPages) expect(getPlatformPage(page.slug)?.download).toBe(false);
	});

	it("gives every product page the copy the layout depends on", () => {
		for (const page of platformPages) {
			expect(["Web workspace", "Self-hosting", "Bulk lists"]).toContain(page.name);
			expect(page.title.length).toBeGreaterThan(0);
			expect(page.lede.length).toBeGreaterThan(0);
			expect(page.summary.length).toBeGreaterThan(0);
			expect(page.sections.length).toBeGreaterThanOrEqual(2);
			expect(page.specs.length).toBeGreaterThanOrEqual(3);

			for (const section of page.sections) {
				expect(section.bullets.length).toBeGreaterThan(0);
				expect(section.screenshot).toBe("/marketing/email-check-preview.svg");
				expect(section.alt.length).toBeGreaterThan(0);
			}
		}
	});
});

describe("product navigation", () => {
	it("keeps every product reachable from the footer product column", () => {
		// The footer renders productLinks.slice(0, 6), so the three surfaces have
		// to stay inside that window or they silently drop out of the footer.
		const footerLinks = productLinks.slice(0, 6).map((link) => link.href);

		for (const page of platformPages) {
			expect(footerLinks).toContain(`/${page.slug}`);
		}
	});
});
