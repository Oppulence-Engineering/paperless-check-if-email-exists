// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { AccountApi, OutcomesApi, V1Api, VerificationApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { listPollInterval, OverviewPanel } from "./overview-panel";

const at = "2026-10-01T00:00:00Z";
const ready = <T,>(data: T) => ({ status: "ready", data });
const snapshot = () => ({
	usage: {
		tenant_id: "77ce1681-7cb0-5f75-8987-6f870baff11d",
		tenant_name: "Team",
		plan_tier: "professional",
		used_this_period: 1200,
		period_reset_at: at,
		quota_unlimited: true,
	},
	alerts: ready({
		total: 1,
		alerts: [
			{
				id: 7,
				type: "became_invalid",
				status: "unread",
				canonical_email: "ana@example.com",
				title: "ana@example.com changed",
				metadata: {},
				created_at: at,
				updated_at: at,
			},
		],
	}),
	lists: ready([
		{
			id: 1,
			job_id: 10,
			name: "Leads",
			status: "processing",
			total_rows: 10,
			total_processed: 4,
			safe_to_send_count: 3,
			ready_to_send: false,
			quality_grade: "B",
		},
		{
			id: 2,
			job_id: 20,
			name: "Customers",
			status: "completed",
			total_rows: 10,
			total_processed: 10,
			safe_to_send_count: 9,
			ready_to_send: true,
			quality_grade: "A",
		},
	]),
	domains: ready({ total: 1, verified: 0, active: 1 }),
	suppressions: ready(3),
	outcomes: ready({ bounced: 501, complained: 2 }),
	reverification: ready({ enabled: false }),
	pipelines: { status: "unavailable" },
});
afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});
function mount(client = new QueryClient({ defaultOptions: { queries: { retry: false } } })) {
	return {
		...render(
			<QueryClientProvider client={client}>
				<OverviewPanel />
			</QueryClientProvider>,
		),
		client,
	};
}
it("renders populated sections from one request and reuses a warm snapshot", async () => {
	const overview = vi
		.spyOn(V1Api.prototype, "v1Overview")
		.mockResolvedValue({ data: snapshot() } as never);
	const detail = vi.spyOn(V1Api.prototype, "v1GetList");
	const keys = vi.spyOn(AccountApi.prototype, "listTenantApiKeys");
	const { unmount, client } = mount();
	expect(await screen.findByText("4 of 10 checked")).toBeInTheDocument();
	expect(screen.getByText("9 safe to send")).toBeInTheDocument();
	expect(screen.getByText("501")).toBeInTheDocument();
	expect(screen.getByText("Pipelines could not be loaded.")).toBeInTheDocument();
	expect(overview).toHaveBeenCalledTimes(1);
	expect(detail).not.toHaveBeenCalled();
	expect(keys).not.toHaveBeenCalled();
	unmount();
	mount(client);
	await screen.findByText("9 safe to send");
	expect(overview).toHaveBeenCalledTimes(1);
	expect(overview.mock.calls[0]?.[0]?.signal).toBeInstanceOf(AbortSignal);
});
it("refreshes after dismissing an alert", async () => {
	const overview = vi
		.spyOn(V1Api.prototype, "v1Overview")
		.mockResolvedValue({ data: snapshot() } as never);
	const update = vi
		.spyOn(VerificationApi.prototype, "v1UpdateAlert")
		.mockResolvedValue({ data: {} } as never);
	mount();
	fireEvent.click(await screen.findByRole("button", { name: "Dismiss" }));
	await waitFor(() =>
		expect(update).toHaveBeenCalledWith({
			alertId: 7,
			updateAlertRequest: { status: "dismissed" },
		}),
	);
	await waitFor(() => expect(overview).toHaveBeenCalledTimes(2));
});
it("loads optional setup only on expansion and keeps denied data unavailable", async () => {
	vi.spyOn(V1Api.prototype, "v1Overview").mockResolvedValue({
		data: { ...snapshot(), outcomes: { status: "forbidden" } },
	} as never);
	const endpoints = vi
		.spyOn(OutcomesApi.prototype, "v1ListProviderEndpoints")
		.mockRejectedValue(new Error("Forbidden"));
	vi.spyOn(AccountApi.prototype, "listTenantApiKeys").mockResolvedValue({
		data: { api_keys: [] },
	} as never);
	mount();
	await screen.findByText("9 safe to send");
	expect(endpoints).not.toHaveBeenCalled();
	expect(screen.getAllByText("Not available").length).toBeGreaterThan(0);
	fireEvent.click(screen.getByText("Workspace setup checklist"));
	expect(await screen.findByText("Set up your workspace")).toBeInTheDocument();
	expect(endpoints).toHaveBeenCalledTimes(1);
});
it("does not poll terminal lists", () => {
	expect(listPollInterval("processing")).toBe(5000);
	for (const state of ["completed", "failed", "cancelled", undefined])
		expect(listPollInterval(state)).toBe(false);
});
