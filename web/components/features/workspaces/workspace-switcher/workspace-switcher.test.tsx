// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import type { ComponentProps } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { DropdownMenuItem } from "@oppulence/ui/components/dropdown-menu";

const mocks = vi.hoisted(() => ({
	createBrowserWorkspace: vi.fn(),
	switchBrowserWorkspace: vi.fn(),
}));

vi.mock("@/lib/auth/client", () => ({
	createBrowserWorkspace: mocks.createBrowserWorkspace,
	switchBrowserWorkspace: mocks.switchBrowserWorkspace,
}));

import { WorkspaceSwitcher } from "./workspace-switcher";

const workspaces = [
	{
		id: "workspace-1",
		name: "Northstar Freight",
		slug: "northstar-freight",
		role: "owner",
		logoUrl: null,
	},
	{
		id: "workspace-2",
		name: "Juniper Health",
		slug: "juniper-health",
		role: "admin",
		logoUrl: null,
	},
];

function renderSwitcher(props: ComponentProps<typeof WorkspaceSwitcher>) {
	return render(<WorkspaceSwitcher {...props} />);
}

describe("WorkspaceSwitcher", () => {
	beforeEach(() => {
		vi.resetAllMocks();
	});
	afterEach(() => {
		cleanup();
		vi.restoreAllMocks();
	});

	it("shows the active workspace and every authorized tenant", async () => {
		renderSwitcher({
			activeWorkspaceId: "workspace-1",
			"aria-label": "Workspace switcher",
			planLabel: "Pro",
			workspaces,
		});

		const component = screen.getByRole("region", {
			name: "Workspace switcher",
		});
		expect(component).toHaveAttribute("data-slot", "workspace-switcher");
		expect(
			screen.getByRole("button", {
				name: /current workspace Northstar Freight/i,
			}),
		).toBeVisible();

		fireEvent.pointerDown(screen.getByRole("button", { name: /current workspace/i }));
		expect(await screen.findByText("Juniper Health")).toBeVisible();
		expect(screen.getByText("Pro")).toBeVisible();
	});

	it("switches to another authorized workspace", async () => {
		mocks.switchBrowserWorkspace.mockImplementation(() => new Promise(() => undefined));
		renderSwitcher({
			activeWorkspaceId: "workspace-1",
			"aria-label": "Workspace switcher",
			workspaces,
		});

		fireEvent.pointerDown(screen.getByRole("button", { name: /current workspace/i }));
		fireEvent.click(await screen.findByText("Juniper Health"));

		await waitFor(() => expect(mocks.switchBrowserWorkspace).toHaveBeenCalledWith("workspace-2"));
		fireEvent.pointerDown(screen.getByRole("button", { name: /current workspace/i }));
		expect(await screen.findByText("Switching…")).toBeVisible();
		expect(screen.getByRole("menuitem", { name: /Juniper Health.*Switching/i })).toHaveAttribute(
			"aria-busy",
			"true",
		);
	});

	it("creates a workspace with an editable generated handle", async () => {
		mocks.createBrowserWorkspace.mockImplementation(() => new Promise(() => undefined));
		renderSwitcher({
			activeWorkspaceId: "workspace-1",
			"aria-label": "Workspace switcher",
			workspaces,
		});

		fireEvent.pointerDown(screen.getByRole("button", { name: /current workspace/i }));
		fireEvent.click(await screen.findByText("Create workspace"));

		const name = await screen.findByRole("textbox", { name: "Workspace name" });
		fireEvent.change(name, { target: { value: "Acme Operations" } });
		expect(screen.getByRole("textbox", { name: "Workspace handle" })).toHaveValue(
			"acme-operations",
		);
		fireEvent.click(screen.getByRole("button", { name: "Create workspace" }));

		await waitFor(() =>
			expect(mocks.createBrowserWorkspace).toHaveBeenCalledWith({
				name: "Acme Operations",
				slug: "acme-operations",
			}),
		);
		expect(screen.getByRole("button", { name: "Creating…" })).toBeDisabled();
	});

	it("composes workspace controls into a custom account trigger", async () => {
		renderSwitcher({
			activeWorkspaceId: "workspace-1",
			"aria-label": "Account and workspace menu",
			trigger: <button type="button">Open account menu</button>,
			workspaces,
			children: <DropdownMenuItem>Account settings</DropdownMenuItem>,
		});

		expect(screen.queryByRole("button", { name: /switch workspace/i })).not.toBeInTheDocument();
		fireEvent.pointerDown(screen.getByRole("button", { name: "Open account menu" }));

		const menu = await screen.findByRole("menu", { name: "Open account menu" });
		expect(menu).toBeVisible();
		expect(menu).toHaveAttribute("aria-label", "Account and workspace menu");
		expect(
			screen.getByRole("menuitem", { name: /Northstar Freight.*Current workspace/i }),
		).toHaveAttribute("aria-current", "true");
		expect(screen.getByRole("menuitem", { name: "Account settings" })).toBeVisible();
		expect(screen.getByRole("menuitem", { name: "Manage workspaces" })).toHaveAttribute(
			"href",
			"/app/settings?settings=workspace",
		);
		expect(screen.getByRole("menuitem", { name: "Members and access" })).toHaveAttribute(
			"href",
			"/app/settings?settings=members",
		);
		expect(screen.getByRole("menuitem", { name: "API keys" })).toHaveAttribute(
			"href",
			"/app/settings?settings=developer",
		);
	});

	it("keeps the creation dialog mounted when opened from the combined menu", async () => {
		renderSwitcher({
			activeWorkspaceId: "workspace-1",
			trigger: <button type="button">Open account menu</button>,
			workspaces,
		});

		fireEvent.pointerDown(screen.getByRole("button", { name: "Open account menu" }));
		fireEvent.click(await screen.findByRole("menuitem", { name: "Create workspace" }));

		expect(await screen.findByRole("dialog", { name: "Create a workspace" })).toBeVisible();
		expect(screen.getByRole("textbox", { name: "Workspace name" })).toHaveFocus();
	});

	it("keeps account actions available when no workspace summary is present", async () => {
		renderSwitcher({
			activeWorkspaceId: "workspace-1",
			trigger: <button type="button">Open account menu</button>,
			workspaces: [],
			children: <DropdownMenuItem>Account settings</DropdownMenuItem>,
		});

		fireEvent.pointerDown(screen.getByRole("button", { name: "Open account menu" }));

		expect(await screen.findByRole("menuitem", { name: "Account settings" })).toBeVisible();
		expect(screen.queryByText("Workspaces")).not.toBeInTheDocument();
	});
});
