import {
	EntitlementsSchema,
	PlanSchema,
	type Entitlements,
	type EntitlementDenial,
	type Feature,
	type Plan,
} from "./entitlements.schema";

/**
 * @oppulence-gen kind=lib
 * entitlements is a server-safe billing helper.
 *
 * One place that answers "is this workspace allowed to do that". The matrix
 * below is the policy a deployment edits; everything else reads it, so the UI
 * and the BFF can never disagree about what a plan buys.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `entitlements.lit.ts`.
 */

/** Ordered weakest to strongest; a plan inherits everything below it. */
const PLAN_ORDER: Plan[] = ["free", "starter", "pro", "enterprise"];

/** The lowest plan that buys each feature. This is the policy. */
const REQUIRES: Record<Feature, Plan> = {
	execute_actions: "starter",
	connect_sources: "free",
	invite_members: "free",
	enterprise_identity: "enterprise",
	api_access: "pro",
	audit_export: "pro",
};

/** Seats each plan includes. null means uncapped. */
const SEATS: Record<Plan, number | null> = {
	free: 3,
	starter: 10,
	pro: 50,
	enterprise: null,
};

function rank(plan: Plan): number {
	return PLAN_ORDER.indexOf(plan);
}

/** Reads a plan name from a backend that may return anything, or nothing. */
export function planFromValue(value: unknown): Plan {
	return PlanSchema.catch("free").parse(typeof value === "string" ? value.toLowerCase() : "free");
}

/** Seat allowance for a plan, so the invite path can check before it sends. */
export function seatLimit(plan: Plan): number | null {
	return SEATS[plan];
}

/**
 * Builds entitlements from what the session already carries. No extra request:
 * the plan travels with the viewer, and member count comes from the workspace
 * the caller already loaded.
 */
export function entitlementsFor(input: {
	plan: unknown;
	status?: string | null;
	memberCount?: number;
}): Entitlements {
	const plan = planFromValue(input.plan);
	const status = (input.status ?? "").toLowerCase();
	return EntitlementsSchema.parse({
		plan,
		// An unknown status is treated as active; only a stated failure is not.
		active: !["canceled", "cancelled", "past_due", "unpaid", "incomplete"].includes(status),
		seats: { used: input.memberCount ?? 0, limit: seatLimit(plan) },
	});
}

/** The question every caller asks. */
export function can(entitlements: Entitlements, feature: Feature): boolean {
	return denial(entitlements, feature) === null;
}

/** The same question, with the reason a person can act on. */
export function denial(entitlements: Entitlements, feature: Feature): EntitlementDenial | null {
	const requiredPlan = REQUIRES[feature];

	if (!entitlements.active) {
		return {
			feature,
			reason: "inactive",
			requiredPlan,
			message: "This workspace's subscription is not active.",
		};
	}

	if (rank(entitlements.plan) < rank(requiredPlan)) {
		return {
			feature,
			reason: "plan",
			requiredPlan,
			message: `This workspace is on the ${entitlements.plan} plan; ${requiredPlan} includes it.`,
		};
	}

	if (feature === "invite_members" && !seatsAvailable(entitlements)) {
		return {
			feature,
			reason: "seats",
			requiredPlan,
			message: `All ${String(entitlements.seats.limit)} seats on the ${entitlements.plan} plan are taken.`,
		};
	}

	return null;
}

/** True while the workspace can still add a member. */
export function seatsAvailable(entitlements: Entitlements): boolean {
	const { used, limit } = entitlements.seats;
	return limit === null || used < limit;
}
