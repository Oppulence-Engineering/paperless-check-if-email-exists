import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Jobs so the product shell can stream.
 * Owned by `jobs.lit.ts`.
 */
export default function JobsLoading() {
	return <DashboardRouteFallback label="Loading Jobs…" />;
}
