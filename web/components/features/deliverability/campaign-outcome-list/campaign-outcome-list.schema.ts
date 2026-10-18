import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Campaign outcome list.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `campaign-outcome-list.lit.ts`.
 */
export const CampaignOutcomeListPropsSchema = z.object({});

export type CampaignOutcomeListPropsFields = z.infer<typeof CampaignOutcomeListPropsSchema>;
