// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { AdminApi, AdminJobsApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { PlatformOperationRunner } from "./platform-operation-runner";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
	vi.unstubAllGlobals();
});

it("creates a tenant through the audited operator adapter with a reason", async () => {
	vi.spyOn(AdminApi.prototype, "listTenants").mockResolvedValue({
		data: { tenants: [], total: 0 },
	} as never);
	vi.spyOn(AdminApi.prototype, "listAllApiKeys").mockResolvedValue({
		data: { api_keys: [], total: 0 },
	} as never);
	vi.spyOn(AdminJobsApi.prototype, "listJobs").mockResolvedValue({
		data: { jobs: [], total: 0 },
	} as never);
	vi.spyOn(AdminApi.prototype, "getTenant").mockResolvedValue({
		data: { id: "tenant-1", name: "Example", status: "active" },
	} as never);
	vi.spyOn(AdminApi.prototype, "getTenantQuota").mockResolvedValue({
		data: { monthly_email_limit: 100, used_this_period: 0 },
	} as never);
	vi.spyOn(AdminApi.prototype, "listApiKeys").mockResolvedValue({
		data: { api_keys: [] },
	} as never);
	vi.spyOn(AdminJobsApi.prototype, "listTenantJobs").mockResolvedValue({
		data: { jobs: [], total: 0 },
	} as never);
	const create = vi
		.spyOn(AdminApi.prototype, "createTenant")
		.mockResolvedValue({ data: { id: "tenant-1" } } as never);
	vi.stubGlobal("confirm", () => true);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<PlatformOperationRunner
				operations={[
					{ id: "create_tenant", method: "POST", path: "/v1/admin/tenants", requestBody: true },
				]}
			/>
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("Reason for changes"), {
		target: { value: "Support request 123" },
	});
	fireEvent.change(screen.getByLabelText("Tenant name"), { target: { value: "Example" } });
	fireEvent.change(screen.getByLabelText("Slug"), { target: { value: "example" } });
	fireEvent.change(screen.getByLabelText("Contact email"), {
		target: { value: "owner@example.com" },
	});
	fireEvent.click(screen.getByRole("button", { name: "Create tenant" }));
	await waitFor(() =>
		expect(create).toHaveBeenCalledWith(
			{
				adminCreateTenantRequest: {
					name: "Example",
					slug: "example",
					contact_email: "owner@example.com",
				},
			},
			{ headers: { "x-admin-reason": "Support request 123" } },
		),
	);
});
