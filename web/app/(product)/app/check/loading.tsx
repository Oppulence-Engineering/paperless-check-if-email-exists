import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Check an email so the product shell can stream.
 * Owned by `check.lit.ts`.
 */
export default function CheckLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Check an email…" />
		</main>
	);
}
