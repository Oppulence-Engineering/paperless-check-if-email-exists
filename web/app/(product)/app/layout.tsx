import type { ReactNode } from "react";

import { QueryProvider } from "@/components/providers/query-provider";
import { listUserWorkspaces, requireSession } from "@/lib/auth/session";
import { branding } from "@/lib/auth/branding";
import { getAuthRuntimeConfig } from "@/lib/auth/config";
import { workspacePlan } from "@/lib/billing/workspace-plan";

import { ProductDashboardClient } from "@/components/features/dashboard/product-dashboard-client/product-dashboard-client";

// Authentication must complete before response headers are committed so an
// anonymous direct request receives a real HTTP redirect, not an in-stream one.
export const instant = false;

export default async function ProductLayout({ children }: { children: ReactNode }) {
	const session = await requireSession("/app");
	const [brand, workspaces, billing] = await Promise.all([
		branding({ organizationId: session.membership.organizationId }),
		listUserWorkspaces(session.user.id),
		workspacePlan(session.membership.organizationId),
	]);
	const initialSession = {
		authenticated: true as const,
		user: {
			id: session.user.id,
			name: session.user.name,
			email: session.user.email,
			emailVerified: session.user.emailVerified,
			sessionId: session.session.id,
			organizationId: session.membership.organizationId,
			role: session.membership.role,
			permissions: [],
		},
		billing: { ...billing, enabled: Boolean(getAuthRuntimeConfig().stripe) },
		expiresAt: Math.floor(session.session.expiresAt.getTime() / 1_000),
	};

	return (
		<QueryProvider
			key={`${session.user.id}:${session.membership.organizationId}`}
			organizationId={session.membership.organizationId}
		>
			{/* The dashboard is shared route UI, so it belongs in the layout. Next
          preserves this instance while replacing the leaf page slot below. */}
			<ProductDashboardClient
				brand={brand}
				devBadgeOffset={process.env.NODE_ENV === "development"}
				initialSession={initialSession}
				workspaces={workspaces}
			>
				{children}
			</ProductDashboardClient>
		</QueryProvider>
	);
}
