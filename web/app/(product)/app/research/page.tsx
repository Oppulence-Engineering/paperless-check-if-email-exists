import { Suspense } from "react";
import { ResearchPanel } from "./_components/research-panel/research-panel";
import { researchSearchParamsCache } from "./search-params";
import ResearchLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Company research route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `research.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Company research - Oppulence",
	description: "Find relevant contacts from company lists and review evidence before export",
};

type ResearchPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function ResearchPage({ searchParams }: ResearchPageProps) {
	return (
		<Suspense fallback={<ResearchLoading />}>
			<ResearchRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function ResearchRouteContent({ searchParams }: ResearchPageProps) {
	const raw = await searchParams;
	const params = researchSearchParamsCache.parse(raw);
	return (
		<ResearchPanel
			initialSessionId={/^[1-9][0-9]*$/.test(params.session) ? params.session : undefined}
		/>
	);
}
