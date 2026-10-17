import Link from "next/link";
import { type ComponentType, Fragment, type ReactNode, type SVGProps } from "react";

import { ArrowRight } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { allCapabilityPages, type CapabilityPage } from "../catalog";
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
	VerifierMark,
} from "./logos";
import { FaqSection, LinkCards } from "./page-kit";
import { ButtonLink, Container, Display, Eyebrow, Lead, Muted, Sheet } from "./ui";

const KIND_PARENT: Record<CapabilityPage["kind"], { name: string; path: string }> = {
	feature: { name: "Features", path: "/features" },
	"use-case": { name: "Use cases", path: "/use-cases" },
	integration: { name: "Integrations", path: "/integrations" },
	guide: { name: "Guides", path: "/guides" },
};

/** A small drawing framed as a numbered plate. */
function Plate({ title, children }: { title: string; children: ReactNode }) {
	return (
		<Sheet bodyClassName="mx-auto w-full max-w-[520px] px-3 pt-2 pb-4" fig="1" title={title}>
			{children}
		</Sheet>
	);
}

/** The figure beside each page title. */
const HERO_FIGURES: Record<string, () => ReactNode> = {
	"/features/single-checks": () => <CheckPipeline fig="1" />,
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
	"/features/api": () => <ApiTerminal fig="1" />,
	"/features/workspace": () => <WorkspaceFigure />,
	"/use-cases/email-marketing": () => <QualityFigure fig="1" />,
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
	"/integrations/api": () => <ApiTerminal fig="1" />,
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

/** Integration pages open with the two logos joined by a line. */
function LogoLockup({ logo: Mark, name }: { logo: Logo; name: string }) {
	return (
		<div aria-label={`${name} and Verifier`} className="mb-8 flex items-center gap-3" role="img">
			<span className="inline-flex size-12 items-center justify-center border border-white/15 bg-[#111111] p-2.5">
				<Mark aria-hidden="true" className="size-full" />
			</span>
			<span aria-hidden="true" className="relative h-px w-16 bg-white/25">
				<span className="ed-travel absolute top-1/2 size-[5px] -translate-x-1/2 -translate-y-1/2 rounded-full bg-[var(--ed-fig)]" />
			</span>
			<span className="inline-flex size-12 items-center justify-center border border-white/15 bg-[#111111] p-2.5">
				<VerifierMark aria-hidden="true" className="size-full" />
			</span>
		</div>
	);
}

function Hero({ page, index, total }: { page: CapabilityPage; index: number; total: number }) {
	const parent = KIND_PARENT[page.kind];
	const Figure = HERO_FIGURES[page.path];
	const integrationLogo =
		page.kind === "integration"
			? INTEGRATION_LOGOS[page.slug as keyof typeof INTEGRATION_LOGOS]
			: undefined;
	const next = page.related.find((item) => item.href !== "/sign-up");
	return (
		<section className="bg-[var(--ed-bg)] pt-12 pb-14 sm:pt-16 lg:pt-20">
			<Container>
				<nav
					aria-label="Breadcrumb"
					className="ed-fig-label flex items-center gap-2 text-[#757575]"
				>
					<Link className="hover:text-[var(--ed-ink)]" href={parent.path}>
						{parent.name}
					</Link>
					<span aria-hidden="true">/</span>
					<span className="text-[var(--ed-fig)]">
						{String(index).padStart(2, "0")} of {String(total).padStart(2, "0")}
					</span>
					<span aria-hidden="true">/</span>
					<span className="text-[#d9d9d9]">{page.eyebrow}</span>
				</nav>
				<div className="mt-10 grid grid-cols-1 items-start gap-12 lg:grid-cols-[minmax(0,1fr)_minmax(0,1.1fr)] lg:gap-16">
					<div className="lg:pt-6">
						{integrationLogo ? <LogoLockup logo={integrationLogo} name={page.eyebrow} /> : null}
						<Display as="h1" className="max-w-[16ch]" size="xl">
							{page.title}
						</Display>
						<Lead className="mt-6 max-w-[52ch]">{page.description}</Lead>
						<div className="mt-8 flex flex-wrap items-center gap-x-6 gap-y-3">
							<ButtonLink arrow href="/sign-up" size="md">
								Get started free
							</ButtonLink>
							{next ? (
								<Link
									className="ed-sans inline-flex items-center gap-1.5 text-[14px] text-[var(--ed-muted)] transition-colors hover:text-[var(--ed-ink)]"
									href={next.href}
								>
									{next.label} <ArrowRight aria-hidden="true" className="size-3.5" />
								</Link>
							) : null}
						</div>
						{HERO_NOTES[page.path] ? (
							<div className="mt-10 flex flex-wrap items-center gap-x-6 gap-y-3 border-t border-[var(--ed-line)] pt-5">
								<span className="ed-fig-label text-[#757575]">{HERO_NOTES[page.path].label}</span>
								{HERO_NOTES[page.path].items.map(([name, ItemLogo]) => (
									<span
										className="ed-sans flex items-center gap-2 text-[14px] text-[var(--ed-ink)]"
										key={name}
									>
										<ItemLogo aria-hidden="true" className="size-5" />
										{name}
									</span>
								))}
							</div>
						) : null}
					</div>
					{Figure ? <div>{Figure()}</div> : null}
				</div>
			</Container>
		</section>
	);
}

function SpecStrip({ specs }: { specs: [string, string][] }) {
	return (
		<section className="bg-[var(--ed-bg)]">
			<Container>
				<dl className="grid grid-cols-2 border-y border-[var(--ed-line)] lg:grid-cols-4">
					{specs.map(([label, value], index) => (
						<div
							className={cn(
								"py-5 pr-4",
								index % 2 === 1 && "border-l border-[var(--ed-line)] pl-4",
								index >= 2 && "border-t border-[var(--ed-line)] lg:border-t-0",
								index === 2 && "lg:border-l lg:pl-4",
							)}
							key={label}
						>
							<dt className="ed-fig-label text-[#757575]">{label}</dt>
							<dd className="ed-sans mt-2 text-[15px] leading-[1.4] text-[var(--ed-ink)]">
								{value}
							</dd>
						</div>
					))}
				</dl>
			</Container>
		</section>
	);
}

/** A numbered chapter: the number and label on the left, the content on the right. */
function Chapter({
	n,
	label,
	title,
	children,
}: {
	n: number;
	label: string;
	title: string;
	children: ReactNode;
}) {
	return (
		<section className="bg-[var(--ed-bg)] py-14 sm:py-20">
			<Container>
				<div className="grid grid-cols-1 gap-8 border-t border-[var(--ed-line)] pt-8 lg:grid-cols-[220px_minmax(0,1fr)] lg:gap-12">
					<div className="lg:sticky lg:top-24 lg:self-start">
						<p className="ed-mono text-[11px] text-[var(--ed-fig)]">§ {n}</p>
						<Eyebrow className="mt-2">{label}</Eyebrow>
					</div>
					<div>
						<Display className="max-w-[24ch]" size="md">
							{title}
						</Display>
						{children}
					</div>
				</div>
			</Container>
		</section>
	);
}

function ReferenceTable({ reference, n }: { reference: Reference; n: number }) {
	return (
		<div className="mt-10">
			<p className="ed-fig-label text-[#757575]">
				Table {n} · {reference.title}
			</p>
			<div className="mt-3 overflow-x-auto border border-[var(--ed-line)]">
				<table className="w-full min-w-[520px] border-collapse text-left">
					<thead>
						<tr className="border-b border-[var(--ed-line)]">
							{reference.columns.map((column) => (
								<th className="ed-fig-label px-4 py-3 font-normal text-[#757575]" key={column}>
									{column}
								</th>
							))}
						</tr>
					</thead>
					<tbody>
						{reference.rows.map((row) => (
							<tr
								className="border-b border-dashed border-[var(--ed-line)] last:border-0"
								key={row.join("|")}
							>
								{row.map((cell, index) => (
									<td
										className={cn(
											"px-4 py-3 align-top text-[13.5px] leading-[1.5]",
											index === 0
												? "ed-mono text-[12.5px] whitespace-nowrap text-[var(--ed-fig)]"
												: "ed-sans text-[var(--ed-body)]",
										)}
										key={`${index}-${cell}`}
									>
										{cell}
									</td>
								))}
							</tr>
						))}
					</tbody>
				</table>
			</div>
		</div>
	);
}

/** Numbered definitions: number, title, and text in rows with hairlines. */
function Definitions({ n, items }: { n: number; items: { title: string; body: string }[] }) {
	return (
		<ol className="mt-10 border-t border-[var(--ed-line)]">
			{items.map((item, index) => (
				<li
					className="grid gap-2 border-b border-[var(--ed-line)] py-5 sm:grid-cols-[56px_220px_1fr] sm:gap-6"
					key={item.title}
				>
					<span className="ed-mono text-[11px] text-[var(--ed-fig)]">
						{n}.{index + 1}
					</span>
					<span className="ed-sans text-[15px] text-[var(--ed-ink)]">{item.title}</span>
					<span className="ed-sans text-[14px] leading-[1.6] text-[var(--ed-muted)]">
						{item.body}
					</span>
				</li>
			))}
		</ol>
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

function WorksWith({ current }: { current?: string }) {
	const items = WORKS_WITH.filter((item) => item.slug !== current);
	return (
		<section className="bg-[var(--ed-bg)] py-14 sm:py-20">
			<Container>
				<div className="flex flex-wrap items-end justify-between gap-4">
					<Display size="sm">
						{current ? (
							<>
								Other <Muted>integrations.</Muted>
							</>
						) : (
							<>
								Works with <Muted>the tools you send from.</Muted>
							</>
						)}
					</Display>
					<Link
						className="ed-sans inline-flex items-center gap-1.5 text-[14px] text-[var(--ed-muted)] hover:text-[var(--ed-ink)]"
						href="/integrations"
					>
						All integrations <ArrowRight aria-hidden="true" className="size-3.5" />
					</Link>
				</div>
				<ul
					className={cn(
						"mt-8 grid grid-cols-2 border-t border-l border-[var(--ed-line)] sm:grid-cols-3",
						items.length === 5 ? "lg:grid-cols-5" : "lg:grid-cols-6",
					)}
				>
					{items.map(({ slug, name, logo: Mark }) => (
						<li className="border-r border-b border-[var(--ed-line)]" key={slug}>
							<Link
								className="group flex h-full flex-col gap-6 p-5 transition-colors hover:bg-[#111111]"
								href={`/integrations/${slug}`}
							>
								<Mark aria-hidden="true" className="size-8" />
								<span className="ed-sans flex items-center justify-between text-[14px] text-[var(--ed-ink)]">
									{name}
									<ArrowRight
										aria-hidden="true"
										className="size-3.5 text-[var(--ed-muted)] transition-transform group-hover:translate-x-0.5"
									/>
								</span>
							</Link>
						</li>
					))}
				</ul>
			</Container>
		</section>
	);
}

/** Second-chapter figure: the setup flow, or the full delivery-feedback loop. */
function HowFigure({ page, fig }: { page: CapabilityPage; fig: string }) {
	if (page.path === "/features/delivery-feedback") return <FeedbackFlow fig={fig} />;
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
			fig={fig}
			steps={page.workflow}
			title={page.kind === "integration" ? `Set up ${page.eyebrow}` : page.howItWorks.title}
		/>
	);
}

/** Without and with the product, one matched row at a time. */
function CompareFigure({ compare, fig }: { compare: Compare; fig: string }) {
	return (
		<Sheet fig={fig} title="Without and with Verifier">
			<div className="px-4 pt-4 pb-2 sm:px-5">
				<div className="ed-fig-label hidden grid-cols-[1fr_40px_1fr] pb-3 text-[#757575] sm:grid">
					<span>Without</span>
					<span />
					<span className="text-[var(--ed-fig)]">With Verifier</span>
				</div>
				<ol>
					{compare.without.map((without, index) => (
						<li
							className="grid gap-2 border-t border-white/10 py-4 sm:grid-cols-[1fr_40px_1fr] sm:items-start sm:gap-0"
							key={without}
						>
							<span className="ed-sans flex gap-3 text-[14px] leading-[1.55] text-[#8a8a8a]">
								<span aria-hidden="true" className="ed-mono text-[#5a5a5a]">
									×
								</span>
								<span>
									<span className="sr-only">Without: </span>
									{without}
								</span>
							</span>
							<span
								aria-hidden="true"
								className="ed-mono hidden text-center text-[13px] text-[#5a5a5a] sm:block"
							>
								→
							</span>
							<span className="ed-sans flex gap-3 text-[14px] leading-[1.55] text-[var(--ed-ink)]">
								<span aria-hidden="true" className="ed-mono text-[var(--ed-fig)]">
									✓
								</span>
								<span>
									<span className="sr-only">With Verifier: </span>
									{compare.with[index]}
								</span>
							</span>
						</li>
					))}
				</ol>
			</div>
		</Sheet>
	);
}

const SAMPLE_KEY = "text-[#fafafa]";
const SAMPLE_STRING = "text-[var(--ed-fig)]";
const SAMPLE_VALUE = "text-[#d9d9d9]";
const SAMPLE_DIM = "text-[#6b6b6b]";
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
function SampleFigure({ sample, fig }: { sample: Sample; fig: string }) {
	return (
		<Sheet caption={sample.caption} fig={fig} title={sample.title}>
			<div className="p-3 sm:p-4">
				{sample.panes.map((pane, index) => (
					<Fragment key={pane.label}>
						{index > 0 ? (
							<div aria-hidden="true" className="ml-6 h-5 w-px bg-[var(--ed-fig)]/50" />
						) : null}
						<div className="border border-white/10 bg-[#0d0d0d]">
							<p className="ed-mono truncate border-b border-white/10 px-3 py-2 text-[11px] text-[#9e9e9e]">
								{pane.label}
							</p>
							<pre className="ed-mono overflow-x-auto p-3 text-[12px] leading-[1.7]">
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
	const facts = PAGE_FACTS[page.path];
	const siblings = allCapabilityPages.filter((item) => item.kind === page.kind);
	const index = siblings.findIndex((item) => item.slug === page.slug) + 1;
	const reference = facts?.reference;
	const compare = PAGE_COMPARE[page.path];
	const sample = PAGE_SAMPLES[page.path];
	const howFig = compare ? "3" : "2";

	return (
		<>
			<JsonLd
				data={breadcrumbJsonLd([
					{ name: "Home", path: "/" },
					{ name: parent.name, path: parent.path },
					{ name: page.eyebrow, path: page.path },
				])}
			/>
			<Hero index={index} page={page} total={siblings.length} />
			{facts ? <SpecStrip specs={facts.specs} /> : null}
			<Chapter label="Why it matters" n={1} title={page.problem.title}>
				<div className="mt-6 grid gap-6 lg:grid-cols-2">
					<p className="ed-sans text-[16px] leading-[1.65] text-[var(--ed-body)]">
						{page.problem.body}
					</p>
					<p className="ed-sans text-[16px] leading-[1.65] text-[var(--ed-muted)]">{page.lede}</p>
				</div>
				{compare ? (
					<div className="mt-10">
						<CompareFigure compare={compare} fig="2" />
					</div>
				) : null}
			</Chapter>
			<Chapter label="How it works" n={2} title={page.howItWorks.title}>
				<p className="ed-sans mt-6 max-w-[62ch] text-[16px] leading-[1.65] text-[var(--ed-body)]">
					{page.howItWorks.body}
				</p>
				{sample ? (
					<div className="mt-10 grid grid-cols-1 gap-10 lg:grid-cols-[minmax(0,240px)_minmax(0,1fr)]">
						<ol className="self-start border-t border-[var(--ed-line)]">
							{page.workflow.map((step, stepIndex) => (
								<li className="flex gap-4 border-b border-[var(--ed-line)] py-4" key={step}>
									<span className="ed-mono pt-0.5 text-[11px] text-[var(--ed-fig)]">
										2.{stepIndex + 1}
									</span>
									<span className="ed-sans text-[14.5px] leading-[1.55] text-[var(--ed-body)]">
										{step}
									</span>
								</li>
							))}
						</ol>
						<SampleFigure fig={howFig} sample={sample} />
					</div>
				) : null}
				{!sample || page.path === "/features/delivery-feedback" ? (
					<div className="mt-10">
						<HowFigure fig={sample ? String(Number(howFig) + 1) : howFig} page={page} />
					</div>
				) : null}
			</Chapter>
			<Chapter
				label="What you get"
				n={3}
				title={page.kind === "guide" ? "What to know." : "What you get."}
			>
				<Definitions items={page.capabilities} n={3} />
				{reference ? <ReferenceTable n={3} reference={reference} /> : null}
			</Chapter>
			{page.examples.length > 0 ? (
				<Chapter label="In practice" n={4} title="Typical uses.">
					<ul className="mt-10 grid gap-px border border-[var(--ed-line)] bg-[var(--ed-line)] md:grid-cols-2">
						{page.examples.map((example, exampleIndex) => (
							<li
								className={cn(
									"bg-[var(--ed-bg)] p-6",
									page.examples.length % 2 === 1 &&
										exampleIndex === page.examples.length - 1 &&
										"md:col-span-2",
								)}
								key={example.title}
							>
								<p className="ed-fig-label text-[var(--ed-fig)]">Case 4.{exampleIndex + 1}</p>
								<p className="ed-sans mt-3 text-[16px] text-[var(--ed-ink)]">{example.title}</p>
								<p className="ed-sans mt-2 text-[14px] leading-[1.6] text-[var(--ed-muted)]">
									{example.body}
								</p>
							</li>
						))}
					</ul>
				</Chapter>
			) : null}
			<WorksWith current={page.kind === "integration" ? page.slug : undefined} />
			<FaqSection faqs={page.faqs} />
			<LinkCards items={page.related} />
		</>
	);
}
