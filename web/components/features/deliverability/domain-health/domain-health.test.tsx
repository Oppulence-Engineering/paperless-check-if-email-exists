// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { SAMPLE_REPUTATION_REPORT } from "@/components/features/deliverability/reputation-report/reputation-report.schema";

import { DomainHealth } from "./domain-health";

function renderHealth() {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<DomainHealth aria-label="Health" domain="example.com" />
		</QueryClientProvider>,
	);
}

describe("DomainHealth", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the saved result and forces a new check on request", async () => {
		const check = vi.spyOn(V1Api.prototype, "v1CheckReputation"); // gitleaks:allow
		check.mockResolvedValueOnce({ data: SAMPLE_REPUTATION_REPORT } as never);
		check.mockResolvedValueOnce({
			data: { ...SAMPLE_REPUTATION_REPORT, cached: false, score: 90 },
		} as never);
		renderHealth();

		expect(await screen.findByText("82/100")).toBeInTheDocument();
		expect(screen.getByRole("region", { name: "Health" })).toHaveAttribute(
			"data-slot",
			"domain-health",
		);
		expect(check).toHaveBeenCalledWith({
			reputationCheckRequest: { domain: "example.com", force_refresh: false },
		});

		fireEvent.click(screen.getByRole("button", { name: "Check again" }));
		expect(await screen.findByText("90/100")).toBeInTheDocument();
		expect(check).toHaveBeenLastCalledWith({
			reputationCheckRequest: { domain: "example.com", force_refresh: true },
		});
		expect(screen.getByText(/New result/)).toBeInTheDocument();
	});

	it("shows the backend error message", async () => {
		const check = vi.spyOn(V1Api.prototype, "v1CheckReputation"); // gitleaks:allow
		check.mockRejectedValue({ response: { data: { error: "Invalid domain" } } });
		renderHealth();

		expect(await screen.findByRole("alert")).toHaveTextContent("Invalid domain");
		await waitFor(() => {
			expect(screen.getByRole("button", { name: "Retry" })).toBeEnabled();
		});
	});
});
