// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { PipelinesApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { PipelinesPanel } from "./pipelines-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("creates a paused list pipeline for review", async () => {
	vi.spyOn(PipelinesApi.prototype, "v1ListPipelines").mockResolvedValue({
		data: { pipelines: [], total: 0 },
	} as never);
	const create = vi
		.spyOn(PipelinesApi.prototype, "v1CreatePipeline")
		.mockResolvedValue({ data: { id: 7 } } as never);
	vi.spyOn(PipelinesApi.prototype, "v1GetPipeline").mockResolvedValue({
		data: {
			id: 7,
			name: "Weekly",
			status: "paused",
			source: { type: "list_snapshot", list_id: 4 },
			schedule: { cron: "0 9 * * 1", timezone: "UTC" },
		},
	} as never);
	vi.spyOn(PipelinesApi.prototype, "v1ListPipelineRuns").mockResolvedValue({
		data: { runs: [], total: 0 },
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<PipelinesPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("Name"), { target: { value: "Weekly" } });
	fireEvent.change(screen.getByLabelText("List ID"), { target: { value: "4" } });
	fireEvent.click(screen.getByRole("button", { name: "Create paused pipeline" }));
	await waitFor(() =>
		expect(create).toHaveBeenCalledWith({
			createPipelineInput: {
				name: "Weekly",
				source: { type: "list_snapshot", list_id: 4 },
				schedule: { cron: "0 9 * * 1", timezone: "UTC" },
				delivery: { dashboard: true },
				status: "paused",
			},
		}),
	);
	expect(await screen.findByText(/Pipeline created in paused state/)).toBeInTheDocument();
});
