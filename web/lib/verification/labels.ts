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
