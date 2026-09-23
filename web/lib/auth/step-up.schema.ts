import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Step up.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `step-up.lit.ts`.
 */
const StepUpMethodSchema = z.enum(["email-otp", "passkey", "totp"]);
const StepUpPurposeSchema = z.enum(["account-delete", "admin"]);

export const StepUpInputSchema = z.object({
	verifiedAt: z.coerce.date().nullable().optional(),
	method: StepUpMethodSchema.nullable().optional(),
	purpose: StepUpPurposeSchema.nullable().optional(),
	requiredPurpose: StepUpPurposeSchema,
	allowedMethods: z.array(StepUpMethodSchema).min(1),
	now: z.number().int().nonnegative().optional(),
});

export type StepUpInput = z.infer<typeof StepUpInputSchema>;
