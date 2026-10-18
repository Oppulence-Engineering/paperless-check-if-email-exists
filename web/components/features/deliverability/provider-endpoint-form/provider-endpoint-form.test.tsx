// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { OutcomesApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { ProviderEndpointForm } from "./provider-endpoint-form";

const endpoint = {
	endpoint_id: "2cb867f0-70d2-4538-af2d-afeb45151c63",
	provider: "postmark",
	label: "Postmark primary",
	status: "paused",
	allowed_ips: ["192.0.2.10"],
	provider_configured: false,
	webhook_path:
		"/v1/inbound/providers/postmark/2cb867f0-70d2-4538-af2d-afeb45151c63/{delivery_token}",
	created_at: "2026-10-18T04:53:29.186130Z",
	updated_at: "2026-10-18T04:53:29.186130Z",
};

function renderForm(props: Parameters<typeof ProviderEndpointForm>[0] = {}) {
	const client = new QueryClient();
	render(
		<QueryClientProvider client={client}>
			<ProviderEndpointForm {...props} />
		</QueryClientProvider>,
	);
}

describe("ProviderEndpointForm", () => {
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("registers a paused Postmark endpoint with an IP allow list and reveals its callback once", async () => {
		const create = vi.spyOn(OutcomesApi.prototype, "v1CreateProviderEndpoint"); // gitleaks:allow
		create.mockResolvedValue({
			data: {
				...endpoint,
				webhook_path: "/v1/inbound/providers/postmark/ep-1/token",
				delivery_token: "token",
			},
		} as never);
		renderForm();

		fireEvent.change(screen.getByLabelText("Label"), { target: { value: "Postmark primary" } });
		fireEvent.change(screen.getByLabelText("Allowed IP addresses"), {
			target: { value: "192.0.2.10\n192.0.2.11, 192.0.2.12" },
		});
		fireEvent.click(screen.getByRole("button", { name: "Create paused endpoint" }));

		await waitFor(() =>
			expect(create).toHaveBeenCalledWith({
				createProviderEndpointInput: {
					provider: "postmark",
					label: "Postmark primary",
					status: "paused",
					provider_config: {},
					allowed_ips: ["192.0.2.10", "192.0.2.11", "192.0.2.12"],
				},
			}),
		);
		expect(await screen.findByText("Copy the callback URL now")).toBeInTheDocument();
		expect(
			screen.getByText(/\/v1\/inbound\/providers\/postmark\/ep-1\/token$/),
		).toBeInTheDocument();
	});

	it("asks for the verification value of a signed provider", async () => {
		const create = vi.spyOn(OutcomesApi.prototype, "v1CreateProviderEndpoint"); // gitleaks:allow
		renderForm();

		fireEvent.change(screen.getByLabelText("Label"), { target: { value: "Mailgun" } });
		fireEvent.change(screen.getByLabelText("Provider"), { target: { value: "mailgun" } });
		expect(screen.getByLabelText("Mailgun signing key")).toBeInTheDocument();
		fireEvent.click(screen.getByRole("button", { name: "Create paused endpoint" }));

		expect(
			await screen.findByText("Enter a label and the provider verification value."),
		).toBeInTheDocument();
		expect(create).not.toHaveBeenCalled();
	});

	it("changes the label and the IP allow list of one endpoint", async () => {
		const update = vi.spyOn(OutcomesApi.prototype, "v1UpdateProviderEndpoint"); // gitleaks:allow
		update.mockResolvedValue({ data: { ...endpoint, allowed_ips: [] } } as never);
		const onSaved = vi.fn();
		renderForm({ endpoint, onSaved });

		expect(screen.queryByLabelText("Provider")).not.toBeInTheDocument();
		expect(screen.getByLabelText("Allowed IP addresses")).toHaveValue("192.0.2.10");
		fireEvent.change(screen.getByLabelText("Allowed IP addresses"), { target: { value: "" } });
		fireEvent.click(screen.getByRole("button", { name: "Save endpoint" }));

		await waitFor(() =>
			expect(update).toHaveBeenCalledWith({
				endpointId: endpoint.endpoint_id,
				updateProviderEndpointInput: { label: "Postmark primary", allowed_ips: [] },
			}),
		);
		await waitFor(() => expect(onSaved).toHaveBeenCalled());
	});

	it("shows the error text of the backend", async () => {
		const update = vi.spyOn(OutcomesApi.prototype, "v1UpdateProviderEndpoint"); // gitleaks:allow
		update.mockRejectedValue(
			Object.assign(new Error("Request failed with status code 400"), {
				response: { data: { error: "allowed_ips contains an invalid IP" } },
			}),
		);
		renderForm({ endpoint });

		fireEvent.change(screen.getByLabelText("Allowed IP addresses"), {
			target: { value: "not-an-ip" },
		});
		fireEvent.click(screen.getByRole("button", { name: "Save endpoint" }));

		expect(await screen.findByRole("alert")).toHaveTextContent(
			"allowed_ips contains an invalid IP",
		);
	});
});
