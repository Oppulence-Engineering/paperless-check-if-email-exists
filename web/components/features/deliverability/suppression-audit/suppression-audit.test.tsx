// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, render, screen } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { SuppressionAudit } from "./suppression-audit";

/** The live response of GET /v1/suppressions/1/events after an add and a remove. */
const events = {
	events: [
		{
			id: 5,
			entry_id: 1,
			canonical_email: "a@example.com",
			event_type: "deactivated",
			from_status: "active",
			to_status: "inactive",
			reason_code: "bounce",
			reason_detail: "550 mailbox unknown",
			source_type: "manual",
			source_ref: null,
			actor_type: "api",
			actor_id: null,
			metadata: {},
			created_at: "2026-10-18T04:52:32.468546+00:00",
		},
		{
			id: 1,
			entry_id: 1,
			canonical_email: "a@example.com",
			event_type: "created",
			from_status: null,
			to_status: "active",
			reason_code: "bounce",
			reason_detail: "550 mailbox unknown",
			source_type: "outcome",
			source_ref: "campaign-7",
			actor_type: "outcome",
			actor_id: "sendgrid",
			metadata: {},
			created_at: "2026-10-18T04:52:32.059053+00:00",
		},
	],
	total: 60,
};

describe("SuppressionAudit", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows status changes, reasons, sources, and actors", async () => {
		// gitleaks reads the method name as a key.
		const list = vi.spyOn(V1Api.prototype, "v1ListSuppressionEvents"); // gitleaks:allow
		list.mockResolvedValue({ data: events } as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<SuppressionAudit aria-label="Audit timeline" entryId={1} />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("From Active to Removed")).toBeInTheDocument();
		expect(screen.getByRole("region", { name: "Audit timeline" })).toHaveAttribute(
			"data-slot",
			"suppression-audit",
		);
		expect(list).toHaveBeenCalledWith({ id: 1 }, { signal: expect.any(AbortSignal) });
		expect(screen.getByText("Set to Active")).toBeInTheDocument();
		expect(screen.getAllByText("Bounce · 550 mailbox unknown")).toHaveLength(2);
		expect(screen.getByText("Delivery feedback · campaign-7")).toBeInTheDocument();
		expect(screen.getByText("Delivery feedback · sendgrid")).toBeInTheDocument();
		expect(screen.getByText("App or API")).toBeInTheDocument();
		expect(screen.getByText(/newest 2 of 60 changes/)).toBeInTheDocument();
	});
});
