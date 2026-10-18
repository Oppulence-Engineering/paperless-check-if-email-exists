// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { TenantApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { OutcomePolicyList } from "./outcome-policy-list";

const rules = {
	hard_bounce: { action: "suppress", score_override: "invalid" },
	complaint: { action: "suppress_and_unsubscribe", score_override: "invalid" },
	soft_bounce: { action: "suppress_after", threshold_count: 3, threshold_window_days: 30 },
	unsubscribe: { action: "suppress" },
	delivered: { action: "score_boost", boost: 5 },
	open: { action: "informational" },
	outcome_ttl_days: 60,
};

function policy(id: number, name: string, isDefault: boolean) {
	return {
		id,
		name,
		is_default: isDefault,
		rules,
		created_at: "2026-10-18T04:53:29.264641Z",
		updated_at: "2026-10-18T04:53:29.264641Z",
	};
}

describe("OutcomePolicyList", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows each rule in plain words, makes a policy the default, and deletes one", async () => {
		const list = vi.spyOn(TenantApi.prototype, "v1ListOutcomePolicies"); // gitleaks:allow
		list.mockResolvedValue({
			data: { total: 2, policies: [policy(1, "default", true), policy(2, "strict", false)] },
		} as never);
		const update = vi.spyOn(TenantApi.prototype, "v1UpdateOutcomePolicy"); // gitleaks:allow
		update.mockResolvedValue({ data: policy(2, "strict", true) } as never);
		const remove = vi.spyOn(TenantApi.prototype, "v1DeleteOutcomePolicy"); // gitleaks:allow
		remove.mockResolvedValue({ data: { deleted: true } } as never);
		vi.spyOn(window, "confirm").mockReturnValue(true);
		const client = new QueryClient();
		render(
			<QueryClientProvider client={client}>
				<OutcomePolicyList />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("strict")).toBeInTheDocument();
		expect(
			screen.getAllByText(
				"Add the address to the suppression list after 3 of these events in 30 days.",
			),
		).toHaveLength(2);
		expect(
			screen.getAllByText("Add the address to the suppression list and mark it as unsubscribed."),
		).toHaveLength(2);
		expect(screen.getAllByText("Raise the score of the address in later checks.")).toHaveLength(2);
		expect(screen.getAllByText("Record the event only.")).toHaveLength(2);
		// The click rule is missing, so the backend uses its default rule.
		expect(screen.getAllByText("Uses the default rule.")).toHaveLength(2);
		expect(screen.getAllByText("Later checks use the events of the last 60 days.")).toHaveLength(2);

		fireEvent.click(screen.getByRole("button", { name: "Make strict the default" }));
		await waitFor(() =>
			expect(update).toHaveBeenCalledWith({
				policyId: 2,
				updateOutcomePolicyRequest: { is_default: true },
			}),
		);
		expect(await screen.findByText("strict is now the default policy.")).toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "Delete strict" }));
		await waitFor(() => expect(remove).toHaveBeenCalledWith({ policyId: 2 }));
		expect(await screen.findByText("Outcome policy deleted.")).toBeInTheDocument();
	});
});
