// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { OutcomesApi, TenantApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { OutcomesPanel } from "./outcomes-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("puts the events first and opens each form in a sheet", async () => {
	const events = vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes"); // gitleaks:allow
	events.mockResolvedValue({ data: { outcomes: [], limit: 20, offset: 0 } } as never);
	const campaign = vi.spyOn(OutcomesApi.prototype, "v1ListCampaignOutcomes"); // gitleaks:allow
	campaign.mockResolvedValue({ data: { outcomes: [], total: 0 } } as never);
	const endpoints = vi.spyOn(OutcomesApi.prototype, "v1ListProviderEndpoints"); // gitleaks:allow
	endpoints.mockResolvedValue({ data: { provider_endpoints: [] } } as never);
	const policies = vi.spyOn(TenantApi.prototype, "v1ListOutcomePolicies"); // gitleaks:allow
	policies.mockResolvedValue({ data: { policies: [], total: 0 } } as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<OutcomesPanel />
		</QueryClientProvider>,
	);

	expect(
		screen.getAllByRole("heading", { level: 2 }).map((heading) => heading.textContent),
	).toEqual(["Recent outcomes", "Campaign results", "Provider endpoints", "Outcome policies"]);
	expect(await screen.findByText("No outcomes found.")).toBeInTheDocument();
	expect(await screen.findByText("No provider endpoints yet.")).toBeInTheDocument();

	fireEvent.click(screen.getByRole("button", { name: "Register a provider" }));
	expect(await screen.findByRole("button", { name: "Create paused endpoint" })).toBeInTheDocument();
	expect(screen.getByLabelText("Label")).toBeInTheDocument();
	fireEvent.keyDown(document.activeElement ?? document.body, { key: "Escape" });

	fireEvent.click(screen.getByRole("button", { name: "Record an outcome" }));
	expect(await screen.findByRole("button", { name: "Record outcome" })).toBeInTheDocument();
});
