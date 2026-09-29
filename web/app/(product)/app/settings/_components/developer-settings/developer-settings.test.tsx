// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { AccountApi } from "@oppulence/reacher-sdk";
import { afterEach, describe, expect, it, vi } from "vitest";

import { DeveloperSettings } from "./developer-settings";

const tenantId = "3e205baa-5595-4c28-9f2b-9ae3e9e98e66";
const keyId = "19c80f2e-e82d-4c97-b4cf-28aebc50a79e";

function renderSettings(role = "owner") {
	const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
	return render(
		<QueryClientProvider client={client}>
			<DeveloperSettings organizationId={tenantId} organizationRole={role} />
		</QueryClientProvider>,
	);
}

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

describe("DeveloperSettings", () => {
	it("loads tenant keys and shows metadata without revealing existing secrets", async () => {
		const list = vi.spyOn(AccountApi.prototype, "listTenantApiKeys").mockResolvedValue({
			data: {
				api_keys: [
					{
						id: keyId,
						tenant_id: tenantId,
						key_prefix: "rch_live_example",
						name: "Production",
						scopes: ["verify"],
						status: "active",
						last_used_at: null,
						expires_at: null,
						created_at: "2026-09-22T00:00:00Z",
					},
				],
			},
		} as never);
		renderSettings();

		expect(await screen.findByText("Production")).toBeInTheDocument();
		expect(screen.getByText(/rch_live_example/)).toBeInTheDocument();
		expect(screen.getByRole("link", { name: "Open schema" })).toHaveAttribute(
			"href",
			"/api/backend/openapi.json",
		);
		expect(list).toHaveBeenCalled();
		expect(screen.queryByLabelText("New API key — copy it now")).not.toBeInTheDocument();
	});

	it("creates an expiring scoped key and reveals its secret once", async () => {
		vi.spyOn(AccountApi.prototype, "listTenantApiKeys").mockResolvedValue({
			data: { api_keys: [] },
		} as never);
		const create = vi.spyOn(AccountApi.prototype, "createTenantApiKey").mockResolvedValue({
			data: {
				id: keyId,
				tenant_id: tenantId,
				key: "rch_live_new_secret",
				key_prefix: "rch_live_new",
				name: "Production",
				scopes: ["verify"],
				status: "active",
				expires_at: "2026-10-22T00:00:00Z",
				created_at: "2026-09-22T00:00:00Z",
			},
		} as never);
		renderSettings();

		fireEvent.change(screen.getByRole("textbox", { name: "Key name" }), {
			target: { value: "Production" },
		});
		fireEvent.click(screen.getByRole("button", { name: "Create API key" }));
		await waitFor(() => expect(create).toHaveBeenCalled());
		expect(create.mock.calls[0]?.[0]?.createApiKeyRequest).toMatchObject({
			name: "Production",
			scopes: ["verify"],
		});
		expect(await screen.findByRole("textbox", { name: "New API key — copy it now" })).toHaveValue(
			"rch_live_new_secret",
		);
		fireEvent.click(screen.getByRole("button", { name: "Done" }));
		expect(
			screen.queryByRole("textbox", { name: "New API key — copy it now" }),
		).not.toBeInTheDocument();
	});

	it("loads one key and updates its name and scopes", async () => {
		const key = {
			id: keyId,
			tenant_id: tenantId,
			key_prefix: "rch_live_example",
			name: "Production",
			scopes: ["verify"],
			status: "active",
			last_used_at: null,
			expires_at: null,
			created_at: "2026-09-22T00:00:00Z",
		};
		vi.spyOn(AccountApi.prototype, "listTenantApiKeys").mockResolvedValue({
			data: { api_keys: [key] },
		} as never);
		const get = vi
			.spyOn(AccountApi.prototype, "getTenantApiKey")
			.mockResolvedValue({ data: key } as never);
		const update = vi.spyOn(AccountApi.prototype, "updateTenantApiKey").mockResolvedValue({
			data: { ...key, name: "Staging", scopes: ["verify", "bulk"] },
		} as never);
		renderSettings();
		fireEvent.click(await screen.findByRole("button", { name: "Manage" }));
		await waitFor(() => expect(get).toHaveBeenCalledWith({ keyId }));
		fireEvent.change(screen.getByLabelText("Name", { selector: "#edit-api-key-name" }), {
			target: { value: "Staging" },
		});
		const editScope = document.getElementById("edit-api-key-scope-pipelines.read");
		if (!editScope) throw new Error("Edit scope control is missing");
		fireEvent.click(editScope);
		fireEvent.click(screen.getByRole("button", { name: "Save key" }));
		await waitFor(() =>
			expect(update).toHaveBeenCalledWith({
				keyId,
				updateApiKeyRequest: { name: "Staging", scopes: ["verify", "pipelines.read"] },
			}),
		);
	});

	it("does not request credentials for members", () => {
		const list = vi.spyOn(AccountApi.prototype, "listTenantApiKeys");
		renderSettings("member");
		expect(screen.getByText(/Only workspace owners and administrators/)).toBeInTheDocument();
		expect(list).not.toHaveBeenCalled();
	});
});
