"use client";

import { WarningCircleIcon } from "@/lib/icons";

import { Button } from "@oppulence/ui/components/button";

export default function SettingsError({
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
					<h1 className="text-lg font-medium text-white">Settings could not be loaded</h1>
					<p className="mt-1 text-sm text-[#858585]">Try the request again.</p>
				</div>
				<Button className="rounded-lg" type="button" onClick={reset}>
					Try again
				</Button>
			</section>
		</main>
	);
}
