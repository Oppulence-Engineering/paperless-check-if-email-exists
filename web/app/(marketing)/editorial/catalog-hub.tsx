"use client";

import Link from "next/link";
import { useMemo, useState } from "react";

import { ArrowRight, MagnifyingGlass } from "@/lib/icons";
import { cn } from "@/lib/utils";

import type { CapabilityPage } from "../catalog";
import { INTEGRATION_LOGOS } from "./logos";
import { Container, Display, Lead } from "./ui";

type Kind = CapabilityPage["kind"];

const CATEGORIES: { kind: Kind; label: string; href: string; numeral: string }[] = [
	{ kind: "feature", label: "Features", href: "/features", numeral: "I" },
	{ kind: "use-case", label: "Use cases", href: "/use-cases", numeral: "II" },
	{ kind: "integration", label: "Integrations", href: "/integrations", numeral: "III" },
	{ kind: "guide", label: "Guides", href: "/guides", numeral: "IV" },
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

/** Index page laid out as a table of contents, with search. */
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
		<section className="bg-[var(--ed-bg)] pt-12 pb-20 sm:pt-16">
			<Container>
				<div className="ed-fig-label flex items-center justify-between border-y border-[var(--ed-line)] py-3 text-[#757575]">
					<span>Verifier · {active.label}</span>
					<span>
						{counts[kind]} {counts[kind] === 1 ? "page" : "pages"}
					</span>
				</div>
				<div className="mt-12 grid gap-10 lg:grid-cols-[1fr_auto] lg:items-end">
					<div>
						<Display as="h1" size="xl">
							{title}
						</Display>
						<Lead className="mt-5 max-w-[56ch]">{lede}</Lead>
					</div>
					<label className="relative w-full lg:w-[320px]">
						<span className="sr-only">Search {active.label.toLowerCase()}</span>
						<MagnifyingGlass
							aria-hidden="true"
							className="pointer-events-none absolute top-1/2 left-3 size-4 -translate-y-1/2 text-[var(--ed-faint)]"
						/>
						<input
							className="ed-sans h-11 w-full border border-[var(--ed-line)] bg-[var(--ed-bg)] pr-3 pl-9 text-[14px] text-[var(--ed-ink)] placeholder:text-[var(--ed-faint)] focus:border-[var(--ed-ink)] focus:outline-none"
							onChange={(event) => setQuery(event.target.value)}
							placeholder={`Search ${active.label.toLowerCase()}`}
							type="search"
							value={query}
						/>
					</label>
				</div>

				<nav aria-label="Categories" className="mt-12">
					<ul className="flex flex-wrap gap-x-8 gap-y-2 border-b border-[var(--ed-line)]">
						{CATEGORIES.map((category) => (
							<li key={category.kind}>
								<Link
									aria-current={category.kind === kind ? "page" : undefined}
									className={cn(
										"ed-sans -mb-px flex items-baseline gap-2 border-b py-3 text-[14px] transition-colors",
										category.kind === kind
											? "border-[var(--ed-ink)] text-[var(--ed-ink)]"
											: "border-transparent text-[var(--ed-muted)] hover:text-[var(--ed-ink)]",
									)}
									href={category.href}
								>
									<span className="ed-mono text-[11px] text-[var(--ed-fig)]">
										{category.numeral}
									</span>
									{category.label}
									<span className="ed-mono text-[11px] text-[var(--ed-faint)]">
										{counts[category.kind]}
									</span>
								</Link>
							</li>
						))}
					</ul>
				</nav>

				{visible.length === 0 ? (
					<p className="ed-sans border-b border-[var(--ed-line)] py-10 text-[14px] text-[var(--ed-muted)]">
						Nothing matches “{query}”.
					</p>
				) : (
					<ol>
						{visible.map((item) => {
							const number = items.indexOf(item) + 1;
							const Logo =
								item.kind === "integration"
									? INTEGRATION_LOGOS[item.slug as keyof typeof INTEGRATION_LOGOS]
									: undefined;
							return (
								<li className="border-b border-[var(--ed-line)]" key={item.href}>
									<Link
										className="group grid gap-4 py-7 md:grid-cols-[64px_1fr_minmax(0,300px)] md:items-center md:gap-8"
										href={item.href}
									>
										<span className="ed-mono text-[12px] text-[var(--ed-fig)]">
											{String(number).padStart(2, "0")}
										</span>
										<span className="flex items-start gap-4">
											{Logo ? (
												<span className="inline-flex size-11 shrink-0 items-center justify-center border border-white/15 bg-[#111111] p-2">
													<Logo aria-hidden="true" className="size-full" />
												</span>
											) : null}
											<span>
												<span className="ed-display block text-[22px] leading-[1.25] text-[var(--ed-ink)] sm:text-[26px]">
													{item.title}
												</span>
												<span className="ed-sans mt-2 block max-w-[60ch] text-[14px] leading-[1.55] text-[var(--ed-muted)]">
													{item.body}
												</span>
											</span>
										</span>
										<span className="flex items-center justify-between gap-4 md:justify-end">
											{item.spec ? (
												<span className="text-left md:text-right">
													<span className="ed-fig-label block text-[#757575]">{item.spec[0]}</span>
													<span className="ed-sans mt-1 block text-[13px] text-[#d9d9d9]">
														{item.spec[1]}
													</span>
												</span>
											) : null}
											<ArrowRight
												aria-hidden="true"
												className="size-4 shrink-0 text-[var(--ed-muted)] transition-transform group-hover:translate-x-1 group-hover:text-[var(--ed-ink)]"
											/>
										</span>
									</Link>
								</li>
							);
						})}
					</ol>
				)}
			</Container>
		</section>
	);
}
