import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Verification history so the product shell can stream.
 * Owned by `history.lit.ts`.
 */
export default function HistoryLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Verification history…" />
		</main>
	);
}
