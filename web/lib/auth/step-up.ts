import { StepUpInputSchema, type StepUpInput } from "./step-up.schema";

/**
 * @oppulence-gen kind=lib
 * stepUp is a server-safe auth helper.
 * Validates recent, purpose-bound identity verification grants.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `step-up.lit.ts`.
 */
const STEP_UP_MAX_AGE_MS = 15 * 60 * 1_000;

export function hasValidStepUp(input: StepUpInput): boolean {
	const value = StepUpInputSchema.parse(input);
	if (!value.verifiedAt || !value.method || !value.purpose) return false;
	if (value.purpose !== value.requiredPurpose || !value.allowedMethods.includes(value.method)) {
		return false;
	}
	const age = (value.now ?? Date.now()) - value.verifiedAt.getTime();
	return age >= 0 && age <= STEP_UP_MAX_AGE_MS;
}
