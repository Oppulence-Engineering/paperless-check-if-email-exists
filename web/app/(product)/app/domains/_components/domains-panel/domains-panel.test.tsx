// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { AccountApi, TenantApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { DomainsPanel } from "./domains-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("adds a domain inactive and shows its current verification state", async () => {
	vi.spyOn(AccountApi.prototype, "v1Me").mockResolvedValue({
		data: { tenant_name: "Team", plan_tier: "pro", status: "active" },
	} as never);
	vi.spyOn(TenantApi.prototype, "v1ListTenantDomains").mockResolvedValue({
		data: {
			domains: [
				{
					domain: "example.com",
					is_active: false,
					is_verified: false,
					created_at: "2026-09-23T00:00:00Z",
				},
			],
		},
	} as never);
	const create = vi
		.spyOn(TenantApi.prototype, "v1CreateTenantDomain")
		.mockResolvedValue({ data: {} } as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<DomainsPanel />
		</QueryClientProvider>,
	);
	expect(await screen.findByText("example.com")).toBeInTheDocument();
	fireEvent.change(screen.getByLabelText("Domain"), { target: { value: "new.example.com" } });
	fireEvent.click(screen.getByRole("button", { name: "Add inactive domain" }));
	await waitFor(() =>
		expect(create).toHaveBeenCalledWith({
			createTenantDomainRequest: {
				domain: "new.example.com",
				notes: undefined,
				is_active: false,
			},
		}),
	);
});
