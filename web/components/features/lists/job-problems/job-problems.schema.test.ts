import { describe, expect, it } from "vitest";

import { JobProblemsPropsSchema } from "./job-problems.schema";

describe("JobProblemsPropsSchema", () => {
	it("parses a positive jobId", () => {
		expect(JobProblemsPropsSchema.safeParse({ jobId: 9 }).success).toBe(true);
	});

	it("rejects a missing jobId", () => {
		expect(JobProblemsPropsSchema.safeParse({}).success).toBe(false);
	});
});
