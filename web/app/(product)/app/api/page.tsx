import { Suspense } from "react";
import { ApiExplorerPanel } from "./_components/api-explorer-panel/api-explorer-panel";
import { apiExplorerSearchParamsCache } from "./search-params";
import ApiExplorerLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated API explorer route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `api.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "API explorer - Oppulence",
	description: "Run workspace API operations from the signed-in app.",
};

type ApiExplorerPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function ApiExplorerPage({ searchParams }: ApiExplorerPageProps) {
	return (
		<Suspense fallback={<ApiExplorerLoading />}>
			<ApiExplorerRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function ApiExplorerRouteContent({ searchParams }: ApiExplorerPageProps) {
	const raw = await searchParams;
	apiExplorerSearchParamsCache.parse(raw);
	return <ApiExplorerPanel />;
}
