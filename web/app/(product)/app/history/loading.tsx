import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Verification history so the product shell can stream.
 * Owned by `history.lit.ts`.
 */
export default function HistoryLoading() {
	return <DashboardRouteFallback label="Loading Verification history…" />;
}
