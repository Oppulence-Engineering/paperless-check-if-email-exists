// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import * as downloads from "@/lib/downloads";

import { SourceQuality } from "./source-quality";

describe("SourceQuality", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows every counter of each source", async () => {
		// Each counter has its own value, so the test finds each one.
		const counters = [
			"total_records",
			"processed_records",
			"safe_to_send_count",
			"valid_count",
			"risky_count",
			"unknown_count",
			"invalid_count",
			"send_count",
			"send_with_caution_count",
			"review_count",
			"suppress_count",
			"drop_count",
			"outcome_count",
			"delivered_count",
			"opened_count",
			"clicked_count",
			"bounced_count",
			"complained_count",
			"unsubscribed_count",
			"risky_pct",
			"invalid_pct",
			"unsafe_recommendation_pct",
			"negative_outcome_pct",
		];
		const quality = vi.spyOn(V1Api.prototype, "v1SourceQuality"); // gitleaks:allow
		quality.mockResolvedValue({
			data: {
				sources: [
					{
						source_key: "webinar",
						quality_grade: "F",
						summary: "This source produces 0% risky contacts.",
						...Object.fromEntries(counters.map((key, index) => [key, 101 + index])),
					},
				],
			},
		} as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<SourceQuality />
			</QueryClientProvider>,
		);

		const source = (await screen.findByRole("heading", { name: "webinar" })).closest("article");
		if (!(source instanceof HTMLElement)) throw new Error("The source card is missing.");
		expect(within(source).getByText("Grade F")).toBeInTheDocument();
		expect(within(source).getByText("This source produces 0% risky contacts.")).toBeInTheDocument();
		counters.forEach((key, index) => {
			const value = `${String(101 + index)}${key.endsWith("_pct") ? "%" : ""}`;
			expect(within(source).getByText(value)).toBeInTheDocument();
		});
		expect(within(source).getByText("Send with caution")).toBeInTheDocument();
		expect(within(source).getByText("Complaint")).toBeInTheDocument();
		const download = vi.spyOn(downloads, "saveFile").mockImplementation(() => {});
		fireEvent.click(screen.getByRole("button", { name: "Download report CSV" }));
		expect(download).toHaveBeenCalledWith(
			expect.stringContaining('"webinar","F"'),
			"lead-source-quality.csv",
		);
		fireEvent.change(screen.getByLabelText("Lead source"), { target: { value: "webinar" } });
		fireEvent.change(screen.getByLabelText("Minimum records"), { target: { value: "100" } });
		fireEvent.click(screen.getByRole("button", { name: "Apply filters" }));
		await waitFor(() =>
			expect(quality).toHaveBeenLastCalledWith(
				{
					sourceKey: "webinar",
					minRecords: 100,
					limit: 50,
				},
				{ signal: expect.any(AbortSignal) },
			),
		);
	});

	it("explains how to get data when there are no sources", async () => {
		const quality = vi.spyOn(V1Api.prototype, "v1SourceQuality"); // gitleaks:allow
		quality.mockResolvedValue({ data: { sources: [] } } as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<SourceQuality />
			</QueryClientProvider>,
		);

		expect(await screen.findByText(/No source quality data yet/)).toBeInTheDocument();
	});
});
