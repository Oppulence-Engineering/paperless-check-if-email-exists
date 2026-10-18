import { describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { TechnicalDetailsPropsSchema } from "./technical-details.schema";

describe("TechnicalDetailsPropsSchema", () => {
	it("parses a check result", () => {
		expect(TechnicalDetailsPropsSchema.safeParse({ result: SAMPLE_CHECK_RESULT }).success).toBe(
			true,
		);
	});

	it("rejects a payload without a result", () => {
		expect(TechnicalDetailsPropsSchema.safeParse({}).success).toBe(false);
	});
});
