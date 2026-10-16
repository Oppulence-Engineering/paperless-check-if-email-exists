import { readFileSync } from "node:fs";

import { describe, expect, it } from "vitest";

import { pricingPlans } from "@/app/(marketing)/marketing-data";
import { PRICING_COMPARISON_SECTIONS } from "@/app/(marketing)/sim-landing/subpages/pricing-comparison-data";
import { MONTHLY_CHECKS, MONTHLY_PRICE_USD } from "@/lib/billing/plans";
import { seatLimit } from "@/lib/entitlements/entitlements";

const PLANS = ["free", "starter", "professional"] as const;
const backendJwt = readFileSync(
	new URL("../../backend/src/tenant/jwt.rs", import.meta.url),
	"utf8",
);

/** Rust writes 4000 as 4_000. */
function rustNumber(value: number): string {
	return value >= 1_000
		? `${String(Math.floor(value / 1_000))}_${String(value % 1_000).padStart(3, "0")}`
		: String(value);
}

describe("plan facts", () => {
	it("sell the prices and allowances the pricing page shows", () => {
		const page = Object.fromEntries(pricingPlans.map((plan) => [plan.name.toLowerCase(), plan]));
		expect(page.starter.price).toBe(`$${String(MONTHLY_PRICE_USD.starter)}`);
		expect(page.professional.price).toBe(`$${String(MONTHLY_PRICE_USD.professional)}`);
		for (const plan of PLANS) {
			expect(page[plan].note).toBe(
				`${(MONTHLY_CHECKS[plan] ?? 0).toLocaleString("en-US")} checks a month`,
			);
		}
	});

	it("give each plan the seats the pricing table shows", () => {
		const seats = PRICING_COMPARISON_SECTIONS.flatMap((section) => section.rows).find(
			(row) => row.label === "Team seats",
		);
		expect(seats?.values).toEqual([
			String(seatLimit("free")),
			String(seatLimit("starter")),
			String(seatLimit("professional")),
			"Unlimited",
		]);
		expect(seatLimit("enterprise")).toBeNull();
	});

	it("are enforced with the same allowance by the backend", () => {
		for (const plan of PLANS) {
			const checks = MONTHLY_CHECKS[plan] ?? 0;
			expect(backendJwt).toContain(`"${plan}" => Some(("${plan}", Some(${rustNumber(checks)})))`);
		}
	});
});
