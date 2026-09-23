"use client";

import "client-only";

import * as React from "react";
import { useSearchParams } from "next/navigation";
import type { ComponentPropsWithoutRef } from "react";

import { switchBrowserWorkspace } from "@/lib/auth/client";

import { type WorkspaceUrlScopePropsFields } from "./workspace-url-scope.schema";

/**
 * @oppulence-gen kind=component
 * WorkspaceUrlScope makes a link carry its tenant.
 *
 * The active workspace lives on the session, so a link shared between two
 * people used to open whichever workspace the recipient happened to be in.
 * `?workspace=<slug>` names it in the URL: if the recipient belongs to that
 * workspace, this switches to it once on arrival. A slug the user is not a
 * member of is ignored rather than refused, because the link is a hint, not a
 * grant — membership is still what decides.
 *
 * Switching is session-wide, so two tabs cannot hold two tenants yet. That
 * needs the backend token to carry an explicit organization claim instead of
 * the session's active one; `docs/backend-integration.md` records the change.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `workspace-url-scope.lit.ts`.
 */
export type WorkspaceUrlScopeProps = WorkspaceUrlScopePropsFields &
	ComponentPropsWithoutRef<"span">;

/** The query parameter every product link may carry. */
export const WORKSPACE_PARAM = "workspace";

export function WorkspaceUrlScope({
	workspaces,
	activeWorkspaceId,
	...props
}: WorkspaceUrlScopeProps) {
	const requested = useSearchParams().get(WORKSPACE_PARAM)?.trim() ?? "";
	const attempted = React.useRef<string | null>(null);

	const target = requested
		? workspaces.find((workspace) => workspace.slug === requested || workspace.id === requested)
		: undefined;

	React.useEffect(() => {
		if (!target || target.id === activeWorkspaceId) return;
		if (attempted.current === target.id) return;
		attempted.current = target.id;
		void switchBrowserWorkspace(target.id).catch(() => {
			// A failed switch leaves the current workspace in place; the switcher
			// remains the way to move deliberately.
			attempted.current = null;
		});
	}, [activeWorkspaceId, target]);

	return <span data-slot="workspace-url-scope" hidden {...props} />;
}
