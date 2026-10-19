import { Suspense } from "react";
import { notFound } from "next/navigation";

import { isPlatformAdmin, recordPlatformAdminAccess } from "@/lib/admin/platform-admin";
import { requireSession } from "@/lib/auth/session";
import { ApiPanel } from "./_components/api-panel/api-panel";
import { apiSearchParamsCache } from "./search-params";
import ApiLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Platform API route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `api.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Platform API - Oppulence",
	description: "Restricted Rust platform control plane.",
};

type ApiPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function ApiPage({ searchParams }: ApiPageProps) {
	return (
		<Suspense fallback={<ApiLoading />}>
			<ApiRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function ApiRouteContent({ searchParams }: ApiPageProps) {
	const raw = await searchParams;
	apiSearchParamsCache.parse(raw);
	const session = await requireSession("/app/admin/api");
	const granted = isPlatformAdmin(session.user.email);
	await recordPlatformAdminAccess({ actorId: session.user.id, action: "backend.view", granted });
	if (!granted) notFound();
	return <ApiPanel />;
}
