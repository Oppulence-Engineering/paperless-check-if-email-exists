import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Outcomes so the product shell can stream.
 * Owned by `outcomes.lit.ts`.
 */
export default function OutcomesLoading() {
	return <DashboardRouteFallback label="Loading Outcomes…" />;
}
