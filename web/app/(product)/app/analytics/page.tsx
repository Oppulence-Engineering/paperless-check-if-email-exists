import { Suspense } from "react";
import { AnalyticsPanel } from "./_components/analytics-panel/analytics-panel";
import { analyticsSearchParamsCache } from "./search-params";
import AnalyticsLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Analytics route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `analytics.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Analytics - Oppulence",
	description: "Explore verification results, activity, source quality, and domain reputation",
};

type AnalyticsPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function AnalyticsPage({ searchParams }: AnalyticsPageProps) {
	return (
		<Suspense fallback={<AnalyticsLoading />}>
			<AnalyticsRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function AnalyticsRouteContent({ searchParams }: AnalyticsPageProps) {
	const raw = await searchParams;
	analyticsSearchParamsCache.parse(raw);
	return <AnalyticsPanel />;
}
