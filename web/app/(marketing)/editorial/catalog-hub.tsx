"use client";

import Link from "next/link";
import { useMemo, useState } from "react";

import { PageHero } from "@/components/platform/landing/blocks";
import { MONO, MUTED, Section } from "@/components/platform/landing/primitives";
import { ArrowUpRight, MagnifyingGlass } from "@/lib/icons";
import { cn } from "@/lib/utils";

import type { CapabilityPage } from "../catalog";
import { INTEGRATION_LOGOS } from "./logos";

type Kind = CapabilityPage["kind"];

const CATEGORIES: { kind: Kind; label: string; href: string }[] = [
	{ kind: "feature", label: "Features", href: "/features" },
	{ kind: "use-case", label: "Use cases", href: "/use-cases" },
	{ kind: "integration", label: "Integrations", href: "/integrations" },
	{ kind: "guide", label: "Guides", href: "/guides" },
];

export type HubItem = {
	kind: Kind;
	slug: string;
	href: string;
	title: string;
	body: string;
	/** One fact from the page, shown at the end of the row: [label, value]. */
	spec?: [string, string];
};

/** Index page: a split hero, category tabs, search, and one card per page. */
export function CatalogHub({
	kind,
	title,
	lede,
	items,
	counts,
}: {
	kind: Kind;
	title: string;
	lede: string;
	items: HubItem[];
	counts: Record<Kind, number>;
}) {
	const [query, setQuery] = useState("");
	const visible = useMemo(() => {
		const needle = query.trim().toLowerCase();
		if (!needle) return items;
		return items.filter((item) => `${item.title} ${item.body}`.toLowerCase().includes(needle));
	}, [items, query]);
	const active = CATEGORIES.find((category) => category.kind === kind) ?? CATEGORIES[0];

	return (
		<>
			<PageHero
				body={lede}
				eyebrow={{
					label: `${counts[kind]} ${counts[kind] === 1 ? "page" : "pages"}`,
					parent: "Verifier",
					href: "/",
				}}
				lead={title}
			/>
			<Section>
				<div className="flex flex-col gap-6 lg:flex-row lg:items-center lg:justify-between">
					<nav aria-label="Categories">
						<ul className="flex flex-wrap items-center gap-1">
							{CATEGORIES.map((category) => (
								<li key={category.kind}>
									<Link
										aria-current={category.kind === kind ? "page" : undefined}
										className={cn(
											"flex items-center gap-2 rounded-full px-3 py-1.5 text-[13px] transition-colors duration-150",
											category.kind === kind
												? "bg-white/[0.09] text-white"
												: "text-[#858585] hover:text-white",
										)}
										href={category.href}
									>
										{category.label}
										<span className={cn(MONO, "text-[11px]", MUTED)}>{counts[category.kind]}</span>
									</Link>
								</li>
							))}
						</ul>
					</nav>
					<label className="relative w-full lg:w-[320px]">
						<span className="sr-only">Search {active.label.toLowerCase()}</span>
						<MagnifyingGlass
							aria-hidden="true"
							className="pointer-events-none absolute top-1/2 left-4 size-4 -translate-y-1/2 text-[#858585]"
						/>
						<input
							className="h-11 w-full rounded-full border border-white/10 bg-[#090909] pr-4 pl-10 text-[15px] text-white placeholder:text-[#858585] focus:border-white/30 focus:outline-none"
							onChange={(event) => setQuery(event.target.value)}
							placeholder={`Search ${active.label.toLowerCase()}`}
							type="search"
							value={query}
						/>
					</label>
				</div>

				{visible.length === 0 ? (
					<p className={cn("mt-12 text-lg", MUTED)}>Nothing matches “{query}”.</p>
				) : (
					<ul className="mt-12 grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
						{visible.map((item) => {
							const Logo =
								item.kind === "integration"
									? INTEGRATION_LOGOS[item.slug as keyof typeof INTEGRATION_LOGOS]
									: undefined;
							return (
								<li key={item.href}>
									<Link
										className="group flex h-full flex-col justify-between gap-12 bg-[#111] p-6 transition-colors hover:bg-[#161616]"
										href={item.href}
									>
										<span className="flex items-start justify-between">
											{Logo ? <Logo aria-hidden="true" className="size-6" /> : <span />}
											<ArrowUpRight
												aria-hidden="true"
												className="size-5 text-[#858585] transition-colors group-hover:text-white"
											/>
										</span>
										<span>
											<span className="block text-lg text-white">{item.title}</span>
											<span className={cn("mt-1 block text-[15px] leading-[1.5]", MUTED)}>
												{item.body}
											</span>
											{item.spec ? (
												<span
													className={cn(
														MONO,
														"mt-6 flex items-baseline justify-between gap-4 border-t border-[#212121] pt-4 text-[12px]",
													)}
												>
													<span className={cn("tracking-[0.06em] uppercase", MUTED)}>
														{item.spec[0]}
													</span>
													<span className="text-right text-white/80">{item.spec[1]}</span>
												</span>
											) : null}
										</span>
									</Link>
								</li>
							);
						})}
					</ul>
				)}
			</Section>
		</>
	);
}
