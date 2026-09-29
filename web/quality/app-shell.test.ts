import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { baseUrl, createMetadata } from "@/lib/metadata";

describe("web application shell", () => {
	it("uses Check If Email Exists metadata and social cards", () => {
		const metadata = createMetadata({ title: "Test page", description: "A test description" });
		expect(metadata.metadataBase).toEqual(baseUrl);
		expect(metadata.openGraph?.siteName).toBe("Check If Email Exists");
		expect(metadata.twitter).toMatchObject({ card: "summary_large_image" });
		expect(metadata.icons).toMatchObject({ icon: [{ url: "/check-email-logo.svg" }] });
	});

	it("keeps the template's providers and development tools", () => {
		const layout = readFileSync(new URL("../app/layout.tsx", import.meta.url), "utf8");
		const providers = readFileSync(
			new URL("../components/providers/app-providers.tsx", import.meta.url),
			"utf8",
		);
		expect(layout).toContain("AppProviders");
		expect(layout).toContain("react-scan");
		expect(layout).toContain("react-grab");
		expect(providers).toContain("ThemeProvider");
		expect(providers).toContain("Toaster");
	});

	it("protects product pages at the server layout", () => {
		const layout = readFileSync(
			new URL("../app/(product)/app/layout.tsx", import.meta.url),
			"utf8",
		);
		expect(layout).toMatch(/requireSession\s*\(/);
	});

	it("keeps the Sim presentation and Fumadocs content pipeline", () => {
		const blog = readFileSync(new URL("../app/(marketing)/blog/page.tsx", import.meta.url), "utf8");
		const config = readFileSync(new URL("../next.config.ts", import.meta.url), "utf8");
		expect(blog).toContain("SimBlogIndexPage");
		expect(blog).toContain("publishedBlogPosts");
		expect(config).toContain("createMDX");
	});
});
