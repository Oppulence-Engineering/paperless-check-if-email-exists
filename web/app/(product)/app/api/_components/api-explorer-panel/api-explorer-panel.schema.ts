import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Api explorer panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `api-explorer-panel.lit.ts`.
 */
export const ApiExplorerPanelPropsSchema = z.object({});

export type ApiExplorerPanelPropsFields = z.infer<typeof ApiExplorerPanelPropsSchema>;
