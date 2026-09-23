import { z } from "zod";

import { WorkspaceSummarySchema } from "@/lib/auth/schemas";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Workspace switcher.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `workspace-switcher.lit.ts`.
 */
export const WorkspaceSwitcherPropsSchema = z.object({
	activeWorkspaceId: z.string().min(1),
	planLabel: z.string().min(1).nullable().optional(),
	workspaces: z.array(WorkspaceSummarySchema),
});

export type WorkspaceSwitcherPropsFields = z.infer<typeof WorkspaceSwitcherPropsSchema>;
