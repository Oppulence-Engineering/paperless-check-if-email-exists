// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, render, screen, waitFor } from "@testing-library/react";
import { JobsApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { etaSeconds, JobProgress, timeLeft } from "./job-progress";

function renderProgress(processed: number) {
	vi.spyOn(JobsApi.prototype, "v1GetBulkJobProgress").mockResolvedValue({
		data: {
			created_at: "2026-10-18T04:00:00Z",
			finished_at: null,
			total_records: 10,
			total_processed: processed,
		},
	} as never);
	// gitleaks reads the method name as a key.
	const latency = vi.spyOn(JobsApi.prototype, "v1JobLatency"); // gitleaks:allow
	latency.mockResolvedValue({
		data: { total_completed: 4, p50_duration_ms: 200, p95_duration_ms: 900 },
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<JobProgress jobId={9} />
		</QueryClientProvider>,
	);
	return { latency };
}

describe("JobProgress", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows progress, time left, and the median check time while rows remain", async () => {
		renderProgress(4);
		expect(await screen.findByText(/4 of 10 rows checked/)).toBeInTheDocument();
		expect(await screen.findByText(/Median check time 200 ms/)).toBeInTheDocument();
		expect(screen.getByRole("progressbar", { name: "Verification progress" })).toBeInTheDocument();
	});

	it("shows nothing and does not read latency when every row is done", async () => {
		const { latency } = renderProgress(10);
		await waitFor(() => expect(JobsApi.prototype.v1GetBulkJobProgress).toHaveBeenCalled());
		expect(screen.queryByRole("progressbar")).not.toBeInTheDocument();
		expect(latency).not.toHaveBeenCalled();
	});

	it("estimates time left from the rate since the job started", () => {
		const start = "2026-10-18T04:00:00Z";
		// 10 rows in 100 seconds leaves 90 rows for 900 seconds.
		expect(etaSeconds(100, 10, start, Date.parse(start) + 100_000)).toBe(900);
		expect(etaSeconds(100, 0, start, Date.parse(start) + 100_000)).toBeNull();
		expect(etaSeconds(100, 100, start, Date.parse(start) + 100_000)).toBeNull();
		expect(timeLeft(900)).toBe("About 15 min left");
		expect(timeLeft(30)).toBe("Less than a minute left");
		expect(timeLeft(null)).toBe("Estimating time left…");
	});
});
