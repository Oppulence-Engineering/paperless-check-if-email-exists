import Link from "next/link";

import { LegalSectionContent } from "@/components/legal/legal-blocks";
import { legalSectionSlug } from "@/components/legal/legal-slug";
import type { LegalRelatedLink, LegalSection } from "@/components/legal/types";

import { Container, Display, Eyebrow } from "./ui";

/** Legal documents: header, sticky contents, and a reading column. */
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
		<article className="bg-[var(--ed-bg)] pt-16 pb-20 sm:pt-20">
			<Container>
				<header className="max-w-[760px] pb-12">
					<Eyebrow>{eyebrow.replace(/[[\]]/g, "")}</Eyebrow>
					<Display as="h1" className="mt-3" id="legal-heading" size="lg">
						{title}
					</Display>
					<dl className="ed-sans mt-5 flex flex-wrap gap-x-6 gap-y-1 text-[13px] text-[var(--ed-muted)]">
						<div className="flex gap-1.5">
							<dt>Effective</dt>
							<dd className="text-[var(--ed-ink)]">{effective}</dd>
						</div>
						<div className="flex gap-1.5">
							<dt>Last updated</dt>
							<dd className="text-[var(--ed-ink)]">{lastUpdated}</dd>
						</div>
						<div className="flex gap-1.5">
							<dt>Contact</dt>
							<dd>
								<a
									className="text-[var(--ed-accent)] hover:underline"
									href={`mailto:${contactEmail}`}
								>
									{contactEmail}
								</a>
							</dd>
						</div>
					</dl>
					<p className="ed-sans mt-6 max-w-[60ch] text-[17px] leading-[1.6] font-[450] text-[var(--ed-body)]">
						{intro}
					</p>
				</header>

				<div className="grid grid-cols-1 gap-10 border-t border-[var(--ed-line)] pt-12 lg:grid-cols-[220px_1fr] lg:gap-14">
					<nav aria-label="Contents" className="lg:sticky lg:top-24 lg:self-start">
						<Eyebrow className="mb-3">On this page</Eyebrow>
						<ol className="flex flex-col gap-1">
							{sections.map((section) => (
								<li key={section.heading}>
									<Link
										className="ed-sans block rounded-lg px-2 py-1.5 text-[13.5px] leading-[1.4] text-[var(--ed-body)] transition-colors hover:bg-[var(--ed-subtle)] hover:text-[var(--ed-ink)]"
										href={`#${legalSectionSlug(section.heading)}`}
									>
										{section.heading}
									</Link>
								</li>
							))}
						</ol>
					</nav>
					<div className="ed-legal flex min-w-0 max-w-[720px] flex-col gap-12">
						{sections.map((section) => (
							<LegalSectionContent key={section.heading} section={section} />
						))}
					</div>
				</div>

				{related.length > 0 ? (
					<div className="mt-16 flex flex-wrap gap-2 border-t border-[var(--ed-line)] pt-8">
						{related.map((link) => (
							<Link
								className="ed-sans rounded-full border border-[var(--ed-line)] px-3.5 py-1.5 text-[13px] text-[var(--ed-body)] transition-colors hover:bg-[var(--ed-subtle)]"
								href={link.href}
								key={link.href}
							>
								{link.label}
							</Link>
						))}
					</div>
				) : null}
			</Container>
		</article>
	);
}
