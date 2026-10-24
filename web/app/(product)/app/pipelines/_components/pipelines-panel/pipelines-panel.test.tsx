// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { PipelinesApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { chooseOption } from "@/quality/test-support/select";

import { PipelinesPanel } from "./pipelines-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

const created = {
	id: 7,
	tenant_id: "9a8b3676-6178-5756-9e7d-1172c6bd5408",
	name: "Weekly",
	status: "paused",
	source: { type: "list_snapshot", list_id: 4 },
	schedule: { cron: "0 9 * * 1", timezone: "UTC" },
	verification: { delta_mode: false },
	policy: { missed_run_window_hours: 24 },
	delivery: { dashboard: true, max_attempts: 5, retry_backoff_seconds: 300 },
	created_at: "2026-10-18T05:28:45.213779Z",
	updated_at: "2026-10-18T05:28:45.213779Z",
};

it("creates a paused list pipeline for review", async () => {
	vi.spyOn(V1Api.prototype, "v1ListLists").mockResolvedValue({
		data: {
			lists: [
				{
					id: 4,
					name: "Launch",
					original_filename: "launch.csv",
					email_column: "email",
					status: "completed",
					total_rows: 2,
					source_key: null,
					completed_at: null,
					created_at: "2026-10-01T00:00:00Z",
				},
			],
			total: 1,
		},
	} as never);
	const list = vi.spyOn(PipelinesApi.prototype, "v1ListPipelines"); // gitleaks:allow
	list.mockResolvedValue({ data: { pipelines: [], total: 0 } } as never);
	const create = vi.spyOn(PipelinesApi.prototype, "v1CreatePipeline"); // gitleaks:allow
	create.mockResolvedValue({ data: created } as never);
	const get = vi.spyOn(PipelinesApi.prototype, "v1GetPipeline"); // gitleaks:allow
	get.mockResolvedValue({ data: created } as never);
	const runs = vi.spyOn(PipelinesApi.prototype, "v1ListPipelineRuns"); // gitleaks:allow
	runs.mockResolvedValue({ data: { runs: [], total: 0 } } as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<PipelinesPanel />
		</QueryClientProvider>,
	);
	expect(await screen.findByText("No pipelines yet.")).toBeInTheDocument();
	fireEvent.click(screen.getByRole("button", { name: "New pipeline" }));
	fireEvent.change(await screen.findByLabelText("Name"), { target: { value: "Weekly" } });
	await chooseOption("Saved list", "Launch");
	fireEvent.click(screen.getByRole("button", { name: "Create paused pipeline" }));
	await waitFor(() =>
		expect(create).toHaveBeenCalledWith({
			createPipelineInput: {
				name: "Weekly",
				source: { type: "list_snapshot", list_id: 4 },
				schedule: { cron: "0 9 * * 1", timezone: "UTC" },
				verification: { delta_mode: false, freshness_days: null },
				policy: { missed_run_window_hours: 24 },
				delivery: { dashboard: true, webhook: null, max_attempts: 5, retry_backoff_seconds: 300 },
				status: "paused",
			},
		}),
	);
	expect(await screen.findByText(/Pipeline created in paused state/)).toBeInTheDocument();
	expect(await screen.findByRole("heading", { name: "Weekly" })).toBeInTheDocument();
	expect(await screen.findByText("No runs yet.")).toBeInTheDocument();
});
