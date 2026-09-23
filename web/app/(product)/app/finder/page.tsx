import { Suspense } from "react";
import { z } from "zod";
import { FinderPanel } from "./_components/finder-panel/finder-panel";
import { finderSearchParamsCache } from "./search-params";
import FinderLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Find an email route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `finder.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Find an email - Oppulence",
	description: "Find and verify likely work addresses for a person and domain",
};

type FinderPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function FinderPage({ searchParams }: FinderPageProps) {
	return (
		<Suspense fallback={<FinderLoading />}>
			<FinderRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function FinderRouteContent({ searchParams }: FinderPageProps) {
	const raw = await searchParams;
	finderSearchParamsCache.parse(raw);
	const job = z.coerce.number().int().positive().safeParse(raw.job);
	return <FinderPanel initialJobId={job.success ? job.data : undefined} />;
}
