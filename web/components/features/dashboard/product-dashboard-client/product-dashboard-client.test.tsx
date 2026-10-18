// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import type { ProductDashboardClientProps } from "./product-dashboard-client";

vi.mock("next/navigation", () => ({
	usePathname: () => "/app/check",
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
});

describe("ProductDashboardClient", () => {
	it("keeps product routes in the icon rail and opens settings beside it", async () => {
		const session = {
			authenticated: true,
			user: { id: "user-1", name: "Owner", email: "owner@example.com", organizationId: "org-1" },
		} as ProductDashboardClientProps["initialSession"];

		const { container } = render(
			<ProductDashboardClient initialSession={session} workspaces={[]}>
				<main>
					<h1>Check an email address</h1>
				</main>
			</ProductDashboardClient>,
		);

		expect(screen.getByRole("navigation", { name: "Main navigation" })).toBeInTheDocument();
		expect(screen.getByRole("link", { name: "Check email" })).toHaveAttribute(
			"aria-current",
			"page",
		);
		expect(screen.getByRole("link", { name: "Lists" })).toHaveAttribute("href", "/app/lists");
		expect(screen.getByRole("link", { name: "History" })).toHaveAttribute("href", "/app/history");
		expect(screen.getByRole("heading", { name: "Check an email address" })).toBeVisible();
		// Pages bring their own landmark, so the shell must not add a second one.
		expect(container.querySelectorAll("main")).toHaveLength(1);

		fireEvent.pointerDown(screen.getByRole("button", { name: "Settings" }));
		expect(await screen.findByRole("menuitem", { name: "Members" })).toHaveAttribute(
			"href",
			"/app/settings?settings=members",
		);
	});
});
