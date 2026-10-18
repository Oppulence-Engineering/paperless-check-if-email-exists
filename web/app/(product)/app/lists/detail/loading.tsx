import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for List detail so the product shell can stream.
 * Owned by `list-detail.lit.ts`.
 */
export default function ListDetailLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading List detail…" />
		</main>
	);
}
