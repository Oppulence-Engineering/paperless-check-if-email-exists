import type { z } from "zod";

import type { V1CheckEmail200Response } from "@/lib/api/generated/zod/v1/v1";
import { parsePolicyEvaluation, parseRecommendation } from "@/lib/api/schemas/decision";

/** One email check result. Single checks, bulk rows, and finder candidates share it. */
export type CheckResult = z.infer<typeof V1CheckEmail200Response>;

type CoreError = { error: { type: string; message?: unknown } };

function isCoreError(value: unknown): value is CoreError {
	return typeof value === "object" && value !== null && "error" in value;
}

/** The error type and a readable message. Some errors send an object as the message. */
function errorOf({ error }: CoreError) {
	const message =
		typeof error.message === "string" ? error.message : (JSON.stringify(error.message) ?? "");
	return { type: error.type, message };
}

/** MX details, or the error the check returned instead. */
export function mxDetails(result: CheckResult) {
	return isCoreError(result.mx) ? { error: errorOf(result.mx) } : { details: result.mx };
}

/** SMTP details, or the error the check returned instead. */
export function smtpDetails(result: CheckResult) {
	return isCoreError(result.smtp) ? { error: errorOf(result.smtp) } : { details: result.smtp };
}

/** Account details, or the error the check returned instead. */
export function miscDetails(result: CheckResult) {
	return isCoreError(result.misc) ? { error: errorOf(result.misc) } : { details: result.misc };
}

/** The decision engine output for this result, when the backend sent one. */
export function recommendationOf(result: CheckResult) {
	return parseRecommendation(result.recommendation);
}

/** The policy evaluation for this result, when the backend sent one. */
export function policyEvaluationOf(result: CheckResult) {
	return parsePolicyEvaluation(result.policy_evaluation);
}

/** The corrected address to offer, from the most specific source first. */
export function suggestedEmail(result: CheckResult): string | null {
	return (
		recommendationOf(result)?.suggested_email ??
		result.score.domain_suggestion ??
		result.syntax.suggestion ??
		null
	);
}

/**
 * The action to show first: the decision engine's action, then the bounce
 * risk model's action, then the score's safe-to-send flag.
 */
export function primaryAction(result: CheckResult): string {
	return (
		recommendationOf(result)?.action ??
		result.bounce_risk?.action ??
		(result.score.safe_to_send ? "send" : "do_not_send")
	);
}

/** A valid result for tests and stories. It follows a real sandbox check. */
export const SAMPLE_CHECK_RESULT: CheckResult = {
	input: "maya@brightlane.co",
	is_reachable: "safe",
	provider: "google_workspace",
	provider_confidence: "high",
	provider_rejection_reason: null,
	provider_rules_applied: true,
	recommendation: {
		action: "send",
		policy_mode: "deliverability",
		confidence: "high",
		priority: "low",
		summary: "The mailbox exists and accepts mail.",
		reasons: [
			{
				code: "deliverable",
				severity: "info",
				message: "The mailbox accepts mail.",
			},
		],
		engine_version: "1",
		evaluated_at: "2026-10-17T12:00:00Z",
	},
	bounce_risk: {
		action: "send",
		category: "safe",
		confidence: 0.92,
		model_version: "1",
		score: 6,
		scored_at: "2026-10-17T12:00:00Z",
		risk_factors: [
			{
				signal: "smtp_is_deliverable",
				description: "The mail server accepts this mailbox.",
				direction: "decreases_risk",
				contribution: 40,
			},
			{
				signal: "is_free_provider",
				description: "The domain is not a free email provider.",
				direction: "decreases_risk",
				contribution: 5,
			},
		],
	},
	score: {
		score: 94,
		category: "valid",
		sub_reason: "deliverable",
		safe_to_send: true,
		reason_codes: ["deliverable"],
		confidence: 90,
		confidence_level: "high",
		confidence_factors: ["SMTP confirmed the mailbox"],
		freshness: "fresh",
		age_days: 0,
		verified_at: "2026-10-17T12:00:00Z",
		signals: {
			has_domain_suggestion: false,
			has_mx_records: true,
			is_disposable: false,
			is_free_provider: false,
			is_role_account: false,
			is_spam_trap_domain: false,
			reachable: "safe",
			smtp_can_connect: true,
			smtp_error: false,
			smtp_has_full_inbox: false,
			smtp_is_catch_all: false,
			smtp_is_deliverable: true,
			smtp_is_disabled: false,
			valid_syntax: true,
		},
	},
	syntax: {
		address: "maya@brightlane.co",
		domain: "brightlane.co",
		username: "maya",
		is_valid_syntax: true,
		normalized_email: "maya@brightlane.co",
		suggestion: null,
	},
	mx: {
		accepts_mail: true,
		records: ["aspmx.l.google.com.", "alt1.aspmx.l.google.com."],
	},
	smtp: {
		can_connect_smtp: true,
		has_full_inbox: false,
		is_catch_all: false,
		is_deliverable: true,
		is_disabled: false,
	},
	misc: {
		gravatar_url: null,
		haveibeenpwned: null,
		is_b2c: false,
		is_disposable: false,
		is_role_account: false,
		is_spam_trap_domain: false,
	},
	debug: {
		backend_name: "backend-1",
		duration: { secs: 1, nanos: 200_000_000 },
		start_time: "2026-10-17T12:00:00Z",
		end_time: "2026-10-17T12:00:01Z",
		smtp: { verif_method: { type: "Smtp" } },
	},
};
