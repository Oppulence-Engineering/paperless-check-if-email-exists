import Link from "next/link";

import { Chip } from "@sim/emcn";
import { Globe, Layout, Mic } from "@sim/emcn/icons";

import { cn } from "@/lib/sim/cn";

import { JsonLd, RelatedPages } from "../../marketing-primitives";
import { faqJsonLd } from "../../metadata";
import { SimCtaLink } from "../primitives";
import { LANDING_CONTENT_WIDTH, LANDING_GUTTER } from "../tokens";
import { ProductHeroPreview } from "./solutions-product/product-hero-preview";
import { SimSubpageFaqSection } from "./sim-subpage-faq";
import { SIM_SURFACE_CARD } from "./sim-surface-card";

const faqs = [
	{
		question: "What do I need to run the app?",
		answer:
			"For local development, install Rust, Node.js, pnpm, and Docker, then run make dev. Production uses the combined container plus PostgreSQL and RabbitMQ.",
	},
	{
		question: "Is there a desktop installer?",
		answer:
			"No. This repository provides a web application, Rust API, worker, and CLI. The old desktop installer links are not part of this product.",
	},
	{
		question: "Can I use only the API?",
		answer: "Yes. The Rust backend has a standalone mode and generated SDKs.",
	},
] as const;

const specs = [
	{ term: "Web", detail: "Next.js app with Better Auth and organization workspaces." },
	{ term: "API", detail: "Rust backend for single checks, lists, jobs, and history." },
	{ term: "Worker", detail: "Processes queued verification jobs." },
	{ term: "Database", detail: "PostgreSQL for application and verification data." },
	{ term: "Queue", detail: "RabbitMQ for background jobs." },
	{ term: "Container", detail: "One image contains the web app, API, and worker." },
] as const;

const SURFACES = [
	{
		id: "web",
		eyebrow: "[web]",
		icon: Globe,
		title: "Web workspace",
		description: "Check an address, upload a CSV, and review results in a browser.",
		learnHref: "/web",
		learnLabel: "Explore the web workspace",
		preview: "web" as const,
	},
	{
		id: "api",
		eyebrow: "[api]",
		icon: Layout,
		title: "Rust API",
		description: "Use the backend API and generated SDKs in your own workflow.",
		learnHref: "/features/governed-actions",
		learnLabel: "Explore the API",
		preview: "desktop" as const,
	},
	{
		id: "bulk",
		eyebrow: "[bulk]",
		icon: Mic,
		title: "Bulk lists",
		description: "Run CSV verification jobs and inspect per-address results.",
		learnHref: "/voice-app",
		learnLabel: "Explore bulk lists",
		preview: "voice" as const,
	},
] as const;

/** Sim download hub — hero preview, three surfaces, specs, FAQ. */
export function SimDownloadPage() {
	return (
		<>
			<JsonLd data={faqJsonLd([...faqs])} />
			<main className="bg-[var(--bg)] pb-16 max-sm:pb-12" id="main-content">
				<section
					aria-labelledby="download-heading"
					className={cn(
						"grid grid-cols-1 gap-10 border-[var(--border)] border-b pb-12 max-sm:gap-8 max-sm:pb-10 lg:grid-cols-2 lg:items-center lg:gap-12",
						LANDING_CONTENT_WIDTH,
						LANDING_GUTTER,
					)}
					id="download"
				>
					<div className="flex flex-col gap-5">
						<p className="text-[12px] text-[var(--text-muted)] uppercase tracking-[0.08em]">
							[download]
						</p>
						<h1
							className="max-w-[14ch] text-balance text-[48px] text-[var(--text-primary)] leading-[1.05] tracking-[-0.025em] max-sm:text-[32px] max-xl:text-[40px]"
							id="download-heading"
						>
							Run the application your way.
						</h1>
						<p className="max-w-[46ch] text-pretty text-[var(--text-body)] text-lg leading-[1.5] max-sm:text-base">
							Start the full stack locally, use the Rust API, or build the combined container for
							your infrastructure.
						</p>
						<div className="flex flex-wrap gap-2">
							{["Web app", "Rust API", "Self-hosted"].map((chip) => (
								<Chip key={chip} variant="outline">
									{chip}
								</Chip>
							))}
						</div>
						<div className="flex flex-wrap gap-2 pt-1">
							<SimCtaLink href="#web">Open web workspace</SimCtaLink>
							<SimCtaLink href="/sign-up" variant="outline">
								Use the web app
							</SimCtaLink>
						</div>
					</div>
					<div className={cn(SIM_SURFACE_CARD, "overflow-hidden")}>
						<ProductHeroPreview product="desktop" />
					</div>
				</section>

				<div className={cn("mt-12 flex flex-col gap-4", LANDING_CONTENT_WIDTH, LANDING_GUTTER)}>
					{SURFACES.map((surface) => (
						<article
							className={cn(SIM_SURFACE_CARD, "overflow-hidden")}
							id={surface.id}
							key={surface.id}
						>
							<div className="grid grid-cols-1 lg:grid-cols-[minmax(0,1.05fr)_minmax(0,1fr)]">
								<div className="border-[var(--border)] border-b p-6 max-lg:border-b lg:border-r lg:border-b-0">
									<div className="mb-4 flex items-center gap-2">
										<surface.icon className="size-[14px] text-[var(--text-icon)]" />
										<p className="text-[12px] text-[var(--text-muted)] uppercase tracking-[0.08em]">
											{surface.eyebrow}
										</p>
									</div>
									<h2 className="text-[24px] text-[var(--text-primary)] leading-[1.15] tracking-[-0.02em]">
										{surface.title}
									</h2>
									<p className="mt-3 max-w-[42ch] text-[15px] text-[var(--text-secondary)] leading-[1.55]">
										{surface.description}
									</p>
									<Link
										className="mt-5 inline-flex text-[14px] text-[var(--text-body)] underline-offset-4 hover:underline"
										href={surface.learnHref}
									>
										{surface.learnLabel}
									</Link>
								</div>
								<div className="flex flex-col">
									<div className="relative h-[280px] overflow-hidden border-[var(--border)] border-b bg-[var(--bg)] max-lg:h-[240px]">
										<ProductHeroPreview compact product={surface.preview} />
									</div>
									<div className="p-6">
										<div className="flex flex-col gap-4">
											<p className="text-[14px] text-[var(--text-secondary)] leading-[1.5]">
												Use the web app or build the open-source stack from this repository.
											</p>
											<SimCtaLink href="/sign-up" withArrow>
												Open the workspace
											</SimCtaLink>
										</div>
									</div>
								</div>
							</div>
						</article>
					))}
				</div>

				<section
					aria-labelledby="download-specs-heading"
					className={cn("mt-12", LANDING_CONTENT_WIDTH, LANDING_GUTTER)}
				>
					<div className={cn(SIM_SURFACE_CARD, "p-6 lg:p-8")}>
						<h2
							className="text-[24px] text-[var(--text-primary)] leading-[1.1] tracking-[-0.02em] lg:text-[28px]"
							id="download-specs-heading"
						>
							What the repository includes
						</h2>
						<p className="mt-2 max-w-[52ch] text-[15px] text-[var(--text-secondary)] leading-[1.55]">
							The source contains the web app, Rust backend, worker, and deployment files.
						</p>
						<dl className="mt-8 grid gap-5 sm:grid-cols-2 lg:grid-cols-3">
							{specs.map((row) => (
								<div className="flex flex-col gap-1.5" key={row.term}>
									<dt className="text-[12px] text-[var(--text-muted)] uppercase tracking-[0.06em]">
										{row.term}
									</dt>
									<dd className="text-[15px] text-[var(--text-body)] leading-[1.55]">
										{row.detail}
									</dd>
								</div>
							))}
						</dl>
					</div>
				</section>

				<div className={cn("mt-12", LANDING_CONTENT_WIDTH, LANDING_GUTTER)}>
					<SimSubpageFaqSection
						faqs={[...faqs]}
						heading="Before you run the stack"
						lede="Review the repository setup guide and infrastructure requirements."
					/>
				</div>

				<div className={cn("mt-10", LANDING_CONTENT_WIDTH, LANDING_GUTTER)}>
					<RelatedPages
						items={[
							{
								label: "Installation guide",
								href: "/guides/install-oppulence",
								description: "Local and container setup.",
							},
							{
								label: "Web vs API",
								href: "/guides/desktop-vs-web",
								description: "Which surface for which job.",
							},
							{
								label: "Pricing",
								href: "/pricing",
								description: "Ways to run the source code.",
							},
							{
								label: "Security",
								href: "/security",
								description: "How workspace and API access work.",
							},
						]}
					/>
				</div>
			</main>
		</>
	);
}
