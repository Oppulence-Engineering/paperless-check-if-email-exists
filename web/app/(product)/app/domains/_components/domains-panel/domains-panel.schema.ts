import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Domains panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `domains-panel.lit.ts`.
 */
export const DomainsPanelPropsSchema = z.object({});

export type DomainsPanelPropsFields = z.infer<typeof DomainsPanelPropsSchema>;
