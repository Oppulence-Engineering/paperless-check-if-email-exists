import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Suppressions panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `suppressions-panel.lit.ts`.
 */
export const SuppressionsPanelPropsSchema = z.object({});

export type SuppressionsPanelPropsFields = z.infer<typeof SuppressionsPanelPropsSchema>;
