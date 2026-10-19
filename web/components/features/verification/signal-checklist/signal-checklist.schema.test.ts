import { describe, expect, it } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { SignalChecklistPropsSchema } from "./signal-checklist.schema";

describe("SignalChecklistPropsSchema", () => {
	it("parses a check result", () => {
		expect(SignalChecklistPropsSchema.safeParse({ result: SAMPLE_CHECK_RESULT }).success).toBe(
			true,
		);
	});

	it("rejects a payload without a result", () => {
		expect(SignalChecklistPropsSchema.safeParse({}).success).toBe(false);
	});
});
