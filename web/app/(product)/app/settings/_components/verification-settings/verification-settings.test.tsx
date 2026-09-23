// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { TenantApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { VerificationSettings } from "./verification-settings";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("saves the shadcn policy selection and a webhook URL without replacing the signing secret", async () => {
	const tenant_id = "3e205baa-5595-4c28-9f2b-9ae3e9e98e66";
	vi.spyOn(TenantApi.prototype, "v1GetTenantUsage").mockResolvedValue({
		data: {
			tenant_id,
			tenant_name: "Example",
			plan_tier: "free",
			monthly_email_limit: 100,
			used_this_period: 4,
			period_reset_at: "2026-10-01T00:00:00Z",
			quota_unlimited: false,
			quota_remaining: 96,
		},
	} as never);
	vi.spyOn(TenantApi.prototype, "v1GetTenantSettings").mockResolvedValue({
		data: {
			tenant_id,
			name: "Example",
			slug: "example",
			monthly_email_limit: 100,
			used_this_period: 4,
			period_reset_at: "2026-10-01T00:00:00Z",
			result_retention_days: 30,
			default_webhook_url: "https://old.example/webhook",
			default_policy_mode: "deliverability",
		},
	} as never);
	vi.spyOn(TenantApi.prototype, "v1GetTenantWebhook").mockResolvedValue({
		data: {
			tenant_id,
			tenant_name: "Example",
			default_webhook_url: "https://old.example/webhook",
			webhook_signing_secret_configured: true,
		},
	} as never);
	const updateSettings = vi.spyOn(TenantApi.prototype, "v1UpdateTenantSettings").mockResolvedValue({
		data: {
			tenant_id,
			name: "Example",
			slug: "example",
			monthly_email_limit: 100,
			used_this_period: 4,
			period_reset_at: "2026-10-01T00:00:00Z",
			result_retention_days: 30,
			default_webhook_url: "https://old.example/webhook",
			default_policy_mode: "deliverability",
		},
	} as never);
	const update = vi.spyOn(TenantApi.prototype, "v1UpdateTenantWebhook").mockResolvedValue({
		data: {
			tenant_id,
			tenant_name: "Example",
			default_webhook_url: "https://new.example/webhook",
			webhook_signing_secret_configured: true,
		},
	} as never);
	const client = new QueryClient({
		defaultOptions: { queries: { retry: false } },
	});
	render(
		<QueryClientProvider client={client}>
			<VerificationSettings organizationId="org-1" organizationRole="owner" />
		</QueryClientProvider>,
	);

	expect(await screen.findByText("96")).toBeInTheDocument();
	fireEvent.click(screen.getByRole("button", { name: "Save verification defaults" }));
	await waitFor(() =>
		expect(updateSettings).toHaveBeenCalledWith({
			updateTenantSettingsRequest: {
				result_retention_days: 30,
				default_policy_mode: "deliverability",
			},
		}),
	);
	fireEvent.change(await screen.findByLabelText("Webhook URL"), {
		target: { value: "https://new.example/webhook" },
	});
	fireEvent.click(screen.getByRole("button", { name: "Save webhook" }));
	await waitFor(() =>
		expect(update).toHaveBeenCalledWith({
			updateWebhookRequest: {
				default_webhook_url: "https://new.example/webhook",
			},
		}),
	);
});

it("shows members usage without requesting administrator settings", async () => {
	const usage = vi.spyOn(TenantApi.prototype, "v1GetTenantUsage").mockResolvedValue({
		data: {
			tenant_id: "3e205baa-5595-4c28-9f2b-9ae3e9e98e66",
			tenant_name: "Example",
			plan_tier: "free",
			monthly_email_limit: 100,
			used_this_period: 4,
			period_reset_at: "2026-10-01T00:00:00Z",
			quota_unlimited: false,
			quota_remaining: 96,
		},
	} as never);
	const settings = vi.spyOn(TenantApi.prototype, "v1GetTenantSettings");
	const client = new QueryClient({
		defaultOptions: { queries: { retry: false } },
	});
	render(
		<QueryClientProvider client={client}>
			<VerificationSettings organizationId="org-1" organizationRole="member" />
		</QueryClientProvider>,
	);
	expect(await screen.findByText("96")).toBeInTheDocument();
	expect(usage).toHaveBeenCalled();
	expect(settings).not.toHaveBeenCalled();
	expect(screen.queryByLabelText("Result retention (days)")).not.toBeInTheDocument();
});

it("loads only the webhook for its focused settings panel", async () => {
	const tenant_id = "3e205baa-5595-4c28-9f2b-9ae3e9e98e66";
	const webhook = vi.spyOn(TenantApi.prototype, "v1GetTenantWebhook").mockResolvedValue({
		data: {
			tenant_id,
			tenant_name: "Example",
			default_webhook_url: null,
			webhook_signing_secret_configured: false,
		},
	} as never);
	const usage = vi.spyOn(TenantApi.prototype, "v1GetTenantUsage");
	const settings = vi.spyOn(TenantApi.prototype, "v1GetTenantSettings");
	const client = new QueryClient({
		defaultOptions: { queries: { retry: false } },
	});
	render(
		<QueryClientProvider client={client}>
			<VerificationSettings organizationId="org-1" organizationRole="owner" scope="webhook" />
		</QueryClientProvider>,
	);

	expect(await screen.findByLabelText("Webhook URL")).toBeInTheDocument();
	expect(webhook).toHaveBeenCalled();
	expect(usage).not.toHaveBeenCalled();
	expect(settings).not.toHaveBeenCalled();
	expect(screen.queryByText("Usage", { selector: "div" })).not.toBeInTheDocument();
});
