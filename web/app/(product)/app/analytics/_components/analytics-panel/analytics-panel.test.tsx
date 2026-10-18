// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { EventsApi, ListsApi, QueryApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

import { AnalyticsPanel } from "./analytics-panel";

let query: ReturnType<typeof vi.spyOn>;

beforeEach(() => {
	query = vi.spyOn(QueryApi.prototype, "v1QueryResults"); // gitleaks:allow
	query.mockResolvedValue({
		data: {
			results: [
				{
					id: 1,
					email: "a@example.com",
					category: "valid",
					score: 90,
					task_state: "completed",
					created_at: "2026-10-18T05:28:53Z",
				},
			],
			total: 1,
		},
	} as never);
	const events = vi.spyOn(EventsApi.prototype, "v1ListEvents"); // gitleaks:allow
	events.mockResolvedValue({ data: { events: [], total: 0 } } as never);
	const sources = vi.spyOn(V1Api.prototype, "v1SourceQuality"); // gitleaks:allow
	sources.mockResolvedValue({ data: { sources: [] } } as never);
});

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

function renderPanel() {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<AnalyticsPanel />
		</QueryClientProvider>,
	);
}

it("filters results", async () => {
	renderPanel();
	expect(
		screen.getAllByRole("heading", { level: 2 }).map((heading) => heading.textContent),
	).toEqual(["Result explorer", "Job activity", "Source quality"]);
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
});

it("opens a saved segment in a sheet and applies its filters", async () => {
	const list = vi.spyOn(ListsApi.prototype, "v1ListSavedSegments"); // gitleaks:allow
	const segment = {
		id: 3,
		name: "Unsafe",
		scope: "tenant",
		filter: { safe_to_send: false, score_max: 40 },
		created_at: "2026-10-18T05:28:56Z",
		updated_at: "2026-10-18T05:28:56Z",
	};
	list.mockResolvedValue({ data: { segments: [segment], total: 1 } } as never);
	const get = vi.spyOn(ListsApi.prototype, "v1GetSavedSegment"); // gitleaks:allow
	get.mockResolvedValue({ data: segment } as never);
	renderPanel();

	fireEvent.click(screen.getByRole("button", { name: "Saved segments" }));
	const sheet = await screen.findByRole("dialog");
	fireEvent.click(await within(sheet).findByRole("button", { name: "Open Unsafe" }));
	await waitFor(() =>
		expect(query).toHaveBeenLastCalledWith(
			expect.objectContaining({ offset: 0, safeToSend: false, maxScore: 40 }),
		),
	);
	await waitFor(() => expect(screen.queryByRole("dialog")).not.toBeInTheDocument());
	expect(screen.getByRole("list", { name: "Active filters" })).toHaveTextContent(
		"Safe to send: No",
	);
});
