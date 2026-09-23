import { Suspense } from "react";
import { CheckPanel } from "./_components/check-panel/check-panel";
import { checkSearchParamsCache } from "./search-params";
import CheckLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Check an email route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `check.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Check an email",
	description: "Verify an email address and review reachability, risk, and domain signals.",
};

type CheckPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function CheckPage({ searchParams }: CheckPageProps) {
	return (
		<Suspense fallback={<CheckLoading />}>
			<CheckRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function CheckRouteContent({ searchParams }: CheckPageProps) {
	const raw = await searchParams;
	checkSearchParamsCache.parse(raw);
	return <CheckPanel />;
}
