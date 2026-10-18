import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Analytics so the product shell can stream.
 * Owned by `analytics.lit.ts`.
 */
export default function AnalyticsLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Analytics…" />
		</main>
	);
}
