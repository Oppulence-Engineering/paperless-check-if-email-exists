import { Suspense } from "react";
import { redirect } from "next/navigation";

import { AcceptPanel } from "./_components/accept-panel/accept-panel";
import { AcceptSearchParamsSchema, acceptSearchParamsCache } from "./search-params";
import AcceptLoading from "./loading";
import { safeReturnTo } from "@/lib/auth/origin";

/**
 * @oppulence-gen kind=page
 * Authenticated invitation confirmation route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `accept.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Accept invitation - Oppulence",
	description: "Review and accept an organization invitation.",
};

type AcceptPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function AcceptPage({ searchParams }: AcceptPageProps) {
	return (
		<Suspense fallback={<AcceptLoading />}>
			<AcceptRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function AcceptRouteContent({ searchParams }: AcceptPageProps) {
	const raw = await searchParams;
	const query = AcceptSearchParamsSchema.safeParse(acceptSearchParamsCache.parse(raw));
	if (!query.success) redirect("/app/settings?settings=identity&invitation_error=invalid");
	return (
		<AcceptPanel
			invitationId={query.data.invitation}
			returnTo={safeReturnTo(query.data.return_to)}
		/>
	);
}
