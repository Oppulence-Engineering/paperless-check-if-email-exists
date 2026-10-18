// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, render, screen } from "@testing-library/react";
import { JobsApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { JobReview } from "./job-review";

describe("JobReview", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the verdict, the category breakdown, and the risk flags", async () => {
		vi.spyOn(JobsApi.prototype, "v1JobApprovalChecklist").mockResolvedValue({
			data: {
				job_id: 9,
				total_records: 6,
				categories: { valid: 0, risky: 1, unknown: 0, invalid: 5, unprocessed: 0 },
				risk_flags: {
					disposable_count: 0,
					catch_all_count: 0,
					role_account_count: 1,
					spam_trap_count: 0,
					suppressed_count: 0,
				},
				safe_to_send_count: 0,
				safe_to_send_pct: 0,
				recommendation: "List quality is poor. Clean the list thoroughly before sending.",
				ready_to_send: false,
			},
		} as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<JobReview aria-label="Review" jobId={9} />
			</QueryClientProvider>,
		);
		expect(await screen.findByText("Review before sending")).toBeInTheDocument();
		expect(screen.getByText(/List quality is poor/)).toBeInTheDocument();
		expect(screen.getByText("0 of 6 addresses are safe to send (0%).")).toBeInTheDocument();
		expect(screen.getByText("Role address").nextSibling).toHaveTextContent("1");
		expect(screen.getByRole("region", { name: "Review" })).toHaveAttribute(
			"data-slot",
			"job-review",
		);
	});
});
