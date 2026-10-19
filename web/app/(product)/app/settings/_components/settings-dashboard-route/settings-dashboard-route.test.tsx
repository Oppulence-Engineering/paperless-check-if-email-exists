// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("../identity-settings/identity-settings", () => ({
	IdentitySettings: ({ scope }: { scope: string }) => <p>identity scope: {scope}</p>,
}));

import { SettingsDashboardRoute } from "./settings-dashboard-route";

const props = {
	section: "overview",
	organizationId: "org-1",
	organizationRole: "owner",
	userId: "user-1",
	userName: "Owner",
	userEmail: "owner@example.com",
};

afterEach(cleanup);

describe("SettingsDashboardRoute", () => {
	it("opens the first section for an old, missing, or unknown key", async () => {
		const { rerender } = render(<SettingsDashboardRoute {...props} />);

		expect(screen.getByRole("heading", { name: "Settings", level: 1 })).toBeInTheDocument();
		expect(await screen.findByText("identity scope: workspace")).toBeInTheDocument();
		expect(
			screen.queryByRole("navigation", { name: "Workspace settings" }),
		).not.toBeInTheDocument();

		rerender(<SettingsDashboardRoute {...props} section="missing" />);
		expect(screen.getByText("identity scope: workspace")).toBeInTheDocument();
	});

	it("shows the account profile with a link to authentication", () => {
		render(<SettingsDashboardRoute {...props} section="account" />);

		expect(screen.getByRole("heading", { name: "Profile" })).toBeInTheDocument();
		expect(screen.getByText("owner@example.com")).toBeInTheDocument();
		expect(
			screen.getByRole("link", { name: "Manage authentication and sessions" }),
		).toHaveAttribute("href", "/app/settings?settings=security");
	});
});
