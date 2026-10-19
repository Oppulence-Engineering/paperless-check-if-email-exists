import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Find an email so the product shell can stream.
 * Owned by `finder.lit.ts`.
 */
export default function FinderLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Find an email…" />
		</main>
	);
}
