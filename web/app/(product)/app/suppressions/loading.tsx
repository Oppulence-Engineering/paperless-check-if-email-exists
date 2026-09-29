import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Suppressions so the product shell can stream.
 * Owned by `suppressions.lit.ts`.
 */
export default function SuppressionsLoading() {
	return <DashboardRouteFallback label="Loading Suppressions…" />;
}
