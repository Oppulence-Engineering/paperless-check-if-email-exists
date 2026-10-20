// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { ListsApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ListExport } from "./list-export";
import { chooseOption } from "@/quality/test-support/select";

vi.mock("@/lib/downloads", () => ({ saveFile: vi.fn() }));

const plan = {
	id: 7,
	status: "completed",
	options: {},
	summary_counts: { fixed: 1, safe: 0, review: 0, drop: 1 },
	created_at: "2026-10-18T04:00:00Z",
	preview_rows: [
		{
			id: 1,
			row_index: 4,
			classification: "drop",
			rule_id: "duplicate_email",
			confidence: "high",
			original_email: "maya@gmail.com",
			effective_email: "maya@gmail.com",
			reasons: [{ code: "duplicate_email", message: "The address is a duplicate." }],
		},
	],
};

describe("ListExport", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("creates a plan, shows its rows, and exports a partition", async () => {
		vi.spyOn(V1Api.prototype, "v1ListLists").mockResolvedValue({
			data: { lists: [], total: 0 },
		} as never);
		vi.spyOn(ListsApi.prototype, "v1GetRemediationPlan").mockRejectedValue(new Error("No plan"));
		const create = vi
			.spyOn(ListsApi.prototype, "v1CreateRemediationPlan")
			.mockResolvedValue({ data: plan } as never);
		const createExport = vi
			.spyOn(ListsApi.prototype, "v1CreateRemediationExport")
			.mockResolvedValue({ data: { id: 11 } } as never);
		const download = vi
			.spyOn(ListsApi.prototype, "v1DownloadRemediationExport")
			.mockResolvedValue({ data: new Blob(["email\n"]) } as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<ListExport listId={3} />
			</QueryClientProvider>,
		);
		fireEvent.click(screen.getByLabelText("Drop duplicate addresses"));
		fireEvent.click(screen.getByRole("button", { name: "Create remediation plan" }));
		await waitFor(() =>
			expect(create).toHaveBeenCalledWith({
				listId: 3,
				requestBody: {
					normalize_emails: true,
					apply_domain_typos: true,
					deduplicate: false,
					drop_suppressed: true,
					allow_partial: false,
				},
			}),
		);
		expect(await screen.findByText("The address is a duplicate.")).toBeInTheDocument();
		expect(screen.getByText("Duplicate email")).toBeInTheDocument();

		await chooseOption("Rows to export", "Drop");
		fireEvent.click(screen.getByRole("button", { name: "Download export" }));
		await waitFor(() => expect(download).toHaveBeenCalled());
		expect(createExport).toHaveBeenCalledWith({
			listId: 3,
			requestBody: { plan_id: 7, partitions: ["drop"], format: "csv" },
		});
		expect(download.mock.calls[0]?.[0]).toEqual({ listId: 3, exportId: 11 });
	});

	it("downloads newly invalid rows against an earlier list", async () => {
		vi.spyOn(V1Api.prototype, "v1ListLists").mockResolvedValue({
			data: {
				lists: [
					{
						id: 2,
						name: "Spring",
						original_filename: "spring.csv",
						email_column: "email",
						status: "completed",
						total_rows: 4,
						created_at: "2026-10-01T00:00:00Z",
						completed_at: "2026-10-01T00:01:00Z",
					},
				],
				total: 1,
			},
		} as never);
		vi.spyOn(ListsApi.prototype, "v1GetRemediationPlan").mockRejectedValue(new Error("No plan"));
		const download = vi
			.spyOn(V1Api.prototype, "v1DownloadList")
			.mockResolvedValue({ data: new Blob(["email\n"]) } as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<ListExport listId={3} />
			</QueryClientProvider>,
		);
		await chooseOption("Rows to include", "Newly invalid since an earlier list");
		expect(screen.getByRole("button", { name: "Download results CSV" })).toBeDisabled();
		await chooseOption("Earlier list", "Spring");
		fireEvent.click(screen.getByRole("button", { name: "Download results CSV" }));
		await waitFor(() =>
			expect(download).toHaveBeenCalledWith(
				{ listId: 3, format: "csv", filter: "newly_invalid", changedSinceListId: 2 },
				{ responseType: "blob" },
			),
		);
	});
});
