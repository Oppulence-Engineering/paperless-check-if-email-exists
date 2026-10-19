"use client";

import "client-only";

import { dashboardRequest, redirectBrowserIfUnauthorized } from "@/lib/auth/dashboard-fetch";
import {
	BrowserSessionResponseSchema,
	WorkspaceCreateResponseSchema,
	WorkspaceSwitchResponseSchema,
	type BrowserSessionResponse,
	type WorkspaceSummary,
} from "@/lib/auth/schemas";

export const WORKSPACE_CHANGE_CHANNEL_NAME = "oppulence.workspace.v1";
export const WORKSPACE_CHANGED_EVENT = "oppulence:workspace-changed";

function notifyBrowserWorkspaceChanged(activeOrganizationId: string): void {
	const detail = { activeOrganizationId };
	window.dispatchEvent(
		new CustomEvent(WORKSPACE_CHANGED_EVENT, {
			detail,
		}),
	);

	if (typeof BroadcastChannel === "undefined") return;
	const channel = new BroadcastChannel(WORKSPACE_CHANGE_CHANNEL_NAME);
	channel.postMessage(detail);
	channel.close();
}

/** Thrown when the database-backed session check is temporarily unavailable. */
class SessionUnavailableError extends Error {
	constructor() {
		super("session refresh is temporarily unavailable");
		this.name = "SessionUnavailableError";
	}
}

/**
 * Loads the current browser session from the server-side auth boundary. The
 * response contains display data only; backend JWTs are never exposed.
 */
export async function loadBrowserSession(): Promise<BrowserSessionResponse> {
	const res = await fetch("/api/auth/session", {
		credentials: "include",
		cache: "no-store",
		headers: { Accept: "application/json" },
		signal: AbortSignal.timeout(10_000),
	});
	if (res.status === 401) return { authenticated: false };
	if (res.status === 502 || res.status === 503) throw new SessionUnavailableError();
	if (!res.ok) throw new Error(`Session check failed: ${res.status}`);
	return BrowserSessionResponseSchema.parse(await res.json());
}

/** Switches the active organization through the audited same-origin auth boundary. */
export async function switchBrowserWorkspace(organizationId: string): Promise<void> {
	const response = await fetch("/api/auth/workspace", {
		method: "POST",
		credentials: "include",
		cache: "no-store",
		headers: { accept: "application/json", "content-type": "application/json" },
		body: JSON.stringify({ organizationId }),
		signal: AbortSignal.timeout(10_000),
	});
	redirectBrowserIfUnauthorized(response.status);
	if (response.status === 401) {
		return;
	}
	if (!response.ok) throw new Error("Workspace switch failed");
	const result = WorkspaceSwitchResponseSchema.parse(await response.json());
	notifyBrowserWorkspaceChanged(result.activeOrganizationId);
}

/** Creates and activates a workspace through the audited same-origin auth boundary. */
export async function createBrowserWorkspace(input: {
	name: string;
	slug: string;
}): Promise<WorkspaceSummary> {
	const response = await fetch("/api/auth/workspace", {
		method: "PUT",
		credentials: "include",
		cache: "no-store",
		headers: { accept: "application/json", "content-type": "application/json" },
		body: JSON.stringify(input),
		signal: AbortSignal.timeout(10_000),
	});
	redirectBrowserIfUnauthorized(response.status);
	if (response.status === 401) {
		throw new Error("Session expired");
	}
	if (!response.ok) throw new Error("Workspace creation failed");
	const result = WorkspaceCreateResponseSchema.parse(await response.json());
	notifyBrowserWorkspaceChanged(result.activeOrganizationId);
	return result.workspace;
}

/**
 * Browser dashboard fetch. Adds the login bounce that Server Components
 * cannot perform. Prefer `requestJson` for new validated JSON reads.
 */
export async function dashboardFetch(
	input: RequestInfo | URL,
	init?: RequestInit,
): Promise<Response> {
	const res = await dashboardRequest(input, init);
	redirectBrowserIfUnauthorized(res.status);
	return res;
}
