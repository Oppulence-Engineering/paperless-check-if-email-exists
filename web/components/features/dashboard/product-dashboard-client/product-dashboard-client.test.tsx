// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, fireEvent, render, screen, within } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import type { ProductDashboardClientProps } from "./product-dashboard-client";

const navigation = vi.hoisted(() => ({ pathname: "/app/check" }));
vi.mock("next/navigation", () => ({
	usePathname: () => navigation.pathname,
	useSearchParams: () => new URLSearchParams(),
}));
vi.mock("@/components/auth/auth-gate", () => ({
	AuthGate: ({ children }: { children: React.ReactNode }) => children,
}));
vi.mock("@/components/features/workspaces/workspace-url-scope/workspace-url-scope", () => ({
	WorkspaceUrlScope: () => null,
}));
vi.mock("@/components/features/workspaces/workspace-switcher/workspace-switcher", () => ({
	WorkspaceSwitcher: () => <button type="button">Account menu</button>,
}));
vi.mock("@/lib/auth/auth-client", () => ({ authClient: { signOut: vi.fn() } }));

import { ProductDashboardClient } from "./product-dashboard-client";

afterEach(() => {
	cleanup();
	navigation.pathname = "/app/check";
});

const session = {
	authenticated: true,
	user: { id: "user-1", name: "Owner", email: "owner@example.com", organizationId: "org-1" },
} as ProductDashboardClientProps["initialSession"];

function shell() {
	return (
		<ProductDashboardClient initialSession={session} workspaces={[]}>
			<main>
				<h1>Check an email address</h1>
			</main>
		</ProductDashboardClient>
	);
}

describe("ProductDashboardClient", () => {
	it("keeps product routes in the icon rail and expands settings below it", async () => {
		const { container, rerender } = render(shell());

		expect(screen.getByRole("navigation", { name: "Main navigation" })).toBeInTheDocument();
		expect(screen.getByRole("link", { name: "Check an address" })).toHaveAttribute(
			"aria-current",
			"page",
		);
		expect(screen.getByRole("link", { name: "Lists" })).toHaveAttribute("href", "/app/lists");
		expect(screen.getByRole("link", { name: "Address history" })).toHaveAttribute(
			"href",
			"/app/history",
		);
		// Overview matches /app exactly, so it is not current on /app/check.
		expect(screen.getByRole("link", { name: "Overview" })).not.toHaveAttribute("aria-current");
		expect(screen.getByRole("heading", { name: "Check an email address" })).toBeVisible();
		// Pages bring their own landmark, so the shell must not add a second one.
		expect(container.querySelectorAll("main")).toHaveLength(1);

		// Settings expands its sections below it in the rail, not in a menu.
		const settings = screen.getByRole("button", { name: "Settings" });
		expect(settings).toHaveAttribute("aria-expanded", "false");
		fireEvent.click(settings);
		expect(settings).toHaveAttribute("aria-expanded", "true");
		expect(await screen.findByRole("link", { name: "Members" })).toHaveAttribute(
			"href",
			"/app/settings?settings=members",
		);

		// The open list closes when you go to another page.
		navigation.pathname = "/app/lists";
		rerender(shell());
		expect(screen.getByRole("button", { name: "Settings" })).toHaveAttribute(
			"aria-expanded",
			"false",
		);
	});

	it("toggles the settings sections in the mobile drawer", async () => {
		render(shell());
		fireEvent.click(screen.getByRole("button", { name: "Open navigation" }));
		const drawer = within(await screen.findByRole("dialog"));
		fireEvent.click(drawer.getByRole("button", { name: "Settings" }));
		expect(drawer.getByRole("link", { name: "Members" })).toHaveAttribute(
			"href",
			"/app/settings?settings=members",
		);
		// Not on the settings page, so no section is current.
		expect(drawer.getByRole("link", { name: "Workspace" })).not.toHaveAttribute("aria-current");
	});
});
