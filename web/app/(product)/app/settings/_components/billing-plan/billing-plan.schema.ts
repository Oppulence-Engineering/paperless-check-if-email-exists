import { z } from "zod";

import { PlanSchema } from "@/lib/entitlements/entitlements.schema";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Billing plan.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `billing-plan.lit.ts`.
 */
export const BillingPlanPropsSchema = z.object({
	/** The plan the workspace is on now. */
	plan: PlanSchema,
	/** False when this deployment has no Stripe keys. */
	billingEnabled: z.boolean(),
	/** Owners and admins can buy a plan and open the billing portal. */
	canManage: z.boolean(),
	pending: z.boolean().optional(),
	error: z.string().optional(),
	/** The paid plan picked on the pricing page, shown first and marked. */
	chosen: z.enum(["starter", "professional"]).optional(),
});

export type BillingPlanPropsFields = z.infer<typeof BillingPlanPropsSchema>;
