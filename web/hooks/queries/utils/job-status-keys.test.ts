import { describe, expect, it } from "vitest";

import { jobStatusKeys, JOB_STATUS_STALE_TIME } from "./job-status-keys";

describe("jobStatusKeys", () => {
	it("nests list and detail keys under the resource root", () => {
		expect(jobStatusKeys.list()[0]).toBe("jobs");
		expect(jobStatusKeys.detail("42")).toEqual(["jobs", 42, "status"]);
		expect(JOB_STATUS_STALE_TIME).toBe(60_000);
	});
});
