import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Check panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `check-panel.lit.ts`.
 */
export const CheckPanelPropsSchema = z.object({});

export type CheckPanelPropsFields = z.infer<typeof CheckPanelPropsSchema>;
