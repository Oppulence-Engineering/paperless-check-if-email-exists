// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { TenantApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { conditionText, ScorePolicyForm } from "./score-policy-form";

const saved = {
	id: 3,
	name: "careful",
	is_default: false,
	rules: {},
	created_at: "2026-10-18T05:28:23.249217Z",
	updated_at: "2026-10-18T05:28:23.249217Z",
};

function renderForm(policy?: typeof saved) {
	const onSaved = vi.fn();
	render(
		<QueryClientProvider client={new QueryClient()}>
			<ScorePolicyForm onSaved={onSaved} policy={policy} />
		</QueryClientProvider>,
	);
	return onSaved;
}

describe("ScorePolicyForm", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("says each condition in plain words", () => {
		expect(conditionText(undefined)).toBe("No rule.");
		expect(conditionText({})).toBe("All addresses.");
		expect(conditionText({ score_min: 40, score_max: 70, safe_to_send: false })).toBe(
			"When the score is 40 to 70 and the address is not safe to send.",
		);
	});

	it("creates a policy with only the rules that have a field set", async () => {
		const create = vi.spyOn(TenantApi.prototype, "v1CreateScorePolicy"); // gitleaks:allow
		create.mockResolvedValue({ data: saved } as never);
		const onSaved = renderForm();

		fireEvent.change(screen.getByLabelText("Name"), { target: { value: "careful" } });
		fireEvent.change(screen.getByLabelText("Suppress: Highest score"), {
			target: { value: "25" },
		});
		fireEvent.click(screen.getByLabelText("Suppress: Invalid"));
		fireEvent.change(screen.getByLabelText("Send: Safe to send"), { target: { value: "true" } });
		fireEvent.change(screen.getByLabelText("Send: Reason codes"), {
			target: { value: "deliverable, free_provider" },
		});
		expect(
			screen.getByText("When the score is 25 or less and the category is invalid."),
		).toBeInTheDocument();
		fireEvent.click(screen.getByRole("button", { name: "Create policy" }));

		await waitFor(() =>
			expect(create).toHaveBeenCalledWith({
				createScorePolicyRequest: {
					name: "careful",
					is_default: false,
					rules: {
						suppress: { score_max: 25, category: ["invalid"] },
						send: { safe_to_send: true, reason_codes_any: ["deliverable", "free_provider"] },
					},
				},
			}),
		);
		await waitFor(() => expect(onSaved).toHaveBeenCalled());
	});

	it("shows the backend error when a save fails", async () => {
		const update = vi.spyOn(TenantApi.prototype, "v1UpdateScorePolicy"); // gitleaks:allow
		update.mockRejectedValue(
			Object.assign(new Error("Bad request"), {
				response: { data: { error: "send.score_min cannot exceed send.score_max" } },
			}),
		);
		renderForm(saved);
		fireEvent.change(screen.getByLabelText("Send: Lowest score"), { target: { value: "90" } });
		fireEvent.change(screen.getByLabelText("Send: Highest score"), { target: { value: "10" } });
		fireEvent.click(screen.getByRole("button", { name: "Save policy" }));
		expect(await screen.findByRole("alert")).toHaveTextContent(
			"send.score_min cannot exceed send.score_max",
		);
		expect(update).toHaveBeenCalledWith({
			policyId: 3,
			updateScorePolicyRequest: {
				name: "careful",
				is_default: false,
				rules: { send: { score_min: 90, score_max: 10 } },
			},
		});
	});
});
