import Link from "next/link";
import type { ReactNode } from "react";

import { ArrowRight } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { pricingPlans } from "../marketing-data";
import {
	ApiTerminal,
	CheckPipeline,
	EventTimelineFigure,
	FeedbackFlow,
	HeroConsole,
	ListSortDemo,
	PlatformMarquee,
	QualityFigure,
	RecheckDialFigure,
	Reveal,
	ScoreScaleFigure,
	SingleCheckDemo,
	SuppressionDemo,
} from "./graphics";
import {
	FeedbackStreamMini,
	IntegrationChipsMini,
	ListBoardMock,
	RecheckMini,
	ScoreMini,
	SingleCheckMini,
	SuppressionMini,
	TeamMini,
} from "./mocks";
import {
	AppWindow,
	ArrowTile,
	ButtonLink,
	Container,
	Display,
	Eyebrow,
	Lead,
	Muted,
	Section,
	Sheet,
	Stage,
} from "./ui";
import { UseCaseTabs } from "./use-case-tabs";

/*
 * Home page in the polar.sh order: a centered hero over a live product
 * console, a platform strip, building blocks, the platform, how a check
 * works, a flow diagram, list quality, the API, pricing, and the closing
 * call to action from the shell.
 */

const CAPABILITIES = [
	{ label: "Single checks", href: "/features/single-checks" },
	{ label: "List cleaning", href: "/features/list-cleaning" },
	{ label: "Scores and reasons", href: "/features/risk-scoring" },
	{ label: "Suppression list", href: "/features/suppressions" },
	{ label: "Delivery feedback", href: "/features/delivery-feedback" },
];

/** Two-line section heading: the first line in white, the second in gray. */
function Heading({
	id,
	eyebrow,
	title,
	muted,
	body,
	center = false,
	className,
}: {
	id: string;
	eyebrow?: string;
	title: string;
	muted: string;
	body?: ReactNode;
	center?: boolean;
	className?: string;
}) {
	return (
		<Reveal className={cn(center && "mx-auto text-center", className)}>
			{eyebrow ? <Eyebrow className="mb-4">{eyebrow}</Eyebrow> : null}
			<Display className={cn("max-w-[24ch]", center && "mx-auto")} id={id} size="md">
				<span className="block">{title}</span>
				<Muted>{muted}</Muted>
			</Display>
			{body ? <Lead className={cn("mt-5 max-w-[56ch]", center && "mx-auto")}>{body}</Lead> : null}
		</Reveal>
	);
}

/* 1. Hero: centered statement, capability links, and the live console. */
function Hero() {
	return (
		<section className="bg-[var(--ed-bg)] pt-16 pb-20 sm:pt-24 lg:pt-28">
			<Container>
				<Reveal className="flex flex-col items-center text-center">
					<Link
						className="ed-sans mb-8 inline-flex items-center gap-2 rounded-full border border-[var(--ed-line)] bg-[var(--ed-subtle)] py-1 pr-3 pl-1 text-[13px] text-[var(--ed-body)] transition-colors hover:text-[var(--ed-ink)]"
						href="/pricing"
					>
						<span className="rounded-full bg-[var(--ed-ink)] px-2 py-0.5 text-[11px] font-[550] text-[var(--ed-bg)]">
							Free
						</span>
						100 checks every month
						<ArrowRight aria-hidden="true" className="size-3.5" />
					</Link>
					<h1 className="ed-display max-w-[18ch] text-[44px] leading-[1.08] text-[var(--ed-ink)] sm:text-[60px] lg:text-[72px]">
						Check email addresses <Muted>before you send.</Muted>
					</h1>
					<p className="ed-sans mt-6 max-w-[48ch] text-[16px] leading-[1.55] font-[450] text-[var(--ed-body)] sm:text-[18px]">
						Find invalid and risky addresses before they bounce. Clean whole lists, suppress what
						bounced, and protect your sender reputation.
					</p>
					<div className="mt-9 flex flex-wrap justify-center gap-2.5">
						<ButtonLink arrow href="/sign-up" size="md">
							Get started free
						</ButtonLink>
					</div>
					<ul className="ed-sans mt-10 flex flex-wrap justify-center gap-x-6 gap-y-2 text-[14px] text-[var(--ed-muted)]">
						{CAPABILITIES.map((item) => (
							<li key={item.href}>
								<Link className="transition-colors hover:text-[var(--ed-ink)]" href={item.href}>
									{item.label}
								</Link>
							</li>
						))}
					</ul>
				</Reveal>
				<Reveal className="mt-16 sm:mt-20" delay={0.15}>
					<HeroConsole />
				</Reveal>
			</Container>
		</section>
	);
}

/* 2. Platform strip: where checks plug in. */
function PlatformStrip() {
	return (
		<section className="border-y border-[var(--ed-line)] bg-[var(--ed-bg)] py-12">
			<Container>
				<p className="ed-sans mb-8 text-center text-[14px] text-[var(--ed-muted)]">
					Works with the platform you already send from
				</p>
			</Container>
			<PlatformMarquee />
		</section>
	);
}

/** A figure plate with its number, then a short title, text, and a link under it. */
function PlateCard({
	fig,
	title,
	body,
	href,
	children,
}: {
	fig: string;
	title: string;
	body: string;
	href: string;
	children: ReactNode;
}) {
	return (
		<article className="flex h-full flex-col">
			<Sheet bodyClassName="px-3 pt-2 pb-3" fig={fig} title={title}>
				{children}
			</Sheet>
			<div className="flex flex-1 flex-col pt-5">
				<p className="ed-mono text-[11px] text-[var(--ed-faint)]">{fig}</p>
				<h3 className="ed-sans mt-1 text-[18px] text-[var(--ed-ink)]">{title}</h3>
				<p className="ed-sans mt-2 text-[14px] leading-[1.55] text-[var(--ed-muted)]">{body}</p>
				<Link
					className="ed-sans mt-auto inline-flex items-center gap-1.5 pt-4 text-[14px] text-[var(--ed-ink)] hover:opacity-80"
					href={href}
				>
					Learn more <ArrowRight aria-hidden="true" className="size-3.5" />
				</Link>
			</div>
		</article>
	);
}

/* 3. Building blocks: three animated plates. */
const BLOCKS = [
	{
		fig: "3.1",
		title: "Single checks",
		body: "Check one address in the app, at signup, or through the API.",
		href: "/features/single-checks",
		Graphic: SingleCheckDemo,
	},
	{
		fig: "3.2",
		title: "List cleaning",
		body: "Upload a CSV and get a result and an action for every row.",
		href: "/features/list-cleaning",
		Graphic: ListSortDemo,
	},
	{
		fig: "3.3",
		title: "Suppression list",
		body: "Keep bounced and complained addresses out of every send.",
		href: "/features/suppressions",
		Graphic: SuppressionDemo,
	},
];

function BuildingBlocks() {
	return (
		<Section labelledBy="blocks-heading">
			<Container>
				<Heading
					body="Single checks, list cleaning, and a suppression list work together on one workspace. Start with one and add the rest when you need them."
					id="blocks-heading"
					muted="that fit the way you send."
					title="Building blocks for clean sending"
				/>
				<div className="mt-14 grid gap-10 lg:grid-cols-3 lg:gap-6">
					{BLOCKS.map(({ Graphic, ...card }, index) => (
						<Reveal delay={index * 0.08} key={card.href}>
							<PlateCard {...card}>
								<Graphic />
							</PlateCard>
						</Reveal>
					))}
				</div>
			</Container>
		</Section>
	);
}

/* 4. Platform: three readings of one workspace. */
const PLATFORM_CARDS = [
	{
		fig: "4.1",
		title: "Scores and reasons",
		body: "Every result has a score from 0 to 100 and the reasons behind it.",
		href: "/features/risk-scoring",
		Graphic: ScoreScaleFigure,
	},
	{
		fig: "4.2",
		title: "Delivery feedback",
		body: "Bounces, complaints, and unsubscribes come back from your email platform.",
		href: "/features/delivery-feedback",
		Graphic: EventTimelineFigure,
	},
	{
		fig: "4.3",
		title: "Ongoing re-checks",
		body: "Old results are checked again on a schedule, so lists stay fresh.",
		href: "/features/reverification",
		Graphic: RecheckDialFigure,
	},
];

function Platform() {
	return (
		<Section labelledBy="platform-heading" tone="subtle">
			<Container>
				<Heading
					body="Check addresses, clean lists, suppress what bounced, and re-check on a schedule. One workspace, from the first upload to a safe send."
					eyebrow="Platform"
					id="platform-heading"
					muted="in one workspace."
					title="A list hygiene team"
				/>
				<div className="mt-14 grid gap-10 lg:grid-cols-3 lg:gap-6">
					{PLATFORM_CARDS.map(({ Graphic, ...card }, index) => (
						<Reveal delay={index * 0.08} key={card.href}>
							<PlateCard {...card}>
								<Graphic />
							</PlateCard>
						</Reveal>
					))}
				</div>
			</Container>
		</Section>
	);
}

/* 5. How a check works: the live pipeline and three numbered steps. */
const STEPS = [
	["Send an address or a list", "Check one address, upload a CSV, or call the API."],
	["We read the path to the inbox", "Format, domain, mail server, and mailbox, in that order."],
	["Act on the result", "Keep, fix, review, or suppress, with a score and a reason."],
];

function HowItWorks() {
	return (
		<Section labelledBy="how-heading">
			<Container>
				<div className="grid items-center gap-12 lg:grid-cols-[1fr_1.15fr] lg:gap-16">
					<div>
						<Heading
							body="A check reads the path to the inbox and stops before anything is delivered. You get the result in seconds."
							eyebrow="How a check works"
							id="how-heading"
							muted="before anything is sent."
							title="Every address checked"
						/>
						<ol className="mt-10 flex flex-col">
							{STEPS.map(([title, body], index) => (
								<li
									className="ed-sans grid grid-cols-[36px_1fr] gap-3 border-t border-[var(--ed-line)] py-4"
									key={title}
								>
									<span className="ed-mono text-[12px] text-[var(--ed-faint)]">
										{String(index + 1).padStart(2, "0")}
									</span>
									<span>
										<span className="block text-[15px] text-[var(--ed-ink)]">{title}</span>
										<span className="mt-0.5 block text-[14px] text-[var(--ed-muted)]">{body}</span>
									</span>
								</li>
							))}
						</ol>
					</div>
					<Reveal delay={0.1}>
						<CheckPipeline />
					</Reveal>
				</div>
			</Container>
		</Section>
	);
}

/* 6. Delivery feedback: a flow from the email platform to the next send. */
function Feedback() {
	return (
		<Section labelledBy="feedback-heading" tone="subtle">
			<Container>
				<Heading
					body="Connect SendGrid, Mailgun, Postmark, or Amazon SES. Bounces, complaints, and unsubscribes update your suppression list."
					center
					eyebrow="Delivery feedback"
					id="feedback-heading"
					muted="and stay out of your next send."
					title="Bounces come back"
				/>
				<Reveal className="mt-14" delay={0.1}>
					<FeedbackFlow />
				</Reveal>
			</Container>
		</Section>
	);
}

/* 7. List quality: a score for the list and a gate for bad uploads. */
function ListQuality() {
	return (
		<Section labelledBy="quality-heading">
			<Container>
				<div className="grid items-center gap-12 lg:grid-cols-[1fr_1.2fr] lg:gap-16">
					<div>
						<Heading
							body="Every list gets a quality score and every row gets a reason. Mark a bad upload for review before it reaches your email platform."
							eyebrow="List quality"
							id="quality-heading"
							muted="before you send it."
							title="See what a list is worth"
						/>
						<ButtonLink
							arrow
							className="mt-8"
							href="/features/list-cleaning"
							size="md"
							variant="secondary"
						>
							See list cleaning
						</ButtonLink>
					</div>
					<Reveal delay={0.1}>
						<QualityFigure />
					</Reveal>
				</div>
			</Container>
		</Section>
	);
}

/* 8. API: copy and a terminal that runs a check. */
function Api() {
	return (
		<Section labelledBy="api-heading" tone="subtle">
			<Container>
				<div className="grid items-center gap-12 lg:grid-cols-[1fr_1.2fr] lg:gap-16">
					<div>
						<Heading
							body="Single checks, list jobs, suppressions, and delivery events are all in the REST API, with Go and TypeScript SDKs and signed webhooks."
							eyebrow="API and SDKs"
							id="api-heading"
							muted="in an afternoon."
							title="Add checks to your signup form"
						/>
						<div className="mt-8 flex flex-wrap gap-2.5">
							<ButtonLink arrow href="/developers" size="md">
								Read the API reference
							</ButtonLink>
							<ButtonLink href="/features/api" size="md" variant="secondary">
								API and SDKs
							</ButtonLink>
						</div>
					</div>
					<Reveal delay={0.1}>
						<ApiTerminal />
					</Reveal>
				</div>
			</Container>
		</Section>
	);
}

/* 9. Pricing: four plans in one row. */
function Pricing() {
	return (
		<Section labelledBy="pricing-heading">
			<Container>
				<div className="flex flex-wrap items-end justify-between gap-6">
					<Heading
						eyebrow="Pricing"
						id="pricing-heading"
						muted="Start free, upgrade as you send more."
						title="Pricing that grows with your lists"
					/>
					<div className="flex gap-2.5">
						<ButtonLink href="/sign-up" size="md">
							Get started
						</ButtonLink>
					</div>
				</div>
				<div className="mt-12 grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
					{pricingPlans.map((plan, index) => (
						<Reveal delay={index * 0.06} key={plan.name}>
							<article
								className={cn(
									"ed-sans flex h-full flex-col rounded-2xl border bg-[var(--ed-subtle)] p-6",
									plan.recommended ? "border-[#155dfc]/60" : "border-[var(--ed-line)]",
								)}
							>
								<div className="flex items-center justify-between">
									<h3 className="text-[16px] text-[var(--ed-ink)]">{plan.name}</h3>
									{plan.recommended ? (
										<span className="rounded-full bg-[#155dfc]/15 px-2.5 py-0.5 text-[11px] text-[#3080ff]">
											Most teams
										</span>
									) : null}
								</div>
								<p className="mt-6 text-[36px] leading-none tracking-[-0.02em] text-[var(--ed-ink)]">
									{plan.price}
									{plan.period ? (
										<span className="ml-1 text-[14px] tracking-normal text-[var(--ed-muted)]">
											{plan.period}
										</span>
									) : null}
								</p>
								<p className="mt-2 text-[14px] text-[var(--ed-body)]">{plan.note}</p>
								<p className="mt-4 text-[14px] leading-[1.55] text-[var(--ed-muted)]">
									{plan.description}
								</p>
							</article>
						</Reveal>
					))}
				</div>
				<Link
					className="ed-sans mt-8 inline-flex items-center gap-1.5 text-[14px] text-[var(--ed-ink)] hover:opacity-80"
					href="/pricing"
				>
					Compare plans <ArrowRight aria-hidden="true" className="size-3.5" />
				</Link>
			</Container>
		</Section>
	);
}

/* Sections shared with the product page. */

/* Sorted list: headline and a staged product view. */
export function SortedList() {
	return (
		<Section labelledBy="sorted-heading">
			<Container>
				<Display className="max-w-[20ch]" id="sorted-heading">
					Every address, sorted <Muted>before you send.</Muted>
				</Display>
				<Lead className="mt-5">
					Upload a list and it sorts itself: valid, risky, invalid, and unknown, each with a score
					and the reason.
				</Lead>
				<Stage className="mt-12">
					<AppWindow className="mx-auto max-w-[920px]" title="Lists · October newsletter">
						<ListBoardMock />
					</AppWindow>
				</Stage>
			</Container>
		</Section>
	);
}

function BentoCard({
	label,
	title,
	href,
	body,
	className,
	children,
}: {
	label: string;
	title: string;
	href: string;
	body?: string;
	className?: string;
	children: React.ReactNode;
}) {
	return (
		<article
			className={`flex flex-col overflow-hidden rounded-2xl border border-[var(--ed-line)] bg-[var(--ed-bg)] ${className ?? ""}`}
		>
			<div className="flex items-start justify-between gap-4 p-5">
				<div>
					<p className="ed-sans text-[12px] text-[var(--ed-muted)]">{label}</p>
					<h3 className="ed-sans mt-1 text-[20px] font-medium text-[var(--ed-ink)]">{title}</h3>
					{body ? (
						<p className="ed-sans mt-2 max-w-[38ch] text-[13.5px] leading-[1.5] text-[var(--ed-body)]">
							{body}
						</p>
					) : null}
				</div>
				<ArrowTile href={href} label={`${label}: ${title}`} />
			</div>
			<div className="flex flex-1 flex-col justify-center bg-[var(--ed-subtle)] p-4 sm:p-5">
				{children}
			</div>
		</article>
	);
}

export function FeatureBento() {
	return (
		<Section labelledBy="bento-heading">
			<Container>
				<Display className="max-w-[22ch]" id="bento-heading">
					From one address <Muted>to every list you send.</Muted>
				</Display>
				<p className="ed-sans mt-5 flex flex-wrap items-center gap-2 text-[13px] text-[var(--ed-muted)]">
					Start with
					<Link
						className="rounded-full border border-[var(--ed-line)] px-3 py-1 text-[var(--ed-ink)] hover:bg-[var(--ed-subtle)]"
						href="/features/single-checks"
					>
						one address
					</Link>
					or
					<Link
						className="rounded-full border border-[var(--ed-line)] px-3 py-1 text-[var(--ed-ink)] hover:bg-[var(--ed-subtle)]"
						href="/features/list-cleaning"
					>
						a whole list
					</Link>
				</p>
				<div className="mt-10 grid gap-3 lg:grid-cols-2">
					<BentoCard href="/features/single-checks" label="Single checks" title="Know one address.">
						<SingleCheckMini />
					</BentoCard>
					<BentoCard
						href="/features/risk-scoring"
						label="Scores and reasons"
						title="See why it is risky."
					>
						<ScoreMini />
					</BentoCard>
					<article className="grid overflow-hidden rounded-2xl border border-[var(--ed-line)] bg-[var(--ed-bg)] lg:col-span-2 lg:grid-cols-[1fr_1.3fr]">
						<div className="flex flex-col justify-between gap-6 p-6">
							<div>
								<p className="ed-sans text-[12px] text-[var(--ed-muted)]">Delivery feedback</p>
								<h3 className="ed-sans mt-1 text-[24px] font-medium text-[var(--ed-ink)]">
									Learn from what bounced.
								</h3>
								<p className="ed-sans mt-3 max-w-[40ch] text-[14px] leading-[1.55] text-[var(--ed-body)]">
									Connect SendGrid, Mailgun, Postmark, or Amazon SES. Bounces, complaints, and
									unsubscribes update your suppression list.
								</p>
							</div>
							<ArrowTile href="/features/delivery-feedback" label="Delivery feedback" />
						</div>
						<div className="bg-[var(--ed-subtle)] p-4 sm:p-6">
							<FeedbackStreamMini />
						</div>
					</article>
				</div>
				<div className="mt-3 grid gap-3 lg:grid-cols-3">
					<BentoCard
						href="/features/suppressions"
						label="Suppression list"
						title="One list, every source."
					>
						<SuppressionMini />
					</BentoCard>
					<BentoCard
						href="/features/reverification"
						label="Ongoing re-checks"
						title="Keep lists fresh."
					>
						<RecheckMini />
					</BentoCard>
					<BentoCard href="/features/workspace" label="Team workspace" title="Work as a team.">
						<TeamMini />
					</BentoCard>
				</div>
				<article className="mt-3 grid overflow-hidden rounded-2xl border border-[var(--ed-line)] bg-[var(--ed-bg)] lg:grid-cols-[1fr_1.3fr]">
					<div className="flex flex-col justify-between gap-6 p-6">
						<div>
							<p className="ed-sans text-[12px] text-[var(--ed-muted)]">Integrations</p>
							<h3 className="ed-sans mt-1 text-[24px] font-medium text-[var(--ed-ink)]">
								Connect your stack.
							</h3>
							<p className="ed-sans mt-3 max-w-[40ch] text-[14px] leading-[1.55] text-[var(--ed-body)]">
								Send delivery events from your email platform, move lists by CSV, and build on the
								REST API with Go and TypeScript SDKs.
							</p>
						</div>
						<ArrowTile href="/integrations" label="Integrations" />
					</div>
					<div className="bg-[var(--ed-subtle)] p-4 sm:p-6">
						<IntegrationChipsMini />
					</div>
				</article>
			</Container>
		</Section>
	);
}

/* Teams: selectable list with a product view. */
export function Teams() {
	return (
		<Section labelledBy="teams-heading">
			<Container>
				<Display className="mx-auto mb-12 max-w-[20ch] text-center" id="teams-heading">
					Built for every team <Muted>that sends.</Muted>
				</Display>
				<UseCaseTabs />
			</Container>
		</Section>
	);
}

export function EditorialHome() {
	return (
		<>
			<Hero />
			<PlatformStrip />
			<BuildingBlocks />
			<Platform />
			<HowItWorks />
			<Feedback />
			<ListQuality />
			<Api />
			<Pricing />
		</>
	);
}
