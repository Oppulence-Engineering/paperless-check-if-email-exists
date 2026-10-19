import { z } from "zod";

import { V1GetPipelineRun200Response } from "@/lib/api/generated/zod/pipelines/pipelines";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Overview panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `overview-panel.lit.ts`.
 */
export const OverviewPanelPropsSchema = z.object({});

export type OverviewPanelPropsFields = z.infer<typeof OverviewPanelPropsSchema>;

/*
 * The contract has no usable 200 schema for these responses, so the panel keeps
 * the fields it reads here. Each shape comes from the backend handler:
 * - /v1/outcomes returns `{ outcomes, limit, offset }` (backend/src/http/v1/outcomes.rs),
 *   but openapi.json documents the campaign outcome shape with `total`.
 * - /v1/me/domains, /v1/me/api-keys, and /v1/bulk/{job_id} are `{}` in openapi.json.
 */
export const OutcomePageSchema = z.object({ outcomes: z.array(z.unknown()) });
export const TenantDomainsSchema = z.object({
	domains: z.array(z.object({ is_active: z.boolean(), is_verified: z.boolean() })),
});
export const ApiKeysSchema = z.object({ api_keys: z.array(z.unknown()) });
export const BulkProgressSchema = z.object({
	total_records: z.number(),
	total_processed: z.number(),
});

/*
 * A pipeline run `stats` object can have keys that the contract does not list
 * (for example `accepted_rows` on a push run), so the strict generated schema
 * can reject a real run. The panel reads only the status, with the generated enum.
 */
export const RunStatusSchema = z.object({ status: V1GetPipelineRun200Response.shape.status });
