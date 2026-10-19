import { z } from "zod";

import { WorkspaceSummarySchema } from "@/lib/auth/schemas";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Workspace url scope.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `workspace-url-scope.lit.ts`.
 */
export const WorkspaceUrlScopePropsSchema = z.object({
	/** Workspaces the signed-in user may enter. A link naming any other is ignored. */
	workspaces: z.array(WorkspaceSummarySchema),
	activeWorkspaceId: z.string().min(1),
});

export type WorkspaceUrlScopePropsFields = z.infer<typeof WorkspaceUrlScopePropsSchema>;
