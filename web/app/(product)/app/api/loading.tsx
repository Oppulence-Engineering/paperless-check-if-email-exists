import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for API explorer so the product shell can stream.
 * Owned by `api-explorer.lit.ts`.
 */
export default function ApiExplorerLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading API explorer…" />
		</main>
	);
}
