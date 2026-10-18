import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Pipelines so the product shell can stream.
 * Owned by `pipelines.lit.ts`.
 */
export default function PipelinesLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Pipelines…" />
		</main>
	);
}
