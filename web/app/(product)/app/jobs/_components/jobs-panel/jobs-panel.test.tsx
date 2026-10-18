// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { JobsApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { JobsPanel, parseEmailBatch } from "./jobs-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("turns pasted addresses into one bulk job and opens its progress", async () => {
	const create = vi
		.spyOn(V1Api.prototype, "v1CreateBulkJob")
		.mockResolvedValue({ data: { job_id: 9 } } as never);
	vi.spyOn(JobsApi.prototype, "v1GetJobStatus").mockResolvedValue({
		data: {
			job_id: 9,
			status: "running",
			total_records: 2,
			created_at: "2026-10-18T04:00:00Z",
			task_summary: { queued: 2, completed: 0 },
		},
	} as never);
	vi.spyOn(JobsApi.prototype, "v1GetBulkJobProgress").mockResolvedValue({
		data: { created_at: "2026-10-18T04:00:00Z", total_records: 2, total_processed: 0 },
	} as never);
	// gitleaks reads the method name as a key.
	const getJobResults = vi.spyOn(JobsApi.prototype, "v1GetJobResults"); // gitleaks:allow
	getJobResults.mockResolvedValue({
		data: { has_more: false, next_cursor: null, results: [] },
	} as never);
	vi.spyOn(JobsApi.prototype, "v1JobApprovalChecklist").mockRejectedValue(new Error("Not ready"));
	const client = new QueryClient({
		defaultOptions: { queries: { retry: false } },
	});
	render(
		<QueryClientProvider client={client}>
			<JobsPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("Email addresses"), {
		target: { value: "a@example.com\nb@example.com" },
	});
	fireEvent.click(screen.getByRole("button", { name: "Create job" }));
	await waitFor(() =>
		expect(create).toHaveBeenCalledWith({
			bulkCreateRequest: {
				input: ["a@example.com", "b@example.com"],
				source_key: undefined,
			},
		}),
	);
	expect(await screen.findByText(/Job 9 created/)).toBeInTheDocument();
	expect(await screen.findByText("2 records")).toBeInTheDocument();
	expect(screen.getByText("Running")).toBeInTheDocument();
	expect(await screen.findByText(/0 of 2 rows checked/)).toBeInTheDocument();
	expect(screen.getByRole("link", { name: "Share this job" })).toHaveAttribute(
		"href",
		"/app/jobs?job=9",
	);
});

it("keeps pasted separators from creating empty rows", () => {
	expect(parseEmailBatch(" a@example.com,\n; b@example.com  ")).toEqual([
		"a@example.com",
		"b@example.com",
	]);
});
