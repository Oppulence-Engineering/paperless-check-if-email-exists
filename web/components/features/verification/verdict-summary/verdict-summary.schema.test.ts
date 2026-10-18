import { describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { VerdictSummaryPropsSchema } from "./verdict-summary.schema";

describe("VerdictSummaryPropsSchema", () => {
	it("parses a check result", () => {
		expect(VerdictSummaryPropsSchema.safeParse({ result: SAMPLE_CHECK_RESULT }).success).toBe(true);
	});

	it("rejects a payload without a result", () => {
		expect(VerdictSummaryPropsSchema.safeParse({}).success).toBe(false);
	});
});
