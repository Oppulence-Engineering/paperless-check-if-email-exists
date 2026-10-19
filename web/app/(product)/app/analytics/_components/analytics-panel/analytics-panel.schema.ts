import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Analytics panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `analytics-panel.lit.ts`.
 */
export const AnalyticsPanelPropsSchema = z.object({});

export type AnalyticsPanelPropsFields = z.infer<typeof AnalyticsPanelPropsSchema>;
