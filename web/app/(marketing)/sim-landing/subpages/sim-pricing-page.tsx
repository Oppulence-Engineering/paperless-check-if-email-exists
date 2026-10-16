import Link from "next/link";

import { Chip } from "@sim/emcn";

import { cn } from "@/lib/sim/cn";

import { pricingPlans } from "../../marketing-data";
import { JsonLd } from "../../marketing-primitives";
import { faqJsonLd } from "../../metadata";
import { SimCtaLink } from "../primitives";
import { LANDING_CONTENT_WIDTH, LANDING_GUTTER } from "../tokens";
import { pricingSectionsForColumn } from "./pricing-comparison-data";
import { SimPricingCard } from "./sim-pricing-card";
import { SimSubpageFaqSection } from "./sim-subpage-faq";
import { SIM_SURFACE_CARD } from "./sim-surface-card";

const faqs = [
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
		question: "Can I run it on my own servers?",
		answer: "Yes. The source code is public. See the self-hosting page for what you need.",
	},
] as const;

const TRUST_CHIPS = [
	"No email sent during checks",
	"Free plan available",
	"Every plan includes delivery feedback",
] as const;

const COUNT_STEPS = [
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
] as const;

const SECTIONS_BY_COLUMN = [
	pricingSectionsForColumn(0),
	pricingSectionsForColumn(1),
	pricingSectionsForColumn(2),
	pricingSectionsForColumn(3),
] as const;

/** Sim `/pricing` board — one self-contained spec sheet for each plan. */
export function SimPricingPage() {
	return (
		<>
			<JsonLd data={faqJsonLd([...faqs])} />
			<main id="main-content">
				<section
					aria-labelledby="pricing-heading"
					className={cn(
						"flex w-full flex-col gap-7 pb-16 max-sm:pb-12",
						LANDING_CONTENT_WIDTH,
						LANDING_GUTTER,
					)}
					id="pricing"
				>
					<div className="flex flex-col items-center gap-4 pt-2 text-center">
						<p className="text-[12px] text-[var(--text-muted)] uppercase tracking-[0.08em]">
							[pricing]
						</p>
						<h1
							className="max-w-[24ch] text-balance text-[30px] text-[var(--text-primary)] leading-[1.05] tracking-[-0.02em] max-sm:text-[26px] lg:text-[36px]"
							id="pricing-heading"
						>
							Simple pricing for clean lists
						</h1>
						<p className="sr-only">
							Free, Starter at $49 a month, Professional at $149 a month, and custom Enterprise
							plans.
						</p>
						<p className="max-w-[52ch] text-pretty text-[var(--text-body)] text-base leading-[1.55]">
							Start free. Move up when you send more. Every plan includes scores, reason codes,
							suppressions, and delivery feedback.
						</p>
						<div className="flex flex-wrap items-center justify-center gap-2 pt-1">
							{TRUST_CHIPS.map((chip) => (
								<Chip key={chip} variant="outline">
									{chip}
								</Chip>
							))}
						</div>
					</div>

					<div className="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-4">
						{pricingPlans.map((plan, index) => (
							<SimPricingCard
								badge={plan.recommended ? "Recommended" : undefined}
								cta={{
									href: plan.ctaHref,
									label: plan.ctaLabel,
									variant: plan.recommended ? "primary" : index === 0 ? "border-shadow" : "primary",
								}}
								description={plan.description}
								key={plan.name}
								name={plan.name}
								price={plan.period ? `${plan.price}${plan.period}` : plan.price}
								priceSubtext={plan.note}
								sections={SECTIONS_BY_COLUMN[index]}
							/>
						))}
					</div>

					<div className="mt-2 border-[var(--border)] border-t pt-10">
						<div className="grid gap-8 lg:grid-cols-[minmax(0,1fr)_minmax(0,1.1fr)] lg:items-start">
							<div className="flex flex-col gap-4">
								<h2 className="max-w-[16ch] text-balance text-[28px] text-[var(--text-primary)] leading-[1.08] tracking-[-0.02em] max-sm:text-[24px] lg:text-[32px]">
									How checks are counted.
								</h2>
								<p className="max-w-[46ch] text-[15px] text-[var(--text-secondary)] leading-[1.55]">
									Every plan counts addresses, not files or jobs. See{" "}
									<Link
										className="underline-offset-4 hover:underline"
										href="/guides/reading-results"
									>
										how to read results
									</Link>{" "}
									before your first list.
								</p>
								<SimCtaLink href="/sign-up" variant="outline" withArrow>
									Start free
								</SimCtaLink>
							</div>
							<ol className="flex flex-col gap-3">
								{COUNT_STEPS.map((step, index) => (
									<li className={cn(SIM_SURFACE_CARD, "flex gap-4 p-5")} key={step.title}>
										<span
											aria-hidden="true"
											className="flex size-8 shrink-0 items-center justify-center rounded-full border border-[var(--border)] font-mono text-[13px] text-[var(--text-muted)] tabular-nums"
										>
											{index + 1}
										</span>
										<div>
											<p className="text-[15px] text-[var(--text-primary)] leading-[1.35]">
												{step.title}
											</p>
											<p className="mt-1 text-[14px] text-[var(--text-secondary)] leading-[1.5]">
												{step.body}
											</p>
										</div>
									</li>
								))}
							</ol>
						</div>
					</div>

					<SimSubpageFaqSection
						faqs={[...faqs]}
						heading="Questions about pricing"
						lede="Prices are in US dollars, per month."
					/>
				</section>
			</main>
		</>
	);
}
