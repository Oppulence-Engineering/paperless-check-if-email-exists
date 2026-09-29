import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

const read = (path: string) => readFileSync(new URL(path, import.meta.url), "utf8");

describe("public site", () => {
	it("lists Fumadocs notes in the sitemap and excludes protected routes", () => {
		const sitemap = read("../app/sitemap.ts");
		const robots = read("../app/robots.ts");
		expect(sitemap).toContain("publishedBlogPosts()");
		expect(sitemap).toContain("/blog");
		expect(robots).toContain('disallow: ["/app/", "/api/"]');
	});

	it("keeps old Rowboat posts as drafts and publishes an email verification note", () => {
		const old = read("../content/blog/what-a-commitment-ledger-is.mdx");
		const current = read("../content/blog/how-email-verification-works.mdx");
		expect(old).toContain("draft: true");
		expect(current).toContain("title: How an email check works");
		expect(current).not.toContain("draft: true");
	});

	it("loads the Sim blog style and Fumadocs MDX configuration", () => {
		const layout = read("../app/(marketing)/layout.tsx");
		const index = read("../app/(marketing)/blog/page.tsx");
		const config = read("../config/fumadocs/source.config.ts");
		expect(layout).toContain("MarketingLayoutClient");
		expect(index).toContain("SimBlogIndexPage");
		expect(config).toContain('dir: "content/blog"');
	});
});
