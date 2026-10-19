import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Lists panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `lists-panel.lit.ts`.
 */
export const ListsPanelPropsSchema = z.object({});

export type ListsPanelPropsFields = z.infer<typeof ListsPanelPropsSchema>;
