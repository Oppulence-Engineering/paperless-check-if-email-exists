// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

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
	it("groups settings into focused sections with direct links", () => {
		render(<SettingsDashboardRoute {...props} />);

		expect(screen.getByRole("heading", { name: "Settings", level: 1 })).toBeInTheDocument();
		expect(screen.getByRole("navigation", { name: "Workspace settings" })).toBeInTheDocument();
		expect(screen.getByRole("link", { name: "Verification" })).toHaveAttribute(
			"href",
			"/app/settings?settings=verification",
		);
		expect(screen.getByRole("link", { name: "Members" })).toHaveAttribute(
			"href",
			"/app/settings?settings=members",
		);
		expect(screen.getByRole("link", { name: "API keys" })).toHaveAttribute(
			"href",
			"/app/settings?settings=developer",
		);
	});

	it("falls back to the overview for an unknown section", () => {
		render(<SettingsDashboardRoute {...props} section="missing" />);
		expect(screen.getByRole("heading", { name: "Settings", level: 1 })).toBeInTheDocument();
	});
});
