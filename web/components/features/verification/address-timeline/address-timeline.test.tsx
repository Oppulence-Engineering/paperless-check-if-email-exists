// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { OutcomesApi, V1Api, VerificationApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { AddressTimeline } from "./address-timeline";

function renderTimeline() {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	return render(
		<QueryClientProvider client={client}>
			<AddressTimeline aria-label="Timeline" email="maya@brightlane.co" />
		</QueryClientProvider>,
	);
}

describe("AddressTimeline", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("merges checks and delivery events and suppresses the address", async () => {
		vi.spyOn(VerificationApi.prototype, "v1EmailHistory").mockResolvedValue({
			data: {
				email: "maya@brightlane.co",
				total: 1,
				history: [
					{
						job_id: 7,
						task_id: 70,
						list_id: 3,
						score: 12,
						category: "invalid",
						previous_category: "valid",
						reason_codes: ["invalid_recipient"],
						recommendation_action: "suppress",
						is_reachable: "invalid",
						completed_at: "2026-10-16T12:00:00Z",
					},
				],
			},
		} as never);
		vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes").mockResolvedValue({
			data: {
				total: 1,
				outcomes: [
					{
						id: 1,
						email: "maya@brightlane.co",
						type: "hard_bounce",
						source: "sendgrid",
						campaign_id: "spring",
						occurred_at: "2026-10-17T09:00:00Z",
						created_at: "2026-10-17T09:00:01Z",
					},
				],
			},
		} as never);
		vi.spyOn(V1Api.prototype, "v1CheckSuppression").mockResolvedValue({
			data: { suppressed: false },
		} as never);
		const add = vi.spyOn(V1Api.prototype, "v1AddSuppressions").mockResolvedValue({
			data: { added: 1, duplicates: 0, updated: 0, entry_ids: [1] },
		} as never);

		renderTimeline();

		expect(await screen.findByText("Hard bounce")).toBeInTheDocument();
		expect(screen.getByText("Changed from Valid to Invalid")).toBeInTheDocument();
		expect(screen.getByText("The mailbox does not exist")).toBeInTheDocument();
		expect(screen.getByRole("link", { name: "Open the list" })).toHaveAttribute(
			"href",
			"/app/lists/detail?list=3",
		);

		fireEvent.click(await screen.findByRole("button", { name: "Add to suppression list" }));
		await vi.waitFor(() =>
			expect(add).toHaveBeenCalledWith({
				addSuppressionsRequest: { emails: ["maya@brightlane.co"], reason: "manual" },
			}),
		);
	});

	it("shows the current suppression", async () => {
		vi.spyOn(VerificationApi.prototype, "v1EmailHistory").mockResolvedValue({
			data: { email: "maya@brightlane.co", total: 0, history: [] },
		} as never);
		vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes").mockResolvedValue({
			data: { total: 0, outcomes: [] },
		} as never);
		vi.spyOn(V1Api.prototype, "v1CheckSuppression").mockResolvedValue({
			data: { suppressed: true, reason: "complaint", created_at: "2026-10-01T00:00:00Z" },
		} as never);

		renderTimeline();

		expect(await screen.findByText("Suppressed")).toBeInTheDocument();
		expect(
			await screen.findByText("No earlier checks or delivery events for this address."),
		).toBeInTheDocument();
	});
});
