import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Entitlements.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `entitlements.lit.ts`.
 */

/** Plans this deployment sells. Rename them; the matrix below follows. */
export const PlanSchema = z.enum(["free", "starter", "pro", "enterprise"]);

export type Plan = z.infer<typeof PlanSchema>;

/**
 * A feature is a capability a plan buys, never a screen. Gate on "execute an
 * action", not on "the actions page", so the same name can be enforced in the
 * BFF and reused in the UI.
 */
export const FeatureSchema = z.enum([
	"execute_actions",
	"connect_sources",
	"invite_members",
	"enterprise_identity",
	"api_access",
	"audit_export",
]);

export type Feature = z.infer<typeof FeatureSchema>;

const SeatUsageSchema = z.object({
	used: z.number().int().nonnegative(),
	/** null means the plan does not cap seats. */
	limit: z.number().int().positive().nullable(),
});

type SeatUsage = z.infer<typeof SeatUsageSchema>;

export const EntitlementsSchema = z.object({
	plan: PlanSchema,
	/** A lapsed subscription keeps the plan name but loses the features. */
	active: z.boolean(),
	seats: SeatUsageSchema,
});

export type Entitlements = z.infer<typeof EntitlementsSchema>;

const EntitlementDenialSchema = z.object({
	feature: FeatureSchema,
	reason: z.enum(["plan", "inactive", "seats"]),
	requiredPlan: PlanSchema,
	message: z.string().min(1),
});

export type EntitlementDenial = z.infer<typeof EntitlementDenialSchema>;
