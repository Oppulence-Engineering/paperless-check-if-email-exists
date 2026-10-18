// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { JobsApi, TenantApi, V1Api } from "@oppulence/reacher-sdk";
import { afterEach, beforeEach, expect, it, vi } from "vitest";

import { VerificationSettings } from "./verification-settings";

vi.mock("@/components/auth/auth-gate", () => ({
	useAuthSession: () => ({ billing: { plan: "free", enabled: false } }),
	useEntitlements: () => ({ plan: "free", active: true, seats: { used: 1, limit: 3 } }),
}));
vi.mock("@/lib/auth/auth-client", () => ({
	authClient: { subscription: { upgrade: vi.fn(), billingPortal: vi.fn() } },
}));

beforeEach(() => {
	vi.spyOn(V1Api.prototype, "v1ReverificationStatus").mockResolvedValue({
		data: { enabled: false },
	} as never);
	const policies = vi.spyOn(TenantApi.prototype, "v1ListScorePolicies"); // gitleaks:allow
	policies.mockResolvedValue({ data: { policies: [], total: 0 } } as never);
});

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
			approval_excellent_threshold: 90,
			approval_good_threshold: 70,
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
			approval_excellent_threshold: 90,
			approval_good_threshold: 70,
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
				approval_excellent_threshold: 90,
				approval_good_threshold: 70,
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

it("shows the last re-check run with its job and the score policies", async () => {
	const status = vi.spyOn(V1Api.prototype, "v1ReverificationStatus"); // gitleaks:allow
	status.mockResolvedValue({
		data: {
			enabled: true,
			staleness_days: 30,
			batch_size: 100,
			last_run_at: "2026-10-17T09:00:00Z",
			next_run_at: "2026-10-18T09:00:00Z",
			last_job_id: 42,
			emails_requeued: 1250,
		},
	} as never);
	const job = vi.spyOn(JobsApi.prototype, "v1GetJobStatus"); // gitleaks:allow
	job.mockResolvedValue({
		data: {
			job_id: 42,
			status: "completed",
			total_records: 100,
			created_at: "2026-10-17T09:00:00Z",
			task_summary: { completed: 100 },
		},
	} as never);
	vi.spyOn(TenantApi.prototype, "v1GetTenantSettings").mockReturnValue(new Promise(() => {}));
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<VerificationSettings organizationId="org-1" organizationRole="owner" scope="verification" />
		</QueryClientProvider>,
	);

	expect(await screen.findByRole("link", { name: "Open job 42" })).toHaveAttribute(
		"href",
		"/app/jobs?job=42",
	);
	expect(await screen.findByText("Completed")).toBeInTheDocument();
	expect(
		screen.getByText(
			/100 addresses in this run · 1,250 addresses sent for a new check in all runs/,
		),
	).toBeInTheDocument();
	expect(job).toHaveBeenCalledWith({ jobId: 42 }, expect.anything());
	expect(screen.getByRole("heading", { name: "Score policies" })).toBeInTheDocument();
	expect(await screen.findByText(/No score policies yet\./)).toBeInTheDocument();
});
