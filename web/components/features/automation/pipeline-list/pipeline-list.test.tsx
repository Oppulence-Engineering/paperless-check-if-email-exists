// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { PipelinesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { PipelineList } from "./pipeline-list";

const base = {
	tenant_id: "9a8b3676-6178-5756-9e7d-1172c6bd5408",
	schedule: { cron: "0 9 * * 1", timezone: "UTC" },
	verification: { delta_mode: false },
	policy: { missed_run_window_hours: 24 },
	delivery: { dashboard: true, max_attempts: 5, retry_backoff_seconds: 300 },
	created_at: "2026-10-18T05:28:45.213779Z",
	updated_at: "2026-10-18T05:28:45.213779Z",
};
const pipelines = [
	{
		...base,
		id: 1,
		name: "Weekly list",
		status: "active",
		source: { type: "list_snapshot", list_id: 14 },
		next_run_at: "2026-10-19T09:00:00Z",
		last_run_id: 3,
	},
	{
		...base,
		id: 2,
		name: "CRM push",
		status: "paused",
		source: { type: "push", token_id: "crm", accepted_format: "json" },
	},
];
const lastRun = {
	id: 3,
	pipeline_id: 1,
	tenant_id: base.tenant_id,
	trigger_type: "schedule",
	status: "completed",
	source_snapshot: { list_id: 14, type: "list_snapshot" },
	stats: { total_processed: 2, total_valid: 1, total_invalid: 1 },
	billed_emails: 2,
	delivery_status: "not_requested",
	delivery_attempts: 0,
	created_at: "2026-10-18T05:28:45.252340Z",
	updated_at: "2026-10-18T05:28:53.465118Z",
};

function renderList(onOpen = vi.fn()) {
	render(
		<QueryClientProvider client={new QueryClient()}>
			<PipelineList onOpen={onOpen} />
		</QueryClientProvider>,
	);
	return onOpen;
}

describe("PipelineList", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the status, the schedule, the next run, and the last run with its counts", async () => {
		const list = vi.spyOn(PipelinesApi.prototype, "v1ListPipelines"); // gitleaks:allow
		list.mockResolvedValue({ data: { pipelines, total: 2 } } as never);
		const run = vi.spyOn(PipelinesApi.prototype, "v1GetPipelineRun"); // gitleaks:allow
		run.mockResolvedValue({ data: lastRun } as never);
		const onOpen = renderList();

		expect(await screen.findByText("Weekly list")).toBeInTheDocument();
		expect(screen.getByText(/Saved list · 0 9 \* \* 1 \(UTC\) · Next run/)).toBeInTheDocument();
		expect(screen.getByText("Push API · Runs when your systems push rows")).toBeInTheDocument();
		expect(screen.getByText("Active", { selector: "span" })).toBeInTheDocument();
		expect(screen.getByText("Paused", { selector: "span" })).toBeInTheDocument();
		expect(await screen.findByText("Completed")).toBeInTheDocument();
		expect(screen.getByText(/2 checked · 1 valid · 1 invalid/)).toBeInTheDocument();
		expect(screen.getByText("No runs yet")).toBeInTheDocument();
		expect(run).toHaveBeenCalledWith({ pipelineId: 1, runId: 3 });

		fireEvent.click(screen.getByRole("button", { name: "Open CRM push" }));
		expect(onOpen).toHaveBeenCalledWith(2);
	});

	it("filters by status", async () => {
		const list = vi.spyOn(PipelinesApi.prototype, "v1ListPipelines"); // gitleaks:allow
		list.mockResolvedValue({ data: { pipelines: [pipelines[1]], total: 1 } } as never);
		renderList();

		fireEvent.change(await screen.findByLabelText("Pipeline status"), {
			target: { value: "paused" },
		});
		await waitFor(() =>
			expect(list).toHaveBeenLastCalledWith({ status: "paused", limit: 20, offset: 0 }),
		);
	});

	it("shows an empty state when there are no pipelines", async () => {
		const list = vi.spyOn(PipelinesApi.prototype, "v1ListPipelines"); // gitleaks:allow
		list.mockResolvedValue({ data: { pipelines: [], total: 0 } } as never);
		renderList();

		expect(await screen.findByText("No pipelines yet.")).toBeInTheDocument();
		expect(screen.queryByLabelText("Pipeline status")).not.toBeInTheDocument();
	});
});
