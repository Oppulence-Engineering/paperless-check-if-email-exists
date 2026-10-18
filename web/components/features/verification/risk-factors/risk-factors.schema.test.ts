import { describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { RiskFactorsPropsSchema } from "./risk-factors.schema";

describe("RiskFactorsPropsSchema", () => {
	it("parses a check result", () => {
		expect(RiskFactorsPropsSchema.safeParse({ result: SAMPLE_CHECK_RESULT }).success).toBe(true);
	});

	it("rejects a payload without a result", () => {
		expect(RiskFactorsPropsSchema.safeParse({}).success).toBe(false);
	});
});
