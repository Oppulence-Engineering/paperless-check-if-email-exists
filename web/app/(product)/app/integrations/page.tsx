import { Suspense } from "react";
import { IntegrationsPanel } from "./_components/integrations-panel/integrations-panel";
import { integrationsSearchParamsCache } from "./search-params";
import IntegrationsLoading from "./loading";

/**
 * @oppulence-gen kind=page
 * Authenticated Integration setup route. The page stays a Server Component; interactivity
 * lives in the route-private panel. Owned by `integrations.lit.ts`.
 *
 * Instant navigation: searchParams and prefetch stay behind Suspense. Do not
 * add `instant = false` here — the product layout already opts the tree out
 * of instant-nav validation because auth must finish before headers.
 */
export const metadata = {
	title: "Integration setup - Oppulence",
	description: "Set up onboarding, provider callbacks, migration, and service status safely",
};

type IntegrationsPageProps = {
	searchParams: Promise<Record<string, string | string[] | undefined>>;
};

export default function IntegrationsPage({ searchParams }: IntegrationsPageProps) {
	return (
		<Suspense fallback={<IntegrationsLoading />}>
			<IntegrationsRouteContent searchParams={searchParams} />
		</Suspense>
	);
}

async function IntegrationsRouteContent({ searchParams }: IntegrationsPageProps) {
	const raw = await searchParams;
	integrationsSearchParamsCache.parse(raw);
	return <IntegrationsPanel />;
}
