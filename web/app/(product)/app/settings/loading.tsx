import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

export default function SettingsLoading() {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<DashboardRouteFallback label="Loading settings…" />
		</main>
	);
}
