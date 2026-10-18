// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { OutcomesApi, V1Api, VerificationApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { HistoryPanel } from "./history-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

function search(history: unknown[]) {
	const lookup = vi.spyOn(VerificationApi.prototype, "v1EmailHistory").mockResolvedValue({
		data: { email: "user@example.com", total: history.length, history },
	} as never);
	vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes").mockResolvedValue({
		data: { limit: 50, offset: 0, outcomes: [] },
	} as never);
	vi.spyOn(V1Api.prototype, "v1CheckSuppression").mockResolvedValue({
		data: { suppressed: false },
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<HistoryPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("Email address"), {
		target: { value: "USER@example.com" },
	});
	fireEvent.click(screen.getByRole("button", { name: "Search" }));
	return lookup;
}

it("looks up an address with the SDK and renders its history", async () => {
	const lookup = search([
		{
			job_id: 4,
			task_id: 41,
			score: 86,
			category: "valid",
			previous_category: "risky",
			previous_score: 60,
			reason_codes: ["deliverable"],
			recommendation_action: "send",
			policy_decision: "review",
			is_reachable: "safe",
			completed_at: "2026-09-22T12:00:00Z",
		},
	]);
	expect(await screen.findByText("The mailbox accepts mail")).toBeInTheDocument();
	expect(screen.getByText("Policy: Review")).toBeInTheDocument();
	expect(screen.getByText("Changed from Risky to Valid")).toBeInTheDocument();
	expect(screen.getByRole("link", { name: "Open job 4" })).toHaveAttribute(
		"href",
		"/app/jobs?job=4",
	);
	expect(lookup).toHaveBeenCalledWith({ email: "user@example.com", limit: 50 }, expect.any(Object));
});

it("says when the address has no earlier results", async () => {
	search([]);
	expect(await screen.findByText("No earlier results for this address.")).toBeInTheDocument();
});
