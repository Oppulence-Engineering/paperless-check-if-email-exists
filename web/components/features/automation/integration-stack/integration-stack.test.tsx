// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, render, screen, within } from "@testing-library/react";
import { AccountApi, OutcomesApi, PipelinesApi, TenantApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { IntegrationStack } from "./integration-stack";

function endpoint(provider: string, status: string) {
	return {
		endpoint_id: crypto.randomUUID(),
		provider,
		label: `${provider} main`,
		status,
		allowed_ips: [],
		provider_configured: true,
		webhook_path: "/v1/provider-outcomes/x",
		created_at: "2026-10-18T04:53:29Z",
		updated_at: "2026-10-18T04:53:29Z",
	};
}

describe("IntegrationStack", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows whether each part is set up and links to its setup page", async () => {
		const endpoints = vi.spyOn(OutcomesApi.prototype, "v1ListProviderEndpoints"); // gitleaks:allow
		endpoints.mockResolvedValue({
			data: {
				provider_endpoints: [
					endpoint("postmark", "active"),
					endpoint("postmark", "paused"),
					endpoint("mailgun", "paused"),
				],
			},
		} as never);
		const webhook = vi.spyOn(TenantApi.prototype, "v1GetTenantWebhook"); // gitleaks:allow
		webhook.mockResolvedValue({
			data: {
				tenant_id: "3e205baa-5595-4c28-9f2b-9ae3e9e98e66",
				tenant_name: "Example",
				default_webhook_url: "https://hooks.example.com/runs",
				webhook_signing_secret_configured: true,
			},
		} as never);
		const keys = vi.spyOn(AccountApi.prototype, "listTenantApiKeys"); // gitleaks:allow
		keys.mockRejectedValue(
			Object.assign(new Error("Forbidden"), {
				response: { data: { error: "Organization role cannot call this operation" } },
			}),
		);
		const pipelines = vi.spyOn(PipelinesApi.prototype, "v1ListPipelines"); // gitleaks:allow
		pipelines.mockResolvedValue({ data: { pipelines: [], total: 0 } } as never);
		const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
		render(
			<QueryClientProvider client={client}>
				<IntegrationStack />
			</QueryClientProvider>,
		);

		const postmark = await screen.findByRole("link", { name: /Postmark/ });
		expect(postmark).toHaveAttribute("href", "/app/outcomes");
		expect(await within(postmark).findByText("Connected")).toBeInTheDocument();
		expect(within(postmark).getByText(/1 of 2 endpoints active\./)).toBeInTheDocument();
		const mailgun = screen.getByRole("link", { name: /Mailgun/ });
		expect(within(mailgun).getByText("Paused")).toBeInTheDocument();
		expect(
			within(screen.getByRole("link", { name: /SendGrid/ })).getByText("Not set up"),
		).toBeInTheDocument();
		const hook = screen.getByRole("link", { name: /Webhook/ });
		expect(hook).toHaveAttribute("href", "/app/settings?settings=webhooks");
		expect(
			await within(hook).findByText(/Sends signed requests to hooks\.example\.com\./),
		).toBeInTheDocument();
		const apiKeys = screen.getByRole("link", { name: /API keys/ });
		expect(apiKeys).toHaveAttribute("href", "/app/settings?settings=developer");
		expect(await within(apiKeys).findByText("Unavailable")).toBeInTheDocument();
		expect(
			within(apiKeys).getByText(/Organization role cannot call this operation/),
		).toBeInTheDocument();
		const pipelineRow = screen.getByRole("link", { name: /Pipelines/ });
		expect(pipelineRow).toHaveAttribute("href", "/app/pipelines");
		expect(await within(pipelineRow).findByText("Not set up")).toBeInTheDocument();
	});
});
