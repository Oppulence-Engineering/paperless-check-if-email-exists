import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Integration setup so the product shell can stream.
 * Owned by `integrations.lit.ts`.
 */
export default function IntegrationsLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Integration setup…" />
		</main>
	);
}
