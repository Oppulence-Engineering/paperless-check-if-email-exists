// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { fireEvent, render, screen } from "@testing-library/react";
import { VerificationApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { HistoryPanel } from "./history-panel";

afterEach(() => vi.restoreAllMocks());

it("looks up an address with the SDK and renders its history", async () => {
	const lookup = vi.spyOn(VerificationApi.prototype, "v1EmailHistory").mockResolvedValue({
		data: {
			email: "user@example.com",
			total: 1,
			history: [
				{
					job_id: 4,
					score: 86,
					category: "safe",
					sub_reason: null,
					safe_to_send: true,
					reason_codes: ["mx_found"],
					recommendation_action: null,
					recommendation: null,
					policy_mode: null,
					policy_decision: null,
					policy_evaluation: null,
					is_reachable: "safe",
					completed_at: "2026-09-22T12:00:00Z",
				},
			],
		},
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<HistoryPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("Email address"), {
		target: { value: "USER@example.com" },
	});
	fireEvent.click(screen.getByRole("button", { name: "Search" }));
	expect(await screen.findByText("mx_found")).toBeInTheDocument();
	expect(lookup).toHaveBeenCalledWith({ email: "user@example.com", limit: 50 }, expect.any(Object));
});
