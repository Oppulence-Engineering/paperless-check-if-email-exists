// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { EventsApi, QueryApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { AnalyticsPanel } from "./analytics-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("filters results and checks a domain in the same workspace", async () => {
	const query = vi.spyOn(QueryApi.prototype, "v1QueryResults").mockResolvedValue({
		data: {
			results: [
				{ id: 1, email: "a@example.com", category: "valid", score: 90, task_state: "completed" },
			],
			total: 1,
		},
	} as never);
	vi.spyOn(EventsApi.prototype, "v1ListEvents").mockResolvedValue({
		data: { events: [], total: 0 },
	} as never);
	vi.spyOn(V1Api.prototype, "v1SourceQuality").mockResolvedValue({
		data: { sources: [] },
	} as never);
	const reputation = vi.spyOn(V1Api.prototype, "v1CheckReputation").mockResolvedValue({
		data: {
			domain: "example.com",
			risk_level: "low",
			score: 90,
			cached: false,
			blacklist_results: [],
		},
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<AnalyticsPanel />
		</QueryClientProvider>,
	);
	expect(await screen.findByText("a@example.com")).toBeInTheDocument();
	fireEvent.change(screen.getByLabelText("Category"), { target: { value: "valid" } });
	await waitFor(() =>
		expect(query).toHaveBeenCalledWith({
			limit: 20,
			offset: 0,
			category: "valid",
			safeToSend: undefined,
		}),
	);
	fireEvent.change(screen.getByLabelText("Domain"), { target: { value: "example.com" } });
	fireEvent.click(screen.getByRole("button", { name: "Check domain" }));
	await waitFor(() =>
		expect(reputation).toHaveBeenCalledWith({ reputationCheckRequest: { domain: "example.com" } }),
	);
});
