import { expect, it, vi } from "vitest";
import { fetchJobStatus, jobStatusPath, loadJobStatus } from "./fetch-job-status";
import type { RequestJsonFn } from "@/lib/api/request-json";
it("rejects invalid IDs before requesting a job", async () => {
	await expect(fetchJobStatus("bad")).rejects.toThrow("Select a valid job");
	await expect(fetchJobStatus("-1")).rejects.toThrow("Select a valid job");
	expect(jobStatusPath("42")).toBe("/v1/jobs/42");
});
it("forwards cancellation and validates the result", async () => {
	const data = {
		job_id: 42,
		status: "failed",
		total_records: 1,
		created_at: "2026-10-21T00:00:00Z",
		task_summary: { failed: 1 },
	};
	const signal = new AbortController().signal;
	const request = vi.fn(async (input: Parameters<RequestJsonFn>[0]) => input.schema.parse(data));
	await expect(loadJobStatus(request, "42", signal)).resolves.toEqual(data);
	expect(request.mock.calls[0]?.[0].signal).toBe(signal);
});
