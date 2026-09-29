import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Api panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `api-panel.lit.ts`.
 */
export const ApiPanelPropsSchema = z.object({});

export type ApiPanelPropsFields = z.infer<typeof ApiPanelPropsSchema>;
