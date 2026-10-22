"use client";

import { Suspense } from "react";
import dynamic from "next/dynamic";

const DevToolkit = dynamic(() =>
	import("@/components/dev/dev-toolkit").then((module) => module.DevToolkit),
);
const RouteDevBadge = dynamic(() =>
	import("@/components/dev/route-dev-badge").then((module) => module.RouteDevBadge),
);

/**
 * Global dev-only overlays.
 *
 * These components are SSR-safe: browser APIs are isolated behind effects and
 * external stores provide server snapshots. Keeping server rendering enabled
 * lets Next validate instant navigation without the synthetic
 * server-render error produced by `next/dynamic({ ssr: false })`.
 */
export function DevProviders({ enabled }: { enabled: boolean }) {
	if (!enabled) return null;

	return (
		<>
			<Suspense fallback={null}>
				<RouteDevBadge />
			</Suspense>
			<Suspense fallback={null}>
				<DevToolkit />
			</Suspense>
		</>
	);
}
