// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { PipelinesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { PipelineDetail } from "./pipeline-detail";

const base = {
	tenant_id: "9a8b3676-6178-5756-9e7d-1172c6bd5408",
	schedule: { cron: "0 9 * * 1", timezone: "UTC" },
	verification: { delta_mode: true, freshness_days: 30 },
	policy: { missed_run_window_hours: 12 },
	delivery: {
		dashboard: true,
		webhook: { url: "https://hooks.example.com/run", headers: { Authorization: "secret" } },
		max_attempts: 3,
		retry_backoff_seconds: 60,
	},
	created_at: "2026-10-18T05:28:45.213779Z",
	updated_at: "2026-10-18T05:28:45.213779Z",
};
const listPipeline = {
	...base,
	id: 1,
	name: "Weekly list",
	status: "active",
	source: { type: "list_snapshot", list_id: 14 },
	next_run_at: "2026-10-19T09:00:00Z",
};
const pushPipeline = {
	...base,
	id: 2,
	name: "CRM push",
	status: "active",
	source: { type: "push", token_id: "crm", accepted_format: "json" },
};

function renderDetail(pipelineId: number, onRun = vi.fn()) {
	render(
		<QueryClientProvider client={new QueryClient()}>
			<PipelineDetail onRun={onRun} pipelineId={pipelineId} />
		</QueryClientProvider>,
	);
	return onRun;
}

describe("PipelineDetail", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the settings and starts a run on demand", async () => {
		const get = vi.spyOn(PipelinesApi.prototype, "v1GetPipeline"); // gitleaks:allow
		get.mockResolvedValue({ data: listPipeline } as never);
		const trigger = vi.spyOn(PipelinesApi.prototype, "v1TriggerPipeline"); // gitleaks:allow
		trigger.mockResolvedValue({ data: { run_id: 9, status: "queued" } } as never);
		vi.spyOn(window, "confirm").mockReturnValue(true);
		const onRun = renderDetail(1);

		expect(await screen.findByRole("heading", { name: "Weekly list" })).toBeInTheDocument();
		expect(screen.getByText("List 14")).toBeInTheDocument();
		expect(screen.getByText("12 hours")).toBeInTheDocument();
		expect(screen.getByText("Only new or changed addresses")).toBeInTheDocument();
		expect(screen.getByText("30 days")).toBeInTheDocument();
		expect(screen.getByText("https://hooks.example.com/run")).toBeInTheDocument();
		expect(screen.getByText("Authorization")).toBeInTheDocument();
		expect(screen.queryByText("secret")).not.toBeInTheDocument();
		expect(screen.getByText("3, 60 seconds apart")).toBeInTheDocument();
		expect(screen.getByRole("button", { name: "Pause" })).toBeEnabled();
		expect(screen.getByRole("button", { name: "Resume" })).toBeDisabled();

		fireEvent.click(screen.getByRole("button", { name: "Run now" }));
		await waitFor(() => expect(onRun).toHaveBeenCalledWith(9));
		expect(trigger).toHaveBeenCalledWith({
			pipelineId: 1,
			triggerPipelineInput: { reason: "Manual run from workspace" },
		});
		expect(await screen.findByText("Run 9 started.")).toBeInTheDocument();
	});

	it("pushes rows and shows the run ID and the accepted rows", async () => {
		const get = vi.spyOn(PipelinesApi.prototype, "v1GetPipeline"); // gitleaks:allow
		get.mockResolvedValue({ data: pushPipeline } as never);
		const push = vi.spyOn(PipelinesApi.prototype, "v1PushPipeline"); // gitleaks:allow
		push.mockResolvedValue({
			data: { batch_id: 4, run_id: 11, status: "running", accepted_rows: 2, replayed: false },
		} as never);
		const onRun = renderDetail(2);

		expect(await screen.findByText("Push token crm")).toBeInTheDocument();
		expect(screen.queryByRole("button", { name: "Run now" })).not.toBeInTheDocument();
		fireEvent.change(screen.getByLabelText("Push email addresses"), {
			target: { value: "a@example.com\nb@example.com" },
		});
		fireEvent.click(screen.getByRole("button", { name: "Push rows" }));

		expect(await screen.findByText(/Run 11 accepted 2 rows/)).toBeInTheDocument();
		expect(onRun).toHaveBeenCalledWith(11);
		expect(push).toHaveBeenCalledWith({
			pipelineId: 2,
			idempotencyKey: expect.any(String) as string,
			pushPipelineInput: {
				rows: [{ email: "a@example.com" }, { email: "b@example.com" }],
				email_column: "email",
			},
		});
	});

	it("asks to resume a paused push pipeline before a push", async () => {
		const get = vi.spyOn(PipelinesApi.prototype, "v1GetPipeline"); // gitleaks:allow
		get.mockResolvedValue({ data: { ...pushPipeline, status: "paused" } } as never);
		renderDetail(2);

		expect(
			await screen.findByText("Resume the pipeline before you push rows."),
		).toBeInTheDocument();
		fireEvent.change(screen.getByLabelText("Push email addresses"), {
			target: { value: "a@example.com" },
		});
		expect(screen.getByRole("button", { name: "Push rows" })).toBeDisabled();
	});
});
