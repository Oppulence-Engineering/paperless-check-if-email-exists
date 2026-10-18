import { z } from "zod";

import { V1CheckReputation200Response } from "@/lib/api/generated/zod/v1/v1";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Reputation report.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `reputation-report.lit.ts`.
 */
export const ReputationReportPropsSchema = z.object({
	report: V1CheckReputation200Response,
});

export type ReputationReportPropsFields = z.infer<typeof ReputationReportPropsSchema>;

/** A sample `POST /v1/reputation/check` response for tests and stories. */
export const SAMPLE_REPUTATION_REPORT = V1CheckReputation200Response.parse({
	domain: "example.com",
	score: 82,
	risk_level: "low",
	cached: true,
	blacklist_results: [
		{ provider: "Spamhaus ZEN", listed: false, lookup_time_ms: 61 },
		{ provider: "SpamCop", listed: true, lookup_time_ms: 69 },
	],
	dns_records: {
		has_spf: true,
		spf_valid: true,
		has_dkim: false,
		has_dmarc: true,
		dmarc_policy: "none",
		has_mx: true,
		mx_records: ["mx1.example.com.", "mx2.example.com."],
	},
	domain_info: {
		domain_age_days: 9000,
		registrar: "Example Registrar, Inc.",
		created_at: "2001-08-14T04:00:00Z",
	},
});
