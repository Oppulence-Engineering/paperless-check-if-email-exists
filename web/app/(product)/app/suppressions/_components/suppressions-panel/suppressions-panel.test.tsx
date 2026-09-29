// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { SuppressionsPanel } from "./suppressions-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("adds an address and shows the resulting decision", async () => {
	vi.spyOn(V1Api.prototype, "v1ListSuppressions").mockResolvedValue({
		data: {
			entries: [
				{
					id: 3,
					email: "bad@example.com",
					reason: "manual",
					status: "active",
					created_at: "2026-09-23T00:00:00Z",
				},
			],
			total: 1,
		},
	} as never);
	const addMethod = "v1AddSuppressions";
	const add = vi
		.spyOn(V1Api.prototype, addMethod)
		.mockResolvedValue({ data: { added: 1 } } as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<SuppressionsPanel />
		</QueryClientProvider>,
	);
	expect(await screen.findByText("bad@example.com")).toBeInTheDocument();
	fireEvent.change(screen.getByLabelText("Addresses"), { target: { value: "new@example.com" } });
	fireEvent.click(screen.getByRole("button", { name: "Add" }));
	await waitFor(() =>
		expect(add).toHaveBeenCalledWith({
			addSuppressionsRequest: { emails: ["new@example.com"], reason: "manual" },
		}),
	);
});
