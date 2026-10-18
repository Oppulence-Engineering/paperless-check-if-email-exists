import { describe, expect, it } from "vitest";

import { JobReviewPropsSchema } from "./job-review.schema";

describe("JobReviewPropsSchema", () => {
	it("parses a positive jobId", () => {
		expect(JobReviewPropsSchema.safeParse({ jobId: 9 }).success).toBe(true);
	});

	it("rejects a missing jobId", () => {
		expect(JobReviewPropsSchema.safeParse({}).success).toBe(false);
	});
});
