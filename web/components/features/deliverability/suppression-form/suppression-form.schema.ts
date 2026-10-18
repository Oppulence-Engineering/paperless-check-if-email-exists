import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Suppression form.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `suppression-form.lit.ts`.
 */
export const SuppressionFormPropsSchema = z.object({
	/** `add` takes typed addresses. `import` takes a CSV file. */
	mode: z.enum(["add", "import"]),
});

export type SuppressionFormPropsFields = z.infer<typeof SuppressionFormPropsSchema>;
