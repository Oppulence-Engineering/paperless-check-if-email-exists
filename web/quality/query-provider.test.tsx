// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { useQueryClient, type QueryClient } from "@tanstack/react-query";
import { cleanup, render, waitFor } from "@testing-library/react";
import { useEffect } from "react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

import { QueryProvider } from "@/components/providers/query-provider";
import { WORKSPACE_CHANGED_EVENT } from "@/lib/auth/client";

vi.mock("@/components/dev/query-devtools", () => ({
	QueryDevtoolsPanel: () => null,
}));

const reload = vi.fn();

class TestBroadcastChannel extends EventTarget {
	static instances = new Set<TestBroadcastChannel>();

	constructor(readonly name: string) {
		super();
		TestBroadcastChannel.instances.add(this);
	}

	postMessage(data: unknown) {
		for (const channel of TestBroadcastChannel.instances) {
			if (channel !== this && channel.name === this.name) {
				channel.dispatchEvent(new MessageEvent("message", { data }));
			}
		}
	}

	close() {
		TestBroadcastChannel.instances.delete(this);
	}
}

function Probe({
	onClient,
	onUnmount,
}: {
	onClient: (client: QueryClient) => void;
	onUnmount: () => void;
}) {
	const client = useQueryClient();
	onClient(client);
	useEffect(() => onUnmount, [onUnmount]);
	return null;
}

describe("QueryProvider", () => {
	beforeEach(() => {
		vi.clearAllMocks();
		TestBroadcastChannel.instances.clear();
		vi.stubGlobal("BroadcastChannel", TestBroadcastChannel);
		Object.defineProperty(window, "location", {
			configurable: true,
			value: { href: "https://ui.example.test/app", reload },
		});
	});

	afterEach(() => {
		cleanup();
		vi.unstubAllGlobals();
	});

	it("replaces the query client and remounts its subtree when the organization changes", async () => {
		const clients: QueryClient[] = [];
		const onUnmount = vi.fn();
		const view = render(
			<QueryProvider organizationId="org-a">
				<Probe
					onClient={(client) => {
						clients.push(client);
					}}
					onUnmount={onUnmount}
				/>
			</QueryProvider>,
		);
		const orgAClient = clients.at(-1);
		expect(orgAClient).toBeDefined();
		if (!orgAClient) throw new Error("Expected the organization A query client");
		orgAClient.setQueryData(["tenant-data"], "org-a");

		view.rerender(
			<QueryProvider organizationId="org-b">
				<Probe
					onClient={(client) => {
						clients.push(client);
					}}
					onUnmount={onUnmount}
				/>
			</QueryProvider>,
		);

		const orgBClient = clients.at(-1);
		expect(orgBClient).toBeDefined();
		if (!orgBClient) throw new Error("Expected the organization B query client");
		expect(orgBClient).not.toBe(orgAClient);
		expect(orgBClient.getQueryData(["tenant-data"])).toBeUndefined();
		await waitFor(() => {
			expect(orgAClient.getQueryData(["tenant-data"])).toBeUndefined();
		});
		expect(onUnmount).toHaveBeenCalledOnce();
	});

	it("clears cached data and reloads when another tab switches workspaces", async () => {
		let client: QueryClient | undefined;
		render(
			<QueryProvider organizationId="org-a">
				<Probe
					onClient={(next) => {
						client = next;
					}}
					onUnmount={() => undefined}
				/>
			</QueryProvider>,
		);
		expect(client).toBeDefined();
		if (!client) throw new Error("Expected QueryProvider to expose its query client");
		client.setQueryData(["tenant-data"], "org-a");
		const cancelQueries = vi.spyOn(client, "cancelQueries");

		const otherTab = new TestBroadcastChannel("oppulence.workspace.v1");
		otherTab.postMessage({
			activeOrganizationId: "org-b",
		});

		await waitFor(() => {
			expect(reload).toHaveBeenCalledOnce();
		});
		expect(cancelQueries).toHaveBeenCalledOnce();
		expect(client.getQueryData(["tenant-data"])).toBeUndefined();
	});

	it("ignores malformed messages and messages for the current organization", async () => {
		let client: QueryClient | undefined;
		render(
			<QueryProvider organizationId="org-a">
				<Probe
					onClient={(next) => {
						client = next;
					}}
					onUnmount={() => undefined}
				/>
			</QueryProvider>,
		);
		client?.setQueryData(["tenant-data"], "org-a");
		const otherTab = new TestBroadcastChannel("oppulence.workspace.v1");

		otherTab.postMessage({ type: "workspace-changed" });
		otherTab.postMessage({
			activeOrganizationId: "org-a",
		});

		await Promise.resolve();
		expect(reload).not.toHaveBeenCalled();
		expect(client?.getQueryData(["tenant-data"])).toBe("org-a");
	});

	it("clears the active query client before reloading after a same-tab switch", async () => {
		let client: QueryClient | undefined;
		render(
			<QueryProvider organizationId="org-a">
				<Probe
					onClient={(next) => {
						client = next;
					}}
					onUnmount={() => undefined}
				/>
			</QueryProvider>,
		);
		expect(client).toBeDefined();
		if (!client) throw new Error("Expected QueryProvider to expose its query client");
		client.setQueryData(["tenant-data"], "org-a");
		const cancelQueries = vi.spyOn(client, "cancelQueries");

		window.dispatchEvent(
			new CustomEvent(WORKSPACE_CHANGED_EVENT, {
				detail: { activeOrganizationId: "org-b" },
			}),
		);

		await waitFor(() => {
			expect(reload).toHaveBeenCalledOnce();
		});
		expect(cancelQueries).toHaveBeenCalledOnce();
		expect(client.getQueryData(["tenant-data"])).toBeUndefined();
	});
});
