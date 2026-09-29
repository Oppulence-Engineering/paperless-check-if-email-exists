// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

const mocks = vi.hoisted(() => ({
	switchBrowserWorkspace: vi.fn(() => Promise.resolve()),
	params: new URLSearchParams(),
}));

vi.mock("@/lib/auth/client", () => ({
	switchBrowserWorkspace: mocks.switchBrowserWorkspace,
}));
vi.mock("next/navigation", () => ({
	useSearchParams: () => mocks.params,
}));

import { WorkspaceUrlScope } from "./workspace-url-scope";

const workspaces = [
	{ id: "org_1", name: "Acme", slug: "acme", role: "owner", logoUrl: null },
	{ id: "org_2", name: "Northwind", slug: "northwind", role: "member", logoUrl: null },
];

function renderScope(search: string) {
	mocks.params = new URLSearchParams(search);
	return render(<WorkspaceUrlScope activeWorkspaceId="org_1" workspaces={workspaces} />);
}

afterEach(() => {
	cleanup();
	mocks.switchBrowserWorkspace.mockClear();
});

describe("WorkspaceUrlScope", () => {
	it("renders nothing a person can see", () => {
		const { container } = renderScope("");
		expect(container.querySelector('[data-slot="workspace-url-scope"]')).toHaveAttribute("hidden");
	});

	it("opens the workspace the link names", async () => {
		renderScope("workspace=northwind");
		await waitFor(() => {
			expect(mocks.switchBrowserWorkspace).toHaveBeenCalledWith("org_2");
		});
	});

	it("accepts an id as readily as a slug", async () => {
		renderScope("workspace=org_2");
		await waitFor(() => {
			expect(mocks.switchBrowserWorkspace).toHaveBeenCalledWith("org_2");
		});
	});

	it("stays put when the link names the workspace already open", () => {
		renderScope("workspace=acme");
		expect(mocks.switchBrowserWorkspace).not.toHaveBeenCalled();
	});

	it("ignores a workspace the user does not belong to", () => {
		// The link is a hint, not a grant: membership decides.
		renderScope("workspace=someone-elses-company");
		expect(mocks.switchBrowserWorkspace).not.toHaveBeenCalled();
	});

	it("does nothing without the parameter", () => {
		renderScope("tab=people");
		expect(mocks.switchBrowserWorkspace).not.toHaveBeenCalled();
	});

	it("switches once, not on every render", async () => {
		const view = renderScope("workspace=northwind");
		await waitFor(() => {
			expect(mocks.switchBrowserWorkspace).toHaveBeenCalledTimes(1);
		});
		view.rerender(<WorkspaceUrlScope activeWorkspaceId="org_1" workspaces={workspaces} />);
		expect(mocks.switchBrowserWorkspace).toHaveBeenCalledTimes(1);
	});
});
