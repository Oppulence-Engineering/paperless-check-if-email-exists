// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import {
	AccountApi,
	JobsApi,
	OutcomesApi,
	PipelinesApi,
	TenantApi,
	V1Api,
	VerificationApi,
} from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { listPollInterval, OverviewPanel } from "./overview-panel";

const at = "2026-10-01T00:00:00Z";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

function list(id: number, name: string, status: string) {
	const item = { id, name, status, total_rows: 10, email_column: "email", created_at: at };
	return { ...item, original_filename: `${name}.csv`, completed_at: null };
}

function mockApi() {
	vi.spyOn(VerificationApi.prototype, "v1ListAlerts").mockResolvedValue({
		data: {
			total: 1,
			alerts: [
				{
					id: 7,
					type: "became_invalid",
					status: "unread",
					canonical_email: "ana@example.com",
					title: "ana@example.com changed",
					body: "Verification changed from valid to invalid",
					metadata: {},
					created_at: at,
					updated_at: at,
				},
			],
		},
	} as never);
	vi.spyOn(TenantApi.prototype, "v1GetTenantUsage").mockResolvedValue({
		data: {
			tenant_id: "77ce1681-7cb0-5f75-8987-6f870baff11d",
			tenant_name: "Team",
			plan_tier: "pro",
			used_this_period: 1200,
			period_reset_at: "2026-11-01T00:00:00+00:00",
			quota_unlimited: true,
		},
	} as never);
	vi.spyOn(TenantApi.prototype, "v1ListTenantDomains").mockResolvedValue({
		data: { domains: [{ domain: "example.com", is_active: true, is_verified: false }] },
	} as never);
	vi.spyOn(V1Api.prototype, "v1ListLists").mockResolvedValue({
		data: { total: 2, lists: [list(1, "Leads", "processing"), list(2, "Customers", "completed")] },
	} as never);
	vi.spyOn(V1Api.prototype, "v1GetList").mockImplementation((({ listId }: { listId: number }) =>
		Promise.resolve({
			data: {
				id: listId,
				job_id: listId * 10,
				name: listId === 1 ? "Leads" : "Customers",
				status: listId === 1 ? "processing" : "completed",
				total_rows: 10,
				email_column: "email",
				summary: {
					total_valid: 0,
					total_risky: 0,
					total_unknown: 0,
					total_invalid: 0,
					total_processed: 0,
				},
			},
		})) as never);
	vi.spyOn(V1Api.prototype, "v1ListSuppressions").mockResolvedValue({
		data: { entries: [], total: 3 },
	} as never);
	vi.spyOn(V1Api.prototype, "v1ReverificationStatus").mockResolvedValue({
		data: { enabled: false },
	} as never);
	vi.spyOn(JobsApi.prototype, "v1GetBulkJobProgress").mockResolvedValue({
		data: { total_records: 10, total_processed: 4 },
	} as never);
	vi.spyOn(JobsApi.prototype, "v1JobApprovalChecklist").mockResolvedValue({
		data: {
			job_id: 20,
			total_records: 10,
			categories: { valid: 9, risky: 1, unknown: 0, invalid: 0, unprocessed: 0 },
			risk_flags: {
				disposable_count: 0,
				catch_all_count: 1,
				role_account_count: 0,
				spam_trap_count: 0,
				suppressed_count: 0,
			},
			safe_to_send_count: 9,
			safe_to_send_pct: 90,
			recommendation: "List quality is excellent. Safe to send.",
			ready_to_send: true,
		},
	} as never);
	vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes").mockResolvedValue({
		data: { outcomes: [{ id: 1 }, { id: 2 }], limit: 500, offset: 0 },
	} as never);
	vi.spyOn(OutcomesApi.prototype, "v1ListProviderEndpoints").mockRejectedValue(
		new Error("Forbidden"),
	);
	vi.spyOn(PipelinesApi.prototype, "v1ListPipelines").mockRejectedValue(new Error("Down"));
	vi.spyOn(AccountApi.prototype, "listTenantApiKeys").mockResolvedValue({
		data: { api_keys: [] },
	} as never);
}

function renderPanel() {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<OverviewPanel aria-label="Overview" />
		</QueryClientProvider>,
	);
}

describe("OverviewPanel", () => {
	it("shows each section from its own request, and a failed request stays in its section", async () => {
		mockApi();
		renderPanel();

		expect(screen.getByRole("region", { name: "Overview" })).toHaveAttribute(
			"data-slot",
			"overview-panel",
		);
		expect(await screen.findByText("ana@example.com changed")).toBeInTheDocument();
		expect(await screen.findAllByText("Unlimited")).toHaveLength(2);
		expect(await screen.findByText("4 of 10 checked")).toBeInTheDocument();
		expect(await screen.findByText("Ready to send")).toBeInTheDocument();
		expect(screen.getByText("9 safe to send")).toBeInTheDocument();
		expect(screen.getByRole("link", { name: /Customers/ })).toHaveAttribute(
			"href",
			"/app/lists/detail?list=2",
		);
		expect(await screen.findByText("0 verified · 1 active of 1")).toBeInTheDocument();
		expect(screen.getByText("3")).toBeInTheDocument();
		expect(await screen.findByText("Pipelines could not be loaded.")).toBeInTheDocument();
		expect(screen.getByLabelText("Email address")).toHaveAttribute("name", "email");

		// The endpoint request failed, so that setup step is hidden; the other steps show.
		expect(await screen.findByText("Set up your workspace")).toBeInTheDocument();
		expect(screen.getByRole("link", { name: /Upload a list.*Done/ })).toBeInTheDocument();
		expect(screen.getByRole("link", { name: /Turn on re-checks.*To do/ })).toBeInTheDocument();
		expect(screen.queryByText("Connect delivery feedback")).not.toBeInTheDocument();
	});

	it("dismisses an alert through the alert API", async () => {
		mockApi();
		const update = vi
			.spyOn(VerificationApi.prototype, "v1UpdateAlert")
			.mockResolvedValue({ data: {} } as never);
		renderPanel();

		fireEvent.click(await screen.findByRole("button", { name: "Dismiss" }));
		await waitFor(() =>
			expect(update).toHaveBeenCalledWith({
				alertId: 7,
				updateAlertRequest: { status: "dismissed" },
			}),
		);
	});

	it("shows the last run status of a pipeline even when its stats have extra keys", async () => {
		mockApi();
		vi.spyOn(PipelinesApi.prototype, "v1ListPipelines").mockResolvedValue({
			data: {
				total: 1,
				pipelines: [
					{
						id: 1,
						tenant_id: "77ce1681-7cb0-5f75-8987-6f870baff11d",
						name: "Weekly cleanup",
						status: "active",
						source: { type: "push", token_id: "t1", accepted_format: "csv" },
						schedule: { cron: "0 9 * * 1", timezone: "UTC" },
						verification: {},
						policy: {},
						delivery: {},
						last_run_id: 3,
						created_at: at,
						updated_at: at,
					},
				],
			},
		} as never);
		vi.spyOn(PipelinesApi.prototype, "v1GetPipelineRun").mockResolvedValue({
			data: { id: 3, status: "completed", stats: { accepted_rows: 4 } },
		} as never);
		renderPanel();

		expect(await screen.findByRole("link", { name: /Weekly cleanup.*Completed/ })).toHaveAttribute(
			"href",
			"/app/pipelines",
		);
	});

	it("polls a list only while the backend checks its rows", () => {
		expect(listPollInterval("uploading")).toBe(5_000);
		expect(listPollInterval("processing")).toBe(5_000);
		expect(listPollInterval("completed")).toBe(false);
		expect(listPollInterval("failed")).toBe(false);
		expect(listPollInterval(undefined)).toBe(false);
	});
});
