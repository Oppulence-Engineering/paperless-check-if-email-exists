import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Email lists so the product shell can stream.
 * Owned by `lists.lit.ts`.
 */
export default function ListsLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Email lists…" />
		</main>
	);
}
