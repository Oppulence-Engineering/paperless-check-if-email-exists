// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { SAMPLE_CHECK_RESULT } from "@/lib/verification/check-result";

import { FinderPanel, finderRefetchInterval } from "./finder-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("starts a finder job and shows the verified best match", async () => {
	const create = vi
		.spyOn(V1Api.prototype, "v1FindEmail")
		.mockResolvedValue({ data: { job_id: 11, status: "queued" } } as never);
	vi.spyOn(V1Api.prototype, "v1GetFindEmail").mockResolvedValue({
		data: {
			job_id: 11,
			bulk_job_id: 31,
			status: "completed",
			domain_has_mx: true,
			domain_is_catch_all: false,
			candidates_checked: 2,
			best_match: { email: "ada@example.com", score: 95, confidence: "high", pattern: "first" },
			results: [],
			alternatives: [],
		},
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<FinderPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("First name"), { target: { value: "Ada" } });
	fireEvent.change(screen.getByLabelText("Last name"), { target: { value: "Lovelace" } });
	fireEvent.change(screen.getByLabelText("Company domain"), { target: { value: "example.com" } });
	fireEvent.click(screen.getByRole("button", { name: "Find email" }));
	await waitFor(() =>
		expect(create).toHaveBeenCalledWith({
			findEmailRequest: {
				first_name: "Ada",
				last_name: "Lovelace",
				domain: "example.com",
				strategy: "parallel",
			},
		}),
	);
	expect(await screen.findByText("ada@example.com")).toBeInTheDocument();
	expect(screen.getByText("Pattern first · High confidence · Score 95/100")).toBeInTheDocument();
	expect(screen.getByRole("link", { name: "Open job 31" })).toHaveAttribute(
		"href",
		"/app/jobs?job=31",
	);
});

function renderJob(data: Record<string, unknown>) {
	vi.spyOn(V1Api.prototype, "v1GetFindEmail").mockResolvedValue({
		data: {
			job_id: 12,
			bulk_job_id: 32,
			domain_has_mx: true,
			domain_is_catch_all: false,
			candidates_checked: 2,
			best_match: null,
			...data,
		},
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<FinderPanel initialJobId={12} />
		</QueryClientProvider>,
	);
}

it("shows each candidate and its full result", async () => {
	const candidate = {
		pattern: "first.last",
		score: 94,
		category: "valid",
		sub_reason: "deliverable",
		is_reachable: "safe",
		confidence: { score: 80, level: "high", factors: [] },
	};
	renderJob({
		status: "completed",
		results: [
			{ ...candidate, email: "maya@brightlane.co", result: SAMPLE_CHECK_RESULT },
			{ ...candidate, email: "m.lane@brightlane.co", result: { input: "m.lane@brightlane.co" } },
		],
	});
	expect(await screen.findByText("m.lane@brightlane.co")).toBeInTheDocument();
	expect(screen.getAllByText("80/100")).toHaveLength(2);
	expect(screen.getAllByText("The mailbox accepts mail")).toHaveLength(2);
	expect(screen.getByText("Quality score")).toBeInTheDocument();
	expect(
		screen.getByText("The full result for this address could not be read."),
	).toBeInTheDocument();
});

it("shows a failed search and stops polling", async () => {
	renderJob({ status: "failed", results: [] });
	expect(await screen.findByRole("alert")).toHaveTextContent("This search failed.");
	expect(finderRefetchInterval("failed")).toBe(false);
	expect(finderRefetchInterval("completed")).toBe(false);
	expect(finderRefetchInterval("running")).toBe(5_000);
});
