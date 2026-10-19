import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

export default function ProductLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading workspace…" />
		</main>
	);
}
