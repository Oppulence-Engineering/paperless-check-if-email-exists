// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { ListsApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { fromSegmentFilter, SavedSegments, toSegmentFilter } from "./saved-segments";

const SEGMENT = {
	id: 3,
	name: "Risky review",
	scope: "tenant",
	filter: { category: ["risky", "unknown"], score_min: 20, policy_decision: "review" },
	created_at: "2026-10-18T05:28:56Z",
	updated_at: "2026-10-18T05:28:56Z",
};

describe("segment filters", () => {
	it("keeps the filters that a segment accepts and names the others", () => {
		const { filter, skipped } = toSegmentFilter({
			category: "valid",
			safeToSend: true,
			minScore: 10,
			maxScore: 90,
			jobId: 18,
			since: "2026-10-01",
			recommendationAction: "send",
			policyMode: "growth",
			policyDecision: "drop",
		});
		expect(JSON.parse(JSON.stringify(filter))).toEqual({
			category: "valid",
			safe_to_send: true,
			score_min: 10,
			score_max: 90,
		});
		expect(skipped).toEqual([
			"Job 18",
			"Checked from 2026-10-01",
			"Recommendation: Send",
			"Policy mode: Growth",
			"Policy decision: Drop",
		]);
	});

	it("reads a segment filter and ignores the keys that the explorer does not have", () => {
		expect(fromSegmentFilter({ ...SEGMENT.filter, is_role_account: true })).toEqual({
			category: "risky",
			minScore: 20,
			policyDecision: "review",
		});
		expect(fromSegmentFilter("not an object")).toEqual({});
	});
});

describe("SavedSegments", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("saves, opens, renames, and deletes segments", async () => {
		const list = vi.spyOn(ListsApi.prototype, "v1ListSavedSegments"); // gitleaks:allow
		list.mockResolvedValue({ data: { segments: [SEGMENT], total: 1 } } as never);
		const create = vi.spyOn(ListsApi.prototype, "v1CreateSavedSegment"); // gitleaks:allow
		create.mockResolvedValue({ data: { ...SEGMENT, id: 4 } } as never);
		const get = vi.spyOn(ListsApi.prototype, "v1GetSavedSegment"); // gitleaks:allow
		get.mockResolvedValue({ data: { ...SEGMENT, filter: { safe_to_send: false } } } as never);
		const update = vi.spyOn(ListsApi.prototype, "v1UpdateSavedSegment"); // gitleaks:allow
		update.mockResolvedValue({ data: { ...SEGMENT, name: "Renamed" } } as never);
		const remove = vi.spyOn(ListsApi.prototype, "v1DeleteSavedSegment"); // gitleaks:allow
		remove.mockResolvedValue({ data: { deleted: true } } as never);
		const onOpen = vi.fn();
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<SavedSegments filters={{ category: "valid", jobId: 18 }} onOpen={onOpen} />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("Risky review")).toBeInTheDocument();
		expect(
			screen.getByText(/Category: Risky · Score 20 to 100 · Policy decision: Review/),
		).toBeInTheDocument();
		expect(list).toHaveBeenCalledWith(
			{ scope: "tenant", limit: 20, offset: 0 },
			{ signal: expect.any(AbortSignal) },
		);
		expect(screen.getByText(/The segment keeps: Category: Valid/)).toBeInTheDocument();
		expect(screen.getByText(/cannot keep these filters: Job 18/)).toBeInTheDocument();

		fireEvent.change(screen.getByLabelText("Segment name"), { target: { value: "Valid only" } });
		fireEvent.click(screen.getByRole("button", { name: "Save current filters" }));
		expect(await screen.findByRole("status")).toHaveTextContent("Segment saved.");
		expect(create).toHaveBeenCalledWith({
			createSavedSegmentRequest: {
				name: "Valid only",
				scope: "tenant",
				filter: { category: "valid" },
			},
		});

		fireEvent.click(screen.getByRole("button", { name: "Open Risky review" }));
		await waitFor(() => expect(onOpen).toHaveBeenCalledWith({ safeToSend: false }));
		expect(get).toHaveBeenCalledWith({ segmentId: 3 });

		fireEvent.click(screen.getByRole("button", { name: "Rename Risky review" }));
		fireEvent.change(screen.getByLabelText("New name for Risky review"), {
			target: { value: "Renamed" },
		});
		fireEvent.click(screen.getByRole("button", { name: "Save name" }));
		expect(await screen.findByText("Segment renamed.")).toBeInTheDocument();
		expect(update).toHaveBeenCalledWith({
			segmentId: 3,
			updateSavedSegmentRequest: { name: "Renamed" },
		});

		fireEvent.click(screen.getByRole("button", { name: "Delete Risky review" }));
		expect(await screen.findByText("Segment deleted.")).toBeInTheDocument();
		expect(remove).toHaveBeenCalledWith({ segmentId: 3 });
	});

	it("shows the backend error when a save fails", async () => {
		const list = vi.spyOn(ListsApi.prototype, "v1ListSavedSegments"); // gitleaks:allow
		list.mockResolvedValue({ data: { segments: [], total: 0 } } as never);
		const create = vi.spyOn(ListsApi.prototype, "v1CreateSavedSegment"); // gitleaks:allow
		create.mockRejectedValue(
			Object.assign(new Error("Request failed"), {
				response: { data: { error: "A saved segment with this name already exists" } },
			}),
		);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<SavedSegments filters={{}} onOpen={vi.fn()} />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("No saved segments yet.")).toBeInTheDocument();
		expect(screen.getByText("The segment keeps: all results.")).toBeInTheDocument();
		fireEvent.change(screen.getByLabelText("Segment name"), { target: { value: "Same" } });
		fireEvent.click(screen.getByRole("button", { name: "Save current filters" }));
		expect(await screen.findByRole("alert")).toHaveTextContent(
			"A saved segment with this name already exists",
		);
	});
});
