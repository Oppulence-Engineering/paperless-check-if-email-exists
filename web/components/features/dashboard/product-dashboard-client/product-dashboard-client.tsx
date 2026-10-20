"use client";

import "client-only";

import type { CSSProperties, ReactNode } from "react";

import { AuthGate } from "@/components/auth/auth-gate";
import { WorkspaceUrlScope } from "@/components/features/workspaces/workspace-url-scope/workspace-url-scope";
import { PlatformSidebar } from "@/components/platform/shell/platform-sidebar";
import { surface } from "@/components/platform/shell/shell-config";
import type { Branding } from "@/lib/auth/branding.schema";
import type { BrowserSessionResponse, WorkspaceSummary } from "@/lib/auth/schemas";
import { cn } from "@/lib/utils";

export type ProductDashboardClientProps = {
	children: ReactNode;
	brand?: Branding;
	devBadgeOffset?: boolean;
	initialSession: Extract<BrowserSessionResponse, { authenticated: true }>;
	workspaces: WorkspaceSummary[];
};

/**
 * Signed-in app chrome: a collapsed icon rail beside the page, with the page
 * on a rounded panel inset from the rail. Pages bring their own `<main>`.
 */
export function ProductDashboardClient({
	brand,
	children,
	devBadgeOffset,
	initialSession,
	workspaces,
}: ProductDashboardClientProps) {
	const brandStyle = brand
		? ({
				"--main-color": brand.primaryColor,
				"--oppulence-orange": brand.accentColor,
			} as CSSProperties)
		: undefined;

	return (
		<div className="contents" data-slot="product-dashboard-client" style={brandStyle}>
			<AuthGate initialSession={initialSession}>
				<WorkspaceUrlScope
					activeWorkspaceId={initialSession.user.organizationId}
					workspaces={workspaces}
				/>
				<div
					className={cn(
						"flex min-h-[var(--app-vh)] flex-col text-foreground md:flex-row",
						surface.base,
					)}
					data-slot="dashboard-shell"
				>
					<PlatformSidebar
						brand={brand}
						devBadgeOffset={devBadgeOffset}
						session={initialSession}
						workspaces={workspaces}
					/>
					<div className="min-w-0 flex-1 md:py-2 md:pr-2">
						<div
							className={cn(
								"min-h-full md:min-h-[calc(var(--app-vh)-1rem)] md:rounded-2xl md:border md:shadow-xs",
								surface.panel,
								surface.border,
							)}
							data-slot="dashboard-workspace"
						>
							{children}
						</div>
					</div>
				</div>
			</AuthGate>
		</div>
	);
}
