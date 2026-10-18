import Link from "next/link";
import type { ReactNode } from "react";

import { ctaClassName, type CtaTone } from "@/components/platform/landing/cta-link";
import { CONTAINER, MONO, MUTED, Window } from "@/components/platform/landing/primitives";
import { ArrowRight, ArrowSquareOut } from "@/lib/icons";
import { cn } from "@/lib/utils";

/*
 * Editorial primitives in the screenshot-studio landing style. They wrap the
 * kit in components/platform/landing, so every page uses the same class
 * strings: the 1280px container, mono labels, two-tone headings, white pill
 * links, and dark windows.
 */

export { Section, Stage } from "@/components/platform/landing/primitives";

/** Content column with the landing gutter. */
export function Container({ className, children }: { className?: string; children: ReactNode }) {
	return <div className={cn(CONTAINER, className)}>{children}</div>;
}

/** Mono label above a heading. */
export function Eyebrow({ className, children }: { className?: string; children: ReactNode }) {
	return (
		<p className={cn(MONO, "text-[12px] tracking-[0.08em] uppercase", MUTED, className)}>
			{children}
		</p>
	);
}

const DISPLAY_SIZE = {
	xl: "text-[40px] leading-[1] tracking-[-0.035em] sm:text-[56px] lg:text-[64px]",
	lg: "text-[40px] leading-[1.05] tracking-[-0.035em] sm:text-[56px]",
	md: "text-[26px] leading-[1.35] tracking-[-0.02em] sm:text-[30px]",
	sm: "text-[24px] leading-[1.3] tracking-[-0.015em] sm:text-[28px]",
} as const;

/** Second tone inside a heading: the continuation reads in the muted grey. */
export function Muted({ children }: { children: ReactNode }) {
	return <span className={MUTED}>{children}</span>;
}

/** Heading in the landing type scale. */
export function Display({
	as: Tag = "h2",
	size = "lg",
	className,
	id,
	children,
}: {
	as?: "h1" | "h2" | "h3" | "p";
	size?: keyof typeof DISPLAY_SIZE;
	className?: string;
	id?: string;
	children: ReactNode;
}) {
	return (
		<Tag className={cn("text-balance text-white", DISPLAY_SIZE[size], className)} id={id}>
			{children}
		</Tag>
	);
}

/** Body text under a hero heading. */
export function Lead({ className, children }: { className?: string; children: ReactNode }) {
	return (
		<p className={cn("max-w-[46ch] text-lg leading-[1.5] text-white/65", className)}>{children}</p>
	);
}

/** Pill call-to-action link with a trailing chevron. */
export function ButtonLink({
	href,
	variant = "primary",
	external = false,
	className,
	children,
}: {
	href: string;
	variant?: CtaTone;
	external?: boolean;
	className?: string;
	children: ReactNode;
}) {
	const classes = cn(ctaClassName(variant), className);
	const Icon = external ? ArrowSquareOut : ArrowRight;
	const icon = (
		<Icon
			aria-hidden="true"
			className="size-4 transition-transform duration-150 group-hover:translate-x-0.5"
		/>
	);
	if (external || href.startsWith("mailto:")) {
		return (
			<a
				className={classes}
				href={href}
				rel={external ? "noopener noreferrer" : undefined}
				target={external ? "_blank" : undefined}
			>
				{children}
				{icon}
			</a>
		);
	}
	return (
		<Link className={classes} href={href}>
			{children}
			{icon}
		</Link>
	);
}

/** A dark product window around a mock. Mocks are illustrations, not screenshots. */
export function AppWindow({
	title,
	className,
	children,
}: {
	title: string;
	className?: string;
	children: ReactNode;
}) {
	return (
		<Window className={cn("w-full", className)} title={title}>
			{children}
		</Window>
	);
}

/** A figure in a dark product window: a title, the drawing, and an optional caption bar. */
export function Sheet({
	title,
	caption,
	className,
	bodyClassName,
	children,
}: {
	title: ReactNode;
	caption?: ReactNode;
	className?: string;
	bodyClassName?: string;
	children: ReactNode;
}) {
	return (
		<Window className={cn("w-full min-w-0", className)} title={title}>
			<div className={bodyClassName}>{children}</div>
			{caption ? (
				<p
					className={cn(
						MONO,
						"rounded-b-xl border-t border-white/[0.06] bg-black/30 px-5 py-3 text-[11.5px] leading-[1.6] text-[#858585]",
					)}
				>
					{caption}
				</p>
			) : null}
		</Window>
	);
}
