// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { ListsPanel } from "./lists-panel";

const push = vi.fn();
vi.mock("next/navigation", () => ({ useRouter: () => ({ push }) }));

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
	push.mockReset();
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
				{
					id: 8,
					name: "Signup",
					original_filename: "signup.csv",
					email_column: "email",
					source_key: null,
					status: "processing",
					total_rows: 10,
					created_at: "2026-09-23T00:00:00Z",
					completed_at: null,
				},
			],
			total: 2,
		},
	} as never);
	vi.spyOn(V1Api.prototype, "v1GetList").mockResolvedValue({
		data: {
			id: 8,
			job_id: 9,
			name: "Signup",
			status: "processing",
			total_rows: 10,
			email_column: "email",
			summary: {
				total_valid: 3,
				total_risky: 0,
				total_unknown: 0,
				total_invalid: 1,
				total_processed: 4,
			},
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
	expect(screen.getByText("Completed")).toBeInTheDocument();
	expect(screen.getByText("Processing")).toBeInTheDocument();
	expect(await screen.findByText("4 of 10")).toBeInTheDocument();
	expect(screen.getByRole("progressbar", { name: "Signup progress" })).toBeInTheDocument();
	expect(list).toHaveBeenCalledWith({ limit: 20, offset: 0 }, expect.any(Object));
});

it("uploads a CSV with its options and opens the new list's workspace", async () => {
	vi.spyOn(V1Api.prototype, "v1ListLists").mockResolvedValue({
		data: { lists: [], total: 0 },
	} as never);
	const upload = vi.spyOn(V1Api.prototype, "v1CreateList").mockResolvedValue({
		data: {
			email_column: "Work email",
			job_id: 5,
			list_id: 3,
			source_key: "hubspot",
			total_rows: 1,
		},
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
	fireEvent.change(screen.getByLabelText("Email column"), { target: { value: "Work email" } });
	fireEvent.change(screen.getByLabelText("Lead source"), { target: { value: "hubspot" } });
	expect(screen.getByRole("button", { name: "Upload" })).not.toBeDisabled();
	const form = screen.getByRole("button", { name: "Upload" }).closest("form");
	if (!form) throw new Error("Upload form missing");
	fireEvent.submit(form);
	await waitFor(() => expect(push).toHaveBeenCalledWith("/app/lists/detail?list=3&accepted=1"));
	expect(upload).toHaveBeenCalledWith(
		expect.objectContaining({
			file: expect.any(File),
			emailColumn: "Work email",
			sourceKey: "hubspot",
		}),
		expect.any(Object),
	);
});
