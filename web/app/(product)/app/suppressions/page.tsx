import { Suspense } from "react";
import { SuppressionsPanel } from "./_components/suppressions-panel/suppressions-panel";
import { suppressionsSearchParamsCache } from "./search-params";
import SuppressionsLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Suppressions route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `suppressions.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Suppressions - Oppulence",
	description: "Search, check, add, import, export, and audit suppressed addresses",
};

type SuppressionsPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function SuppressionsPage({ searchParams }: SuppressionsPageProps) {
	return (
		<Suspense fallback={<SuppressionsLoading />}>
			<SuppressionsRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function SuppressionsRouteContent({ searchParams }: SuppressionsPageProps) {
	const raw = await searchParams;
	suppressionsSearchParamsCache.parse(raw);
	return <SuppressionsPanel />;
}
