import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for History panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `history-panel.lit.ts`.
 */
export const HistoryPanelPropsSchema = z.object({});

export type HistoryPanelPropsFields = z.infer<typeof HistoryPanelPropsSchema>;
