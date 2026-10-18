// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, render, screen } from "@testing-library/react";
import { HealthApi, SystemApi } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";

import { IntegrationsPanel } from "./integrations-panel";

vi.mock("@/components/features/automation/integration-stack/integration-stack", () => ({
	IntegrationStack: () => <p>Connection rows</p>,
}));

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("shows live status and safe setup journeys", async () => {
	vi.spyOn(HealthApi.prototype, "healthz").mockResolvedValue({ status: 200 } as never);
	vi.spyOn(HealthApi.prototype, "readyz").mockResolvedValue({ status: 200 } as never);
	vi.spyOn(SystemApi.prototype, "getVersion").mockResolvedValue({
		data: { version: "4.3.0" },
	} as never);
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<IntegrationsPanel />
		</QueryClientProvider>,
	);
	expect(screen.getByRole("heading", { name: "Connect your stack" })).toBeInTheDocument();
	expect(screen.getByText("Connection rows")).toBeInTheDocument();
	expect(await screen.findByText("Ready")).toBeInTheDocument();
	expect(await screen.findByText("4.3.0")).toBeInTheDocument();
	expect(screen.getByRole("link", { name: "Callback contract" })).toHaveAttribute(
		"href",
		"/developers/reference#tag/provider-callback/v1_ingest_provider_outcomes",
	);
	expect(screen.queryByRole("button", { name: /send.*callback/i })).not.toBeInTheDocument();
});
