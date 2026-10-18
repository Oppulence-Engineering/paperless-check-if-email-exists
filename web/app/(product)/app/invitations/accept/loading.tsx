import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Accept invitation so the product shell can stream.
 * Owned by `accept.lit.ts`.
 */
export default function AcceptLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Accept invitation…" />
		</main>
	);
}
