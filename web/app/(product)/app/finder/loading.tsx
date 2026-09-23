import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Find an email so the product shell can stream.
 * Owned by `finder.lit.ts`.
 */
export default function FinderLoading() {
	return <DashboardRouteFallback label="Loading Find an email…" />;
}
