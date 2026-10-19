import { Suspense } from "react";
import { OutcomesPanel } from "./_components/outcomes-panel/outcomes-panel";
import { outcomesSearchParamsCache } from "./search-params";
import OutcomesLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Outcomes route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `outcomes.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Outcomes - Oppulence",
	description: "Review delivery outcomes and configure provider callbacks",
};

type OutcomesPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function OutcomesPage({ searchParams }: OutcomesPageProps) {
	return (
		<Suspense fallback={<OutcomesLoading />}>
			<OutcomesRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function OutcomesRouteContent({ searchParams }: OutcomesPageProps) {
	const raw = await searchParams;
	outcomesSearchParamsCache.parse(raw);
	return <OutcomesPanel />;
}
