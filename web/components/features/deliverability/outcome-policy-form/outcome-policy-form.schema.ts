import { z } from "zod";

import { V1GetOutcomePolicy200Response } from "@/lib/api/generated/zod/tenant/tenant";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Outcome policy form.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `outcome-policy-form.lit.ts`.
 *
 * Without `policy` the form creates a policy. With it, the form edits that policy.
 */
export const OutcomePolicyFormPropsSchema = z.object({
	policy: V1GetOutcomePolicy200Response.optional(),
});

export type OutcomePolicyFormPropsFields = z.infer<typeof OutcomePolicyFormPropsSchema>;
