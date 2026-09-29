import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Domains so the product shell can stream.
 * Owned by `domains.lit.ts`.
 */
export default function DomainsLoading() {
	return <DashboardRouteFallback label="Loading Domains…" />;
}
