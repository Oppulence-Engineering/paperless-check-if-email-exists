// @vitest-environment jsdom

import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import {
	createBrowserWorkspace,
	switchBrowserWorkspace,
	WORKSPACE_CHANGE_CHANNEL_NAME,
	WORKSPACE_CHANGED_EVENT,
} from "@/lib/auth/client";

const postedMessages: unknown[] = [];

class TestBroadcastChannel {
	constructor(readonly name: string) {}

	postMessage(data: unknown) {
		postedMessages.push(data);
	}

	close() {}
}

function jsonResponse(body: unknown): Response {
	return new Response(JSON.stringify(body), {
		headers: { "content-type": "application/json" },
		status: 200,
	});
}

describe("browser workspace changes", () => {
	beforeEach(() => {
		postedMessages.length = 0;
		vi.stubGlobal("BroadcastChannel", TestBroadcastChannel);
	});

	afterEach(() => {
		vi.restoreAllMocks();
		vi.unstubAllGlobals();
	});

	it("notifies the active tab and other tabs after a validated workspace switch", async () => {
		vi.stubGlobal(
			"fetch",
			vi.fn().mockResolvedValue(jsonResponse({ activeOrganizationId: "organization-2" })),
		);
		const localChanges: unknown[] = [];
		window.addEventListener(WORKSPACE_CHANGED_EVENT, (event) => {
			localChanges.push((event as CustomEvent<unknown>).detail);
		});

		await switchBrowserWorkspace("organization-2");

		expect(localChanges).toEqual([{ activeOrganizationId: "organization-2" }]);
		expect(postedMessages).toEqual([{ activeOrganizationId: "organization-2" }]);
		expect(new TestBroadcastChannel(WORKSPACE_CHANGE_CHANNEL_NAME).name).toBe(
			WORKSPACE_CHANGE_CHANNEL_NAME,
		);
	});

	it("does not notify any tab when the switch response fails validation", async () => {
		vi.stubGlobal("fetch", vi.fn().mockResolvedValue(jsonResponse({ activeOrganizationId: "" })));
		const localChange = vi.fn();
		window.addEventListener(WORKSPACE_CHANGED_EVENT, localChange);

		await expect(switchBrowserWorkspace("organization-2")).rejects.toThrow();

		expect(localChange).not.toHaveBeenCalled();
		expect(postedMessages).toEqual([]);
	});

	it("notifies tabs with the authoritative organization returned on creation", async () => {
		vi.stubGlobal(
			"fetch",
			vi.fn().mockResolvedValue(
				jsonResponse({
					activeOrganizationId: "organization-created",
					workspace: {
						id: "organization-created",
						logoUrl: null,
						name: "Created Workspace",
						role: "owner",
						slug: "created-workspace",
					},
				}),
			),
		);
		const localChanges: unknown[] = [];
		window.addEventListener(WORKSPACE_CHANGED_EVENT, (event) => {
			localChanges.push((event as CustomEvent<unknown>).detail);
		});

		const workspace = await createBrowserWorkspace({
			name: "Created Workspace",
			slug: "created-workspace",
		});

		expect(workspace.id).toBe("organization-created");
		expect(localChanges).toEqual([{ activeOrganizationId: "organization-created" }]);
		expect(postedMessages).toEqual([{ activeOrganizationId: "organization-created" }]);
	});
});
