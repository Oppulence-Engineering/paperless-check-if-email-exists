// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { OutcomesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { CampaignOutcomeList } from "./campaign-outcome-list";
import { chooseOption } from "@/quality/test-support/select";

describe("CampaignOutcomeList", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the policy action of each result and pages with the total", async () => {
		const list = vi.spyOn(OutcomesApi.prototype, "v1ListCampaignOutcomes"); // gitleaks:allow
		list.mockResolvedValue({
			data: {
				total: 25,
				outcomes: [
					{
						id: 1,
						email: "c@example.com",
						type: "hard_bounce",
						occurred_at: "2026-10-02T10:00:00Z",
						source: "mailchimp",
						campaign_id: "spring",
						policy_action: "suppressed",
						created_at: "2026-10-18T04:53:29.267860Z",
					},
				],
			},
		} as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<CampaignOutcomeList />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("c@example.com")).toBeInTheDocument();
		expect(screen.getByText("Hard bounce", { selector: "span" })).toBeInTheDocument();
		expect(screen.getByText("Suppressed")).toBeInTheDocument();
		expect(screen.getByText("Rows 1–1 of 25")).toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "Next results" }));
		await waitFor(() =>
			expect(list).toHaveBeenLastCalledWith(expect.objectContaining({ offset: 20 })),
		);

		await chooseOption("Result type", "Complaint");
		fireEvent.change(screen.getByLabelText("Result source"), { target: { value: "mailchimp" } });
		fireEvent.click(screen.getByRole("button", { name: "Search results" }));
		await waitFor(() =>
			expect(list).toHaveBeenLastCalledWith({
				limit: 20,
				offset: 0,
				email: undefined,
				type: "complaint",
				source: "mailchimp",
				since: undefined,
			}),
		);
	});
});
