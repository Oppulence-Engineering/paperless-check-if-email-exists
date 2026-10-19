import { describe, expect, it } from "vitest";

import { JobProgressPropsSchema } from "./job-progress.schema";

describe("JobProgressPropsSchema", () => {
	it("parses a positive jobId", () => {
		expect(JobProgressPropsSchema.safeParse({ jobId: 9 }).success).toBe(true);
	});

	it("rejects a missing jobId", () => {
		expect(JobProgressPropsSchema.safeParse({}).success).toBe(false);
	});
});
