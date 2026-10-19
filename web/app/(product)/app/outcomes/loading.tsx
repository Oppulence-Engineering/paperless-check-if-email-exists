import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Outcomes so the product shell can stream.
 * Owned by `outcomes.lit.ts`.
 */
export default function OutcomesLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Outcomes…" />
		</main>
	);
}
