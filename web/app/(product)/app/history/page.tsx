import { Suspense } from "react";
import { HistoryPanel } from "./_components/history-panel/history-panel";
import { historySearchParamsCache } from "./search-params";
import HistoryLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Verification history route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `history.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Verification history | Check If Email Exists",
	description: "Look up earlier checks",
};

type HistoryPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function HistoryPage({ searchParams }: HistoryPageProps) {
	return (
		<Suspense fallback={<HistoryLoading />}>
			<HistoryRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function HistoryRouteContent({ searchParams }: HistoryPageProps) {
	const raw = await searchParams;
	historySearchParamsCache.parse(raw);
	return <HistoryPanel />;
}
