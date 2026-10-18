import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Domains so the product shell can stream.
 * Owned by `domains.lit.ts`.
 */
export default function DomainsLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading Domains…" />
		</main>
	);
}
