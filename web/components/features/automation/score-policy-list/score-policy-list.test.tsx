// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { TenantApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ScorePolicyList } from "./score-policy-list";

const strict = {
	id: 2,
	name: "strict",
	is_default: true,
	rules: {
		suppress: { score_max: 30, category: ["invalid", "risky"] },
		send: { score_min: 80, safe_to_send: true },
		review: { reason_codes_any: ["catch_all"] },
	},
	created_at: "2026-10-18T05:28:23.249217Z",
	updated_at: "2026-10-18T05:28:23.249217Z",
};

describe("ScorePolicyList", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows each rule in plain words, loads one policy to edit, and deletes one", async () => {
		const list = vi.spyOn(TenantApi.prototype, "v1ListScorePolicies"); // gitleaks:allow
		list.mockResolvedValue({ data: { total: 1, policies: [strict] } } as never);
		const get = vi.spyOn(TenantApi.prototype, "v1GetScorePolicy"); // gitleaks:allow
		get.mockResolvedValue({ data: strict } as never);
		const remove = vi.spyOn(TenantApi.prototype, "v1DeleteScorePolicy"); // gitleaks:allow
		remove.mockResolvedValue({ data: { deleted: true } } as never);
		vi.spyOn(window, "confirm").mockReturnValue(true);
		render(
			<QueryClientProvider client={new QueryClient()}>
				<ScorePolicyList />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("strict")).toBeInTheDocument();
		expect(screen.getByText("Default")).toBeInTheDocument();
		expect(
			screen.getByText("When the score is 30 or less and the category is invalid or risky."),
		).toBeInTheDocument();
		expect(
			screen.getByText("When the score is 80 or more and the address is safe to send."),
		).toBeInTheDocument();
		expect(
			screen.getByText(
				"When the result has one of these reasons: The domain accepts all addresses.",
			),
		).toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "Edit strict" }));
		await waitFor(() => expect(get).toHaveBeenCalledWith({ policyId: 2 }, expect.anything()));
		expect(await screen.findByLabelText("Send: Lowest score")).toHaveValue(80);
		expect(screen.getByLabelText("Suppress: Highest score")).toHaveValue(30);
		fireEvent.keyDown(document.activeElement ?? document.body, { key: "Escape" });

		fireEvent.click(await screen.findByRole("button", { name: "Delete strict" }));
		await waitFor(() => expect(remove).toHaveBeenCalledWith({ policyId: 2 }));
		expect(await screen.findByText("Score policy deleted.")).toBeInTheDocument();
	});

	it("explains the built-in rule when there is no policy", async () => {
		const list = vi.spyOn(TenantApi.prototype, "v1ListScorePolicies"); // gitleaks:allow
		list.mockResolvedValue({ data: { total: 0, policies: [] } } as never);
		render(
			<QueryClientProvider client={new QueryClient()}>
				<ScorePolicyList />
			</QueryClientProvider>,
		);
		expect(await screen.findByText(/No score policies yet\./)).toBeInTheDocument();
		expect(screen.getByRole("button", { name: "New score policy" })).toBeInTheDocument();
	});
});
