// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { AdminPanel } from "./admin-panel";

const tenants = [
	{
		id: "org_1",
		name: "Acme Operations",
		slug: "acme-operations",
		createdAt: new Date("2026-02-01T00:00:00.000Z"),
		archivedAt: null,
		memberCount: 4,
	},
];

afterEach(cleanup);

describe("AdminPanel", () => {
	it("forwards accessible section props and renders its content", () => {
		render(<AdminPanel aria-label="Tenants" tenants={tenants} />);

		const panel = screen.getByRole("region", { name: "Tenants" });
		expect(panel).toHaveAttribute("data-slot", "admin-panel");
	});

	it("shows what support needs and no tenant content", () => {
		render(<AdminPanel aria-label="Tenants" tenants={tenants} />);

		expect(screen.getByText("Acme Operations")).toBeInTheDocument();
		expect(screen.getByText("acme-operations")).toBeInTheDocument();
		expect(screen.getByText("4 members")).toBeInTheDocument();
		expect(screen.getByText("1 workspace")).toBeInTheDocument();
	});

	it("says so when the deployment has no workspaces", () => {
		render(<AdminPanel aria-label="Tenants" tenants={[]} />);

		expect(screen.getByText("No workspaces yet.")).toBeInTheDocument();
		expect(screen.getByText("0 workspaces")).toBeInTheDocument();
	});
});
