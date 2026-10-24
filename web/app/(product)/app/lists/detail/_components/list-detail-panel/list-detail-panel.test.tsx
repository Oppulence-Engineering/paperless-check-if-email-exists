// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { CommentsApi, JobsApi, ListsApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

import { ListDetailPanel } from "./list-detail-panel";

const push = vi.fn();
vi.mock("next/navigation", () => ({
	useRouter: () => ({ push }),
	useSearchParams: () => new URLSearchParams(),
}));

beforeEach(() => {
	vi.stubGlobal(
		"IntersectionObserver",
		class {
			observe() {}
			disconnect() {}
		},
	);
});

afterEach(() => {
	vi.unstubAllGlobals();
	cleanup();
	vi.restoreAllMocks();
	push.mockReset();
});

it("uses unique completed quality counts instead of duplicate and rejected row totals", async () => {
	vi.spyOn(V1Api.prototype, "v1GetList").mockResolvedValue({
		data: {
			id: 5,
			name: "Campaign",
			original_filename: "campaign.csv",
			total_rows: 5,
			status: "completed",
			email_column: "email",
			source_key: "hubspot",
			job_id: 9,
			unique_emails: 2,
			deduplicated_count: 1,
			summary: {
				total_valid: 3,
				total_risky: 0,
				total_unknown: 0,
				total_invalid: 2,
				total_processed: 5,
			},
		},
	} as never);
	vi.spyOn(V1Api.prototype, "v1ListLists").mockResolvedValue({
		data: {
			lists: [
				{
					id: 5,
					name: "Campaign",
					original_filename: "campaign.csv",
					email_column: "email",
					status: "completed",
					total_rows: 5,
					created_at: "2026-10-01T00:00:00Z",
					completed_at: "2026-10-01T00:01:00Z",
				},
			],
			total: 1,
		},
	} as never);
	vi.spyOn(ListsApi.prototype, "v1ListQuality").mockResolvedValue({
		data: {
			total_rows: 5,
			processed: 2,
			avg_score: 91.5,
			quality_grade: "A",
			safe_to_send_count: 2,
			safe_to_send_pct: 100,
			categories: { valid: 2, risky: 0, unknown: 0, invalid: 0 },
			risk_breakdown: { disposable: 0, catch_all: 0, role_account: 0, spam_trap: 0, full_inbox: 0 },
		},
	} as never);
	vi.spyOn(JobsApi.prototype, "v1GetJobStatus").mockResolvedValue({
		data: {
			job_id: 9,
			status: "completed",
			created_at: "2026-10-01T00:00:00Z",
			total_records: 2,
			task_summary: { completed: 2 },
		},
	} as never);
	vi.spyOn(JobsApi.prototype, "v1JobApprovalChecklist").mockResolvedValue({
		data: {
			job_id: 9,
			total_records: 2,
			categories: { valid: 2, risky: 0, unknown: 0, invalid: 0, unprocessed: 0 },
			risk_flags: {
				disposable_count: 0,
				catch_all_count: 0,
				role_account_count: 0,
				spam_trap_count: 0,
				suppressed_count: 0,
			},
			safe_to_send_count: 2,
			safe_to_send_pct: 100,
			recommendation: "List quality is excellent.",
			ready_to_send: true,
		},
	} as never);
	vi.spyOn(CommentsApi.prototype, "v1ListComments").mockResolvedValue({
		data: { comments: [] },
	} as never);
	const remove = vi.spyOn(V1Api.prototype, "v1DeleteList").mockResolvedValue({} as never);
	vi.spyOn(window, "confirm").mockReturnValue(true);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<ListDetailPanel accepted listId={5} />
		</QueryClientProvider>,
	);
	expect(await screen.findByRole("heading", { name: "Campaign" })).toBeInTheDocument();
	expect(screen.queryByText(/List accepted/)).not.toBeInTheDocument();
	expect(
		await screen.findByText("campaign.csv · Email column: email · Source: hubspot"),
	).toBeInTheDocument();
	expect(await screen.findByText("Grade A")).toBeInTheDocument();
	const qualitySection = screen.getByRole("heading", { name: "List quality" }).parentElement;
	if (!qualitySection) throw new Error("List quality section is missing");
	const quality = within(qualitySection);
	expect(quality.getByText("Valid").parentElement).toHaveTextContent("Valid2");
	expect(quality.getByText("Invalid").parentElement).toHaveTextContent("Invalid0");
	expect(quality.getByText(/2 unique addresses checked/)).toBeInTheDocument();
	expect(screen.getByText("Rows processed")).toBeInTheDocument();
	expect(await screen.findByText("Ready to send")).toBeInTheDocument();
	expect(screen.getByRole("link", { name: "Bulk job 9" })).toHaveAttribute(
		"href",
		"/app/jobs?job=9",
	);
	expect(screen.getAllByRole("tab").map((tab) => tab.textContent)).toEqual([
		"Review",
		"Rows",
		"Clean & export",
		"Problems",
		"Compare",
	]);

	fireEvent.click(screen.getByRole("button", { name: "Delete list" }));
	await waitFor(() => expect(remove).toHaveBeenCalledWith({ listId: 5 }));
	await waitFor(() => expect(push).toHaveBeenCalledWith("/app/lists"));
});
