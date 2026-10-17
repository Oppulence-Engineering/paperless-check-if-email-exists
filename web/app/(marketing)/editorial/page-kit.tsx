import Link from "next/link";
import type { ReactNode } from "react";

import { ArrowRight, CaretDown, Check } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { JsonLd } from "../marketing-primitives";
import { faqJsonLd } from "../metadata";
import { ButtonLink, Container, Display, Eyebrow, Lead, Section } from "./ui";

type Action = { label: string; href: string };

/** Two-column page hero: copy on the left, a product view on the right. */
export function PageHero({
	eyebrow,
	chip,
	title,
	lede,
	primary = { label: "Start free", href: "/sign-up" },
	secondary = null,
	visual,
}: {
	eyebrow: string;
	chip?: string;
	title: string;
	lede: string;
	primary?: Action;
	secondary?: Action | null;
	visual: ReactNode;
}) {
	return (
		<section className="bg-[var(--ed-bg)] pt-14 pb-12 sm:pt-20 lg:pt-24 lg:pb-16">
			<Container>
				<div className="grid items-center gap-12 lg:grid-cols-[1fr_1.05fr]">
					<div>
						<p className="ed-fig-label flex flex-wrap items-center gap-2 text-[#757575]">
							<span>{eyebrow}</span>
							{chip ? (
								<>
									<span aria-hidden="true">/</span>
									<span className="text-[#d9d9d9]">{chip}</span>
								</>
							) : null}
						</p>
						<Display as="h1" className="mt-6 max-w-[15ch]" size="xl">
							{title}
						</Display>
						<Lead className="mt-6">{lede}</Lead>
						<div className="mt-8 flex flex-wrap gap-2.5">
							<ButtonLink arrow href={primary.href}>
								{primary.label}
							</ButtonLink>
							{secondary ? (
								<ButtonLink href={secondary.href} variant="secondary">
									{secondary.label}
								</ButtonLink>
							) : null}
						</div>
					</div>
					<div className="min-w-0">{visual}</div>
				</div>
			</Container>
		</section>
	);
}

/** Centered hero for index pages and pricing. */
export function CenteredHero({
	eyebrow,
	title,
	lede,
	sans = false,
	children,
}: {
	eyebrow?: string;
	title: ReactNode;
	lede?: string;
	sans?: boolean;
	children?: ReactNode;
}) {
	return (
		<section className="relative overflow-hidden bg-[var(--ed-bg)] pt-16 pb-12 sm:pt-24 sm:pb-14">
			<Container className="relative flex flex-col items-center text-center">
				{eyebrow ? <Eyebrow tone="accent">{eyebrow}</Eyebrow> : null}
				{sans ? (
					<h1 className="ed-sans mt-4 max-w-[18ch] text-[40px] leading-[1.05] font-medium tracking-[-0.035em] text-[var(--ed-ink)] sm:text-[56px]">
						{title}
					</h1>
				) : (
					<Display as="h1" className="mt-4 max-w-[18ch]" size="xl">
						{title}
					</Display>
				)}
				{lede ? <Lead className="mx-auto mt-5 max-w-[52ch]">{lede}</Lead> : null}
				{children ? <div className="mt-8 w-full">{children}</div> : null}
			</Container>
		</section>
	);
}

export function SectionTitle({
	eyebrow,
	title,
	lede,
	center = false,
	id,
}: {
	eyebrow?: string;
	title: string;
	lede?: string;
	center?: boolean;
	id?: string;
}) {
	return (
		<div className={cn("mb-10", center && "flex flex-col items-center text-center")}>
			{eyebrow ? <Eyebrow>{eyebrow}</Eyebrow> : null}
			<Display className={cn("mt-2 max-w-[20ch]", center && "mx-auto")} id={id} size="md">
				{title}
			</Display>
			{lede ? <Lead className={cn("mt-4", center && "mx-auto")}>{lede}</Lead> : null}
		</div>
	);
}

/** Accordion of questions with FAQ structured data. */
export function FaqSection({
	title = "Before you start.",
	faqs,
}: {
	title?: string;
	faqs: { question: string; answer: string }[];
}) {
	if (faqs.length === 0) return null;
	return (
		<Section className="py-12 sm:py-16 lg:py-20">
			<JsonLd data={faqJsonLd(faqs)} />
			<Container>
				<div className="grid gap-10 lg:grid-cols-[0.8fr_1.2fr]">
					<div>
						<Eyebrow>Questions</Eyebrow>
						<Display className="mt-2" size="md">
							{title}
						</Display>
					</div>
					<div className="border-t border-[var(--ed-line)]">
						{faqs.map((faq) => (
							<details className="group border-b border-[var(--ed-line)]" key={faq.question}>
								<summary className="ed-sans flex cursor-pointer list-none items-center justify-between gap-6 py-5 text-[15px] font-medium text-[var(--ed-ink)] [&::-webkit-details-marker]:hidden">
									{faq.question}
									<CaretDown
										aria-hidden="true"
										className="size-4 shrink-0 text-[var(--ed-muted)] transition-transform group-open:rotate-180"
									/>
								</summary>
								<p className="ed-sans max-w-[62ch] pb-5 text-[14px] leading-[1.6] text-[var(--ed-body)]">
									{faq.answer}
								</p>
							</details>
						))}
					</div>
				</div>
			</Container>
		</Section>
	);
}

/** Closing band for a page. */
export function CtaBand({
	title,
	lede,
	primary = { label: "Start free", href: "/sign-up" },
	secondary = null,
}: {
	title: string;
	lede?: string;
	primary?: Action;
	secondary?: Action | null;
}) {
	return (
		<section className="bg-[var(--ed-bg)] px-4 pb-16 sm:px-6 lg:px-8">
			<div className="mx-auto max-w-[1280px] rounded-3xl border border-[var(--ed-line)] bg-[var(--ed-bg)] px-6 py-14 text-center sm:py-20">
				<Display className="mx-auto max-w-[18ch]" size="md">
					{title}
				</Display>
				{lede ? <Lead className="mx-auto mt-4">{lede}</Lead> : null}
				<div className="mt-8 flex flex-wrap justify-center gap-2.5">
					<ButtonLink arrow href={primary.href} size="md">
						{primary.label}
					</ButtonLink>
					{secondary ? (
						<ButtonLink href={secondary.href} size="md" variant="secondary">
							{secondary.label}
						</ButtonLink>
					) : null}
				</div>
			</div>
		</section>
	);
}

/** Small check row used under pricing and heroes. */
export function CheckRow({ items }: { items: string[] }) {
	return (
		<ul className="ed-sans flex flex-wrap justify-center gap-x-6 gap-y-2 text-[13px] text-[var(--ed-body)]">
			{items.map((item) => (
				<li className="flex items-center gap-1.5" key={item}>
					<Check aria-hidden="true" className="size-3.5 text-[var(--ed-good)]" />
					{item}
				</li>
			))}
		</ul>
	);
}

/** Numbered points in columns, divided by hairlines. */
export function NumberedPoints({
	eyebrow,
	title,
	items,
}: {
	eyebrow?: string;
	title: string;
	items: { title: string; body: string }[];
}) {
	return (
		<Section className="py-12 sm:py-16 lg:py-20">
			<Container>
				<SectionTitle eyebrow={eyebrow} title={title} />
				<ol className="grid border-t border-[var(--ed-line)] md:grid-cols-3">
					{items.map((item, index) => (
						<li
							className={cn(
								"border-b border-[var(--ed-line)] py-6 md:border-b-0 md:px-6 md:py-8",
								index === 0 && "md:pl-0",
								index > 0 && "md:border-l",
							)}
							key={item.title}
						>
							<span className="ed-mono text-[11px] text-[var(--ed-fig)]">
								{String(index + 1).padStart(2, "0")}
							</span>
							<h3 className="ed-sans mt-4 text-[17px] text-[var(--ed-ink)]">{item.title}</h3>
							<p className="ed-sans mt-2 text-[14px] leading-[1.6] text-[var(--ed-muted)]">
								{item.body}
							</p>
						</li>
					))}
				</ol>
			</Container>
		</Section>
	);
}

/** Numbered steps on one line, joined by a hairline. */
export function StepCards({
	eyebrow = "How it works",
	title,
	steps,
}: {
	eyebrow?: string;
	title: string;
	steps: { title: string; body: string }[];
}) {
	return (
		<Section className="py-12 sm:py-16 lg:py-20">
			<Container>
				<SectionTitle eyebrow={eyebrow} title={title} />
				<ol
					className={cn(
						"relative grid gap-8",
						steps.length >= 4 ? "md:grid-cols-4" : "md:grid-cols-3",
					)}
				>
					<span
						aria-hidden="true"
						className="absolute top-[7px] right-0 left-0 hidden h-px bg-[var(--ed-line)] md:block"
					/>
					{steps.map((step, index) => (
						<li className="relative" key={step.title}>
							<span
								aria-hidden="true"
								className="relative block size-[15px] rounded-full border border-[var(--ed-fig)] bg-[var(--ed-bg)]"
							/>
							<span className="ed-mono mt-5 block text-[11px] text-[var(--ed-fig)]">
								Step {String(index + 1).padStart(2, "0")}
							</span>
							<h3 className="ed-sans mt-2 text-[17px] text-[var(--ed-ink)]">{step.title}</h3>
							{step.body ? (
								<p className="ed-sans mt-2 text-[14px] leading-[1.6] text-[var(--ed-muted)]">
									{step.body}
								</p>
							) : null}
						</li>
					))}
				</ol>
			</Container>
		</Section>
	);
}

/** Two columns of numbered facts, divided by hairlines. */
export function ChecklistCard({
	eyebrow,
	title,
	items,
}: {
	eyebrow: string;
	title: string;
	items: { title: string; detail?: string }[];
}) {
	return (
		<Section className="py-12 sm:py-16">
			<Container>
				<SectionTitle eyebrow={eyebrow} title={title} />
				<ol className="grid border-t border-[var(--ed-line)] md:grid-cols-2 md:gap-x-12">
					{items.map((item, index) => (
						<li
							className="grid grid-cols-[40px_1fr] gap-3 border-b border-[var(--ed-line)] py-4"
							key={item.title}
						>
							<span className="ed-mono pt-0.5 text-[11px] text-[var(--ed-fig)]">
								{String(index + 1).padStart(2, "0")}
							</span>
							<span className="ed-sans">
								<span className="block text-[14.5px] text-[var(--ed-ink)]">{item.title}</span>
								{item.detail ? (
									<span className="mt-1 block text-[13.5px] leading-[1.55] text-[var(--ed-muted)]">
										{item.detail}
									</span>
								) : null}
							</span>
						</li>
					))}
				</ol>
			</Container>
		</Section>
	);
}

/** Links to read next, laid out as contents rows. */
export function LinkCards({
	eyebrow = "Continue reading",
	title,
	items,
}: {
	eyebrow?: string;
	title?: string;
	items: { label: string; href: string; description?: string }[];
}) {
	if (items.length === 0) return null;
	return (
		<Section className="py-12 sm:py-16">
			<Container>
				{title ? (
					<SectionTitle eyebrow={eyebrow} title={title} />
				) : (
					<Eyebrow className="mb-4">{eyebrow}</Eyebrow>
				)}
				<ol className="border-t border-[var(--ed-line)]">
					{items.map((item, index) => (
						<li className="border-b border-[var(--ed-line)]" key={item.href + item.label}>
							<Link
								className="group grid grid-cols-[40px_1fr_auto] items-center gap-4 py-5"
								href={item.href}
							>
								<span className="ed-mono text-[11px] text-[var(--ed-fig)]">
									{String(index + 1).padStart(2, "0")}
								</span>
								<span className="flex min-w-0 items-center gap-4">
									<span className="ed-sans shrink-0 text-[16px] text-[var(--ed-ink)]">
										{item.label}
									</span>
									{item.description ? (
										<span className="ed-sans hidden truncate text-[13.5px] text-[var(--ed-muted)] md:block">
											{item.description}
										</span>
									) : null}
									<span
										aria-hidden="true"
										className="hidden h-px flex-1 border-t border-dotted border-[var(--ed-line)] sm:block"
									/>
								</span>
								<ArrowRight
									aria-hidden="true"
									className="size-4 text-[var(--ed-muted)] transition-transform group-hover:translate-x-1 group-hover:text-[var(--ed-ink)]"
								/>
							</Link>
						</li>
					))}
				</ol>
			</Container>
		</Section>
	);
}
