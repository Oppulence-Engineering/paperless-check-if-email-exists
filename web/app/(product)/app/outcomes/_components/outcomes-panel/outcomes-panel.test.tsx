// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { OutcomesApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { OutcomesPanel } from "./outcomes-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("registers a paused provider endpoint and reveals its callback once", async () => {
	vi.spyOn(OutcomesApi.prototype, "v1ListOutcomes").mockResolvedValue({
		data: { outcomes: [], limit: 20, offset: 0 },
	} as never);
	vi.spyOn(OutcomesApi.prototype, "v1ListProviderEndpoints").mockResolvedValue({
		data: { provider_endpoints: [] },
	} as never);
	const create = vi.spyOn(OutcomesApi.prototype, "v1CreateProviderEndpoint").mockResolvedValue({
		data: {
			endpoint_id: "ep-1",
			webhook_path: "/v1/inbound/providers/postmark/ep-1/token",
			delivery_token: "token",
		},
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<OutcomesPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("Label"), { target: { value: "Postmark primary" } });
	fireEvent.click(screen.getByRole("button", { name: "Create paused endpoint" }));
	await waitFor(() =>
		expect(create).toHaveBeenCalledWith({
			createProviderEndpointInput: {
				provider: "postmark",
				label: "Postmark primary",
				status: "paused",
				provider_config: {},
			},
		}),
	);
	expect(await screen.findByText("Copy the callback URL now")).toBeInTheDocument();
});
