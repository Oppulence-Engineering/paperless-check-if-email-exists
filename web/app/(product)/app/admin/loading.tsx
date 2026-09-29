import { DashboardRouteFallback } from "@/lib/query/prefetch-hydration";

/**
 * @oppulence-gen kind=page
 * Independent loading boundary for Platform admin so the product shell can stream.
 * Owned by `admin.lit.ts`.
 */
export default function AdminLoading() {
	return <DashboardRouteFallback label="Loading Platform admin…" />;
}
