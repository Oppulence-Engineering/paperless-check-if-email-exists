// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { PipelinesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { PipelineForm } from "./pipeline-form";

const pipeline = {
	id: 7,
	tenant_id: "9a8b3676-6178-5756-9e7d-1172c6bd5408",
	name: "Weekly",
	status: "active" as const,
	source: { type: "list_snapshot" as const, list_id: 4 },
	schedule: { cron: "0 9 * * 1", timezone: "UTC" },
	verification: { delta_mode: true, freshness_days: 30 },
	policy: { missed_run_window_hours: 12, policy_id: 3 },
	delivery: {
		dashboard: false,
		webhook: { url: "https://hooks.example.com/run", headers: { "X-Team": "growth" } },
		max_attempts: 3,
		retry_backoff_seconds: 60,
	},
	created_at: "2026-10-18T05:28:45.213779Z",
	updated_at: "2026-10-18T05:28:45.213779Z",
};

function renderForm(props: Parameters<typeof PipelineForm>[0] = {}) {
	render(
		<QueryClientProvider client={new QueryClient()}>
			<PipelineForm {...props} />
		</QueryClientProvider>,
	);
}

function type(label: string, value: string) {
	fireEvent.change(screen.getByLabelText(label), { target: { value } });
}

describe("PipelineForm", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("creates a paused list pipeline with verification, webhook, and retry settings", async () => {
		const create = vi.spyOn(PipelinesApi.prototype, "v1CreatePipeline"); // gitleaks:allow
		create.mockResolvedValue({ data: { ...pipeline, status: "paused" } } as never);
		const onSaved = vi.fn();
		renderForm({ onSaved });

		type("Name", " Weekly ");
		type("List ID", "4");
		type("Missed-run window (hours)", "12");
		fireEvent.click(screen.getByLabelText("Check only new or changed addresses"));
		type("Check again after (days)", "30");
		type("Webhook URL", "https://hooks.example.com/run");
		type("Webhook headers", "X-Team: growth");
		type("Delivery attempts", "3");
		type("Wait between attempts (seconds)", "60");
		fireEvent.click(screen.getByRole("button", { name: "Create paused pipeline" }));

		await waitFor(() =>
			expect(create).toHaveBeenCalledWith({
				createPipelineInput: {
					name: "Weekly",
					source: { type: "list_snapshot", list_id: 4 },
					schedule: { cron: "0 9 * * 1", timezone: "UTC" },
					verification: { delta_mode: true, freshness_days: 30 },
					policy: { missed_run_window_hours: 12 },
					delivery: {
						dashboard: true,
						webhook: { url: "https://hooks.example.com/run", headers: { "X-Team": "growth" } },
						max_attempts: 3,
						retry_backoff_seconds: 60,
					},
					status: "paused",
				},
			}),
		);
		await waitFor(() => expect(onSaved).toHaveBeenCalled());
	});

	it("offers only the list and push sources and hides the schedule for push", () => {
		renderForm();
		fireEvent.click(screen.getByLabelText("Source"));
		expect(screen.getAllByRole("option").map((option) => option.textContent)).toEqual([
			"Saved list",
			"Push API",
		]);
		fireEvent.click(screen.getByRole("option", { name: "Push API" }));
		expect(screen.getByLabelText("Push token ID")).toBeInTheDocument();
		expect(screen.queryByLabelText("Cron schedule")).not.toBeInTheDocument();
	});

	it("shows a readable error and does not call the API for a bad header", async () => {
		const create = vi.spyOn(PipelinesApi.prototype, "v1CreatePipeline"); // gitleaks:allow
		renderForm();
		type("Name", "Weekly");
		type("List ID", "4");
		type("Webhook headers", "no colon");
		fireEvent.click(screen.getByRole("button", { name: "Create paused pipeline" }));
		expect(await screen.findByRole("alert")).toHaveTextContent(
			'Write each webhook header as "Name: value"',
		);
		expect(create).not.toHaveBeenCalled();
	});

	it("edits a pipeline and keeps the settings that the form does not show", async () => {
		const update = vi.spyOn(PipelinesApi.prototype, "v1UpdatePipeline"); // gitleaks:allow
		update.mockResolvedValue({ data: pipeline } as never);
		renderForm({ pipeline });

		expect(screen.getByLabelText("Webhook headers")).toHaveValue("X-Team: growth");
		type("Name", "Monday check");
		fireEvent.click(screen.getByRole("button", { name: "Save pipeline" }));

		await waitFor(() =>
			expect(update).toHaveBeenCalledWith({
				pipelineId: 7,
				updatePipelineInput: {
					name: "Monday check",
					source: { type: "list_snapshot", list_id: 4 },
					schedule: { cron: "0 9 * * 1", timezone: "UTC" },
					verification: { delta_mode: true, freshness_days: 30 },
					policy: { missed_run_window_hours: 12, policy_id: 3 },
					delivery: {
						dashboard: false,
						webhook: { url: "https://hooks.example.com/run", headers: { "X-Team": "growth" } },
						max_attempts: 3,
						retry_backoff_seconds: 60,
					},
				},
			}),
		);
	});
});
