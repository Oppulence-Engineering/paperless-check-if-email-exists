import {
	FaqBlock,
	LinkCards as LinkCardGrid,
	TileGrid,
} from "@/components/platform/landing/blocks";
import {
	Check,
	MONO,
	MUTED,
	Section,
	SectionIntro,
} from "@/components/platform/landing/primitives";
import { cn } from "@/lib/utils";

import { JsonLd } from "../marketing-primitives";
import { faqJsonLd } from "../metadata";

/*
 * Page sections in the screenshot-studio landing style: a label on the left,
 * the heading on the right, and the content 96px below.
 */

/** Questions and answers with FAQ structured data. */
export function FaqSection({ faqs }: { faqs: { question: string; answer: string }[] }) {
	if (faqs.length === 0) return null;
	return (
		<>
			<JsonLd data={faqJsonLd(faqs)} />
			<FaqBlock items={faqs.map((faq) => ({ q: faq.question, a: faq.answer }))} />
		</>
	);
}

/** Numbered points as flat tiles. */
export function NumberedPoints({
	eyebrow,
	title,
	items,
}: {
	eyebrow: string;
	title: string;
	items: { title: string; body: string }[];
}) {
	return (
		<Section>
			<SectionIntro label={eyebrow} lead={title} />
			<div className="mt-24">
				<TileGrid
					items={items.map((item, index) => ({
						...item,
						snippet: String(index + 1).padStart(2, "0"),
					}))}
				/>
			</div>
		</Section>
	);
}

/** Numbered steps as hairline rows. */
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
		<Section>
			<SectionIntro label={eyebrow} lead={title} />
			<ol className="mt-24 border-t border-[#212121]">
				{steps.map((step, index) => (
					<li
						className="grid gap-2 border-b border-[#212121] py-6 sm:grid-cols-[140px_1fr_1.4fr] sm:items-baseline sm:gap-8"
						key={step.title}
					>
						<span className={cn(MONO, "text-[12px] tracking-[0.06em] text-white uppercase")}>
							Step {String(index + 1).padStart(2, "0")}
						</span>
						<span className="text-lg text-white">{step.title}</span>
						<span className={cn("text-base", MUTED)}>{step.body}</span>
					</li>
				))}
			</ol>
		</Section>
	);
}

/** What is included, as a ticked list in two columns. */
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
		<Section>
			<SectionIntro label={eyebrow} lead={title} />
			<ul className="mt-24 grid gap-x-8 gap-y-4 sm:grid-cols-2">
				{items.map((item) => (
					<li className="flex items-start gap-3 text-[15px] text-white/85" key={item.title}>
						<Check className="mt-0.5" />
						<span>
							{item.title}
							{item.detail ? (
								<span className={cn("mt-1 block leading-[1.55]", MUTED)}>{item.detail}</span>
							) : null}
						</span>
					</li>
				))}
			</ul>
		</Section>
	);
}

/** Links to read next, as flat cards. */
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
		<Section>
			{title ? (
				<SectionIntro label={eyebrow} lead={title} />
			) : (
				<p className="text-[26px] tracking-[-0.02em] text-white sm:text-[30px]">{eyebrow}</p>
			)}
			<div className={title ? "mt-24" : "mt-12"}>
				<LinkCardGrid
					items={items.map((item) => ({
						href: item.href,
						label: item.label,
						blurb: item.description,
					}))}
				/>
			</div>
		</Section>
	);
}
