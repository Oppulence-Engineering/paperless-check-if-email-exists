import { Mark, PageHero, RowTable } from "@/components/platform/landing/blocks";
import { CtaLink } from "@/components/platform/landing/cta-link";
import {
	Check,
	MONO,
	MUTED,
	Section,
	SectionIntro,
} from "@/components/platform/landing/primitives";
import { cn } from "@/lib/utils";

import { pricingPlans } from "../marketing-data";
import {
	PRICING_COMPARISON_SECTIONS,
	type PricingColumn,
} from "../sim-landing/subpages/pricing-comparison-data";
import { FaqSection, StepCards } from "./page-kit";

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

const ROWS = PRICING_COMPARISON_SECTIONS.flatMap((section) => section.rows);

/** Limits such as checks and seats: rows whose values are text. */
const ALLOWANCES = ROWS.filter((row) => row.values.every((value) => typeof value === "string"));

/** Included features: rows whose values are yes or no. */
const FEATURES = ROWS.filter((row) => row.values.every((value) => typeof value === "boolean"));

/** Features a plan has that the plan before it does not. The first plan lists all of its features. */
function featuresFor(column: PricingColumn): string[] {
	return FEATURES.filter(
		(row) => row.values[column] === true && (column === 0 || row.values[column - 1] !== true),
	).map((row) => row.label);
}

const FACTS = [
	"No email sent during checks",
	"Duplicates in a list count once",
	"Prices in US dollars",
];

/** Plan columns after screenshot-studio `PlanCards`: price, limits, and what each plan adds. */
export function PlanCards() {
	return (
		<div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
			{pricingPlans.map((plan, index) => {
				const column = index as PricingColumn;
				const previous = index === 0 ? null : pricingPlans[index - 1];
				const features = featuresFor(column);
				return (
					<div
						className={cn(
							"flex flex-col p-6",
							plan.recommended ? "bg-[#181818] ring-1 ring-white/15" : "bg-[#111]",
						)}
						key={plan.name}
					>
						<div className="flex items-center justify-between">
							<h3 className="text-2xl tracking-[-0.015em] text-white">{plan.name}</h3>
							{plan.recommended ? (
								<span
									className={cn(
										MONO,
										"rounded-full bg-white px-2 py-0.5 text-[10px] tracking-[0.06em] text-black uppercase",
									)}
								>
									Most teams
								</span>
							) : null}
						</div>
						<p className={cn("mt-1 min-h-[56px] text-[17px] leading-snug", MUTED)}>
							{plan.description}
						</p>
						<p className="mt-8 text-[30px] leading-none tracking-[-0.03em] text-white">
							{plan.price}
							{plan.period ? (
								<span className={cn("ml-1 text-[15px] tracking-normal", MUTED)}>{plan.period}</span>
							) : null}
						</p>
						<p className={cn("mt-2 text-[15px]", MUTED)}>{plan.note}</p>
						<CtaLink
							className="mt-6 justify-center"
							href={plan.ctaHref}
							label={plan.ctaLabel}
							tone="secondary"
						/>
						<dl className="mt-8 space-y-2.5 border-t border-[#212121] pt-6 text-[15px]">
							{ALLOWANCES.map((row) => (
								<div className="flex items-baseline justify-between gap-4" key={row.label}>
									<dt className={MUTED}>{row.label}</dt>
									<dd className={cn(MONO, "text-[13px] text-white")}>{row.values[column]}</dd>
								</div>
							))}
						</dl>
						<div className="mt-6 flex-1 border-t border-[#212121] pt-6">
							{previous ? (
								<p className={cn("mb-3 text-[13px]", MUTED)}>
									Everything in {previous.name}
									{features.length > 0 ? ", plus" : null}
								</p>
							) : null}
							<ul className="space-y-3">
								{features.map((feature) => (
									<li className="flex items-start gap-3 text-[15px] text-white/85" key={feature}>
										<Check className="mt-0.5" />
										{feature}
									</li>
								))}
							</ul>
						</div>
					</div>
				);
			})}
		</div>
	);
}

/** Every limit and feature side by side. */
function PlanComparison() {
	return (
		<RowTable
			caption="Plan comparison"
			head={["", ...pricingPlans.map((plan) => plan.name)]}
			rows={ROWS.map((row) => ({
				label: row.label,
				cells: row.values.map((value, index) =>
					typeof value === "string" ? (
						<span className={cn(MONO, "text-[13px] text-white")} key={index}>
							{value}
						</span>
					) : (
						<Mark key={index} value={value} />
					),
				),
			}))}
		/>
	);
}

export function EditorialPricing() {
	return (
		<>
			<PageHero
				body="Every plan includes scores, reason codes, suppressions, and delivery feedback. No email is sent during a check."
				eyebrow={{ label: "Pricing" }}
				lead="Start free."
				trail="Grow when you send more."
			>
				<PlanCards />
				<ul className="mt-4 grid grid-cols-1 gap-px overflow-hidden border border-[#212121] bg-[#212121] sm:grid-cols-3">
					{FACTS.map((fact) => (
						<li className="bg-[#090909] px-5 py-5 text-[15px] text-white/80" key={fact}>
							{fact}
						</li>
					))}
				</ul>
			</PageHero>
			<Section>
				<SectionIntro label="Compare" lead="Compare plans" />
				<div className="mt-24">
					<PlanComparison />
				</div>
			</Section>
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
			<FaqSection faqs={pricingFaqs} />
		</>
	);
}
