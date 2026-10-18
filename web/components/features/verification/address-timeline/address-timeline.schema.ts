import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Address timeline.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `address-timeline.lit.ts`.
 */
export const AddressTimelinePropsSchema = z.object({
	email: z.string().min(1),
});

export type AddressTimelinePropsFields = z.infer<typeof AddressTimelinePropsSchema>;
