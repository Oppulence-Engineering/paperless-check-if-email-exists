import Link from "next/link";
import type { ComponentProps, ReactNode } from "react";

import { ArrowRight, ArrowSquareOut, Check } from "@/lib/icons";
import { cn } from "@/lib/utils";

/** Content column used by every editorial section. */
export function Container({
	className,
	wide = false,
	children,
}: {
	className?: string;
	wide?: boolean;
	children: ReactNode;
}) {
	return (
		<div
			className={cn(
				"mx-auto w-full px-4 sm:px-6 lg:px-8",
				wide ? "max-w-[1440px]" : "max-w-[1280px]",
				className,
			)}
		>
			{children}
		</div>
	);
}

export function Section({
	className,
	tone = "plain",
	id,
	labelledBy,
	children,
}: {
	className?: string;
	tone?: "plain" | "subtle";
	id?: string;
	labelledBy?: string;
	children: ReactNode;
}) {
	return (
		<section
			aria-labelledby={labelledBy}
			className={cn(
				"relative w-full py-16 sm:py-20 lg:py-28",
				tone === "subtle" && "border-y border-[var(--ed-line)]",
				"bg-[var(--ed-bg)]",
				className,
			)}
			id={id}
		>
			{children}
		</section>
	);
}

export function Eyebrow({
	className,
	tone = "muted",
	children,
}: {
	className?: string;
	tone?: "muted" | "accent" | "good";
	children: ReactNode;
}) {
	return (
		<p
			className={cn(
				"ed-mono text-[10.5px] uppercase tracking-[0.2em]",
				tone === "accent" && "text-[var(--ed-accent)]",
				tone === "good" && "text-[var(--ed-good)]",
				tone === "muted" && "text-[var(--ed-muted)]",
				className,
			)}
		>
			{children}
		</p>
	);
}

const DISPLAY_SIZE = {
	xl: "text-[42px] leading-[1.12] sm:text-[54px] lg:text-[64px]",
	lg: "text-[36px] leading-[1.15] sm:text-[44px] lg:text-[52px]",
	md: "text-[30px] leading-[1.2] sm:text-[36px] lg:text-[40px]",
	sm: "text-[24px] leading-[1.3] sm:text-[28px]",
	xs: "text-[20px] leading-[1.35]",
} as const;

/** Second tone inside a heading: the continuation reads in the muted color. */
export function Muted({ children }: { children: ReactNode }) {
	return <span className="text-[var(--ed-muted)]">{children}</span>;
}

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
		<Tag className={cn("ed-display text-[var(--ed-ink)]", DISPLAY_SIZE[size], className)} id={id}>
			{children}
		</Tag>
	);
}

export function Lead({ className, children }: { className?: string; children: ReactNode }) {
	return (
		<p
			className={cn(
				"ed-sans max-w-[46ch] text-pretty text-[15px] leading-[1.6] font-[450] text-[var(--ed-body)] sm:text-[16px]",
				className,
			)}
		>
			{children}
		</p>
	);
}

const BUTTON_VARIANT = {
	primary:
		"bg-[var(--ed-ink-strong)] text-[var(--ed-bg)] hover:opacity-90 border border-[var(--ed-ink-strong)]",
	secondary:
		"bg-[var(--ed-bg)] text-[var(--ed-ink)] border border-[var(--ed-line)] hover:bg-[var(--ed-subtle)]",
	accent: "bg-[var(--ed-accent)] text-white border border-[var(--ed-accent)] hover:opacity-90",
	inverse: "bg-white text-[#0c0a09] border border-white hover:bg-white/90",
	ghost:
		"bg-transparent text-[var(--ed-ink)] border border-transparent hover:bg-[var(--ed-subtle)]",
} as const;

const BUTTON_SIZE = {
	sm: "h-9 px-4 text-[13.5px]",
	md: "h-10 px-5 text-[14px]",
} as const;

type ButtonLinkProps = {
	href: string;
	variant?: keyof typeof BUTTON_VARIANT;
	size?: keyof typeof BUTTON_SIZE;
	arrow?: boolean;
	external?: boolean;
	className?: string;
	children: ReactNode;
};

/** Square call-to-action link. */
export function ButtonLink({
	href,
	variant = "primary",
	size = "sm",
	arrow = false,
	external = false,
	className,
	children,
}: ButtonLinkProps) {
	const classes = cn(
		"ed-sans inline-flex shrink-0 items-center justify-center gap-1.5 rounded-full font-[550] whitespace-nowrap transition-[background-color,opacity] duration-150 focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-[var(--ed-accent)]",
		BUTTON_VARIANT[variant],
		BUTTON_SIZE[size],
		className,
	);
	const icon = arrow ? (
		external ? (
			<ArrowSquareOut aria-hidden="true" className="size-3.5" />
		) : (
			<ArrowRight aria-hidden="true" className="size-3.5" />
		)
	) : null;
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

/** Square icon link used in bento cards. */
export function ArrowTile({ href, label }: { href: string; label: string }) {
	return (
		<Link
			aria-label={label}
			className="inline-flex size-8 shrink-0 items-center justify-center rounded-full bg-[var(--ed-ink-strong)] text-[var(--ed-bg)] transition-opacity hover:opacity-85"
			href={href}
		>
			<ArrowRight aria-hidden="true" className="size-3.5" />
		</Link>
	);
}

export function Chip({
	className,
	icon,
	children,
}: {
	className?: string;
	icon?: ReactNode;
	children: ReactNode;
}) {
	return (
		<span
			className={cn(
				"ed-sans inline-flex h-7 items-center gap-1.5 rounded-full border border-[var(--ed-line)] bg-[var(--ed-bg)] px-3 text-[12.5px] text-[var(--ed-body)]",
				className,
			)}
		>
			{icon}
			{children}
		</span>
	);
}

export function Card({ className, ...props }: ComponentProps<"div">) {
	return (
		<div
			className={cn("rounded-2xl border border-[var(--ed-line)] bg-[var(--ed-bg)]", className)}
			{...props}
		/>
	);
}

const ICON_TONE = {
	neutral: "bg-[var(--ed-panel)] text-[var(--ed-ink)]",
	accent: "bg-[var(--ed-accent-soft)] text-[var(--ed-accent)]",
	good: "bg-[var(--ed-good-soft)] text-[var(--ed-good)]",
	warn: "bg-[var(--ed-warn-soft)] text-[var(--ed-warn)]",
	bad: "bg-[var(--ed-bad-soft)] text-[var(--ed-bad)]",
} as const;

export function IconTile({
	tone = "good",
	className,
	children,
}: {
	tone?: keyof typeof ICON_TONE;
	className?: string;
	children: ReactNode;
}) {
	return (
		<span
			aria-hidden="true"
			className={cn(
				"inline-flex size-8 items-center justify-center rounded-lg [&_svg]:size-4",
				ICON_TONE[tone],
				className,
			)}
		>
			{children}
		</span>
	);
}

export function CheckItem({ title, detail }: { title: string; detail?: string }) {
	return (
		<li className="flex gap-3">
			<Check aria-hidden="true" className="mt-[3px] size-3.5 shrink-0 text-[var(--ed-good)]" />
			<span className="ed-sans">
				<span className="block text-[14px] text-[var(--ed-ink)]">{title}</span>
				{detail ? <span className="block text-[13px] text-[var(--ed-muted)]">{detail}</span> : null}
			</span>
		</li>
	);
}

/** Browser chrome around a product mock. Mocks are illustrations, not screenshots. */
export function AppWindow({
	title,
	className,
	children,
}: {
	title?: string;
	className?: string;
	children: ReactNode;
}) {
	return (
		<div
			className={cn(
				"ed-sans overflow-hidden rounded-xl border border-[#212121] bg-[#111111] text-white shadow-[var(--ed-shadow)]",
				className,
			)}
		>
			<div className="flex h-8 items-center gap-1.5 border-b border-[#212121] bg-[#181818] px-3">
				<span className="size-2.5 rounded-full bg-[#2e2e2e]" />
				<span className="size-2.5 rounded-full bg-[#2e2e2e]" />
				<span className="size-2.5 rounded-full bg-[#2e2e2e]" />
				{title ? <span className="ml-3 text-[11px] text-[#858585]">{title}</span> : null}
			</div>
			{children}
		</div>
	);
}

const CORNERS = [
	"-top-[6px] -left-[6px]",
	"-top-[6px] -right-[6px]",
	"-bottom-[6px] -left-[6px]",
	"-right-[6px] -bottom-[6px]",
];

/** Crosshair marks on the four corners of a figure, as on a drawing plate. */
export function Crosshairs() {
	return (
		<>
			{CORNERS.map((position) => (
				<svg
					aria-hidden="true"
					className={cn("pointer-events-none absolute size-3 text-[var(--ed-fig)]", position)}
					key={position}
					viewBox="0 0 12 12"
				>
					<path d="M6 0v12M0 6h12" stroke="currentColor" strokeWidth="1" />
				</svg>
			))}
		</>
	);
}

/** Drawing plate that frames a product mock: plain black ground, hairline frame, corner marks. */
export function Stage({ className, children }: { className?: string; children: ReactNode }) {
	return (
		<div className={cn("ed-sheet relative px-4 py-8 sm:px-10 sm:py-12", className)}>
			<Crosshairs />
			{children}
		</div>
	);
}

/** Figure plate with a figure number, a title, and an optional caption. */
export function Sheet({
	fig,
	title,
	caption,
	className,
	bodyClassName,
	children,
}: {
	fig?: string;
	title?: string;
	caption?: ReactNode;
	className?: string;
	bodyClassName?: string;
	children: ReactNode;
}) {
	return (
		<figure className={cn("ed-sheet relative", className)}>
			<Crosshairs />
			{fig || title ? (
				<div className="flex items-center justify-between gap-4 px-4 pt-3.5 sm:px-5">
					<span className="ed-fig-label shrink-0 text-[var(--ed-fig)]">
						{fig ? `Fig. ${fig}` : ""}
					</span>
					<span className="ed-fig-label truncate text-[#9e9e9e]">{title}</span>
				</div>
			) : null}
			<div className={bodyClassName}>{children}</div>
			{caption ? (
				<figcaption className="ed-fig-label border-t border-white/10 px-4 py-3 text-[#6b6b6b] [overflow-wrap:anywhere] sm:px-5">
					{caption}
				</figcaption>
			) : null}
		</figure>
	);
}
