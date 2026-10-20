import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Company research so the product shell can stream.
 * Owned by `research.lit.ts`.
 */
export default function ResearchLoading() {
	return <DashboardRouteFallback label="Loading Company research…" />;
}
