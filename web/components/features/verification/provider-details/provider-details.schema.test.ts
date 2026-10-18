import { describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { ProviderDetailsPropsSchema } from "./provider-details.schema";

describe("ProviderDetailsPropsSchema", () => {
	it("parses a check result", () => {
		expect(ProviderDetailsPropsSchema.safeParse({ result: SAMPLE_CHECK_RESULT }).success).toBe(
			true,
		);
	});

	it("rejects a payload without a result", () => {
		expect(ProviderDetailsPropsSchema.safeParse({}).success).toBe(false);
	});
});
