import { z } from "zod";

/*
 * Runtime contracts for list responses that backend/openapi.json types as `{}`.
 * The shapes follow the Rust handlers in backend/src/http/v1/lists. They
 * accept extra fields, so a new backend field does not break the page.
 */

/** The list states that can still change (`list_status` in the migrations). */
export const ACTIVE_LIST_STATES = ["uploading", "processing"];

/** GET /v1/lists/{list_id}/quality (lists/quality.rs). */
export const ListQualitySchema = z.looseObject({
	total_rows: z.number(),
	processed: z.number(),
	avg_score: z.number(),
	quality_grade: z.string(),
	safe_to_send_count: z.number(),
	safe_to_send_pct: z.number(),
	risk_breakdown: z.record(z.string(), z.number()),
});

/** GET and POST /v1/lists/{list_id}/remediation-plan (lists/remediation.rs). */
export const RemediationPlanSchema = z.looseObject({
	id: z.number(),
	status: z.string(),
	options: z.record(z.string(), z.unknown()),
	summary_counts: z.record(z.string(), z.number()),
	created_at: z.string(),
	preview_rows: z.array(
		z.looseObject({
			id: z.number(),
			row_index: z.number(),
			classification: z.string(),
			rule_id: z.string(),
			confidence: z.string(),
			original_email: z.string(),
			effective_email: z.string(),
			reasons: z.array(z.looseObject({ code: z.string(), message: z.string().nullish() })),
		}),
	),
});

/** POST /v1/lists/{list_id}/remediation-exports (lists/remediation.rs). */
export const RemediationExportSchema = z.looseObject({ id: z.number() });
