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
		question: "Is there a published hosted price?",
		answer:
			"No hosted subscription price is configured in this repository. The source code is available under the repository license.",
	},
	{
		question: "What does self-hosting require?",
		answer:
			"The combined application image uses external PostgreSQL and RabbitMQ. You pay for and manage your own infrastructure.",
	},
	{
		question: "Can I use the API?",
		answer: "Yes. The Rust backend exposes an API and this repository includes generated SDKs.",
	},
] as const;

const TRUST_CHIPS = ["Open source", "Web and API", "Self-hosted option"] as const;

const UPGRADE_STEPS = [
	{
		title: "Start in the web workspace",
		body: "Check one address and review its signals.",
	},
	{
		title: "Use bulk lists for CSV data",
		body: "Review per-address results in a verification job.",
	},
	{
		title: "Deploy the API in your environment",
		body: "Build the container and connect PostgreSQL and RabbitMQ.",
	},
] as const;

const SECTIONS_BY_COLUMN = [
	pricingSectionsForColumn(0),
	pricingSectionsForColumn(1),
	pricingSectionsForColumn(2),
] as const;

/** Sim `/pricing` board — three self-contained spec sheets with Oppulence tiers. */
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
							Ways to run email verification
						</h1>
						<p className="sr-only">
							The source includes a web workspace, bulk jobs, and a self-hosted API.
						</p>
						<p className="max-w-[52ch] text-pretty text-[var(--text-body)] text-base leading-[1.55]">
							This repository does not configure hosted billing. Choose the workflow you need and
							review the repository license and infrastructure requirements before deployment.
						</p>
						<div className="flex flex-wrap items-center justify-center gap-2 pt-1">
							{TRUST_CHIPS.map((chip) => (
								<Chip key={chip} variant="outline">
									{chip}
								</Chip>
							))}
						</div>
					</div>

					<div className="grid grid-cols-1 gap-4 lg:grid-cols-3">
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
								priceSubtext="See repository license"
								sections={SECTIONS_BY_COLUMN[index]}
							/>
						))}
					</div>

					<div className="mt-2 border-[var(--border)] border-t pt-10">
						<div className="grid gap-8 lg:grid-cols-[minmax(0,1fr)_minmax(0,1.1fr)] lg:items-start">
							<div className="flex flex-col gap-4">
								<h2 className="max-w-[16ch] text-balance text-[28px] text-[var(--text-primary)] leading-[1.08] tracking-[-0.02em] max-sm:text-[24px] lg:text-[32px]">
									Start with the workflow you need.
								</h2>
								<p className="max-w-[46ch] text-[15px] text-[var(--text-secondary)] leading-[1.55]">
									Run the app locally or deploy the combined image. See{" "}
									<Link className="underline-offset-4 hover:underline" href="/download">
										download
									</Link>{" "}
									and{" "}
									<Link
										className="underline-offset-4 hover:underline"
										href="/guides/desktop-vs-web"
									>
										which surface to use
									</Link>
									.
								</p>
								<SimCtaLink href="/sign-up" variant="outline" withArrow>
									Start checking
								</SimCtaLink>
							</div>
							<ol className="flex flex-col gap-3">
								{UPGRADE_STEPS.map((step, index) => (
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
						heading="How access works"
						lede="Hosted subscription pricing is not configured in this repository."
					/>
				</section>
			</main>
		</>
	);
}
