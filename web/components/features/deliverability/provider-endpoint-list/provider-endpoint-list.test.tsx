// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { OutcomesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ProviderEndpointList } from "./provider-endpoint-list";

const endpoint = {
	endpoint_id: "2cb867f0-70d2-4538-af2d-afeb45151c63",
	provider: "postmark",
	label: "Postmark primary",
	status: "paused",
	allowed_ips: ["192.0.2.10", "192.0.2.11"],
	provider_configured: false,
	webhook_path:
		"/v1/inbound/providers/postmark/2cb867f0-70d2-4538-af2d-afeb45151c63/{delivery_token}",
	created_at: "2026-10-18T04:53:29.186130Z",
	updated_at: "2026-10-18T04:53:29.186130Z",
};

describe("ProviderEndpointList", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the IP allow list, opens the editor, and reveals a rotated callback", async () => {
		const list = vi.spyOn(OutcomesApi.prototype, "v1ListProviderEndpoints"); // gitleaks:allow
		list.mockResolvedValue({ data: { provider_endpoints: [endpoint] } } as never);
		const update = vi.spyOn(OutcomesApi.prototype, "v1UpdateProviderEndpoint"); // gitleaks:allow
		update.mockResolvedValue({
			data: {
				...endpoint,
				webhook_path: "/v1/inbound/providers/postmark/ep/new-token",
				delivery_token: "new-token",
			},
		} as never);
		vi.spyOn(window, "confirm").mockReturnValue(true);
		const client = new QueryClient();
		render(
			<QueryClientProvider client={client}>
				<ProviderEndpointList />
			</QueryClientProvider>,
		);

		expect(await screen.findByText("Postmark primary · postmark")).toBeInTheDocument();
		expect(screen.getByText("Allowed IPs: 192.0.2.10, 192.0.2.11")).toBeInTheDocument();

		fireEvent.click(screen.getByRole("button", { name: "Edit Postmark primary" }));
		expect(await screen.findByLabelText("Allowed IP addresses")).toHaveValue(
			"192.0.2.10\n192.0.2.11",
		);
		fireEvent.keyDown(document.activeElement ?? document.body, { key: "Escape" });

		fireEvent.click(screen.getByRole("button", { name: "Rotate token" }));
		await waitFor(() =>
			expect(update).toHaveBeenCalledWith({
				endpointId: endpoint.endpoint_id,
				updateProviderEndpointInput: { status: undefined, rotate_delivery_token: true },
			}),
		);
		expect(await screen.findByText("Copy the callback URL now")).toBeInTheDocument();
	});
});
