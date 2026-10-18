import Link from "next/link";
import type { ComponentType, ReactNode } from "react";

import { ArrowRight, ArrowUpRight, X } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { CtaLink } from "./cta-link";
import { CONTAINER, Check, MONO, MUTED, Section, Stage } from "./primitives";

/*
 * Landing blocks copied from screenshot-studio
 * (components/platform-landing/blocks.tsx). Keep the class strings identical.
 */

/** An icon component from `@/lib/icons`. */
type Icon = ComponentType<{ className?: string; strokeWidth?: number; "aria-hidden"?: boolean }>;

/** A call to action rendered as a pill link. */
export type Cta = { href: string; label: string };

/** Sim-style split hero: two-tone headline left, body and calls to action right, visual below. */
export function PageHero({
	body,
	children,
	eyebrow,
	lead,
	primary = { href: "/sign-up", label: "Get started free" },
	secondary = { href: "/contact", label: "Talk to us" },
	trail,
}: {
	body: ReactNode;
	children?: ReactNode;
	eyebrow: { label: string; href?: string; parent?: string };
	lead: string;
	primary?: Cta;
	secondary?: Cta | null;
	trail?: string;
}) {
	return (
		<section className={cn(CONTAINER, "pt-16 pb-24 sm:pt-24 sm:pb-32")}>
			<p
				className={cn(
					MONO,
					"flex items-center gap-2 text-[12px] tracking-[0.08em] uppercase",
					MUTED,
				)}
			>
				{eyebrow.parent && eyebrow.href ? (
					<>
						<Link
							className="inline-flex items-center transition-colors hover:text-white"
							href={eyebrow.href}
						>
							{eyebrow.parent}
						</Link>
						<span aria-hidden="true">/</span>
					</>
				) : null}
				<span className="text-white">{eyebrow.label}</span>
			</p>
			<div className="mt-8 grid gap-10 lg:grid-cols-[1.3fr_1fr] lg:items-end lg:gap-16">
				<h1 className="text-[40px] leading-[1] tracking-[-0.035em] text-balance sm:text-[56px] lg:text-[64px]">
					<span className="block text-white">{lead}</span>
					{trail ? <span className={cn("block", MUTED)}>{trail}</span> : null}
				</h1>
				<div className="lg:pb-2">
					<p className="max-w-[46ch] text-lg leading-[1.5] text-white/65">{body}</p>
					<div className="mt-8 flex flex-wrap gap-3">
						<CtaLink href={primary.href} label={primary.label} />
						{secondary ? (
							<CtaLink href={secondary.href} label={secondary.label} tone="secondary" />
						) : null}
					</div>
				</div>
			</div>
			{children ? <div className="mt-16 sm:mt-20">{children}</div> : null}
		</section>
	);
}

/** A stage holding one centered product window, the default hero visual. */
export function HeroStage({ children, variant = 0 }: { children: ReactNode; variant?: number }) {
	return (
		<Stage className="items-center px-4 py-12 sm:px-12 sm:py-20" variant={variant}>
			{children}
		</Stage>
	);
}

/** Polar-style flat tiles: an optional mono snippet or icon, a title, and one paragraph. */
export function TileGrid({
	columns = 3,
	items,
}: {
	columns?: 2 | 3;
	items: { title: string; body: ReactNode; snippet?: string; icon?: Icon }[];
}) {
	return (
		<ul
			className={cn(
				"grid gap-4 sm:grid-cols-2",
				columns === 3 ? "lg:grid-cols-3" : "lg:grid-cols-2",
			)}
		>
			{items.map(({ body, icon: ItemIcon, snippet, title }) => (
				<li className="flex flex-col bg-[#111] p-8 sm:p-10" key={title}>
					{snippet ? (
						<code className={cn(MONO, "truncate text-[12px] text-white/50")}>{snippet}</code>
					) : null}
					{ItemIcon ? (
						<ItemIcon aria-hidden={true} className="size-5 text-white/80" strokeWidth={1.5} />
					) : null}
					<h3
						className={cn(
							"text-[22px] tracking-[-0.015em] text-white",
							snippet || ItemIcon ? "mt-12" : null,
						)}
					>
						{title}
					</h3>
					<div className={cn("mt-3 text-base leading-[1.6]", MUTED)}>{body}</div>
				</li>
			))}
		</ul>
	);
}

/** Sim-style capability grid: cells divided by 1px lines, each with an icon, title, and line. */
export function ControlGrid({
	items,
}: {
	items: { icon: Icon; title: string; body: ReactNode }[];
}) {
	return (
		<ul className="grid gap-px overflow-hidden rounded-xl border border-[#212121] bg-[#212121] sm:grid-cols-2 lg:grid-cols-3">
			{items.map(({ body, icon: ItemIcon, title }) => (
				<li className="bg-[#090909] p-7" key={title}>
					<ItemIcon aria-hidden={true} className="size-5 text-white/80" strokeWidth={1.5} />
					<h3 className="mt-6 text-base text-white">{title}</h3>
					<p className={cn("mt-1.5 text-[15px] leading-[1.55]", MUTED)}>{body}</p>
				</li>
			))}
		</ul>
	);
}

/** A boolean or text cell in a comparison table. */
export function Mark({ value }: { value: boolean | string }) {
	if (value === true) return <Check />;
	if (value === false)
		return (
			<span className="inline-flex size-5 items-center justify-center text-white/25">
				<X aria-label="Not included" className="size-3.5" />
			</span>
		);
	return <span className="text-white/85">{value}</span>;
}

/** A hairline table. The first column is the row label; the rest are values. */
export function RowTable({
	caption,
	head,
	monoLabels = false,
	rows,
}: {
	caption?: string;
	head?: ReactNode[];
	monoLabels?: boolean;
	rows: { label: ReactNode; cells: ReactNode[] }[];
}) {
	return (
		<div className="min-w-0 overflow-x-auto">
			<table className="w-full min-w-[560px] border-collapse text-left text-[15px]">
				{caption ? <caption className="sr-only">{caption}</caption> : null}
				{head ? (
					<thead>
						<tr className="border-b border-[#212121]">
							{head.map((cell, index) => (
								<th
									className={cn("py-4 pr-6 font-normal", MUTED, index === 0 ? "w-[34%]" : null)}
									key={index}
									scope="col"
								>
									{cell}
								</th>
							))}
						</tr>
					</thead>
				) : null}
				<tbody>
					{rows.map((row, index) => (
						<tr className="border-b border-[#212121] align-top" key={index}>
							<th
								className={cn(
									"py-4 pr-6 font-normal text-white",
									monoLabels && cn(MONO, "text-[13px]"),
								)}
								scope="row"
							>
								{row.label}
							</th>
							{row.cells.map((cell, cellIndex) => (
								<td className={cn("py-4 pr-6", MUTED)} key={cellIndex}>
									{cell}
								</td>
							))}
						</tr>
					))}
				</tbody>
			</table>
		</div>
	);
}

/** A question and its answer. */
export type Faq = { q: string; a: ReactNode };

/** FAQ entries as native details elements. No client script. */
function FaqList({ items }: { items: Faq[] }) {
	return (
		<div className="border-t border-[#212121]">
			{items.map((faq) => (
				<details className="group border-b border-[#212121]" key={faq.q}>
					<summary className="flex cursor-pointer list-none items-center justify-between gap-6 py-6 text-lg text-white [&::-webkit-details-marker]:hidden">
						{faq.q}
						<span
							aria-hidden="true"
							className="relative size-3.5 shrink-0 before:absolute before:inset-x-0 before:top-1/2 before:h-px before:bg-white after:absolute after:inset-y-0 after:left-1/2 after:w-px after:bg-white after:transition-transform group-open:after:scale-y-0"
						/>
					</summary>
					<div className={cn("max-w-[600px] pb-7 text-base leading-[1.65]", MUTED)}>{faq.a}</div>
				</details>
			))}
		</div>
	);
}

/** A full FAQ section: label on the left, questions on the right. */
export function FaqBlock({ items }: { items: Faq[] }) {
	return (
		<Section>
			<div className="grid gap-12 md:grid-cols-2 md:gap-8">
				<div>
					<p className="text-[26px] tracking-[-0.02em] text-white sm:text-[30px]">Questions</p>
					<p className={cn("mt-3 max-w-[360px] text-lg", MUTED)}>
						Anything else?{" "}
						<Link className="text-white underline-offset-4 hover:underline" href="/contact">
							Talk to us
						</Link>
						.
					</p>
				</div>
				<FaqList items={items} />
			</div>
		</Section>
	);
}

/** Cards linking to other pages. */
export function LinkCards({
	columns = 3,
	items,
}: {
	columns?: 2 | 3 | 4;
	items: { href: string; label: string; blurb?: string }[];
}) {
	return (
		<ul
			className={cn(
				"grid gap-4 sm:grid-cols-2",
				columns === 3 && "lg:grid-cols-3",
				columns === 4 && "lg:grid-cols-4",
			)}
		>
			{items.map((item) => (
				<li key={item.href}>
					<Link
						className="group flex h-full flex-col justify-between gap-12 bg-[#111] p-6 transition-colors hover:bg-[#161616]"
						href={item.href}
					>
						<ArrowUpRight
							aria-hidden="true"
							className="size-5 self-end text-[#858585] transition-colors group-hover:text-white"
						/>
						<span>
							<span className="block text-lg text-white">{item.label}</span>
							{item.blurb ? (
								<span className={cn("mt-1 block text-[15px] leading-[1.5]", MUTED)}>
									{item.blurb}
								</span>
							) : null}
						</span>
					</Link>
				</li>
			))}
		</ul>
	);
}

/** Text beside a staged visual, alternating sides down the page. */
export function SplitFeature({
	body,
	bullets,
	children,
	index = 0,
	link,
	title,
}: {
	body: ReactNode;
	bullets?: string[];
	children: ReactNode;
	index?: number;
	link?: Cta;
	title: string;
}) {
	const flip = index % 2 === 1;
	return (
		<div className="grid items-center gap-10 lg:grid-cols-2 lg:gap-16">
			<div className={cn(flip && "lg:order-2")}>
				<p className={cn(MONO, "text-[13px]", MUTED)}>{String(index + 1).padStart(2, "0")}</p>
				<h3 className="mt-3 text-[28px] leading-[1.2] tracking-[-0.02em] text-white">{title}</h3>
				<div className={cn("mt-4 text-lg leading-[1.55]", MUTED)}>{body}</div>
				{bullets ? (
					<ul className="mt-8 space-y-3">
						{bullets.map((bullet) => (
							<li className="flex items-start gap-3 text-[15px] text-white/85" key={bullet}>
								<Check className="mt-0.5" />
								{bullet}
							</li>
						))}
					</ul>
				) : null}
				{link ? (
					<ArrowLink className="mt-8" href={link.href}>
						{link.label}
					</ArrowLink>
				) : null}
			</div>
			<Stage className="min-h-[340px] items-center px-5 py-10 sm:p-12" variant={index + 1}>
				<div className="w-full max-w-[460px]">{children}</div>
			</Stage>
		</div>
	);
}

/** Closing call to action over a faded stage, after sim.ai's finale. */
export function FinalCta({
	lead = "From a raw list",
	note,
	trail = "to a safe send.",
}: {
	lead?: string;
	note?: ReactNode;
	trail?: string;
}) {
	return (
		<section className={cn(CONTAINER, "pb-24 sm:pb-32")}>
			<Stage variant={0}>
				<div className="absolute inset-0 bg-[radial-gradient(closest-side,rgba(9,9,9,0.92),rgba(9,9,9,0.6)_60%,rgba(9,9,9,0.15))]" />
				<div className="relative w-full px-6 py-28 text-center sm:py-40">
					<h2 className="text-[40px] leading-[1.05] tracking-[-0.035em] text-balance sm:text-[64px]">
						<span className="block text-white">{lead}</span>
						<span className="block text-white/55">{trail}</span>
					</h2>
					<div className="mt-12 flex flex-wrap justify-center gap-3">
						<CtaLink className="h-12" href="/sign-up" label="Get started free" />
						<CtaLink
							className="h-12 bg-black/40 backdrop-blur"
							href="/contact"
							label="Talk to us"
							tone="secondary"
						/>
					</div>
					{note ? <p className="mt-8 text-[15px] text-white/65">{note}</p> : null}
				</div>
			</Stage>
		</section>
	);
}

/** A text link with a trailing arrow. */
export function ArrowLink({
	children,
	className,
	href,
}: {
	children: ReactNode;
	className?: string;
	href: string;
}) {
	return (
		<Link
			className={cn(
				"group inline-flex items-center gap-1.5 text-[15px] text-white transition-colors hover:text-white/75",
				className,
			)}
			href={href}
		>
			{children}
			<ArrowRight
				aria-hidden="true"
				className="size-4 transition-transform group-hover:translate-x-0.5"
			/>
		</Link>
	);
}
