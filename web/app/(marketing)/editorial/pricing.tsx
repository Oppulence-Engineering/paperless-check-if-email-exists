import { Check } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { pricingPlans } from "../marketing-data";
import {
	PRICING_COMPARISON_SECTIONS,
	type PricingColumn,
} from "../sim-landing/subpages/pricing-comparison-data";
import { CenteredHero, CheckRow, CtaBand, FaqSection, StepCards } from "./page-kit";
import { ButtonLink, Container, Muted } from "./ui";

export const pricingFaqs = [
	{
		question: "What counts as a check?",
		answer:
			"Each address you check counts as one check, in the app or through the API. A list with 1,000 addresses uses 1,000 checks. Re-checks count too.",
	},
	{
		question: "What happens when I run out of checks?",
		answer:
			"New checks stop until your checks reset at the start of the next period. Contact us to move to a larger plan.",
	},
	{
		question: "Do unused checks roll over?",
		answer: "No. Your checks reset at the start of each period.",
	},
	{
		question: "Which plans include the API?",
		answer: "Professional and Enterprise.",
	},
	{
		question: "Do you offer volume pricing?",
		answer: "Yes. Contact us about the Enterprise plan.",
	},
];

const CARD_TONE = ["muted", "highlight", "plain", "dark"] as const;
const CARD_LABEL = ["To try it", "Most teams", "At volume", "For large senders"];

/** Rows that apply to one plan: usage values and included features. */
function planRows(column: PricingColumn) {
	const rows: string[] = [];
	for (const section of PRICING_COMPARISON_SECTIONS) {
		for (const row of section.rows) {
			const value = row.values[column];
			if (value === true) rows.push(row.label);
			else if (typeof value === "string" && value) rows.push(`${value} ${row.label.toLowerCase()}`);
		}
	}
	return rows;
}

function PlanCard({ index }: { index: number }) {
	const plan = pricingPlans[index];
	const tone = CARD_TONE[index] ?? "plain";
	const dark = tone === "dark";
	return (
		<article
			className={cn(
				"relative flex flex-col rounded-3xl border p-6",
				tone === "muted" && "border-[var(--ed-line)] bg-[var(--ed-subtle)]",
				tone === "highlight" &&
					"border-[var(--ed-accent)] bg-[var(--ed-bg)] shadow-[var(--ed-shadow)]",
				tone === "plain" && "border-[var(--ed-line)] bg-[var(--ed-bg)]",
				dark && "border-[var(--ed-line)] bg-[var(--ed-subtle)] text-white",
			)}
		>
			<div className="flex items-center justify-between">
				<p className={cn("ed-sans text-[12px]", dark ? "text-white/60" : "text-[var(--ed-muted)]")}>
					{CARD_LABEL[index]}
				</p>
				{plan.recommended ? (
					<span className="ed-sans rounded-full border border-[var(--ed-accent)]/30 bg-[var(--ed-accent-soft)] px-2.5 py-0.5 text-[11px] text-[var(--ed-accent)]">
						Recommended
					</span>
				) : null}
			</div>
			<h2
				className={cn(
					"ed-sans mt-1 text-[18px] font-medium",
					dark ? "text-white" : "text-[var(--ed-ink)]",
				)}
			>
				{plan.name}
			</h2>
			<p className="mt-8 flex items-baseline gap-1.5">
				<span
					className={cn(
						"ed-display text-[44px] leading-none font-[400]",
						dark ? "text-white" : "text-[var(--ed-ink)]",
					)}
				>
					{plan.price}
				</span>
				{plan.period ? (
					<span
						className={cn("ed-sans text-[13px]", dark ? "text-white/60" : "text-[var(--ed-muted)]")}
					>
						/ month
					</span>
				) : null}
			</p>
			<p
				className={cn("ed-sans mt-2 text-[13px]", dark ? "text-white/70" : "text-[var(--ed-good)]")}
			>
				{plan.note}
			</p>
			<p
				className={cn(
					"ed-sans mt-3 min-h-[40px] text-[13.5px] leading-[1.5]",
					dark ? "text-white/70" : "text-[var(--ed-body)]",
				)}
			>
				{plan.description}
			</p>
			<ButtonLink
				arrow
				className="mt-6 w-full"
				href={plan.ctaHref}
				size="md"
				variant={dark ? "inverse" : tone === "highlight" ? "accent" : "secondary"}
			>
				{plan.ctaLabel}
			</ButtonLink>
			<ul
				className={cn(
					"mt-6 flex flex-col gap-2.5 border-t pt-6",
					dark ? "border-white/10" : "border-[var(--ed-line)]",
				)}
			>
				{planRows(index as PricingColumn).map((row) => (
					<li
						className={cn(
							"ed-sans flex gap-2.5 text-[13px] leading-[1.45]",
							dark ? "text-white/85" : "text-[var(--ed-body)]",
						)}
						key={row}
					>
						<span
							className={cn(
								"mt-0.5 inline-flex size-4 shrink-0 items-center justify-center rounded-full",
								dark
									? "bg-white/15 text-white"
									: "bg-[var(--ed-accent-soft)] text-[var(--ed-accent)]",
							)}
						>
							<Check aria-hidden="true" className="size-2.5" />
						</span>
						{row}
					</li>
				))}
			</ul>
		</article>
	);
}

export function EditorialPricing() {
	return (
		<>
			<CenteredHero
				eyebrow="Pricing"
				lede="Every plan includes scores, reason codes, suppressions, and delivery feedback. No email is sent during a check."
				sans
				title={
					<>
						Start free. <Muted>Grow when you send more.</Muted>
					</>
				}
			/>
			<section className="bg-[var(--ed-bg)] pb-10">
				<Container wide>
					<div className="grid gap-3 md:grid-cols-2 xl:grid-cols-4">
						{pricingPlans.map((plan, index) => (
							<PlanCard index={index} key={plan.name} />
						))}
					</div>
					<div className="mt-3 rounded-2xl border border-[var(--ed-line)] px-5 py-4">
						<CheckRow
							items={[
								"No email sent during checks",
								"Duplicates in a list count once",
								"Prices in US dollars",
								"Contact us for paid plans",
							]}
						/>
					</div>
				</Container>
			</section>
			<StepCards
				eyebrow="How checks are counted"
				steps={[
					{
						title: "One address, one check",
						body: "Each address you check uses one check from your monthly allowance.",
					},
					{
						title: "Lists count per address",
						body: "A CSV with 1,000 addresses uses 1,000 checks. Duplicates in the same file count once.",
					},
					{
						title: "Re-checks count too",
						body: "Scheduled re-checks use checks from the same allowance.",
					},
				]}
				title="You pay for addresses, not files."
			/>
			<FaqSection faqs={pricingFaqs} title="Questions about pricing." />
			<CtaBand lede="Start with 100 free checks a month." title="Clean your next list for free." />
		</>
	);
}
