import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Integrations panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `integrations-panel.lit.ts`.
 */
export const IntegrationsPanelPropsSchema = z.object({});

export type IntegrationsPanelPropsFields = z.infer<typeof IntegrationsPanelPropsSchema>;
