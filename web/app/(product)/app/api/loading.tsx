import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for API explorer so the product shell can stream.
 * Owned by `api-explorer.lit.ts`.
 */
export default function ApiExplorerLoading() {
	return <DashboardRouteFallback label="Loading API explorer…" />;
}
