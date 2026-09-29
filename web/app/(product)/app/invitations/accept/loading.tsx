import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Accept invitation so the product shell can stream.
 * Owned by `accept.lit.ts`.
 */
export default function AcceptLoading() {
	return <DashboardRouteFallback label="Loading Accept invitation…" />;
}
