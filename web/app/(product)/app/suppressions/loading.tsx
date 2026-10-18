import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Suppressions so the product shell can stream.
 * Owned by `suppressions.lit.ts`.
 */
export default function SuppressionsLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Suppressions…" />
		</main>
	);
}
