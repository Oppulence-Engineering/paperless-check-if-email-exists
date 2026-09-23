// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { ListsPanel } from "./lists-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("shows tenant lists returned by the SDK", async () => {
	const list = vi.spyOn(V1Api.prototype, "v1ListLists").mockResolvedValue({
		data: {
			lists: [
				{
					id: 7,
					name: "Launch",
					original_filename: "launch.csv",
					email_column: "email",
					source_key: null,
					status: "completed",
					total_rows: 12,
					created_at: "2026-09-22T00:00:00Z",
					completed_at: "2026-09-22T00:01:00Z",
				},
			],
			total: 1,
		},
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<ListsPanel />
		</QueryClientProvider>,
	);
	expect(await screen.findByText("Launch")).toBeInTheDocument();
	expect(screen.getByText("launch.csv · 12 rows")).toBeInTheDocument();
	expect(list).toHaveBeenCalledWith({ limit: 20, offset: 0 }, expect.any(Object));
});

it("uploads a CSV with the SDK", async () => {
	vi.spyOn(V1Api.prototype, "v1ListLists").mockResolvedValue({
		data: { lists: [], total: 0 },
	} as never);
	const upload = vi.spyOn(V1Api.prototype, "v1CreateList").mockResolvedValue({
		data: { email_column: "email", job_id: 5, list_id: 3, source_key: null, total_rows: 1 },
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<ListsPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("CSV file"), {
		target: { files: [new File(["email\na@example.com\n"], "one.csv", { type: "text/csv" })] },
	});
	expect(screen.getByRole("button", { name: "Upload" })).not.toBeDisabled();
	const form = screen.getByRole("button", { name: "Upload" }).closest("form");
	if (!form) throw new Error("Upload form missing");
	fireEvent.submit(form);
	await waitFor(() => expect(upload).toHaveBeenCalled());
	expect(await screen.findByText(/List accepted/)).toBeInTheDocument();
	expect(upload).toHaveBeenCalledWith(
		expect.objectContaining({ file: expect.any(File) }),
		expect.any(Object),
	);
});
