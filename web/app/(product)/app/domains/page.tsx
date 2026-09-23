import { Suspense } from "react";
import { DomainsPanel } from "./_components/domains-panel/domains-panel";
import { domainsSearchParamsCache } from "./search-params";
import DomainsLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Domains route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `domains.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Domains - Oppulence",
	description: "Review and manage verified sending domains for this workspace",
};

type DomainsPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function DomainsPage({ searchParams }: DomainsPageProps) {
	return (
		<Suspense fallback={<DomainsLoading />}>
			<DomainsRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function DomainsRouteContent({ searchParams }: DomainsPageProps) {
	const raw = await searchParams;
	domainsSearchParamsCache.parse(raw);
	return <DomainsPanel />;
}
