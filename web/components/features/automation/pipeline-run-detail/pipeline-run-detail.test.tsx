// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, render, screen } from "@testing-library/react";
import { PipelinesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { V1GetPipelineRun200Response } from "@/lib/api/generated/zod/pipelines/pipelines";

import { pipelineRunQuery, PipelineRunDetail, runCounts } from "./pipeline-run-detail";

// The shape of a live run from GET /v1/pipelines/1/runs/1.
const run = {
	id: 1,
	pipeline_id: 1,
	tenant_id: "9a8b3676-6178-5756-9e7d-1172c6bd5408",
	trigger_type: "manual",
	status: "completed",
	started_at: "2026-10-18T05:28:45.260845Z",
	completed_at: "2026-10-18T05:28:45.694637Z",
	job_id: 16,
	list_id: 15,
	source_snapshot: { list_id: 14, type: "list_snapshot" },
	stats: {
		source_name: "Probe list",
		source_row_count: 2,
		selected_unique_emails: 2,
		total_processed: 2,
		total_risky: 1,
		total_invalid: 1,
		total_valid: 0,
		trigger_reason: "probe",
	},
	billed_emails: 2,
	result_location: { download_url: "/v1/lists/15/download" },
	delivery_status: "retry_scheduled",
	delivery_attempts: 2,
	delivery_error: "Webhook returned 500",
	created_at: "2026-10-18T05:28:45.252340Z",
	updated_at: "2026-10-18T05:28:53.465118Z",
};

describe("PipelineRunDetail", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the stats, the delivery, the trigger, and the result links", async () => {
		const get = vi.spyOn(PipelinesApi.prototype, "v1GetPipelineRun"); // gitleaks:allow
		get.mockResolvedValue({ data: run } as never);
		render(
			<QueryClientProvider client={new QueryClient()}>
				<PipelineRunDetail pipelineId={1} runId={1} />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("Run 1")).toBeInTheDocument();
		expect(screen.getByRole("status")).toHaveTextContent("Completed");
		expect(screen.getByText("Manual run")).toBeInTheDocument();
		expect(screen.getByText("probe")).toBeInTheDocument();
		expect(screen.getByText("Probe list")).toBeInTheDocument();
		expect(screen.getByText("Webhook retry scheduled")).toBeInTheDocument();
		expect(screen.getByRole("alert")).toHaveTextContent("Webhook returned 500");
		expect(screen.getByRole("link", { name: "Download results" })).toHaveAttribute(
			"href",
			"/api/backend/v1/lists/15/download",
		);
		expect(screen.getByRole("link", { name: "Open result list" })).toHaveAttribute(
			"href",
			"/app/lists/detail?list=15",
		);
		expect(screen.getByRole("link", { name: "View job results" })).toHaveAttribute(
			"href",
			"/app/jobs?job=16",
		);
		expect(get).toHaveBeenCalledWith(
			{ pipelineId: 1, runId: 1 },
			{ signal: expect.any(AbortSignal) },
		);
	});

	it("polls only while the run is active", () => {
		const { refetchInterval } = pipelineRunQuery(1, 1);
		const parsed = V1GetPipelineRun200Response.parse(run);
		expect(refetchInterval({ state: { data: { ...parsed, status: "running" } } })).toBe(5_000);
		expect(refetchInterval({ state: { data: { ...parsed, status: "completed" } } })).toBe(false);
		expect(refetchInterval({ state: { data: { ...parsed, status: "failed" } } })).toBe(false);
	});

	it("summarises the counts that have a value", () => {
		expect(runCounts(V1GetPipelineRun200Response.parse(run).stats)).toBe(
			"2 checked · 1 risky · 1 invalid",
		);
		expect(runCounts({ accepted_rows: 3 })).toBe("3 accepted");
	});
});
