import { notFound } from "next/navigation";
import { Suspense } from "react";

import {
	isPlatformAdmin,
	listActiveTenants,
	recordPlatformAdminAccess,
} from "@/lib/admin/platform-admin";
import { requireSession } from "@/lib/auth/session";
import { AdminPanel } from "./_components/admin-panel/admin-panel";
import { adminSearchParamsCache } from "./search-params";
import AdminLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Platform admin route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `admin.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Platform admin - Oppulence",
	description: "Cross-tenant view for support, restricted to the platform admin allowlist.",
};

type AdminPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function AdminPage({ searchParams }: AdminPageProps) {
	return (
		<Suspense fallback={<AdminLoading />}>
			<AdminRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function AdminRouteContent({ searchParams }: AdminPageProps) {
	const raw = await searchParams;
	adminSearchParamsCache.parse(raw);

	const session = await requireSession("/app/admin");
	const granted = isPlatformAdmin(session.user.email);
	await recordPlatformAdminAccess({
		actorId: session.user.id,
		action: "tenants.list",
		granted,
	});
	// Not 403: a person without platform access should not learn the page is
	// there at all.
	if (!granted) notFound();

	return <AdminPanel tenants={await listActiveTenants()} />;
}
