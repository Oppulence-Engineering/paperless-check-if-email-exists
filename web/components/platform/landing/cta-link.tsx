import Link from "next/link";

import { ArrowRight } from "@/lib/icons";
import { cn } from "@/lib/utils";

/** Visual weight of a landing call to action. */
export type CtaTone = "primary" | "secondary";

const TONES: Record<CtaTone, string> = {
	primary: "bg-white text-black hover:bg-white/90",
	secondary: "border border-white/15 text-white hover:border-white/30 hover:bg-white/[0.04]",
};

/** The pill style CTA links and buttons share (copied from screenshot-studio CtaLink). */
export function ctaClassName(tone: CtaTone): string {
	return cn(
		"group inline-flex h-11 shrink-0 items-center gap-1.5 whitespace-nowrap rounded-full pl-5 pr-4 text-[15px] font-medium tracking-[-0.01em] transition-[background-color,border-color,transform] duration-150 active:scale-[0.97] focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/40",
		TONES[tone],
	);
}

/** A pill link with a trailing chevron. */
export function CtaLink({
	className,
	href,
	label,
	tone = "primary",
}: {
	className?: string;
	href: string;
	label: string;
	tone?: CtaTone;
}) {
	return (
		<Link className={cn(ctaClassName(tone), className)} href={href}>
			{label}
			<ArrowRight
				aria-hidden="true"
				className="size-4 transition-transform duration-150 group-hover:translate-x-0.5"
			/>
		</Link>
	);
}
