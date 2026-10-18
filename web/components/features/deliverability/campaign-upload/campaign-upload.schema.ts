import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Campaign upload.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `campaign-upload.lit.ts`.
 */
export const CampaignUploadPropsSchema = z.object({});

export type CampaignUploadPropsFields = z.infer<typeof CampaignUploadPropsSchema>;
