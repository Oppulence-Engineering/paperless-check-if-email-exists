import Link from "next/link";
import type { ComponentType, ReactNode, SVGProps } from "react";

import { ArrowLink, FaqBlock, LinkCards, SplitFeature } from "@/components/platform/landing/blocks";
import { CtaLink } from "@/components/platform/landing/cta-link";
import {
	CONTAINER,
	Check,
	MONO,
	MUTED,
	Section,
	SectionIntro,
	Stage,
} from "@/components/platform/landing/primitives";
import { ArrowRight } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { homepageFaqs } from "../catalog";
import { headerNav } from "../site";
import { PAGE_COMPARE } from "./capability-content";
import {
	ApiTerminal,
	CheckPipeline,
	EventTimelineFigure,
	FeedbackFlow,
	HeroConsole,
	ListSortDemo,
	QualityFigure,
	RecheckDialFigure,
	ScoreScaleFigure,
	SingleCheckDemo,
	SuppressionDemo,
} from "./graphics";
import { AmazonSesLogo, ApiLogo, CsvLogo, MailgunLogo, PostmarkLogo, SendGridLogo } from "./logos";
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
import { PlanCards } from "./pricing";
import { AppWindow, Sheet } from "./ui";
import { UseCaseTabs } from "./use-case-tabs";

/*
 * Home page in the screenshot-studio `PlatformLanding` order: a split hero
 * over the product console, the platforms strip, figures on hairline rows,
 * a before and after, the features, the API, pricing, and questions. The
 * shell adds the closing call to action and the footer.
 */

const STACK: { logo: ComponentType<SVGProps<SVGSVGElement>>; name: string }[] = [
	{ logo: SendGridLogo, name: "SendGrid" },
	{ logo: MailgunLogo, name: "Mailgun" },
	{ logo: PostmarkLogo, name: "Postmark" },
	{ logo: AmazonSesLogo, name: "Amazon SES" },
	{ logo: CsvLogo, name: "CSV files" },
	{ logo: ApiLogo, name: "REST API" },
];

/* Hero: split headline, the console on a stage, and the platforms strip. */
function Hero() {
	return (
		<section className={cn(CONTAINER, "pt-16 sm:pt-24")}>
			<Link
				className="inline-flex h-8 items-center gap-2 rounded-full border border-[#2a2a2a] pr-3 pl-1 text-[13px] transition-colors hover:border-[#3a3a3a]"
				href="/pricing"
			>
				<span
					className={cn(
						MONO,
						"rounded-full bg-white px-2 py-0.5 text-[10px] font-medium tracking-[0.06em] text-black uppercase",
					)}
				>
					Free
				</span>
				<span className="text-white">100 checks every month</span>
				<span className={cn("hidden sm:inline", MUTED)}>· No email is sent during a check</span>
				<ArrowRight aria-hidden="true" className="size-3.5 text-white/60" />
			</Link>

			<div className="mt-8 grid gap-10 lg:grid-cols-[1.3fr_1fr] lg:items-end lg:gap-16">
				<h1 className="text-[46px] leading-[0.98] tracking-[-0.035em] text-balance sm:text-[64px] lg:text-[72px]">
					<span className="block text-white">Check email addresses</span>{" "}
					<span className={cn("block", MUTED)}>before you send.</span>
				</h1>
				<div className="lg:pb-2">
					<p className="max-w-[46ch] text-lg leading-[1.5] text-white/65">
						Find invalid and risky addresses before they bounce. Clean whole lists, suppress what
						bounced, and protect your sender reputation.
					</p>
					<div className="mt-8 flex flex-wrap gap-3">
						<CtaLink href="/sign-up" label="Get started free" />
						<CtaLink href="/contact" label="Talk to us" tone="secondary" />
					</div>
				</div>
			</div>

			<Stage className="mt-16 p-3 sm:mt-20 sm:p-10 lg:p-14" variant={0}>
				<HeroConsole />
			</Stage>

			<div className="mt-10 flex flex-col gap-6 sm:flex-row sm:items-center sm:justify-between">
				<p className={cn("text-[15px]", MUTED)}>Works with the platform you already send from</p>
				<ul className="flex flex-wrap items-center gap-x-7 gap-y-3">
					{STACK.map(({ logo: Logo, name }) => (
						<li className="flex items-center gap-2 text-[15px] text-white/55" key={name}>
							<Logo aria-hidden="true" className="size-[18px] grayscale" />
							{name}
						</li>
					))}
				</ul>
			</div>
		</section>
	);
}

/* Stats: a statement beside large figures on hairline rows. */
const STATS = [
	{ value: "4", label: "categories: valid, risky, invalid, and unknown" },
	{ value: "0–100", label: "score on every result, with the reasons behind it" },
	{ value: "0", label: "emails sent during a check" },
];

function StatsSection() {
	return (
		<section className={cn(CONTAINER, "py-24 sm:py-32")}>
			<div className="grid gap-12 lg:grid-cols-2 lg:gap-24">
				<p className="text-[24px] leading-[1.3] tracking-[-0.015em] text-balance text-white sm:text-[30px]">
					Every address, sorted before you send.{" "}
					<span className={MUTED}>
						Upload a list and it sorts itself: valid, risky, invalid, and unknown, each with a score
						and the reason.
					</span>
				</p>
				<dl className="divide-y divide-[#212121] border-y border-[#212121]">
					{STATS.map((stat) => (
						<div className="flex items-end justify-between gap-8 py-7" key={stat.label}>
							<dt className="sr-only">{stat.label}</dt>
							<dd className="text-[64px] leading-[0.95] tracking-[-0.045em] text-white sm:text-[84px]">
								{stat.value}
							</dd>
							<dd className={cn("max-w-[220px] pb-2 text-right text-base", MUTED)}>{stat.label}</dd>
						</div>
					))}
				</dl>
			</div>
		</section>
	);
}

/* Problem: the same work without and with checks. */
const COMPARE = ["/features/single-checks", "/features/list-cleaning"].map(
	(path) => PAGE_COMPARE[path],
);
const WITHOUT = COMPARE.flatMap((compare) => compare.without);
const WITH = COMPARE.flatMap((compare) => compare.with);

function ProblemSection() {
	return (
		<Section>
			<SectionIntro
				body="Check one address or a whole list. Each address gets a category, a score, and the reasons, and bounces from your email platform keep your suppression list current."
				label="Why it matters"
				lead="You find a bad address"
				trail="when it bounces."
			/>
			<div className="mt-24 grid gap-4 md:grid-cols-2">
				<div className="border border-[#212121] p-8 sm:p-12">
					<p className={cn("text-[15px]", MUTED)}>Without</p>
					<ul className="mt-8 space-y-4">
						{WITHOUT.map((item) => (
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
						{WITH.map((item) => (
							<li className="flex items-center gap-4 text-lg text-white" key={item}>
								<Check />
								{item}
							</li>
						))}
					</ul>
				</div>
			</div>
		</Section>
	);
}

/* Building blocks: three staged figures in a row. */
const BLOCKS = [
	{
		title: "Single checks",
		body: "Check one address in the app, at signup, or through the API.",
		href: "/features/single-checks",
		Graphic: SingleCheckDemo,
	},
	{
		title: "List cleaning",
		body: "Upload a CSV and get a result and an action for every row.",
		href: "/features/list-cleaning",
		Graphic: ListSortDemo,
	},
	{
		title: "Suppression list",
		body: "Keep bounced and complained addresses out of every send.",
		href: "/features/suppressions",
		Graphic: SuppressionDemo,
	},
];

function BuildingBlocks() {
	return (
		<Section>
			<SectionIntro
				body="Single checks, list cleaning, and a suppression list work together on one workspace. Start with one and add the rest when you need them."
				label="Building blocks"
				lead="Building blocks for clean sending"
				trail="that fit the way you send."
			/>
			<div className="mt-24 grid gap-4 md:grid-cols-3">
				{BLOCKS.map(({ Graphic, ...card }, index) => (
					<div key={card.href}>
						<Stage className="aspect-[405/320] items-center p-8" variant={index + 1}>
							<Sheet bodyClassName="p-3" className="max-w-[300px]" title={card.title}>
								<Graphic />
							</Sheet>
						</Stage>
						<h3 className="mt-7 text-xl tracking-[-0.015em] text-white">{card.title}</h3>
						<p className={cn("mt-2 text-base leading-[1.55]", MUTED)}>{card.body}</p>
						<ArrowLink className="mt-4" href={card.href}>
							Learn more
						</ArrowLink>
					</div>
				))}
			</div>
		</Section>
	);
}

/* Platform: three readings of one workspace, text beside each figure. */
const PLATFORM = [
	{
		title: "Scores and reasons",
		body: "Every result has a score from 0 to 100 and the reasons behind it.",
		href: "/features/risk-scoring",
		Graphic: ScoreScaleFigure,
	},
	{
		title: "Delivery feedback",
		body: "Bounces, complaints, and unsubscribes come back from your email platform.",
		href: "/features/delivery-feedback",
		Graphic: EventTimelineFigure,
	},
	{
		title: "Ongoing re-checks",
		body: "Old results are checked again on a schedule, so lists stay fresh.",
		href: "/features/reverification",
		Graphic: RecheckDialFigure,
	},
];

function Platform() {
	return (
		<Section>
			<SectionIntro
				body="Check addresses, clean lists, suppress what bounced, and re-check on a schedule. One workspace, from the first upload to a safe send."
				label="Platform"
				lead="A list hygiene team"
				trail="in one workspace."
			/>
			<div className="mt-24 space-y-24">
				{PLATFORM.map(({ Graphic, ...card }, index) => (
					<SplitFeature
						body={card.body}
						index={index}
						key={card.href}
						link={{ href: card.href, label: "Learn more" }}
						title={card.title}
					>
						<Sheet bodyClassName="p-4" title={card.title}>
							<Graphic />
						</Sheet>
					</SplitFeature>
				))}
			</div>
		</Section>
	);
}

/* How a check works: the pipeline on a stage, then three numbered steps. */
const STEPS = [
	["Send an address or a list", "Check one address, upload a CSV, or call the API."],
	["We read the path to the inbox", "Format, domain, mail server, and mailbox, in that order."],
	["Act on the result", "Keep, fix, review, or suppress, with a score and a reason."],
];

function HowItWorks() {
	return (
		<Section>
			<SectionIntro
				body="A check reads the path to the inbox and stops before anything is delivered. You get the result in seconds."
				label="How a check works"
				lead="Every address checked"
				trail="before anything is sent."
			/>
			<Stage className="mt-24 px-4 py-12 sm:px-12 sm:py-20" variant={2}>
				<div className="w-full min-w-0 max-w-[760px]">
					<CheckPipeline />
				</div>
			</Stage>
			<ol className="mt-16 border-t border-[#212121]">
				{STEPS.map(([title, body], index) => (
					<li
						className="grid gap-2 border-b border-[#212121] py-6 sm:grid-cols-[140px_1fr_1.4fr] sm:items-baseline sm:gap-8"
						key={title}
					>
						<span className={cn(MONO, "text-[12px] tracking-[0.06em] text-white uppercase")}>
							{String(index + 1).padStart(2, "0")}
						</span>
						<span className="text-lg text-white">{title}</span>
						<span className={cn("text-base", MUTED)}>{body}</span>
					</li>
				))}
			</ol>
		</Section>
	);
}

/* Delivery feedback: the flow from the email platform to the next send. */
function Feedback() {
	return (
		<Section>
			<SectionIntro
				body="Connect SendGrid, Mailgun, Postmark, or Amazon SES. Bounces, complaints, and unsubscribes update your suppression list."
				label="Delivery feedback"
				lead="Bounces come back"
				trail="and stay out of your next send."
			>
				<div className="mt-10">
					<ArrowLink href="/features/delivery-feedback">Learn more</ArrowLink>
				</div>
			</SectionIntro>
			<Stage className="mt-24 px-4 py-12 sm:px-12 sm:py-20" variant={3}>
				<div className="w-full min-w-0">
					<FeedbackFlow />
				</div>
			</Stage>
		</Section>
	);
}

/* List quality: a score for the list and a gate for bad uploads. */
function ListQuality() {
	return (
		<Section>
			<SectionIntro
				body="Every list gets a quality score and every row gets a reason. Mark a bad upload for review before it reaches your email platform."
				label="List quality"
				lead="See what a list is worth"
				trail="before you send it."
			>
				<div className="mt-10">
					<ArrowLink href="/features/list-cleaning">See list cleaning</ArrowLink>
				</div>
			</SectionIntro>
			<Stage className="mt-24 px-4 py-12 sm:px-12 sm:py-20" variant={1}>
				<div className="w-full min-w-0 max-w-[760px]">
					<QualityFigure />
				</div>
			</Stage>
		</Section>
	);
}

/* API: the request and response on a stage, then links for developers. */
const productNav = headerNav.flatMap((group) => group.items);
const navBlurb = (href: string) => productNav.find((item) => item.href === href)?.description;

function Api() {
	return (
		<Section id="developers">
			<SectionIntro
				body="Single checks, list jobs, suppressions, and delivery events are all in the REST API, with Go and TypeScript SDKs and signed webhooks."
				label="API and SDKs"
				lead="Add checks to your signup form"
				trail="in an afternoon."
			>
				<div className="mt-10">
					<ArrowLink href="/developers">Read the API reference</ArrowLink>
				</div>
			</SectionIntro>
			<Stage className="mt-24 px-4 py-12 sm:px-12 sm:py-20" variant={3}>
				<div className="w-full min-w-0 max-w-[760px]">
					<ApiTerminal />
				</div>
			</Stage>
			<div className="mt-4">
				<LinkCards
					columns={4}
					items={[
						{
							href: "/developers",
							label: "API reference",
							blurb: "Every endpoint, with request and response shapes.",
						},
						{ href: "/features/api", label: "API and SDKs", blurb: navBlurb("/features/api") },
						{
							href: "/use-cases/signup-forms",
							label: "Signup forms",
							blurb: navBlurb("/use-cases/signup-forms"),
						},
						{ href: "/integrations", label: "Integrations", blurb: navBlurb("/integrations") },
					]}
				/>
			</div>
		</Section>
	);
}

/* Pricing: four plans in one row. */
function Pricing() {
	return (
		<Section id="pricing">
			<SectionIntro
				body="Every plan includes scores, reason codes, suppressions, and delivery feedback. No email is sent during a check."
				label="Pricing"
				lead="Pricing that grows with your lists"
				trail="Start free, upgrade as you send more."
			>
				<div className="mt-10">
					<ArrowLink href="/pricing">Compare plans</ArrowLink>
				</div>
			</SectionIntro>
			<div className="mt-24">
				<PlanCards />
			</div>
		</Section>
	);
}

/* Sections shared with the product page. */

/* Sorted list: headline and a staged product view. */
export function SortedList() {
	return (
		<Section>
			<SectionIntro
				body="Upload a list and it sorts itself: valid, risky, invalid, and unknown, each with a score and the reason."
				label="Lists"
				lead="Every address, sorted"
				trail="before you send."
			/>
			<Stage className="mt-24 px-4 py-12 sm:px-12 sm:py-20" variant={1}>
				<AppWindow className="max-w-[920px]" title="Lists · October newsletter">
					<ListBoardMock />
				</AppWindow>
			</Stage>
		</Section>
	);
}

/* Feature cards: a staged mock, a label, a title, and a link. */
const BENTO: { label: string; title: string; body?: string; href: string; mock: ReactNode }[] = [
	{
		label: "Single checks",
		title: "Know one address.",
		href: "/features/single-checks",
		mock: <SingleCheckMini />,
	},
	{
		label: "Scores and reasons",
		title: "See why it is risky.",
		href: "/features/risk-scoring",
		mock: <ScoreMini />,
	},
	{
		label: "Delivery feedback",
		title: "Learn from what bounced.",
		body: "Connect SendGrid, Mailgun, Postmark, or Amazon SES. Bounces, complaints, and unsubscribes update your suppression list.",
		href: "/features/delivery-feedback",
		mock: <FeedbackStreamMini />,
	},
	{
		label: "Suppression list",
		title: "One list, every source.",
		href: "/features/suppressions",
		mock: <SuppressionMini />,
	},
	{
		label: "Ongoing re-checks",
		title: "Keep lists fresh.",
		href: "/features/reverification",
		mock: <RecheckMini />,
	},
	{
		label: "Team workspace",
		title: "Work as a team.",
		href: "/features/workspace",
		mock: <TeamMini />,
	},
];

export function FeatureBento() {
	return (
		<Section>
			<SectionIntro label="Features" lead="From one address" trail="to every list you send.">
				<p className={cn("mt-10 text-lg", MUTED)}>
					Start with{" "}
					<Link
						className="text-white underline-offset-4 hover:underline"
						href="/features/single-checks"
					>
						one address
					</Link>{" "}
					or{" "}
					<Link
						className="text-white underline-offset-4 hover:underline"
						href="/features/list-cleaning"
					>
						a whole list
					</Link>
					.
				</p>
			</SectionIntro>
			<ul className="mt-24 grid gap-x-4 gap-y-16 md:grid-cols-3">
				{BENTO.map((card, index) => (
					<li key={card.href}>
						<Stage className="items-center px-5 py-10 sm:aspect-[5/4] sm:p-8" variant={index}>
							<div className="w-full max-w-[400px]">{card.mock}</div>
						</Stage>
						<p className={cn(MONO, "mt-7 text-[13px]", MUTED)}>{card.label}</p>
						<h3 className="mt-2 text-2xl tracking-[-0.02em] text-white">{card.title}</h3>
						{card.body ? (
							<p className={cn("mt-3 max-w-[560px] text-[17px] leading-[1.55]", MUTED)}>
								{card.body}
							</p>
						) : null}
						<ArrowLink className="mt-4" href={card.href}>
							Learn more
						</ArrowLink>
					</li>
				))}
			</ul>
			<div className="mt-16 grid gap-12 bg-[#111] p-8 sm:p-12 lg:grid-cols-2">
				<div>
					<p className={cn(MONO, "text-[13px]", MUTED)}>Integrations</p>
					<p className="mt-2 text-[24px] leading-[1.35] tracking-[-0.02em] text-balance text-white sm:text-[28px]">
						Connect your stack.{" "}
						<span className={MUTED}>
							Send delivery events from your email platform, move lists by CSV, and build on the
							REST API with Go and TypeScript SDKs.
						</span>
					</p>
					<ArrowLink className="mt-8" href="/integrations">
						Integrations
					</ArrowLink>
				</div>
				<div className="self-center">
					<IntegrationChipsMini />
				</div>
			</div>
		</Section>
	);
}

/* Teams: selectable list with a product view. */
export function Teams() {
	return (
		<Section>
			<SectionIntro label="Use cases" lead="Built for every team" trail="that sends." />
			<div className="mt-24">
				<UseCaseTabs />
			</div>
		</Section>
	);
}

export function EditorialHome() {
	return (
		<>
			<Hero />
			<StatsSection />
			<ProblemSection />
			<BuildingBlocks />
			<Platform />
			<HowItWorks />
			<Feedback />
			<ListQuality />
			<Api />
			<Pricing />
			<FaqBlock items={homepageFaqs.map((faq) => ({ q: faq.question, a: faq.answer }))} />
		</>
	);
}
