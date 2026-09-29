import { Suspense } from "react";
import { ListsPanel } from "./_components/lists-panel/lists-panel";
import { listsSearchParamsCache } from "./search-params";
import ListsLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Email lists route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `lists.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Email lists | Check If Email Exists",
	description: "Manage uploaded lists",
};

type ListsPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function ListsPage({ searchParams }: ListsPageProps) {
	return (
		<Suspense fallback={<ListsLoading />}>
			<ListsRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function ListsRouteContent({ searchParams }: ListsPageProps) {
	const raw = await searchParams;
	listsSearchParamsCache.parse(raw);
	return <ListsPanel />;
}
