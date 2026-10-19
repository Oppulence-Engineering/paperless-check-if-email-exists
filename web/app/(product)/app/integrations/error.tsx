"use client";

import { WarningCircleIcon } from "@/lib/icons";

import { Button } from "@oppulence/ui/components/button";

/**
 * @oppulence-gen kind=page
 * Route error boundary for Integration setup. Next.js requires a Client Component here
 * so the reset callback can remount the segment. Owned by `integrations.lit.ts`.
 */
export default function IntegrationsError({
	reset,
}: {
	error: Error & { digest?: string };
	reset: () => void;
}) {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<section className="flex max-w-md flex-col items-center gap-4 text-center" role="alert">
				<WarningCircleIcon aria-hidden className="size-8 text-destructive" />
				<div>
					<h1 className="text-lg font-medium text-white">Integration setup could not be loaded</h1>
					<p className="mt-1 text-sm text-[#858585]">Try the request again.</p>
				</div>
				<Button className="rounded-lg" type="button" onClick={reset}>
					Try again
				</Button>
			</section>
		</main>
	);
}
