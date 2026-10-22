// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { EventsApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ActivityFeed } from "./activity-feed";
import { chooseDate } from "@/quality/test-support/date";
import { chooseOption } from "@/quality/test-support/select";

function renderFeed() {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<ActivityFeed />
		</QueryClientProvider>,
	);
}

describe("ActivityFeed", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the type, job, row, actor, and data of each event, and filters and pages", async () => {
		const list = vi.spyOn(EventsApi.prototype, "v1ListEvents"); // gitleaks:allow
		list.mockResolvedValue({
			data: {
				total: 45,
				events: [
					{
						id: 131,
						job_id: 18,
						task_id: 85,
						event_type: "task.delayed_recheck_scheduled",
						event_data: { delay_seconds: 300, retry_count: 1 },
						actor: "backend-dev",
						created_at: "2026-10-18T05:28:54Z",
					},
					{
						id: 1,
						job_id: 17,
						event_type: "brand.new_event",
						event_data: null,
						actor: null,
						created_at: "2026-10-18T05:28:50Z",
					},
				],
			},
		} as never);
		renderFeed();

		expect(await screen.findByRole("link", { name: "Job 18" })).toHaveAttribute(
			"href",
			"/app/jobs?job=18",
		);
		// The event type select has the same text as an option.
		expect(screen.getAllByText("Recheck scheduled")).toHaveLength(2);
		expect(screen.getByText("Row 85")).toBeInTheDocument();
		expect(screen.getByText(/backend-dev/)).toBeInTheDocument();
		expect(screen.getByText("Delay seconds:")).toBeInTheDocument();
		expect(screen.getByText("300")).toBeInTheDocument();
		expect(screen.getByText("Brand.new event")).toBeInTheDocument();
		expect(screen.getByText(/System/)).toBeInTheDocument();
		expect(screen.getByText("Events 1–2 of 45")).toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "Next" }));
		await waitFor(() =>
			expect(list).toHaveBeenLastCalledWith(expect.objectContaining({ limit: 20, offset: 20 }), {
				signal: expect.any(AbortSignal),
			}),
		);

		await chooseOption("Event type", "Row checked");
		fireEvent.change(screen.getByLabelText("Actor"), { target: { value: "api" } });
		fireEvent.change(screen.getByLabelText("Job ID"), { target: { value: "18" } });
		await chooseDate("From", "2026-10-01");
		await chooseDate("To", "2026-10-18");
		fireEvent.click(screen.getByRole("button", { name: "Filter" }));
		await waitFor(() =>
			expect(list).toHaveBeenLastCalledWith(
				{
					limit: 20,
					offset: 0,
					eventType: "task.completed",
					actor: "api",
					jobId: 18,
					since: new Date("2026-10-01T00:00").toISOString(),
					until: new Date("2026-10-18T23:59:59.999").toISOString(),
				},
				{ signal: expect.any(AbortSignal) },
			),
		);
	});

	it("rejects a job ID that is not a positive whole number", async () => {
		const list = vi.spyOn(EventsApi.prototype, "v1ListEvents"); // gitleaks:allow
		list.mockResolvedValue({ data: { total: 0, events: [] } } as never);
		renderFeed();

		expect(await screen.findByText("No activity found.")).toBeInTheDocument();
		fireEvent.change(screen.getByLabelText("Job ID"), { target: { value: "abc" } });
		fireEvent.click(screen.getByRole("button", { name: "Filter" }));
		expect(screen.getByRole("alert")).toHaveTextContent("positive whole number");
		expect(list).toHaveBeenCalledTimes(1);
	});
});
