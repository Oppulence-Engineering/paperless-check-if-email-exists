import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Platform API so the product shell can stream.
 * Owned by `api.lit.ts`.
 */
export default function ApiLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Platform API…" />
		</main>
	);
}
