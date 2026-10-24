// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { PipelinesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { PipelineRuns } from "./pipeline-runs";

const run = {
	id: 5,
	pipeline_id: 1,
	tenant_id: "9a8b3676-6178-5756-9e7d-1172c6bd5408",
	trigger_type: "manual",
	status: "failed",
	source_snapshot: { list_id: 14, type: "list_snapshot" },
	stats: { total_processed: 4, total_risky: 2 },
	billed_emails: 4,
	delivery_status: "delivered",
	delivery_attempts: 1,
	created_at: "2026-10-18T05:28:45.252340Z",
	updated_at: "2026-10-18T05:28:53.465118Z",
};

describe("PipelineRuns", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("lists each run with its trigger, status, delivery, and counts", async () => {
		const list = vi.spyOn(PipelinesApi.prototype, "v1ListPipelineRuns"); // gitleaks:allow
		list.mockResolvedValue({ data: { runs: [run], total: 1 } } as never);
		const onInspect = vi.fn();
		render(
			<QueryClientProvider client={new QueryClient()}>
				<PipelineRuns onInspect={onInspect} pipelineId={1} />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("Run 5 · Manual run")).toBeInTheDocument();
		expect(screen.getByText("Failed")).toBeInTheDocument();
		expect(screen.getByText(/Webhook delivered · 4 checked · 2 risky/)).toBeInTheDocument();
		expect(list).toHaveBeenCalledWith(
			{ pipelineId: 1, limit: 20, offset: 0 },
			{ signal: expect.any(AbortSignal) },
		);

		fireEvent.click(screen.getByRole("button", { name: "Inspect" }));
		expect(onInspect).toHaveBeenCalledWith(5);
	});

	it("says when there are no runs", async () => {
		const list = vi.spyOn(PipelinesApi.prototype, "v1ListPipelineRuns"); // gitleaks:allow
		list.mockResolvedValue({ data: { runs: [], total: 0 } } as never);
		render(
			<QueryClientProvider client={new QueryClient()}>
				<PipelineRuns pipelineId={1} />
			</QueryClientProvider>,
		);
		expect(await screen.findByText("No runs yet.")).toBeInTheDocument();
	});
});
