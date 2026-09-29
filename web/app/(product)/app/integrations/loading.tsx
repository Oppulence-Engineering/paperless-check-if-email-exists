import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Integration setup so the product shell can stream.
 * Owned by `integrations.lit.ts`.
 */
export default function IntegrationsLoading() {
	return <DashboardRouteFallback label="Loading Integration setup…" />;
}
