// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { JobsApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { JobProblems } from "./job-problems";

describe("JobProblems", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows failures and events, and retries the failed rows", async () => {
		vi.spyOn(JobsApi.prototype, "v1GetJobFailureCenter").mockResolvedValue({
			data: {
				status: "completed",
				total_records: 6,
				processed_rows: 6,
				progress_pct: 100,
				task_states: { completed: 5, failed: 1 },
				failures_total: 1,
				retry_summary: {
					retryable_rows: 1,
					permanent_failures: 0,
					rows_with_retries: 1,
					max_retry_count: 1,
				},
				estimated_completion_seconds: null,
				failure_breakdown: [{ error: "SMTP timeout", count: 1 }],
				failed_rows: [
					{
						id: 4,
						row_index: 2,
						input: "slow@example.com",
						task_state: "failed",
						error: "SMTP timeout",
						retry_count: 1,
					},
				],
			},
		} as never);
		vi.spyOn(JobsApi.prototype, "v1GetJobEvents").mockResolvedValue({
			data: {
				total: 1,
				events: [
					{
						id: 1,
						task_id: 4,
						event_type: "task.delayed_recheck_scheduled",
						event_data: { delay_seconds: 300 },
						actor: "backend-dev",
						created_at: "2026-10-18T04:00:00Z",
					},
				],
			},
		} as never);
		// gitleaks reads the method name as a key.
		const jobLatency = vi.spyOn(JobsApi.prototype, "v1JobLatency"); // gitleaks:allow
		jobLatency.mockResolvedValue({
			data: { total_completed: 5, p50_duration_ms: 200, p95_duration_ms: 900 },
		} as never);
		const retry = vi.spyOn(JobsApi.prototype, "v1RetryJob").mockResolvedValue({
			data: { job_id: 9, status: "running", tasks_retried: 1 },
		} as never);
		vi.spyOn(window, "confirm").mockReturnValue(true);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<JobProblems jobId={9} />
			</QueryClientProvider>,
		);
		expect(await screen.findByText("slow@example.com")).toBeInTheDocument();
		expect(screen.getByText("Task delayed recheck scheduled")).toBeInTheDocument();
		expect(screen.getByText(/backend-dev · delay_seconds: 300/)).toBeInTheDocument();
		expect(await screen.findByText(/Median check time 200 ms/)).toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "Retry failed" }));
		await waitFor(() => expect(retry).toHaveBeenCalledWith({ jobId: 9 }));
		expect(await screen.findByText("1 failed rows are queued again.")).toBeInTheDocument();
	});
});
