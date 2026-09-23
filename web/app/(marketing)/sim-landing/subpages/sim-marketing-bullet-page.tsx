import Image from "next/image";

import { cn } from "@/lib/sim/cn";

import type { MarketingPage } from "../../marketing-data";
import { JsonLd } from "../../marketing-primitives";
import { breadcrumbJsonLd } from "../../metadata";
import { SimCtaLink } from "../primitives";
import {
	SimBorderedColumn,
	SimPageDivider,
	SimPageFooterRule,
	SimSubpageHero,
} from "./sim-subpage-hero";
import { LANDING_CONTENT_WIDTH, LANDING_GUTTER } from "../tokens";
import { SIM_SURFACE_CARD } from "./sim-surface-card";

/** Sim fallback for legacy marketing archive pages with bullet lists. */
export function SimMarketingBulletPage({
	page,
	screenshot,
	screenshotAlt,
}: {
	page: MarketingPage;
	screenshot?: string;
	screenshotAlt?: string;
}) {
	return (
		<>
			<JsonLd
				data={breadcrumbJsonLd([
					{ name: "Home", path: "/" },
					{ name: page.eyebrow, path: `/${page.path}` },
				])}
			/>
			<div className="bg-[var(--bg)] pb-16 max-sm:pb-12">
				<div className={cn(LANDING_CONTENT_WIDTH, LANDING_GUTTER)}>
					<SimSubpageHero
						description={page.description}
						eyebrow={`[${page.eyebrow.toLowerCase()}]`}
						title={page.title}
						titleId="marketing-bullet-heading"
					>
						<div className="pt-4">
							<SimCtaLink href={page.ctaHref ?? "/sign-up"}>
								{page.ctaLabel ?? "Start for free"}
							</SimCtaLink>
						</div>
					</SimSubpageHero>
				</div>

				<SimPageDivider />

				<SimBorderedColumn>
					{screenshot ? (
						<>
							<section className="px-6 py-10">
								<div className={cn(SIM_SURFACE_CARD, "overflow-hidden")}>
									<Image
										alt={screenshotAlt ?? page.title}
										className="h-auto w-full"
										height={900}
										src={screenshot}
										width={1400}
									/>
								</div>
							</section>
							<div className="h-px w-full bg-[var(--border)]" />
						</>
					) : null}

					<section className="px-6 py-10">
						<div className="grid gap-4 md:grid-cols-3">
							{page.bullets.map((bullet, index) => (
								<article className={cn(SIM_SURFACE_CARD, "flex flex-col gap-2 p-5")} key={bullet}>
									<p className="text-[12px] text-[var(--text-muted)] tabular-nums">
										{String(index + 1).padStart(2, "0")}
									</p>
									<p className="text-[15px] text-[var(--text-body)] leading-[1.55]">{bullet}</p>
								</article>
							))}
						</div>
					</section>
				</SimBorderedColumn>

				<SimPageFooterRule />
			</div>
		</>
	);
}

/** Sim blog archive body — static editorial paragraphs from the legacy template. */
export function SimBlogArchivePage({ page }: { page: MarketingPage }) {
	return (
		<>
			<JsonLd
				data={breadcrumbJsonLd([
					{ name: "Home", path: "/" },
					{ name: "Blog", path: "/blog" },
					{ name: page.title, path: `/${page.path}` },
				])}
			/>
			<div className="bg-[var(--bg)] pb-16 max-sm:pb-12">
				<div className={cn(LANDING_CONTENT_WIDTH, LANDING_GUTTER)}>
					<SimSubpageHero
						description={page.description}
						eyebrow="[blog archive]"
						title={page.title}
						titleId="blog-archive-heading"
					/>
				</div>

				<SimPageDivider />

				<SimBorderedColumn>
					<div className="px-6 py-10">
						<div className="max-w-none text-[15px] text-[var(--text-body)] leading-[1.75]">
							<p className="my-4">
								This legacy article URL remains available for existing links. The current product
								focuses on checking email addresses before they are used.
							</p>
							<p className="my-4">
								A check reviews address syntax, DNS, MX, and mailbox signals without sending a
								message. You can check one address or upload a CSV list.
							</p>
							<p className="my-4">
								See the current guides for result limits and deployment details.
							</p>
						</div>
					</div>
				</SimBorderedColumn>

				<SimPageFooterRule />
			</div>
		</>
	);
}
