import { z } from "zod";

/*
 * Runtime contract for the decision engine output. backend/openapi.json types
 * `recommendation` and `policy_evaluation` as empty objects, so these schemas
 * follow backend/src/decision/types.rs. They accept extra fields, so a new
 * backend field does not break the page.
 */

export const DecisionReasonSchema = z.looseObject({
	code: z.string(),
	severity: z.string(),
	message: z.string(),
});

export const RecommendationSchema = z.looseObject({
	action: z.string(),
	policy_mode: z.string(),
	policy_profile_key: z.string().nullish(),
	confidence: z.string(),
	priority: z.string(),
	summary: z.string(),
	reasons: z.array(DecisionReasonSchema),
	suggested_email: z.string().nullish(),
	engine_version: z.string(),
	evaluated_at: z.string(),
});

export const PolicyEvaluationSchema = z.looseObject({
	mode: z.string(),
	policy_profile_key: z.string().nullish(),
	decision: z.string(),
	reasons: z.array(DecisionReasonSchema),
	evaluated_at: z.string(),
	result_age_days: z.number().nullish(),
	engine_version: z.string(),
});

export type Recommendation = z.infer<typeof RecommendationSchema>;
export type PolicyEvaluation = z.infer<typeof PolicyEvaluationSchema>;

/** The recommendation, or null when the value is missing or has another shape. */
export function parseRecommendation(value: unknown): Recommendation | null {
	const parsed = RecommendationSchema.safeParse(value);
	return parsed.success ? parsed.data : null;
}

/** The policy evaluation, or null when the value is missing or has another shape. */
export function parsePolicyEvaluation(value: unknown): PolicyEvaluation | null {
	const parsed = PolicyEvaluationSchema.safeParse(value);
	return parsed.success ? parsed.data : null;
}
