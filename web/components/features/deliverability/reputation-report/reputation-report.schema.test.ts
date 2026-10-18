import { describe, expect, it } from "vitest";

import { ReputationReportPropsSchema, SAMPLE_REPUTATION_REPORT } from "./reputation-report.schema";

describe("ReputationReportPropsSchema", () => {
	it("parses a reputation check response", () => {
		expect(
			ReputationReportPropsSchema.safeParse({ report: SAMPLE_REPUTATION_REPORT }).success,
		).toBe(true);
	});

	it("accepts unknown response keys", () => {
		const parsed = ReputationReportPropsSchema.safeParse({
			report: { ...SAMPLE_REPUTATION_REPORT, checked_by: "new-field" },
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects a payload without a report", () => {
		expect(ReputationReportPropsSchema.safeParse({}).success).toBe(false);
	});
});
