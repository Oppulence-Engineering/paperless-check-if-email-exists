// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { TenantApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { OutcomePolicyForm } from "./outcome-policy-form";
import { chooseOption } from "@/quality/test-support/select";

const saved = {
	id: 3,
	name: "strict",
	is_default: false,
	rules: {},
	created_at: "2026-10-18T04:53:29.264641Z",
	updated_at: "2026-10-18T04:53:29.264641Z",
};

function renderForm(props: Parameters<typeof OutcomePolicyForm>[0] = {}) {
	const client = new QueryClient();
	render(
		<QueryClientProvider client={client}>
			<OutcomePolicyForm {...props} />
		</QueryClientProvider>,
	);
}

describe("OutcomePolicyForm", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("creates a policy from the default rules and sends only the fields each action uses", async () => {
		const create = vi.spyOn(TenantApi.prototype, "v1CreateOutcomePolicy"); // gitleaks:allow
		create.mockResolvedValue({ data: saved } as never);
		const onSaved = vi.fn();
		renderForm({ onSaved });

		fireEvent.change(screen.getByLabelText("Name"), { target: { value: "strict" } });
		fireEvent.click(screen.getByLabelText("Use as the default policy"));
		fireEvent.change(screen.getByLabelText("Soft bounce events in"), { target: { value: "2" } });
		await chooseOption("Delivered", "Suppress after repeats");
		await chooseOption("Clicked", "Record only");
		expect(
			screen.getByText(
				"Add the address to the suppression list after 2 of these events in 30 days.",
			),
		).toBeInTheDocument();
		fireEvent.change(screen.getByLabelText("Event age in days"), { target: { value: "30" } });
		fireEvent.click(screen.getByRole("button", { name: "Create policy" }));

		await waitFor(() =>
			expect(create).toHaveBeenCalledWith({
				createOutcomePolicyRequest: {
					name: "strict",
					is_default: true,
					rules: {
						hard_bounce: { action: "suppress", score_override: "invalid" },
						soft_bounce: {
							action: "suppress_after",
							threshold_count: 2,
							threshold_window_days: 30,
						},
						complaint: { action: "suppress_and_unsubscribe", score_override: "invalid" },
						unsubscribe: { action: "suppress" },
						delivered: { action: "suppress_after", threshold_count: 3, threshold_window_days: 30 },
						open: { action: "score_boost", boost: 3 },
						click: { action: "informational" },
						outcome_ttl_days: 30,
					},
				},
			}),
		);
		await waitFor(() => expect(onSaved).toHaveBeenCalled());
	});

	it("saves the changes to one policy", async () => {
		const update = vi.spyOn(TenantApi.prototype, "v1UpdateOutcomePolicy"); // gitleaks:allow
		update.mockResolvedValue({ data: saved } as never);
		renderForm({
			policy: {
				...saved,
				rules: { hard_bounce: { action: "informational" }, outcome_ttl_days: 45 },
			},
		});

		expect(screen.getByLabelText("Hard bounce")).toHaveTextContent("Record only");
		expect(screen.getByLabelText("Event age in days")).toHaveValue(45);
		fireEvent.click(screen.getByRole("button", { name: "Save policy" }));

		await waitFor(() =>
			expect(update).toHaveBeenCalledWith({
				policyId: 3,
				updateOutcomePolicyRequest: expect.objectContaining({
					name: "strict",
					is_default: false,
					rules: expect.objectContaining({
						hard_bounce: { action: "informational" },
						outcome_ttl_days: 45,
					}),
				}),
			}),
		);
	});
});
