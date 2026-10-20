import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Research panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `research-panel.lit.ts`.
 */
export const ResearchPanelPropsSchema = z.object({
	initialSessionId: z
		.string()
		.regex(/^[1-9][0-9]*$/)
		.optional(),
});

export type ResearchPanelPropsFields = z.infer<typeof ResearchPanelPropsSchema>;
