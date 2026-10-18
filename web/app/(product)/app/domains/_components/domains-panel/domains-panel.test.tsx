// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor, within } from "@testing-library/react";
import { AccountApi, TenantApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { SAMPLE_REPUTATION_REPORT } from "@/components/features/deliverability/reputation-report/reputation-report.schema";

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
	const check = vi.spyOn(V1Api.prototype, "v1CheckReputation"); // gitleaks:allow
	check.mockResolvedValue({ data: SAMPLE_REPUTATION_REPORT } as never);
	const create = vi
		.spyOn(TenantApi.prototype, "v1CreateTenantDomain")
		.mockResolvedValue({ data: {} } as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<DomainsPanel />
		</QueryClientProvider>,
	);
	expect(await screen.findByRole("heading", { name: "example.com" })).toBeInTheDocument();
	const health = screen.getByRole("region", { name: "Health of example.com" });
	expect(await within(health).findByText("82/100")).toBeInTheDocument();
	expect(check).toHaveBeenCalledWith({
		reputationCheckRequest: { domain: "example.com", force_refresh: false },
	});
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

it("checks the health of any domain", async () => {
	vi.spyOn(AccountApi.prototype, "v1Me").mockResolvedValue({
		data: { tenant_name: "Team", plan_tier: "pro", status: "active" },
	} as never);
	vi.spyOn(TenantApi.prototype, "v1ListTenantDomains").mockResolvedValue({
		data: { domains: [] },
	} as never);
	const check = vi.spyOn(V1Api.prototype, "v1CheckReputation"); // gitleaks:allow
	check.mockResolvedValue({ data: { ...SAMPLE_REPUTATION_REPORT, domain: "other.org" } } as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<DomainsPanel />
		</QueryClientProvider>,
	);
	expect(await screen.findByText("No domains added.")).toBeInTheDocument();
	fireEvent.change(screen.getByLabelText("Domain to check"), { target: { value: " Other.org " } });
	fireEvent.click(screen.getByRole("button", { name: "Check domain" }));
	const health = await screen.findByRole("region", { name: "Health of other.org" });
	expect(await within(health).findByText("82/100")).toBeInTheDocument();
	expect(check).toHaveBeenCalledWith({
		reputationCheckRequest: { domain: "other.org", force_refresh: false },
	});
});
