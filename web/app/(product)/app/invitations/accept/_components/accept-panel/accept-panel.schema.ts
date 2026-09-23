import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Accept panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `accept-panel.lit.ts`.
 */
export const AcceptPanelPropsSchema = z.object({
	invitationId: z.string().min(1),
	returnTo: z.string().startsWith("/"),
});

export type AcceptPanelPropsFields = z.infer<typeof AcceptPanelPropsSchema>;
