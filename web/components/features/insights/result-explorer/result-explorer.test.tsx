// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { useState } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { JobsApi, OutcomesApi, QueryApi, V1Api, VerificationApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { describeFilters, ResultExplorer } from "./result-explorer";
import { type ExplorerFilters } from "./result-explorer.schema";
import { chooseOption } from "@/quality/test-support/select";

function row(id: number, extra: Record<string, unknown> = {}) {
	return {
		id,
		job_id: 18,
		email: `user${String(id)}@example.com`,
		score: 55,
		category: "risky",
		safe_to_send: false,
		reason_codes: ["smtp_error"],
		recommendation_action: "fix_then_send",
		recommendation: null,
		policy_mode: "deliverability",
		policy_decision: "review",
		policy_evaluation: null,
		task_state: "completed",
		completed_at: "2026-10-18T05:28:54Z",
		created_at: "2026-10-18T05:28:53Z",
		...extra,
	};
}

function Harness({ initial = {} }: { initial?: ExplorerFilters }) {
	const [filters, setFilters] = useState(initial);
	return <ResultExplorer filters={filters} onFiltersChange={setFilters} />;
}

function renderExplorer(initial?: ExplorerFilters) {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	return render(
		<QueryClientProvider client={client}>
			<Harness initial={initial} />
		</QueryClientProvider>,
	);
}

describe("ResultExplorer", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the recommendation and its reasons, filters, and pages with the total", async () => {
		const query = vi.spyOn(QueryApi.prototype, "v1QueryResults"); // gitleaks:allow
		query.mockResolvedValue({
			data: {
				total: 45,
				results: [
					row(1, {
						recommendation: {
							action: "fix_then_send",
							policy_mode: "deliverability",
							confidence: "medium",
							priority: "high",
							summary: "Fix the address first.",
							reasons: [
								{ code: "typo", severity: "warning", message: "The domain may have a typo." },
							],
							engine_version: "decision_v1",
							evaluated_at: "2026-10-18T05:28:54Z",
						},
					}),
					row(2, { job_id: null, email: "single@example.com" }),
				],
			},
		} as never);
		renderExplorer();

		// The first page can take longer than the default 1 s when the whole suite runs.
		expect(
			await screen.findByRole("button", { name: "user1@example.com" }, { timeout: 5_000 }),
		).toBeInTheDocument();
		expect(screen.getAllByText("Fix, then send")).toHaveLength(2);
		expect(screen.getByText("The domain may have a typo.")).toBeInTheDocument();
		// Without decision reasons, the row shows the score reason codes.
		expect(screen.getByText("The mail server returned an error")).toBeInTheDocument();
		expect(screen.getAllByText("Review")).toHaveLength(2);
		expect(screen.getByText("Single check")).toBeInTheDocument();
		expect(screen.getByText("Rows 1–2 of 45")).toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "Next" }));
		await waitFor(() =>
			expect(query).toHaveBeenLastCalledWith(expect.objectContaining({ limit: 20, offset: 20 })),
		);

		await chooseOption("Category", "Valid");
		await waitFor(() =>
			expect(query).toHaveBeenLastCalledWith(
				expect.objectContaining({ offset: 0, category: "valid" }),
			),
		);
		await chooseOption("Safe to send", "No");
		await waitFor(() =>
			expect(query).toHaveBeenLastCalledWith(
				expect.objectContaining({ category: "valid", safeToSend: false }),
			),
		);
		expect(screen.getByRole("list", { name: "Active filters" })).toHaveTextContent(
			"Category: Valid",
		);
		fireEvent.click(screen.getByRole("button", { name: "Clear filters" }));
		await waitFor(() => expect(query.mock.lastCall?.[0]).toEqual({ limit: 20, offset: 0 }));
	});

	it("sends every filter to the API and edits them in a sheet", async () => {
		const query = vi.spyOn(QueryApi.prototype, "v1QueryResults"); // gitleaks:allow
		query.mockResolvedValue({ data: { total: 0, results: [] } } as never);
		renderExplorer({
			minScore: 10,
			maxScore: 90,
			jobId: 18,
			since: "2026-10-01",
			until: "2026-10-18",
			recommendationAction: "review",
			policyMode: "growth",
			policyDecision: "suppress",
		});

		expect(await screen.findByText("No results match these filters.")).toBeInTheDocument();
		expect(query).toHaveBeenCalledWith({
			limit: 20,
			offset: 0,
			minScore: 10,
			maxScore: 90,
			jobId: 18,
			since: new Date("2026-10-01T00:00").toISOString(),
			until: new Date("2026-10-18T23:59:59.999").toISOString(),
			recommendationAction: "review",
			policyMode: "growth",
			policyDecision: "suppress",
		});

		fireEvent.click(screen.getByRole("button", { name: "More filters (8)" }));
		const sheet = await screen.findByRole("dialog");
		fireEvent.change(within(sheet).getByLabelText("Highest score"), { target: { value: "70" } });
		fireEvent.click(within(sheet).getByRole("button", { name: "Apply filters" }));
		await waitFor(() =>
			expect(query).toHaveBeenLastCalledWith(expect.objectContaining({ maxScore: 70 })),
		);
		expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
	});

	it("opens one result in a sheet with the shared verification parts", async () => {
		const query = vi.spyOn(QueryApi.prototype, "v1QueryResults"); // gitleaks:allow
		query.mockResolvedValue({ data: { total: 1, results: [row(7)] } } as never);
		const results = vi.spyOn(JobsApi.prototype, "v1GetJobResults"); // gitleaks:allow
		results.mockResolvedValue({
			data: {
				has_more: false,
				next_cursor: null,
				results: [{ id: 7, task_state: "completed", retry_count: 0, result: SAMPLE_CHECK_RESULT }],
			},
		} as never);
		vi.spyOn(VerificationApi.prototype, "v1EmailHistory").mockResolvedValue({
			data: { email: "user7@example.com", total: 0, history: [] },
		} as never);
		vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes").mockResolvedValue({
			data: { outcomes: [], total: 0 },
		} as never);
		vi.spyOn(V1Api.prototype, "v1CheckSuppression").mockResolvedValue({
			data: { email: "user7@example.com", suppressed: false },
		} as never);
		renderExplorer();

		fireEvent.click(await screen.findByRole("button", { name: "user7@example.com" }));
		const sheet = await screen.findByRole("dialog");
		expect(within(sheet).getByText("Job 18 · Checked", { exact: false })).toBeInTheDocument();
		expect(await within(sheet).findByText("Quality score")).toBeInTheDocument();
		expect(results).toHaveBeenCalledWith({ jobId: 18, cursor: 6, limit: 1, state: "completed" });
	});

	it("describes the active filters in words", () => {
		expect(describeFilters({})).toEqual([]);
		expect(describeFilters({ minScore: 10, safeToSend: true, policyMode: "growth" })).toEqual([
			"Safe to send: Yes",
			"Score 10 to 100",
			"Policy mode: Growth",
		]);
	});
});
