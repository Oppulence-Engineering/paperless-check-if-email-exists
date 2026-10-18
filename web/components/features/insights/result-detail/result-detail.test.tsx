// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, render, screen } from "@testing-library/react";
import { JobsApi, OutcomesApi, V1Api, VerificationApi } from "@oppulence/reacher-sdk";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { ResultDetail } from "./result-detail";

const ROW = {
	id: 7,
	job_id: 18,
	email: "maya@brightlane.co",
	task_state: "completed",
	created_at: "2026-10-18T05:28:53Z",
	recommendation_action: "review",
	recommendation: {
		action: "review",
		policy_mode: "deliverability",
		confidence: "medium",
		priority: "high",
		summary: "Check this address by hand.",
		reasons: [{ code: "role", severity: "info", message: "This is a role address." }],
		engine_version: "decision_v1",
		evaluated_at: "2026-10-18T05:28:54Z",
	},
	policy_mode: "growth",
	policy_decision: "suppress",
	policy_evaluation: null,
};

function renderDetail(row: Record<string, unknown>) {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<ResultDetail row={row as never} />
		</QueryClientProvider>,
	);
}

describe("ResultDetail", () => {
	beforeEach(() => {
		vi.spyOn(VerificationApi.prototype, "v1EmailHistory").mockResolvedValue({
			data: { email: "maya@brightlane.co", total: 0, history: [] },
		} as never);
		vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes").mockResolvedValue({
			data: { outcomes: [], total: 0 },
		} as never);
		vi.spyOn(V1Api.prototype, "v1CheckSuppression").mockResolvedValue({
			data: { email: "maya@brightlane.co", suppressed: false },
		} as never);
	});

	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("reads the full result of a job row and shows the shared parts", async () => {
		const results = vi.spyOn(JobsApi.prototype, "v1GetJobResults"); // gitleaks:allow
		results.mockResolvedValue({
			data: {
				has_more: true,
				next_cursor: 7,
				results: [{ id: 7, task_state: "completed", retry_count: 0, result: SAMPLE_CHECK_RESULT }],
			},
		} as never);
		renderDetail(ROW);

		expect(await screen.findByText("Quality score")).toBeInTheDocument();
		expect(screen.getByText("Technical details")).toBeInTheDocument();
		expect(results).toHaveBeenCalledWith({ jobId: 18, cursor: 6, limit: 1, state: "completed" });
	});

	it("shows the decision of a single check from the query row", async () => {
		const results = vi.spyOn(JobsApi.prototype, "v1GetJobResults"); // gitleaks:allow
		renderDetail({ ...ROW, job_id: null });

		expect(screen.getByText(/This was a single check/)).toBeInTheDocument();
		expect(screen.getByText("Check this address by hand.")).toBeInTheDocument();
		expect(screen.getByText("This is a role address.")).toBeInTheDocument();
		expect(screen.getByText("Suppress")).toBeInTheDocument();
		expect(screen.getByText("Growth")).toBeInTheDocument();
		expect(await screen.findByText("No earlier results for this address.")).toBeInTheDocument();
		expect(results).not.toHaveBeenCalled();
	});
});
