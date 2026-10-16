import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

import { describe, expect, it } from "vitest";

import { pricingPlans } from "@/app/(marketing)/marketing-data";

const blogDir = join(process.cwd(), "content/blog");
const customerDir = join(process.cwd(), "content/customers");

function mdxFiles(dir: string) {
	return readdirSync(dir).filter((file) => file.endsWith(".mdx"));
}

describe("editorial content collections", () => {
	it("ships published blog notes without fabricated proof", () => {
		const files = mdxFiles(blogDir);
		expect(files.length).toBeGreaterThanOrEqual(6);
		const joined = files.map((file) => readFileSync(join(blogDir, file), "utf8")).join("\n");
		expect(joined).not.toMatch(/trusted by \d+|as featured in|SOC 2|HIPAA/i);
		expect(joined).toMatch(/category: product/);
		expect(joined).toMatch(/category: workflow/);
		expect(joined).toMatch(/category: security/);
		expect(joined).toMatch(/category: install/);
	});

	it("keeps customer stories unpublished until a real write-up exists", () => {
		const visible = mdxFiles(customerDir).filter((file) => !file.startsWith("_"));
		expect(visible.length).toBeGreaterThan(0);
		const published = visible.filter((file) =>
			/^published:\s*true\s*$/m.test(readFileSync(join(customerDir, file), "utf8")),
		);
		expect(published).toEqual([]);
		expect(readFileSync(join(customerDir, "example.mdx"), "utf8")).toMatch(/published:\s*false/);
	});

	it("keeps the published pricing names and amounts", () => {
		expect(pricingPlans.map((plan) => `${plan.name}:${plan.price}${plan.period}`)).toEqual([
			"Free:$0/month",
			"Starter:$49/month",
			"Professional:$149/month",
			"Enterprise:Custom",
		]);
		// Onboarding is self-serve, but no checkout exists yet: paid plans start
		// free through sign-up and must not promise a purchase.
		for (const plan of pricingPlans.slice(1, 3)) {
			expect(plan.ctaHref).toBe("/sign-up");
			expect(plan.ctaLabel).toBe("Start free");
		}
		expect(pricingPlans[3].ctaHref).toBe("/contact");
	});
});
