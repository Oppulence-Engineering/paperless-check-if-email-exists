// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { JobsApi, OutcomesApi, V1Api, VerificationApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { JobRows } from "./job-rows";

describe("JobRows", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("lists results, filters them, and opens one row in a sheet", async () => {
		// gitleaks reads the method name as a key.
		const results = vi.spyOn(JobsApi.prototype, "v1GetJobResults"); // gitleaks:allow
		results.mockResolvedValue({
			data: {
				has_more: false,
				next_cursor: null,
				results: [
					{ id: 1, task_state: "completed", retry_count: 0, result: SAMPLE_CHECK_RESULT },
					{
						id: 2,
						task_state: "completed",
						retry_count: 0,
						result: {
							...SAMPLE_CHECK_RESULT,
							input: "gone@example.com",
							score: { ...SAMPLE_CHECK_RESULT.score, category: "invalid" },
						},
					},
				],
			},
		} as never);
		vi.spyOn(VerificationApi.prototype, "v1EmailHistory").mockResolvedValue({
			data: { email: "maya@brightlane.co", total: 0, history: [] },
		} as never);
		vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes").mockResolvedValue({
			data: { outcomes: [], total: 0 },
		} as never);
		vi.spyOn(V1Api.prototype, "v1CheckSuppression").mockResolvedValue({
			data: { email: "maya@brightlane.co", suppressed: false },
		} as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<JobRows jobId={9} />
			</QueryClientProvider>,
		);
		expect(await screen.findByRole("button", { name: "gone@example.com" })).toBeInTheDocument();
		expect(results).toHaveBeenCalledWith({
			jobId: 9,
			cursor: undefined,
			limit: 50,
			state: "completed",
		});

		fireEvent.change(screen.getByLabelText("Category"), { target: { value: "valid" } });
		expect(screen.queryByRole("button", { name: "gone@example.com" })).not.toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "maya@brightlane.co" }));
		expect(await screen.findByRole("dialog")).toHaveTextContent("Recommendation");

		fireEvent.change(screen.getByLabelText("Row state"), { target: { value: "failed" } });
		await waitFor(() =>
			expect(results).toHaveBeenCalledWith(expect.objectContaining({ state: "failed" })),
		);
	});
});
