import { Suspense } from "react";
import { PipelinesPanel } from "./_components/pipelines-panel/pipelines-panel";
import { pipelinesSearchParamsCache } from "./search-params";
import PipelinesLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Pipelines route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `pipelines.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Pipelines - Oppulence",
	description: "Create, run, pause, resume, and inspect verification pipelines",
};

type PipelinesPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function PipelinesPage({ searchParams }: PipelinesPageProps) {
	return (
		<Suspense fallback={<PipelinesLoading />}>
			<PipelinesRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function PipelinesRouteContent({ searchParams }: PipelinesPageProps) {
	const raw = await searchParams;
	pipelinesSearchParamsCache.parse(raw);
	return <PipelinesPanel />;
}
