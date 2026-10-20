import type { PillTone } from "@/components/platform/page-kit";

/*
 * Human labels and tones for the verification enums in backend/openapi.json.
 * labels.test.ts fails when the backend adds a value that has no label here.
 */

type Labelled = { label: string; tone: PillTone };

/** Turns `smtp_is_catch_all` into "Smtp is catch all" for values without a label. */
export function humanize(value: string): string {
	const text = value.replaceAll("_", " ").trim();
	return text.charAt(0).toUpperCase() + text.slice(1);
}

/** The label for a value, or the humanized value when the map has no entry. */
export function labelFor(map: Record<string, string>, value: string | null | undefined): string {
	if (!value) return "Not available";
	return map[value] ?? humanize(value);
}

const FACTOR_TOPICS: Record<string, string> = {
	tenant_history: "Your earlier checks",
	fresh_history: "Last check",
	outcome_feedback: "Delivery feedback",
	auth_records: "Sender authentication",
	mx: "Mail server",
	smtp: "Mail server check",
};

/**
 * A readable confidence factor. The backend sends `topic:detail` codes, for
 * example `provider_reputation:gmail` (backend/src/scoring/mod.rs).
 */
export function confidenceFactor(code: string): string {
	const [topic = "", ...details] = code.split(":");
	const label = labelFor(FACTOR_TOPICS, topic);
	return details.length
		? `${label}: ${details.map((detail) => detail.replaceAll("_", " ")).join(", ")}`
		: label;
}

function entry<T extends Record<string, Labelled>>(map: T) {
	return {
		label: (value: string | null | undefined) =>
			value ? (map[value]?.label ?? humanize(value)) : "Not available",
		tone: (value: string | null | undefined): PillTone =>
			(value ? map[value]?.tone : undefined) ?? "gray",
		values: Object.keys(map),
	};
}

export const category = entry({
	valid: { label: "Valid", tone: "green" },
	risky: { label: "Risky", tone: "yellow" },
	unknown: { label: "Unknown", tone: "gray" },
	invalid: { label: "Invalid", tone: "red" },
});

export const reachability = entry({
	safe: { label: "Safe", tone: "green" },
	risky: { label: "Risky", tone: "yellow" },
	unknown: { label: "Unknown", tone: "gray" },
	invalid: { label: "Invalid", tone: "red" },
});

export const recommendation = entry({
	send: { label: "Send", tone: "green" },
	send_with_caution: { label: "Send with caution", tone: "yellow" },
	verify_manually: { label: "Review manually", tone: "blue" },
	do_not_send: { label: "Do not send", tone: "red" },
	// The decision engine uses these values (backend/src/decision/types.rs).
	review: { label: "Review", tone: "blue" },
	fix_then_send: { label: "Fix, then send", tone: "yellow" },
	suppress: { label: "Suppress", tone: "red" },
	drop: { label: "Drop", tone: "red" },
});

export const policyDecision = entry({
	send: { label: "Send", tone: "green" },
	review: { label: "Review", tone: "blue" },
	suppress: { label: "Suppress", tone: "red" },
	drop: { label: "Drop", tone: "red" },
});

export const reasonSeverity = entry({
	info: { label: "Info", tone: "gray" },
	warning: { label: "Warning", tone: "yellow" },
	critical: { label: "Critical", tone: "red" },
	blocking: { label: "Blocking", tone: "red" },
});

export const suppressionReason = entry({
	manual: { label: "Added by hand", tone: "gray" },
	bounce: { label: "Bounce", tone: "red" },
	invalid: { label: "Invalid address", tone: "red" },
	spam_trap: { label: "Spam trap", tone: "red" },
	unsubscribe: { label: "Unsubscribed", tone: "gray" },
	complaint: { label: "Complaint", tone: "red" },
	auto_invalid: { label: "Invalid in a check", tone: "red" },
});

export const POLICY_MODE_LABELS: Record<string, string> = {
	growth: "Growth",
	deliverability: "Deliverability",
	signup_protection: "Signup protection",
	enterprise_strict: "Enterprise strict",
	custom: "Custom",
};

export const bounceRisk = entry({
	safe: { label: "Safe", tone: "green" },
	low: { label: "Low risk", tone: "green" },
	medium: { label: "Medium risk", tone: "yellow" },
	high: { label: "High risk", tone: "red" },
	dangerous: { label: "Dangerous", tone: "red" },
});

export const freshness = entry({
	fresh: { label: "Fresh", tone: "green" },
	recent: { label: "Recent", tone: "green" },
	aging: { label: "Aging", tone: "yellow" },
	stale: { label: "Stale", tone: "yellow" },
	expired: { label: "Expired", tone: "red" },
});

export const confidenceLevel = entry({
	high: { label: "High confidence", tone: "green" },
	medium: { label: "Medium confidence", tone: "yellow" },
	low: { label: "Low confidence", tone: "yellow" },
	very_low: { label: "Very low confidence", tone: "red" },
});

export const providerConfidence = entry({
	high: { label: "High", tone: "green" },
	medium: { label: "Medium", tone: "yellow" },
	low: { label: "Low", tone: "gray" },
});

export const catchAllSeverity = entry({
	low: { label: "Low", tone: "green" },
	medium: { label: "Medium", tone: "yellow" },
	high: { label: "High", tone: "red" },
});

export const outcomeType = entry({
	delivered: { label: "Delivered", tone: "green" },
	open: { label: "Opened", tone: "blue" },
	click: { label: "Clicked", tone: "blue" },
	soft_bounce: { label: "Soft bounce", tone: "yellow" },
	hard_bounce: { label: "Hard bounce", tone: "red" },
	complaint: { label: "Complaint", tone: "red" },
	unsubscribe: { label: "Unsubscribed", tone: "gray" },
	// Provider outcome events use these values (backend/src/http/v1/outcomes.rs).
	bounced: { label: "Bounced", tone: "red" },
	opened: { label: "Opened", tone: "blue" },
	clicked: { label: "Clicked", tone: "blue" },
	complained: { label: "Complaint", tone: "red" },
	unsubscribed: { label: "Unsubscribed", tone: "gray" },
});

export const PROVIDER_LABELS: Record<string, string> = {
	gmail: "Gmail",
	google_workspace: "Google Workspace",
	outlook_consumer: "Outlook.com",
	microsoft365: "Microsoft 365",
	yahoo: "Yahoo",
	apple_icloud: "Apple iCloud",
	proton: "Proton",
	zoho: "Zoho",
};

export const SMTP_UNCERTAINTY_LABELS: Record<string, string> = {
	transient: "Temporary mail server error",
	policy_block: "The mail server blocked the check",
	timeout: "The mail server did not answer in time",
	network: "Network error",
	ambiguous_response: "The mail server gave an unclear answer",
	smtp_unreachable: "The mail server is not reachable",
};

/** Reason codes and sub reasons share most values, so one map serves both. */
export const REASON_LABELS: Record<string, string> = {
	external_verification: "Checked by an external service",
	deliverable: "The mailbox accepts mail",
	invalid_syntax: "The address format is not valid",
	invalid_recipient: "The mailbox does not exist",
	provider_rejected: "The mailbox provider rejects this format",
	provider_local_part_too_short: "The name before @ is too short for this provider",
	provider_local_part_too_long: "The name before @ is too long for this provider",
	provider_invalid_character: "The address has a character this provider does not allow",
	provider_consecutive_special_characters: "The address has two special characters in a row",
	provider_invalid_start_character:
		"The address starts with a character this provider does not allow",
	provider_invalid_end_character: "The address ends with a character this provider does not allow",
	provider_plus_addressing_not_supported: "This provider does not support + addresses",
	provider_reserved_word: "The address uses a word this provider reserves",
	provider_format_violation: "The address breaks this provider's format rules",
	smtp_undeliverable: "The mail server rejects this mailbox",
	disabled_mailbox: "The mailbox is disabled",
	no_mx: "The domain has no mail server",
	smtp_error: "The mail server returned an error",
	smtp_unreachable: "The mail server is not reachable",
	catch_all: "The domain accepts all addresses",
	full_inbox: "The mailbox is full",
	disposable: "Disposable address",
	role_account: "Role address, such as info@ or sales@",
	spam_trap: "Possible spam trap",
	unknown_deliverability: "Deliverability is not known",
	free_provider: "Free email provider",
	possible_typo: "The domain may have a typo",
	provider_reputation: "Based on the mailbox provider's record",
	tenant_history_positive: "Earlier sends to this address went well",
	tenant_history_inconsistent: "Earlier results for this address do not agree",
	catch_all_low_confidence: "Catch-all domain, low confidence",
	catch_all_medium_confidence: "Catch-all domain, medium confidence",
	catch_all_high_risk: "Catch-all domain, high risk",
	partial_confidence: "The check could not finish, so confidence is partial",
	transient_smtp: "Temporary mail server error",
	smtp_policy_block: "The mail server blocked the check",
	smtp_timeout: "The mail server did not answer in time",
	smtp_network: "Network error during the check",
	smtp_ambiguous: "The mail server gave an unclear answer",
	outcome_hard_bounce: "An earlier send hard bounced",
	outcome_complaint: "An earlier send got a spam complaint",
	outcome_engagement: "An earlier send was opened or clicked",
	risky: "Risky",
	unknown: "Unknown",
};

/** `good` is the value that lowers bounce risk; `null` means neither value is a problem. */
export const SIGNALS: { key: string; label: string; good: boolean | null }[] = [
	{ key: "valid_syntax", label: "Valid address format", good: true },
	{ key: "has_mx_records", label: "Domain has a mail server", good: true },
	{ key: "smtp_can_connect", label: "Mail server answers", good: true },
	{ key: "smtp_is_deliverable", label: "Mailbox accepts mail", good: true },
	{ key: "smtp_is_disabled", label: "Mailbox is disabled", good: false },
	{ key: "smtp_has_full_inbox", label: "Mailbox is full", good: false },
	{ key: "smtp_is_catch_all", label: "Domain accepts all addresses", good: false },
	{ key: "smtp_error", label: "Mail server returned an error", good: false },
	{ key: "is_disposable", label: "Disposable address", good: false },
	{ key: "is_role_account", label: "Role address", good: false },
	{ key: "is_spam_trap_domain", label: "Known spam trap domain", good: false },
	{ key: "has_domain_suggestion", label: "Domain may have a typo", good: false },
	{ key: "is_free_provider", label: "Free email provider", good: null },
];

/** Alert types are the change types in backend/src/list_intelligence.rs (`classify_change`). */
export const alertType = entry({
	became_invalid: { label: "Became invalid", tone: "red" },
	became_risky: { label: "Became risky", tone: "yellow" },
	became_safe: { label: "Became safe", tone: "green" },
	improved: { label: "Improved", tone: "green" },
	degraded: { label: "Degraded", tone: "yellow" },
});

/** The `list_status` database enum (backend/migrations/20260318000003_email_lists.up.sql). */
export const listStatus = entry({
	uploading: { label: "Uploading", tone: "blue" },
	processing: { label: "Processing", tone: "blue" },
	completed: { label: "Completed", tone: "green" },
	failed: { label: "Failed", tone: "red" },
	deleted: { label: "Deleted", tone: "gray" },
});

export const pipelineRunStatus = entry({
	queued: { label: "Queued", tone: "blue" },
	preparing: { label: "Preparing", tone: "blue" },
	fetching_source: { label: "Fetching source", tone: "blue" },
	publishing: { label: "Publishing", tone: "blue" },
	running: { label: "Running", tone: "blue" },
	delivering: { label: "Delivering", tone: "blue" },
	completed: { label: "Completed", tone: "green" },
	failed: { label: "Failed", tone: "red" },
	cancelled: { label: "Cancelled", tone: "gray" },
	skipped: { label: "Skipped", tone: "gray" },
});

/** The `job_state` database enum (backend/migrations/20260316000003_enhanced_job_lifecycle.up.sql). */
export const jobStatus = entry({
	pending: { label: "Waiting", tone: "blue" },
	running: { label: "Running", tone: "blue" },
	completed: { label: "Completed", tone: "green" },
	cancelling: { label: "Cancelling", tone: "yellow" },
	cancelled: { label: "Cancelled", tone: "gray" },
	failed: { label: "Failed", tone: "red" },
});

/** The `task_state` database enum: the state of one row in a job. */
export const taskState = entry({
	queued: { label: "Queued", tone: "gray" },
	running: { label: "Running", tone: "blue" },
	completed: { label: "Completed", tone: "green" },
	retrying: { label: "Retrying", tone: "yellow" },
	failed: { label: "Failed", tone: "red" },
	cancelled: { label: "Cancelled", tone: "gray" },
	dead_lettered: { label: "Stopped after retries", tone: "red" },
});

/** Remediation classes (backend/src/http/v1/lists/remediation.rs). */
export const remediationClass = entry({
	fixed: { label: "Fixed", tone: "blue" },
	safe: { label: "Safe", tone: "green" },
	review: { label: "Review", tone: "yellow" },
	drop: { label: "Drop", tone: "red" },
});

/** Domain reputation risk levels (backend/src/reputation/scorer.rs). */
export const reputationRisk = entry({
	unknown: { label: "Incomplete", tone: "gray" },
	low: { label: "Low risk", tone: "green" },
	medium: { label: "Medium risk", tone: "yellow" },
	high: { label: "High risk", tone: "red" },
	critical: { label: "Critical risk", tone: "red" },
});

/** The DMARC `p=` policy values. "none" only monitors and does not protect the domain. */
export const dmarcPolicy = entry({
	none: { label: "Monitor only", tone: "yellow" },
	quarantine: { label: "Quarantine", tone: "green" },
	reject: { label: "Reject", tone: "green" },
});

/** Suppression entry states (backend/src/http/v1/suppressions/list.rs). */
export const suppressionStatus = entry({
	active: { label: "Active", tone: "red" },
	inactive: { label: "Removed", tone: "gray" },
	merged: { label: "Merged", tone: "gray" },
});

/**
 * Suppression source types. The API keeps any text, so a new value shows in
 * plain words. The Suppressions page sets `import` for a CSV file.
 */
export const suppressionSource = entry({
	manual: { label: "Manual", tone: "gray" },
	import: { label: "CSV import", tone: "gray" },
	outcome: { label: "Delivery feedback", tone: "gray" },
	auto_action: { label: "Automatic action", tone: "gray" },
});

/** Suppression audit events (backend/src/http/v1/suppressions/add.rs and delete.rs). */
export const suppressionEvent = entry({
	created: { label: "Added", tone: "red" },
	updated: { label: "Updated", tone: "blue" },
	deactivated: { label: "Removed", tone: "gray" },
});

/** Who changed a suppression entry (`actor_type` in `v1_suppression_events`). */
export const SUPPRESSION_ACTOR_LABELS: Record<string, string> = {
	api: "App or API",
	policy: "Policy",
	outcome: "Delivery feedback",
	outcome_ingest: "Delivery feedback",
	worker: "Automatic check",
};

/** Provider event families (`event_family` in backend/src/http/v1/provider_outcomes.rs). */
export const eventFamily = entry({
	delivery: { label: "Delivery", tone: "blue" },
	engagement: { label: "Engagement", tone: "green" },
	negative_feedback: { label: "Negative feedback", tone: "red" },
	routing: { label: "Routing", tone: "gray" },
});

/** Whether the backend matched a provider event to a send. It writes `unmatched` only today. */
export const correlationStatus = entry({
	unmatched: { label: "Not matched", tone: "gray" },
});

/** What a campaign outcome policy did with an event (`action_label` in backend/src/outcomes.rs). */
export const policyAction = entry({
	suppressed: { label: "Suppressed", tone: "red" },
	pending: { label: "Waiting for more events", tone: "yellow" },
	boosted: { label: "Score raised", tone: "green" },
	informational: { label: "Recorded only", tone: "gray" },
});

/** Outcome policy rule actions (`RuleAction` in backend/src/outcomes.rs). */
export const ruleAction = entry({
	suppress: { label: "Suppress", tone: "red" },
	suppress_and_unsubscribe: { label: "Suppress and unsubscribe", tone: "red" },
	suppress_after: { label: "Suppress after repeats", tone: "yellow" },
	score_boost: { label: "Raise the score", tone: "green" },
	informational: { label: "Record only", tone: "gray" },
});

/** Pipeline states (`PipelineStatus` in backend/src/pipelines.rs). */
export const pipelineStatus = entry({
	active: { label: "Active", tone: "green" },
	paused: { label: "Paused", tone: "gray" },
	deleted: { label: "Deleted", tone: "gray" },
});

/** Webhook delivery states of a pipeline run (`PipelineDeliveryStatus` in backend/src/pipelines.rs). */
export const pipelineDeliveryStatus = entry({
	not_requested: { label: "No webhook", tone: "gray" },
	pending: { label: "Webhook waiting", tone: "blue" },
	delivered: { label: "Webhook delivered", tone: "green" },
	retry_scheduled: { label: "Webhook retry scheduled", tone: "yellow" },
	failed: { label: "Webhook failed", tone: "red" },
});

/** Pipeline source types (`PipelineSourceType`). The backend accepts only list_snapshot and push. */
export const PIPELINE_SOURCE_LABELS: Record<string, string> = {
	list_snapshot: "Saved list",
	push: "Push API",
	integration: "Integration",
	bucket: "Storage bucket",
};

/** What started a pipeline run (`trigger_type` in backend/src/pipelines.rs). */
export const PIPELINE_TRIGGER_LABELS: Record<string, string> = {
	schedule: "Schedule",
	manual: "Manual run",
	push: "Push",
};

/** Job events (`event_type` in job_events: bulk/post.rs, jobs/cancel.rs, jobs/retry.rs, do_work.rs, delayed_recheck.rs). */
export const jobEventType = entry({
	"job.created": { label: "Job created", tone: "blue" },
	"job.cancellation_requested": { label: "Cancel requested", tone: "yellow" },
	"job.retry_requested": { label: "Retry requested", tone: "blue" },
	"task.completed": { label: "Row checked", tone: "green" },
	"task.retrying": { label: "Row retrying", tone: "yellow" },
	"task.dead_lettered": { label: "Row stopped after retries", tone: "red" },
	"task.partial_confidence": { label: "Row has partial confidence", tone: "yellow" },
	"task.delayed_recheck_scheduled": { label: "Recheck scheduled", tone: "blue" },
	"task.delayed_recheck_published": { label: "Recheck started", tone: "blue" },
	"task.delayed_recheck_completed": { label: "Recheck finished", tone: "green" },
	"task.delayed_recheck_failed": { label: "Recheck failed", tone: "red" },
});

/** Source quality grades from A (best) to F (`quality_grade` in source_quality.rs). */
export const qualityGrade = entry({
	pending: { label: "Awaiting checks", tone: "gray" },
	A: { label: "Grade A", tone: "green" },
	B: { label: "Grade B", tone: "green" },
	C: { label: "Grade C", tone: "yellow" },
	D: { label: "Grade D", tone: "red" },
	F: { label: "Grade F", tone: "red" },
});
