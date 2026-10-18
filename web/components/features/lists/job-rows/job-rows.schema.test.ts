import { describe, expect, it } from "vitest";

import { JobRowsPropsSchema } from "./job-rows.schema";

describe("JobRowsPropsSchema", () => {
	it("parses a positive jobId", () => {
		expect(JobRowsPropsSchema.safeParse({ jobId: 9 }).success).toBe(true);
	});

	it("rejects a missing jobId", () => {
		expect(JobRowsPropsSchema.safeParse({}).success).toBe(false);
	});
});
