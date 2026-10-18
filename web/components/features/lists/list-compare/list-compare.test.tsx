// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { ListsApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ListCompare } from "./list-compare";

const empty = { count: 0, rows: [] };

describe("ListCompare", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("compares this list with a chosen earlier list", async () => {
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
		const diff = vi.spyOn(ListsApi.prototype, "v1DiffLists").mockResolvedValue({
			data: {
				base_list_id: 2,
				compare_list_id: 3,
				added: empty,
				removed: empty,
				unchanged: empty,
				improved: empty,
				degraded: empty,
				newly_risky: empty,
				newly_safe: empty,
				newly_invalid: {
					count: 1,
					rows: [
						{
							canonical_email: "maya@gmail.com",
							base_category: "valid",
							base_score: 90,
							compare_category: "invalid",
							compare_score: 0,
							change_type: "became_invalid",
						},
					],
				},
			},
		} as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<ListCompare listId={3} />
			</QueryClientProvider>,
		);
		await screen.findByRole("option", { name: "Spring" });
		fireEvent.change(screen.getByLabelText("Compare with"), { target: { value: "2" } });
		await waitFor(() =>
			expect(diff).toHaveBeenCalledWith({ baseListId: 2, compareListId: 3, limit: 50 }),
		);
		expect(await screen.findByText("maya@gmail.com")).toBeInTheDocument();
		expect(screen.getByText("Valid · 90")).toBeInTheDocument();
		expect(screen.getByText("Invalid · 0")).toBeInTheDocument();
	});
});
