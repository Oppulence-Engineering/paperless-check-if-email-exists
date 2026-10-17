import { readdirSync, readFileSync } from "node:fs";
import { join, relative, sep } from "node:path";

import { describe, expect, it } from "vitest";

import { allCapabilityPages, homepageFaqs, indexCopy } from "@/app/(marketing)/catalog";
import { platformPages, pricingPlans, productPage } from "@/app/(marketing)/marketing-data";
import { footerGroups, headerNav, headerUtilityLinks } from "@/app/(marketing)/site";
import { marketingRedirects } from "@/config/marketing-redirects";

const appDir = join(process.cwd(), "app");

/** Static routes from `page.tsx` files, with route groups removed. */
function staticRoutes(dir = appDir): string[] {
	return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
		const full = join(dir, entry.name);
		if (entry.isDirectory()) return entry.name.startsWith("[") ? [] : staticRoutes(full);
		if (entry.name !== "page.tsx") return [];
		const route = relative(appDir, dir)
			.split(sep)
			.filter((segment) => segment && !segment.startsWith("("))
			.join("/");
		return [`/${route}`];
	});
}

/** Published Fumadocs posts. Drafts are not public routes. */
function blogRoutes(): string[] {
	const dir = join(process.cwd(), "content/blog");
	return readdirSync(dir)
		.filter((file) => file.endsWith(".mdx") && !file.startsWith("_"))
		.filter((file) => !/^draft:\s*true\s*$/m.test(readFileSync(join(dir, file), "utf8")))
		.map((file) => `/blog/${file.replace(/\.mdx$/, "")}`);
}

const liveRoutes = new Set([
	...staticRoutes(),
	...allCapabilityPages.map((page) => page.path),
	...blogRoutes(),
]);

const redirectSources = new Set(marketingRedirects.map((redirect) => redirect.source));

const internal = (href: string) => href.startsWith("/");

const siteLinks = [
	...headerNav.flatMap((group) => [group.href, ...group.items.map((item) => item.href)]),
	...headerUtilityLinks.map((link) => link.href),
	...footerGroups.flatMap((group) => group.items.map((item) => item.href)),
	...allCapabilityPages.flatMap((page) => [
		...page.related.map((link) => link.href),
		...(page.integrations ?? []).map((link) => link.href),
	]),
	...pricingPlans.map((plan) => plan.ctaHref),
	...platformPages.flatMap((page) => (page.cta ? [page.cta.href] : [])),
].filter((href): href is string => typeof href === "string" && internal(href));

describe("marketing links", () => {
	it("points every navigation and catalog link at a live route", () => {
		expect(siteLinks.length).toBeGreaterThan(30);
		for (const href of siteLinks) expect(liveRoutes, href).toContain(href.split("?")[0]);
	});

	it("never links to a URL that only redirects", () => {
		for (const href of siteLinks) expect(redirectSources.has(href), href).toBe(false);
	});
});

describe("marketing redirects", () => {
	it("are permanent and land on live routes without chains", () => {
		for (const redirect of marketingRedirects) {
			expect(redirect.permanent).toBe(true);
			expect(liveRoutes, redirect.source).toContain(redirect.destination);
			expect(redirectSources.has(redirect.destination), redirect.destination).toBe(false);
		}
	});

	it("never hide a live page", () => {
		for (const source of redirectSources) expect(liveRoutes.has(source), source).toBe(false);
	});
});

describe("public copy", () => {
	const publicCopy = JSON.stringify([
		headerNav,
		footerGroups,
		allCapabilityPages,
		homepageFaqs,
		indexCopy,
		productPage,
		pricingPlans,
		platformPages,
	]);

	it("does not offer self-hosting", () => {
		expect(publicCopy).not.toMatch(/self[- ]?host|own servers|open[- ]source|AGPL/i);
	});

	it("does not describe the template's former product", () => {
		expect(publicCopy).not.toMatch(
			/commitment|relationship memory|help center|knowledge base|voice app|desktop app/i,
		);
	});
});
