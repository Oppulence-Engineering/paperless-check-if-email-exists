import { describe, expect, it } from "vitest";

import {
	can,
	denial,
	entitlementsFor,
	planFromValue,
	seatLimit,
	seatsAvailable,
} from "./entitlements";

const pro = entitlementsFor({ plan: "pro", status: "active", memberCount: 2 });
const free = entitlementsFor({ plan: "free", status: "active", memberCount: 1 });

describe("entitlementsFor", () => {
	it("reads whatever the backend sent without trusting it", () => {
		expect(planFromValue("PRO")).toBe("pro");
		expect(planFromValue("gold")).toBe("free");
		expect(planFromValue(undefined)).toBe("free");
		expect(planFromValue(7)).toBe("free");
	});

	it("treats a stated payment failure as inactive and silence as active", () => {
		expect(entitlementsFor({ plan: "pro", status: "past_due" }).active).toBe(false);
		expect(entitlementsFor({ plan: "pro", status: "canceled" }).active).toBe(false);
		expect(entitlementsFor({ plan: "pro", status: null }).active).toBe(true);
	});

	it("carries the seat allowance of the plan", () => {
		expect(seatLimit("free")).toBe(3);
		expect(seatLimit("enterprise")).toBeNull();
		expect(entitlementsFor({ plan: "starter", memberCount: 4 }).seats).toEqual({
			used: 4,
			limit: 10,
		});
	});
});

describe("can", () => {
	it("lets a plan do everything the plans below it can", () => {
		expect(can(pro, "connect_sources")).toBe(true);
		expect(can(pro, "execute_actions")).toBe(true);
		expect(can(pro, "api_access")).toBe(true);
	});

	it("stops a plan short of what it did not buy", () => {
		expect(can(free, "execute_actions")).toBe(false);
		expect(can(pro, "enterprise_identity")).toBe(false);
	});

	it("revokes every feature while the subscription is inactive", () => {
		const lapsed = entitlementsFor({ plan: "enterprise", status: "unpaid" });
		expect(can(lapsed, "connect_sources")).toBe(false);
	});
});

describe("denial", () => {
	it("names the plan that would grant the feature", () => {
		expect(denial(free, "execute_actions")).toMatchObject({
			reason: "plan",
			requiredPlan: "starter",
		});
		expect(denial(free, "api_access")?.requiredPlan).toBe("pro");
	});

	it("separates a lapsed subscription from a plan that is simply too small", () => {
		const lapsed = entitlementsFor({ plan: "pro", status: "canceled" });
		expect(denial(lapsed, "api_access")?.reason).toBe("inactive");
	});

	it("stops an invitation when the seats are gone", () => {
		const full = entitlementsFor({ plan: "free", status: "active", memberCount: 3 });
		expect(seatsAvailable(full)).toBe(false);
		expect(denial(full, "invite_members")).toMatchObject({ reason: "seats" });
		expect(denial(full, "connect_sources")).toBeNull();
	});

	it("never caps seats on a plan that does not cap them", () => {
		const enterprise = entitlementsFor({ plan: "enterprise", memberCount: 5_000 });
		expect(seatsAvailable(enterprise)).toBe(true);
		expect(denial(enterprise, "invite_members")).toBeNull();
	});

	it("returns null when nothing is wrong", () => {
		expect(denial(pro, "execute_actions")).toBeNull();
	});
});
