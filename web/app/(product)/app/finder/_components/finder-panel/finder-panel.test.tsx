// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { FinderPanel } from "./finder-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("starts a finder job and shows the verified best match", async () => {
	const create = vi
		.spyOn(V1Api.prototype, "v1FindEmail")
		.mockResolvedValue({ data: { job_id: 11, status: "queued" } } as never);
	vi.spyOn(V1Api.prototype, "v1GetFindEmail").mockResolvedValue({
		data: {
			status: "completed",
			domain_has_mx: true,
			domain_is_catch_all: false,
			candidates_checked: 2,
			best_match: { email: "ada@example.com", score: 95, confidence: "high" },
			results: [],
		},
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<FinderPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("First name"), { target: { value: "Ada" } });
	fireEvent.change(screen.getByLabelText("Last name"), { target: { value: "Lovelace" } });
	fireEvent.change(screen.getByLabelText("Company domain"), { target: { value: "example.com" } });
	fireEvent.click(screen.getByRole("button", { name: "Find email" }));
	await waitFor(() =>
		expect(create).toHaveBeenCalledWith({
			findEmailRequest: {
				first_name: "Ada",
				last_name: "Lovelace",
				domain: "example.com",
				strategy: "parallel",
			},
		}),
	);
	expect(await screen.findByText("ada@example.com")).toBeInTheDocument();
});
