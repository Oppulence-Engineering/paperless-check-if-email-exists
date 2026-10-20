// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { OutcomesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { OutcomeEntry } from "./outcome-entry";
import { chooseDate } from "@/quality/test-support/date";
import { chooseOption } from "@/quality/test-support/select";

describe("OutcomeEntry", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("records one outcome with the time, the source key, and the campaign", async () => {
		const ingest = vi.spyOn(OutcomesApi.prototype, "v1IngestOutcomes"); // gitleaks:allow
		ingest.mockResolvedValue({ data: { ingested: 1, auto_suppressed: 1, ignored: 0 } } as never);
		vi.spyOn(window, "confirm").mockReturnValue(true);
		const client = new QueryClient();
		const invalidate = vi.spyOn(client, "invalidateQueries");
		render(
			<QueryClientProvider client={client}>
				<OutcomeEntry />
			</QueryClientProvider>,
		);

		fireEvent.change(screen.getByLabelText("Email"), { target: { value: "a@example.com" } });
		await chooseOption("Event", "Bounced");
		await chooseDate("Occurred at", "2026-10-01");
		fireEvent.change(screen.getByLabelText("Time"), { target: { value: "10:30" } });
		fireEvent.change(screen.getByLabelText("Source key"), { target: { value: "newsletter" } });
		fireEvent.change(screen.getByLabelText("Campaign"), { target: { value: "spring" } });
		fireEvent.click(screen.getByRole("button", { name: "Record outcome" }));

		await waitFor(() =>
			expect(ingest).toHaveBeenCalledWith({
				outcomeIngestRequest: {
					provider: "manual",
					outcomes: [
						{
							email: "a@example.com",
							event_type: "bounced",
							occurred_at: new Date("2026-10-01T10:30").toISOString(),
							source_key: "newsletter",
							campaign_id: "spring",
						},
					],
				},
			}),
		);
		expect(
			await screen.findByText("Recorded 1 outcome(s); 1 auto-suppressed; 0 skipped."),
		).toBeInTheDocument();
		for (const key of ["suppressions", "source-quality", "list-detail"]) {
			expect(invalidate).toHaveBeenCalledWith({ queryKey: [key] });
		}
	});
});
