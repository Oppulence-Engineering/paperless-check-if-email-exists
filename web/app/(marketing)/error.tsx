"use client";

import { Button } from "@oppulence/ui/components/button";

/**
 * Marketing-segment error boundary. Keeps the public chrome from the
 * group layout and recovers with `retry`.
 */
export default function MarketingError({
	retry,
}: {
	error: Error & { digest?: string };
	retry: () => void;
}) {
	return (
		<main className="flex min-h-[60vh] items-center justify-center px-6">
			<section className="flex max-w-md flex-col items-center gap-4 text-center" role="alert">
				<h1 className="text-[28px] leading-tight tracking-[-0.02em] text-white">
					This page could not be loaded
				</h1>
				<p className="text-[15px] leading-6 text-[#858585]">
					Try again. If it keeps failing, go back to the homepage.
				</p>
				<Button
					className="h-11 rounded-full bg-white px-5 text-[15px] font-medium text-black hover:bg-white/90"
					onClick={() => retry()}
					type="button"
				>
					Try again
				</Button>
			</section>
		</main>
	);
}
