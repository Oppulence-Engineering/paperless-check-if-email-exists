import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Integration stack.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `integration-stack.lit.ts`.
 */
export const IntegrationStackPropsSchema = z.object({});

export type IntegrationStackPropsFields = z.infer<typeof IntegrationStackPropsSchema>;
