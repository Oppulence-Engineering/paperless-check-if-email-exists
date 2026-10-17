import type { Plan } from "@/lib/entitlements/entitlements.schema";

/**
 * Monthly check allowance for each plan. The backend enforces the same numbers
 * when it reads the plan from the backend token (backend/src/tenant/jwt.rs).
 * null means the allowance is set by contract.
 */
export const MONTHLY_CHECKS: Record<Plan, number | null> = {
	free: 100,
	starter: 4_000,
	professional: 12_000,
	enterprise: null,
};

/** Monthly price in US dollars for the plans sold online. */
export const MONTHLY_PRICE_USD = { starter: 49, professional: 149 } as const;

export type PaidPlan = keyof typeof MONTHLY_PRICE_USD;

export const PAID_PLANS = Object.keys(MONTHLY_PRICE_USD) as PaidPlan[];

/** The plan card URL with a plan chosen, for example from the pricing page. */
function planChoicePath(plan: PaidPlan): string {
	return `/app/settings?settings=usage&plan=${plan}`;
}

/** Sign-up that returns to the plan card with the plan chosen. */
export function signUpForPlan(plan: PaidPlan): string {
	return `/sign-up?${new URLSearchParams({ return_to: planChoicePath(plan) }).toString()}`;
}
