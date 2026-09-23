import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Verification settings.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `verification-settings.lit.ts`.
 */
export const VerificationSettingsPropsSchema = z.object({
	organizationId: z.string().min(1),
	organizationRole: z.string().min(1),
	scope: z.enum(["all", "verification", "usage", "webhook"]).optional(),
});

export type VerificationSettingsPropsFields = z.infer<typeof VerificationSettingsPropsSchema>;
