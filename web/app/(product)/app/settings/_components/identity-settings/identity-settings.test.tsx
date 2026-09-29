// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	dashboardRequest: vi.fn(),
	entitlements: vi.fn(() => ({
		plan: "pro" as const,
		active: true,
		seats: { used: 1, limit: 50 },
	})),
	refresh: vi.fn(),
	replace: vi.fn(),
	switchBrowserWorkspace: vi.fn(),
}));

vi.mock("next/navigation", () => ({
	useRouter: () => ({ refresh: mocks.refresh, replace: mocks.replace }),
}));
vi.mock("@/lib/auth/dashboard-fetch", () => ({
	dashboardRequest: mocks.dashboardRequest,
}));
vi.mock("@/lib/auth/client", () => ({
	switchBrowserWorkspace: mocks.switchBrowserWorkspace,
}));
vi.mock("@/components/auth/auth-gate", () => ({
	// The panel always renders inside AuthGate; the plan is what it reads.
	useEntitlements: () => mocks.entitlements(),
}));

import { IdentitySettings } from "./identity-settings";

const fullOrganization = {
	id: "org_1",
	name: "Acme Operations",
	slug: "acme-operations",
	logo: "https://cdn.example.test/acme.png",
	requireAdminStepUp: true,
	members: [
		{
			id: "member_1",
			userId: "user_1",
			role: "owner",
			user: { id: "user_1", name: "Ada", email: "ada@example.com" },
		},
	],
	invitations: [],
};

function mockWorkspaceRequests() {
	mocks.dashboardRequest.mockImplementation(async (path: RequestInfo | URL) => {
		const requestPath = String(path);
		if (requestPath.endsWith("/organization/list")) return Response.json([]);
		if (requestPath.endsWith("/organization/get-full-organization")) {
			return Response.json(fullOrganization);
		}
		if (requestPath === "/api/auth/workspace-admin") return Response.json({ ok: true });
		return Response.json({});
	});
}

describe("IdentitySettings", () => {
	beforeEach(() => {
		mocks.dashboardRequest.mockResolvedValue(new Response("{}"));
		vi.stubGlobal(
			"ResizeObserver",
			class ResizeObserver {
				observe() {}
				unobserve() {}
				disconnect() {}
			},
		);
	});
	afterEach(() => {
		cleanup();
		vi.clearAllMocks();
		vi.unstubAllGlobals();
	});

	it("forwards accessible section props and renders its content", () => {
		render(
			<IdentitySettings
				aria-label="Example identity-settings"
				organizationId="org_1"
				organizationRole="member"
				userId="user_1"
			/>,
		);

		const component = screen.getByRole("region", {
			name: "Example identity-settings",
		});
		expect(component).toHaveAttribute("data-slot", "identity-settings");
		expect(component).toHaveTextContent("Identity and organization");
	});

	it("shows only the requested settings scope", () => {
		render(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="member"
				scope="security"
				userId="user_1"
			/>,
		);

		expect(screen.getByRole("heading", { name: "Security" })).toBeVisible();
		expect(screen.getByRole("heading", { name: "Sessions" })).toBeVisible();
		expect(screen.queryByRole("heading", { name: "Active workspace" })).not.toBeInTheDocument();
	});

	it("separates workspace identity from membership administration", async () => {
		render(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="member"
				scope="workspace"
				userId="user_1"
			/>,
		);

		expect(screen.getByRole("heading", { name: "Workspace details" })).toBeVisible();
		expect(screen.getByRole("heading", { name: "Workspace identity" })).toBeVisible();
		expect(
			screen.queryByRole("heading", { name: "Members and invitations" }),
		).not.toBeInTheDocument();

		await waitFor(() => expect(mocks.dashboardRequest).toHaveBeenCalledTimes(2));
		const paths = mocks.dashboardRequest.mock.calls.map(([path]) => String(path));
		expect(paths).toEqual([
			"/api/auth/organization/list",
			"/api/auth/organization/get-full-organization",
		]);
		expect(paths).not.toContain("/api/auth/passkey/list-user-passkeys");
		expect(paths).not.toContain("/api/auth/sessions");
	});

	it("shows logo upload and owner-only workspace lifecycle controls", () => {
		const { rerender } = render(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="owner"
				scope="workspace"
				userId="user_1"
			/>,
		);

		expect(screen.getByLabelText("Upload a logo")).toHaveAttribute(
			"accept",
			"image/png,image/jpeg,image/webp",
		);
		expect(screen.getByRole("heading", { name: "Workspace lifecycle" })).toBeVisible();
		expect(screen.getByRole("button", { name: "Archive workspace" })).toBeVisible();
		expect(screen.getByRole("button", { name: "Delete workspace" })).toBeVisible();

		rerender(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="admin"
				scope="workspace"
				userId="user_1"
			/>,
		);
		expect(screen.queryByRole("heading", { name: "Workspace lifecycle" })).not.toBeInTheDocument();
	});

	it("keeps ownership transfer owner-only and policy in security and compliance", () => {
		const { rerender } = render(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="owner"
				scope="organization"
				userId="user_1"
			/>,
		);
		expect(screen.getByRole("heading", { name: "Transfer ownership" })).toBeVisible();

		rerender(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="admin"
				scope="enterprise"
				userId="user_1"
			/>,
		);
		expect(screen.queryByRole("heading", { name: "Transfer ownership" })).not.toBeInTheDocument();
		expect(screen.getByRole("heading", { name: "Security and compliance" })).toBeVisible();
		expect(screen.getByRole("heading", { name: "Administrator mutation policy" })).toBeVisible();
		expect(screen.getByLabelText("Require fresh administrator step-up")).toBeChecked();
	});

	it("submits validated workspace identity updates through the administration boundary", async () => {
		mockWorkspaceRequests();
		render(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="owner"
				scope="workspace"
				userId="user_1"
			/>,
		);

		await waitFor(() =>
			expect(screen.getByLabelText("Workspace name")).toHaveValue("Acme Operations"),
		);
		const name = screen.getByLabelText("Workspace name");
		fireEvent.change(name, { target: { value: "Acme Revenue" } });
		fireEvent.change(screen.getByLabelText("Workspace slug"), {
			target: { value: "acme-revenue" },
		});
		fireEvent.change(screen.getByLabelText("Logo URL"), {
			target: { value: "https://cdn.example.test/revenue.png" },
		});
		fireEvent.click(screen.getByRole("button", { name: "Save workspace" }));

		await waitFor(() => {
			const call = mocks.dashboardRequest.mock.calls.find(
				([path]) => path === "/api/auth/workspace-admin",
			);
			expect(call).toBeDefined();
			expect(JSON.parse(String(call?.[1]?.body))).toEqual({
				action: "update_workspace",
				name: "Acme Revenue",
				slug: "acme-revenue",
				logo: "https://cdn.example.test/revenue.png",
			});
		});
	});

	it("requires the workspace slug before archiving and redirects after success", async () => {
		mockWorkspaceRequests();
		render(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="owner"
				scope="workspace"
				userId="user_1"
			/>,
		);

		const confirmation = await screen.findByLabelText("Type acme-operations to confirm", {
			selector: "#archive-confirmation",
		});
		fireEvent.change(confirmation, { target: { value: "acme-operations" } });
		fireEvent.click(screen.getByRole("button", { name: "Archive workspace" }));

		await waitFor(() => {
			const call = mocks.dashboardRequest.mock.calls.find(
				([path, init]) =>
					path === "/api/auth/workspace-admin" &&
					JSON.parse(String(init?.body)).action === "archive_workspace",
			);
			expect(call).toBeDefined();
			expect(mocks.replace).toHaveBeenCalledWith("/app");
			expect(mocks.refresh).toHaveBeenCalledOnce();
		});
	});

	it("stops an invitation when the plan has no seat left and says why", async () => {
		mocks.entitlements.mockReturnValue({
			plan: "free" as const,
			active: true,
			seats: { used: 1, limit: 1 },
		});
		mockWorkspaceRequests();
		render(
			<IdentitySettings
				organizationId="org_1"
				organizationRole="owner"
				scope="organization"
				userId="user_1"
			/>,
		);

		await waitFor(() => {
			expect(screen.getByRole("button", { name: "Invite" })).toBeDisabled();
		});
		expect(screen.getByRole("status")).toHaveTextContent(/All 1 seats on the free plan are taken/);
	});
});
