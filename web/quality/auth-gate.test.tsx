// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import type { BrowserSessionResponse } from "@/lib/auth/schemas";

const authClient = vi.hoisted(() => ({
	loadBrowserSession: vi.fn(),
}));
const storageCleanup = vi.hoisted(() => ({
	removeBrowserStorageKeys: vi.fn(),
}));

vi.mock("@/lib/auth/client", async (importOriginal) => {
	const actual = await importOriginal<typeof import("@/lib/auth/client")>();
	return {
		...actual,
		loadBrowserSession: authClient.loadBrowserSession,
	};
});
vi.mock("@/lib/storage/scoped-storage", () => storageCleanup);

import { AuthGate, useAuthSession } from "@/components/auth/auth-gate";

const initialSession: Extract<BrowserSessionResponse, { authenticated: true }> = {
	authenticated: true,
	expiresAt: 2_000_000_000,
	user: {
		id: "user-1",
		email: "operator@example.com",
		organizationId: "org-1",
		permissions: [],
	},
};

function SessionConsumer() {
	const session = useAuthSession();
	return <p>{session.user.email}</p>;
}

afterEach(() => {
	cleanup();
	vi.clearAllMocks();
});

describe("AuthGate", () => {
	it("renders the server-verified session without a client loading flash", () => {
		render(
			<AuthGate initialSession={initialSession}>
				<SessionConsumer />
			</AuthGate>,
		);

		expect(screen.getByText("operator@example.com")).toBeVisible();
		expect(screen.queryByText("Checking session")).not.toBeInTheDocument();
		expect(authClient.loadBrowserSession).not.toHaveBeenCalled();
		expect(storageCleanup.removeBrowserStorageKeys).toHaveBeenCalledWith([
			"oppulence.relationship-graph.saved-views.v1",
			"oppulence.relationship-graph.saved-views.v2-imported",
		]);
	});
});
