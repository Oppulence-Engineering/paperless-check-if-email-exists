// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { OutcomesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { OutcomeEvents } from "./outcome-events";
import { chooseOption } from "@/quality/test-support/select";

function event(id: number, extra: Record<string, unknown> = {}) {
	return {
		id,
		email: `user${String(id)}@example.com`,
		canonical_email: `user${String(id)}@example.com`,
		provider: "postmark",
		event_type: "bounced",
		correlation_status: "unmatched",
		metadata: {},
		occurred_at: "2026-10-01T10:00:00Z",
		created_at: "2026-10-01T10:00:01Z",
		...extra,
	};
}

describe("OutcomeEvents", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the event family, the send match, and the provider IDs, and pages with filters", async () => {
		const list = vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes"); // gitleaks:allow
		list.mockResolvedValue({
			data: {
				limit: 20,
				offset: 0,
				outcomes: [
					event(1, {
						campaign_id: "spring",
						provider_message_id: "pm-1",
						source_key: "newsletter",
					}),
					...Array.from({ length: 19 }, (_, index) =>
						event(index + 2, { event_type: "opened", event_family: "engagement" }),
					),
				],
			},
		} as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<OutcomeEvents />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("user1@example.com")).toBeInTheDocument();
		// Manual events have no family, so the page uses the family of the event type.
		expect(screen.getByText("Delivery")).toBeInTheDocument();
		expect(screen.getAllByText("Engagement")).toHaveLength(19);
		expect(screen.getAllByText("Not matched")).toHaveLength(20);
		expect(screen.getByText("Message pm-1")).toBeInTheDocument();
		expect(screen.getByText("spring")).toBeInTheDocument();
		expect(screen.getByText("Source newsletter")).toBeInTheDocument();
		expect(screen.getByText("Rows 1–20")).toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "Next" }));
		await waitFor(() =>
			expect(list).toHaveBeenLastCalledWith(expect.objectContaining({ limit: 20, offset: 20 })),
		);

		await chooseOption("Event type", "Complaint");
		fireEvent.change(screen.getByLabelText("Since"), { target: { value: "2026-10-01" } });
		fireEvent.click(screen.getByRole("button", { name: "Search" }));
		await waitFor(() =>
			expect(list).toHaveBeenLastCalledWith({
				limit: 20,
				offset: 0,
				email: undefined,
				eventType: "complained",
				since: new Date("2026-10-01T00:00").toISOString(),
			}),
		);
	});
});
