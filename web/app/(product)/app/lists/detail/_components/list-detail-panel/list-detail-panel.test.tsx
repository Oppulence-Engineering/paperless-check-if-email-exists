// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { CommentsApi, ListsApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { ListDetailPanel } from "./list-detail-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("shows list quality and creates a safe remediation plan", async () => {
	vi.spyOn(V1Api.prototype, "v1GetList").mockResolvedValue({
		data: {
			id: 5,
			name: "Campaign",
			total_rows: 2,
			status: "completed",
			email_column: "email",
			job_id: 9,
		},
	} as never);
	vi.spyOn(ListsApi.prototype, "v1ListQuality").mockResolvedValue({
		data: {
			processed: 2,
			quality_grade: "A",
			safe_to_send_count: 2,
			safe_to_send_pct: 100,
			categories: { valid: 2 },
		},
	} as never);
	vi.spyOn(ListsApi.prototype, "v1GetRemediationPlan").mockRejectedValue(new Error("No plan"));
	const create = vi.spyOn(ListsApi.prototype, "v1CreateRemediationPlan").mockResolvedValue({
		data: { id: 7, status: "completed", summary_counts: { safe: 2 }, preview_rows: [] },
	} as never);
	vi.spyOn(CommentsApi.prototype, "v1ListComments").mockResolvedValue({
		data: { comments: [] },
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<ListDetailPanel listId={5} />
		</QueryClientProvider>,
	);
	expect(await screen.findByText("Campaign")).toBeInTheDocument();
	expect(await screen.findByText("Grade A")).toBeInTheDocument();
	fireEvent.click(screen.getByRole("button", { name: "Create remediation plan" }));
	await waitFor(() => expect(create).toHaveBeenCalledWith({ listId: 5, requestBody: {} }));
});
