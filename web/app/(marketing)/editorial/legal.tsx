import Link from "next/link";

import { LegalSectionContent } from "@/components/legal/legal-blocks";
import { legalSectionSlug } from "@/components/legal/legal-slug";
import type { LegalRelatedLink, LegalSection } from "@/components/legal/types";
import { ArrowLink } from "@/components/platform/landing/blocks";
import { CONTAINER, MONO, MUTED } from "@/components/platform/landing/primitives";
import { cn } from "@/lib/utils";

/** Legal documents: header, sticky contents, and a reading column, in the landing style. */
export function EditorialLegalDocument({
	eyebrow,
	title,
	effective,
	lastUpdated,
	intro,
	sections,
	contactEmail,
	related,
}: {
	eyebrow: string;
	title: string;
	effective: string;
	lastUpdated: string;
	intro: string;
	sections: LegalSection[];
	contactEmail: string;
	related: LegalRelatedLink[];
}) {
	return (
		<article className={cn(CONTAINER, "pt-16 pb-24 sm:pt-24 sm:pb-32")}>
			<header className="max-w-[760px]">
				<p className={cn(MONO, "text-[12px] tracking-[0.08em] text-white uppercase")}>
					{eyebrow.replace(/[[\]]/g, "")}
				</p>
				<h1
					className="mt-8 text-[40px] leading-[1] tracking-[-0.035em] text-balance text-white sm:text-[56px] lg:text-[64px]"
					id="legal-heading"
				>
					{title}
				</h1>
				<dl className={cn("mt-8 flex flex-wrap gap-x-6 gap-y-1 text-[15px]", MUTED)}>
					<div className="flex gap-1.5">
						<dt>Effective</dt>
						<dd className="text-white">{effective}</dd>
					</div>
					<div className="flex gap-1.5">
						<dt>Last updated</dt>
						<dd className="text-white">{lastUpdated}</dd>
					</div>
					<div className="flex gap-1.5">
						<dt>Contact</dt>
						<dd>
							<a
								className="text-white underline-offset-4 hover:underline"
								href={`mailto:${contactEmail}`}
							>
								{contactEmail}
							</a>
						</dd>
					</div>
				</dl>
				<p className="mt-8 max-w-[60ch] text-lg leading-[1.5] text-white/65">{intro}</p>
			</header>

			<div className="mt-24 grid grid-cols-1 gap-10 border-t border-[#212121] pt-12 lg:grid-cols-[220px_1fr] lg:gap-14">
				<nav aria-label="Contents" className="lg:sticky lg:top-24 lg:self-start">
					<p className={cn(MONO, "mb-3 text-[12px] tracking-[0.08em] uppercase", MUTED)}>
						On this page
					</p>
					<ol className="flex flex-col gap-1">
						{sections.map((section) => (
							<li key={section.heading}>
								<Link
									className="block rounded-lg px-2 py-1.5 text-[15px] leading-[1.4] text-[#858585] transition-colors hover:bg-white/[0.05] hover:text-white"
									href={`#${legalSectionSlug(section.heading)}`}
								>
									{section.heading}
								</Link>
							</li>
						))}
					</ol>
				</nav>
				<div className="ed-legal flex max-w-[720px] min-w-0 flex-col gap-12">
					{sections.map((section) => (
						<LegalSectionContent key={section.heading} section={section} />
					))}
				</div>
			</div>

			{related.length > 0 ? (
				<div className="mt-16 flex flex-wrap gap-x-8 gap-y-3 border-t border-[#212121] pt-8">
					{related.map((link) => (
						<ArrowLink href={link.href} key={link.href}>
							{link.label}
						</ArrowLink>
					))}
				</div>
			) : null}
		</article>
	);
}
