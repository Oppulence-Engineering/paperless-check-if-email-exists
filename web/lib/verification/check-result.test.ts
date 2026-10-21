import { describe, expect, it } from "vitest";

import { V1CheckEmail200Response } from "@/lib/api/generated/zod/v1/v1";

import {
	mxDetails,
	freeProviderOf,
	primaryAction,
	recommendationOf,
	SAMPLE_CHECK_RESULT,
	smtpDetails,
	suggestedEmail,
	type CheckResult,
} from "./check-result";

describe("check result helpers", () => {
	it("keeps the sample valid against the generated contract", () => {
		expect(V1CheckEmail200Response.safeParse(SAMPLE_CHECK_RESULT).success).toBe(true);
	});

	it("reads the decision engine output", () => {
		expect(recommendationOf(SAMPLE_CHECK_RESULT)?.summary).toBe(
			"The mailbox exists and accepts mail.",
		);
		expect(
			recommendationOf({
				...SAMPLE_CHECK_RESULT,
				recommendation: { action: 1 },
			}),
		).toBeNull();
	});

	it("picks the action from the most specific source", () => {
		expect(primaryAction(SAMPLE_CHECK_RESULT)).toBe("send");
		const withoutDecision: CheckResult = {
			...SAMPLE_CHECK_RESULT,
			recommendation: undefined,
		};
		expect(primaryAction({ ...withoutDecision, bounce_risk: null })).toBe("send");
		expect(
			primaryAction({
				...withoutDecision,
				bounce_risk: null,
				score: { ...SAMPLE_CHECK_RESULT.score, safe_to_send: false },
			}),
		).toBe("do_not_send");
	});

	it("separates details from check errors", () => {
		expect(mxDetails(SAMPLE_CHECK_RESULT).details?.records).toHaveLength(2);
		const failed = {
			...SAMPLE_CHECK_RESULT,
			smtp: { error: { type: "Timeout", message: "No answer" } },
		};
		expect(smtpDetails(failed).error?.message).toBe("No answer");
	});

	it("accepts the error shape that the backend sends", () => {
		// A timeout sends its duration as an object.
		const failed = V1CheckEmail200Response.parse({
			...SAMPLE_CHECK_RESULT,
			smtp: {
				error: { type: "Timeout", message: { secs: 10, nanos: 0 } },
			},
		});
		expect(smtpDetails(failed).error).toEqual({
			type: "Timeout",
			message: '{"secs":10,"nanos":0}',
		});
	});

	it("offers a corrected address only when one exists", () => {
		expect(suggestedEmail(SAMPLE_CHECK_RESULT)).toBeNull();
		expect(
			suggestedEmail({
				...SAMPLE_CHECK_RESULT,
				recommendation: undefined,
				score: {
					...SAMPLE_CHECK_RESULT.score,
					domain_suggestion: "maya@gmail.com",
				},
			}),
		).toBe("maya@gmail.com");
	});
});

it("keeps provider labels consistent and leaves missing or conflicting evidence unknown", () => {
	expect(freeProviderOf(SAMPLE_CHECK_RESULT)).toBe(false);
	const conflicting = {
		...SAMPLE_CHECK_RESULT,
		score: {
			...SAMPLE_CHECK_RESULT.score,
			signals: { ...SAMPLE_CHECK_RESULT.score.signals, is_free_provider: true },
		},
	};
	expect(freeProviderOf(conflicting)).toBeUndefined();
	const missing = V1CheckEmail200Response.parse({
		...SAMPLE_CHECK_RESULT,
		misc: { error: { type: "Unavailable" } },
	});
	expect(freeProviderOf(missing)).toBeUndefined();
});
