import Link from "next/link";
import { type ComponentType, Fragment, type ReactNode, type SVGProps } from "react";

import {
	ArrowLink,
	HeroStage,
	PageHero,
	RowTable,
	TileGrid,
} from "@/components/platform/landing/blocks";
import {
	Check,
	MONO,
	MUTED,
	Section,
	SectionIntro,
	Stage,
} from "@/components/platform/landing/primitives";
import { cn } from "@/lib/utils";

import type { CapabilityPage } from "../catalog";
import { JsonLd } from "../marketing-primitives";
import { breadcrumbJsonLd } from "../metadata";
import {
	type Compare,
	PAGE_COMPARE,
	PAGE_FACTS,
	PAGE_SAMPLES,
	type Reference,
	type Sample,
	type SamplePane,
} from "./capability-content";
import {
	ApiTerminal,
	CatchAllFigure,
	CheckPipeline,
	CsvFigure,
	EventTimelineFigure,
	FeedbackFlow,
	FinderFigure,
	GateFigure,
	ListSortDemo,
	ProviderFigure,
	QualityFigure,
	RecheckDialFigure,
	ScoreScaleFigure,
	SignupFigure,
	SourceCompareFigure,
	StepFlowFigure,
	SuppressionDemo,
	WorkspaceFigure,
} from "./graphics";
import {
	AmazonSesLogo,
	ApiLogo,
	CsvLogo,
	GoLogo,
	HubSpotLogo,
	INTEGRATION_LOGOS,
	MailgunLogo,
	PostmarkLogo,
	SalesforceLogo,
	SendGridLogo,
	TypeScriptLogo,
} from "./logos";
import { FaqSection, LinkCards } from "./page-kit";
import { Sheet } from "./ui";

const KIND_PARENT: Record<CapabilityPage["kind"], { name: string; path: string }> = {
	feature: { name: "Features", path: "/features" },
	"use-case": { name: "Use cases", path: "/use-cases" },
	integration: { name: "Integrations", path: "/integrations" },
	guide: { name: "Guides", path: "/guides" },
};

/** A small drawing in a product window. */
function Plate({ title, children }: { title: string; children: ReactNode }) {
	return (
		<Sheet bodyClassName="mx-auto w-full max-w-[520px] px-3 pt-2 pb-4" title={title}>
			{children}
		</Sheet>
	);
}

/** The figure under each page title. */
const HERO_FIGURES: Record<string, () => ReactNode> = {
	"/features/single-checks": () => <CheckPipeline />,
	"/features/find-email": () => <FinderFigure />,
	"/features/list-cleaning": () => (
		<Plate title="One file, four actions">
			<ListSortDemo />
		</Plate>
	),
	"/features/risk-scoring": () => (
		<Plate title="Scores on one scale">
			<ScoreScaleFigure />
		</Plate>
	),
	"/features/delivery-feedback": () => (
		<Plate title="A day of delivery events">
			<EventTimelineFigure />
		</Plate>
	),
	"/features/suppressions": () => (
		<Plate title="Three sources, one list">
			<SuppressionDemo />
		</Plate>
	),
	"/features/reverification": () => (
		<Plate title="Re-check schedule">
			<RecheckDialFigure />
		</Plate>
	),
	"/features/api": () => <ApiTerminal />,
	"/features/workspace": () => <WorkspaceFigure />,
	"/use-cases/email-marketing": () => <QualityFigure />,
	"/use-cases/marketing-operations": () => <SourceCompareFigure />,
	"/use-cases/signup-forms": () => <SignupFigure />,
	"/use-cases/sending-platforms": () => <GateFigure />,
	"/integrations/sendgrid": () => <ProviderFigure logo={<SendGridLogo />} provider="sendgrid" />,
	"/integrations/mailgun": () => <ProviderFigure logo={<MailgunLogo />} provider="mailgun" />,
	"/integrations/postmark": () => <ProviderFigure logo={<PostmarkLogo />} provider="postmark" />,
	"/integrations/amazon-ses": () => (
		<ProviderFigure logo={<AmazonSesLogo />} provider="amazon-ses" />
	),
	"/integrations/csv": () => <CsvFigure />,
	"/integrations/api": () => <ApiTerminal />,
	"/guides/reading-results": () => (
		<Plate title="Scores on one scale">
			<ScoreScaleFigure />
		</Plate>
	),
	"/guides/catch-all-domains": () => <CatchAllFigure />,
	"/guides/list-hygiene": () => (
		<Plate title="Re-check schedule">
			<RecheckDialFigure />
		</Plate>
	),
};
type Logo = ComponentType<SVGProps<SVGSVGElement>>;

const SDKS = {
	label: "SDKs",
	items: [
		["Go", GoLogo],
		["TypeScript", TypeScriptLogo],
	] as [string, Logo][],
};

/** Tools named on a page, shown with their logos under the hero buttons. */
const HERO_NOTES: Record<string, { label: string; items: [string, Logo][] }> = {
	"/features/api": SDKS,
	"/integrations/api": SDKS,
	"/integrations/csv": {
		label: "Works with exports from",
		items: [
			["HubSpot", HubSpotLogo],
			["Salesforce", SalesforceLogo],
		],
	},
};
/** Split hero, then the page figure on a stage, the tools strip, and four facts. */
function Hero({ page }: { page: CapabilityPage }) {
	const parent = KIND_PARENT[page.kind];
	const Figure = HERO_FIGURES[page.path];
	const note = HERO_NOTES[page.path];
	const specs = PAGE_FACTS[page.path]?.specs;
	const next = page.related.find((item) => item.href !== "/sign-up");
	return (
		<PageHero
			body={page.description}
			eyebrow={{ label: page.eyebrow, parent: parent.name, href: parent.path }}
			lead={page.title}
			secondary={next ? { href: next.href, label: next.label } : null}
		>
			{Figure ? (
				<HeroStage>
					<div className="w-full min-w-0 max-w-[760px]">{Figure()}</div>
				</HeroStage>
			) : null}
			{note ? (
				<div className="mt-10 flex flex-col gap-6 sm:flex-row sm:items-center sm:justify-between">
					<p className={cn("text-[15px]", MUTED)}>{note.label}</p>
					<ul className="flex flex-wrap items-center gap-x-7 gap-y-3">
						{note.items.map(([name, ItemLogo]) => (
							<li className="flex items-center gap-2 text-[15px] text-white/55" key={name}>
								<ItemLogo aria-hidden="true" className="size-[18px] grayscale" />
								{name}
							</li>
						))}
					</ul>
				</div>
			) : null}
			{specs ? (
				<dl className="mt-10 grid grid-cols-2 gap-px overflow-hidden border border-[#212121] bg-[#212121] lg:grid-cols-4">
					{specs.map(([label, value]) => (
						<div className="bg-[#090909] px-5 py-5" key={label}>
							<dt className={cn("text-[13px]", MUTED)}>{label}</dt>
							<dd className="mt-1 text-[15px] text-white/80">{value}</dd>
						</div>
					))}
				</dl>
			) : null}
		</PageHero>
	);
}

/** A reference table under a short title. */
function ReferenceTable({ reference }: { reference: Reference }) {
	return (
		<div className="mt-16">
			<p className={cn("text-[15px]", MUTED)}>{reference.title}</p>
			<div className="mt-4">
				<RowTable
					caption={reference.title}
					head={reference.columns}
					monoLabels
					rows={reference.rows.map(([label, ...cells]) => ({ label, cells }))}
				/>
			</div>
		</div>
	);
}

const WORKS_WITH: {
	slug: keyof typeof INTEGRATION_LOGOS;
	name: string;
	logo: Logo;
}[] = [
	{ slug: "sendgrid", name: "SendGrid", logo: SendGridLogo },
	{ slug: "mailgun", name: "Mailgun", logo: MailgunLogo },
	{ slug: "postmark", name: "Postmark", logo: PostmarkLogo },
	{ slug: "amazon-ses", name: "Amazon SES", logo: AmazonSesLogo },
	{ slug: "csv", name: "CSV files", logo: CsvLogo },
	{ slug: "api", name: "REST API", logo: ApiLogo },
];
/** Logo tiles that link to the other integration pages. */
function WorksWith({ current }: { current?: string }) {
	const items = WORKS_WITH.filter((item) => item.slug !== current);
	return (
		<Section>
			<SectionIntro
				label="Integrations"
				lead={current ? "Other integrations" : "Works with"}
				trail={current ? undefined : "the tools you send from."}
			>
				<div className="mt-10">
					<ArrowLink href="/integrations">All integrations</ArrowLink>
				</div>
			</SectionIntro>
			<ul className="mt-24 grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-6">
				{items.map(({ slug, name, logo: Mark }) => (
					<li key={slug}>
						<Link
							className="flex aspect-[4/3] flex-col justify-between bg-[#111] p-5 transition-colors hover:bg-[#161616]"
							href={`/integrations/${slug}`}
						>
							<Mark aria-hidden="true" className="size-6" />
							<span className="text-[17px] text-white">{name}</span>
						</Link>
					</li>
				))}
			</ul>
		</Section>
	);
}

/** Second-section figure: the setup flow, or the full delivery-feedback loop. */
function HowFigure({ page }: { page: CapabilityPage }) {
	if (page.path === "/features/delivery-feedback") return <FeedbackFlow />;
	return (
		<StepFlowFigure
			caption={
				page.kind === "guide"
					? "What to do"
					: ["sendgrid", "mailgun", "postmark", "amazon-ses"].includes(page.slug)
						? "One private URL per endpoint"
						: page.path === "/features/workspace"
							? "Each teammate signs in with an account"
							: "No email is sent during a check"
			}
			steps={page.workflow}
			title={page.kind === "integration" ? `Set up ${page.eyebrow}` : page.howItWorks.title}
		/>
	);
}

/** Without and with the product, as the landing before and after. */
function CompareBlock({ compare }: { compare: Compare }) {
	return (
		<div className="mt-24 grid gap-4 md:grid-cols-2">
			<div className="border border-[#212121] p-8 sm:p-12">
				<p className={cn("text-[15px]", MUTED)}>Without</p>
				<ul className="mt-8 space-y-4">
					{compare.without.map((item) => (
						<li
							className="flex items-center gap-4 text-lg text-white/55 line-through decoration-white/20"
							key={item}
						>
							<span aria-hidden="true" className="size-1.5 shrink-0 rounded-full bg-white/25" />
							{item}
						</li>
					))}
				</ul>
			</div>
			<div className="bg-[#111] p-8 sm:p-12">
				<p className={cn("text-[15px]", MUTED)}>With Verifier</p>
				<ul className="mt-8 space-y-4">
					{compare.with.map((item) => (
						<li className="flex items-center gap-4 text-lg text-white" key={item}>
							<Check />
							{item}
						</li>
					))}
				</ul>
			</div>
		</div>
	);
}

const SAMPLE_KEY = "text-[#fafafa]";
const SAMPLE_STRING = "text-[var(--ed-fig)]";
const SAMPLE_VALUE = "text-[#d9d9d9]";
const SAMPLE_DIM = "text-[#858585]";
const JSON_TOKEN = /("(?:[^"\\]|\\.)*")(\s*:)?|\b(?:true|false|null)\b|\b\d+(?:\.\d+)?\b/g;
const CODE_TOKEN = /("(?:[^"\\]|\\.)*")|(\/\/.*$)/g;
const CSV_ACTION = /^(safe|fixed|review|drop|valid|risky|invalid|unknown)$/;

/** Colors one line of a sample: JSON keys and strings, code strings and comments, CSV actions. */
function sampleLine(line: string, lang: SamplePane["lang"], first: boolean): [string, string][] {
	if (lang === "text") return [[line, SAMPLE_VALUE]];
	if (lang === "csv") {
		if (first) return [[line, SAMPLE_DIM]];
		return line
			.split(",")
			.flatMap((cell, index): [string, string][] => [
				...(index > 0 ? [[",", SAMPLE_DIM] as [string, string]] : []),
				[cell, CSV_ACTION.test(cell) ? SAMPLE_STRING : SAMPLE_VALUE],
			]);
	}
	const parts: [string, string][] = [];
	const base = lang === "json" ? SAMPLE_DIM : SAMPLE_VALUE;
	let last = 0;
	for (const match of line.matchAll(lang === "json" ? JSON_TOKEN : CODE_TOKEN)) {
		if (match.index > last) parts.push([line.slice(last, match.index), base]);
		if (lang === "code") {
			parts.push([match[0], match[1] ? SAMPLE_STRING : SAMPLE_DIM]);
		} else if (match[1]) {
			parts.push([match[1], match[2] ? SAMPLE_KEY : SAMPLE_STRING]);
			if (match[2]) parts.push([match[2], SAMPLE_DIM]);
		} else {
			parts.push([match[0], SAMPLE_VALUE]);
		}
		last = match.index + match[0].length;
	}
	if (last < line.length) parts.push([line.slice(last), base]);
	return parts;
}

/** Real data formats, one block under the other: what goes in and what comes back. */
function SampleFigure({ sample }: { sample: Sample }) {
	return (
		<Sheet caption={sample.caption} title={sample.title}>
			<div className="p-3 sm:p-4">
				{sample.panes.map((pane, index) => (
					<Fragment key={pane.label}>
						{index > 0 ? <div aria-hidden="true" className="ml-6 h-5 w-px bg-white/15" /> : null}
						<div className="overflow-hidden rounded-xl border border-white/[0.06] bg-[#090909]">
							<p
								className={cn(
									MONO,
									"truncate border-b border-white/[0.06] px-4 py-2.5 text-[12px] text-[#858585]",
								)}
							>
								{pane.label}
							</p>
							<pre className={cn(MONO, "overflow-x-auto p-4 text-[12.5px] leading-[1.7]")}>
								<code>
									{pane.code.split("\n").map((line, lineIndex) => (
										<span className="block min-h-[1.7em] whitespace-pre" key={lineIndex}>
											{sampleLine(line, pane.lang, lineIndex === 0).map(([text, color], part) => (
												<span className={color} key={part}>
													{text}
												</span>
											))}
										</span>
									))}
								</code>
							</pre>
						</div>
					</Fragment>
				))}
			</div>
		</Sheet>
	);
}

/** Feature, use-case, integration, and guide detail pages. */
export function EditorialCapabilityPage({ page }: { page: CapabilityPage }) {
	const parent = KIND_PARENT[page.kind];
	const reference = PAGE_FACTS[page.path]?.reference;
	const compare = PAGE_COMPARE[page.path];
	const sample = PAGE_SAMPLES[page.path];

	return (
		<>
			<JsonLd
				data={breadcrumbJsonLd([
					{ name: "Home", path: "/" },
					{ name: parent.name, path: parent.path },
					{ name: page.eyebrow, path: page.path },
				])}
			/>
			<Hero page={page} />
			<Section>
				<SectionIntro body={page.problem.body} label="Why it matters" lead={page.problem.title}>
					<p
						className={cn(
							"mt-6 max-w-[632px] text-xl leading-[1.4] tracking-[-0.01em] sm:text-2xl",
							MUTED,
						)}
					>
						{page.lede}
					</p>
				</SectionIntro>
				{compare ? <CompareBlock compare={compare} /> : null}
			</Section>
			<Section>
				<SectionIntro
					body={page.howItWorks.body}
					label="How it works"
					lead={page.howItWorks.title}
				/>
				{sample ? (
					<div className="mt-24 grid grid-cols-1 gap-10 lg:grid-cols-[minmax(0,280px)_minmax(0,1fr)]">
						<ol className="self-start border-t border-[#212121]">
							{page.workflow.map((step, stepIndex) => (
								<li className="flex gap-4 border-b border-[#212121] py-5" key={step}>
									<span className={cn(MONO, "pt-0.5 text-[12px] text-white")}>
										{String(stepIndex + 1).padStart(2, "0")}
									</span>
									<span className="text-base leading-[1.55] text-white/85">{step}</span>
								</li>
							))}
						</ol>
						<Stage className="items-center px-4 py-12 sm:px-12 sm:py-20" variant={3}>
							<div className="w-full min-w-0">
								<SampleFigure sample={sample} />
							</div>
						</Stage>
					</div>
				) : null}
				{!sample || page.path === "/features/delivery-feedback" ? (
					<Stage className="mt-24 px-4 py-12 sm:px-12 sm:py-20" variant={2}>
						<div className="w-full min-w-0 max-w-[920px]">
							<HowFigure page={page} />
						</div>
					</Stage>
				) : null}
			</Section>
			<Section>
				<SectionIntro
					label="Details"
					lead={page.kind === "guide" ? "What to know." : "What you get."}
				/>
				<div className="mt-24">
					<TileGrid
						columns={page.capabilities.length % 3 === 0 || page.capabilities.length > 4 ? 3 : 2}
						items={page.capabilities}
					/>
				</div>
				{reference ? <ReferenceTable reference={reference} /> : null}
			</Section>
			{page.examples.length > 0 ? (
				<Section>
					<SectionIntro label="In practice" lead="Typical uses." />
					<div className="mt-24">
						<TileGrid columns={2} items={page.examples} />
					</div>
				</Section>
			) : null}
			<WorksWith current={page.kind === "integration" ? page.slug : undefined} />
			<FaqSection faqs={page.faqs} />
			<LinkCards items={page.related} />
		</>
	);
}
