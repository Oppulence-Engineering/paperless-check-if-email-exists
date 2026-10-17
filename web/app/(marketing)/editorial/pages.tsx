import Link from "next/link";

import {
	ArrowRight,
	BookOpen,
	CheckCircle,
	Code,
	Key,
	LinkSimple,
	ListChecks,
	LockSimple,
	SealCheck,
	ShieldCheck,
	UserCircle,
} from "@/lib/icons";
import { cn } from "@/lib/utils";

import { guidePages, homepageFaqs } from "../catalog";
import { productPage, type PlatformPage } from "../marketing-data";
import { ContactForm } from "./contact-form";
import { CheckPipeline } from "./graphics";
import { FeatureBento, SortedList, Teams } from "./home";
import { ApiKeysMock, FixPlanTableMock, RetentionMock } from "./mocks";
import {
	CenteredHero,
	ChecklistCard,
	CtaBand,
	FaqSection,
	NumberedPoints,
	LinkCards,
	PageHero,
	StepCards,
} from "./page-kit";
import {
	AppWindow,
	ButtonLink,
	Card,
	Container,
	Display,
	Eyebrow,
	IconTile,
	Lead,
	Muted,
	Section,
	Stage,
} from "./ui";

const PLATFORMS = ["SendGrid", "Mailgun", "Postmark", "Amazon SES"] as const;

/* ---------- Product ---------- */

export function EditorialProduct() {
	return (
		<>
			<PageHero
				chip="Email verification"
				eyebrow="Product"
				lede={productPage.description}
				title={productPage.title}
				visual={<CheckPipeline fig="1" />}
			/>
			<StepCards
				steps={[
					{
						title: "Address format",
						body: "The address is parsed and checked for typos and impossible formats.",
					},
					{
						title: "Domain and mail servers",
						body: "We look up the domain and the mail servers that accept its mail.",
					},
					{
						title: "Mailbox",
						body: "The mail server is asked about the mailbox. Nothing is delivered.",
					},
					{
						title: "Score and reasons",
						body: "Every signal becomes a category, a score, and reason codes.",
					},
				]}
				title="What happens in one check."
			/>
			<SortedList />
			<FeatureBento />
			<Teams />
			<FaqSection faqs={homepageFaqs} />
			<CtaBand
				lede="Start with 100 free checks a month."
				title="Send only to addresses that exist."
			/>
		</>
	);
}

/* ---------- Web workspace ---------- */

export function EditorialWeb({ page }: { page: PlatformPage }) {
	return (
		<>
			<PageHero
				chip={page.name}
				eyebrow="Product"
				lede={page.lede}
				primary={page.cta ?? { label: "Start free", href: "/sign-up" }}
				title={page.title}
				visual={
					<Stage className="flex min-h-[340px] items-center justify-center">
						<AppWindow className="w-full max-w-[520px]" title="Fix plan · October newsletter">
							<FixPlanTableMock />
						</AppWindow>
					</Stage>
				}
			/>
			<SortedList />
			<NumberedPoints
				eyebrow="In the workspace"
				items={page.sections.map((section) => ({
					title: section.title,
					body: section.body,
				}))}
				title="Everything a sending team needs."
			/>
			<ChecklistCard
				eyebrow="Details"
				items={[
					...page.sections.flatMap((section) =>
						section.bullets.map((bullet) => ({ title: bullet })),
					),
					...page.specs.map((spec) => ({
						title: spec.term,
						detail: spec.detail,
					})),
				]}
				title="What is included."
			/>
			<CtaBand
				lede="Invite your team and clean your first list today."
				title="Bring your lists into one place."
			/>
		</>
	);
}

/* ---------- Security ---------- */

const SECURITY_CARDS = [
	{
		icon: UserCircle,
		title: "Organization scope",
		body: "Lists, results, history, and API keys belong to your organization. Other organizations cannot see them.",
	},
	{
		icon: LinkSimple,
		title: "Signed webhooks",
		body: "Webhooks we send are signed with your secret. Delivery events from SendGrid, Mailgun, and Amazon SES are signature-checked.",
	},
	{
		icon: LockSimple,
		title: "No credentials in the browser",
		body: "The browser never receives the credentials our servers use to talk to each other.",
	},
	{
		icon: CheckCircle,
		title: "No email sent",
		body: "A check never delivers a message to the recipient.",
	},
] as const;

export const securityFaqs = [
	{
		question: "Where is my data stored?",
		answer: "In our hosted service. Access is limited to your organization.",
	},
	{
		question: "Does the browser receive server credentials?",
		answer:
			"No. Requests from the browser go through our servers, and server credentials never reach the browser.",
	},
	{
		question: "Does a check send an email?",
		answer: "No. A check never delivers a message to the recipient.",
	},
];

export function EditorialSecurity() {
	return (
		<>
			<section className="bg-[var(--ed-bg)] px-4 pt-8 pb-16 sm:px-6 lg:px-8">
				<div className="relative mx-auto max-w-[1280px] overflow-hidden rounded-3xl border border-[var(--ed-line)] bg-[var(--ed-bg)] text-white">
					<div className="relative grid items-center gap-10 p-8 sm:p-12 lg:grid-cols-[1fr_0.95fr]">
						<div>
							<h1 className="ed-display text-[40px] leading-[1.12] sm:text-[52px]">
								Your lists stay yours.{" "}
								<span className="text-white/60">Inside your organization.</span>
							</h1>
							<p className="ed-sans mt-5 max-w-[46ch] text-[15px] leading-[1.6] font-[450] text-white/75">
								Every list, result, and key is scoped to your organization. You choose how long
								results are kept, and server credentials never reach the browser.
							</p>
							<div className="mt-8 flex flex-wrap gap-2.5">
								<ButtonLink href="/sign-up" variant="inverse">
									Start free
								</ButtonLink>
							</div>
						</div>
						<Stage className="flex items-center justify-center">
							<div className="w-full max-w-[360px]">
								<RetentionMock />
							</div>
						</Stage>
					</div>
					<div className="relative border-t border-white/10 px-8 py-6 text-center sm:px-12">
						<p className="ed-mono text-[10px] tracking-[0.3em] text-white/50 uppercase">
							Works with your email platform
						</p>
						<p className="ed-sans mt-3 flex flex-wrap justify-center gap-x-10 gap-y-1 text-[17px] font-medium text-white/80">
							{PLATFORMS.map((name) => (
								<span key={name}>{name}</span>
							))}
						</p>
					</div>
				</div>
			</section>

			<Section className="pt-6 sm:pt-8 lg:pt-10">
				<Container>
					<Eyebrow>How it is built</Eyebrow>
					<Display className="mt-2 max-w-[20ch]" size="lg">
						Protection built in. <Muted>Not added later.</Muted>
					</Display>
					<div className="mt-12 grid gap-3 lg:grid-cols-[1.2fr_1fr]">
						<Stage className="flex min-h-[340px] items-center justify-center">
							<div className="w-full max-w-[460px]">
								<ApiKeysMock />
								<p className="ed-sans mt-4 text-center text-[13px] text-white/90">
									Give each key only the scopes it needs. Revoke it at any time.
								</p>
							</div>
						</Stage>
						<div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-1">
							{SECURITY_CARDS.slice(0, 2).map((card) => (
								<Card className="p-6" key={card.title}>
									<IconTile tone="accent">
										<card.icon aria-hidden="true" />
									</IconTile>
									<h3 className="ed-sans mt-8 text-[16px] font-medium text-[var(--ed-ink)]">
										{card.title}
									</h3>
									<p className="ed-sans mt-2 text-[13.5px] leading-[1.55] text-[var(--ed-body)]">
										{card.body}
									</p>
								</Card>
							))}
						</div>
					</div>
					<div className="mt-3 grid gap-3 md:grid-cols-3">
						{SECURITY_CARDS.slice(2).map((card) => (
							<Card className="p-6" key={card.title}>
								<IconTile tone="good">
									<card.icon aria-hidden="true" />
								</IconTile>
								<h3 className="ed-sans mt-8 text-[16px] font-medium text-[var(--ed-ink)]">
									{card.title}
								</h3>
								<p className="ed-sans mt-2 text-[13.5px] leading-[1.55] text-[var(--ed-body)]">
									{card.body}
								</p>
							</Card>
						))}
						<Card className="flex flex-col justify-between p-6">
							<div>
								<IconTile tone="warn">
									<Key aria-hidden="true" />
								</IconTile>
								<h3 className="ed-sans mt-8 text-[16px] font-medium text-[var(--ed-ink)]">
									Retention you choose
								</h3>
								<p className="ed-sans mt-2 text-[13.5px] leading-[1.55] text-[var(--ed-body)]">
									Choose how many days results are kept in your workspace.
								</p>
							</div>
						</Card>
					</div>
				</Container>
			</Section>
			<FaqSection faqs={securityFaqs} />
			<LinkCards
				items={[
					{
						label: "Privacy",
						href: "/privacy",
						description: "How account and verification data are handled.",
					},
					{
						label: "Responsible disclosure",
						href: "/responsible-disclosure",
						description: "Report a vulnerability through the security channel.",
					},
					{
						label: "Delivery feedback",
						href: "/features/delivery-feedback",
						description: "How delivery events reach your workspace.",
					},
				]}
			/>
			<CtaBand
				title="Questions about security?"
				lede="Tell us what your review needs, and we will answer it."
			/>
		</>
	);
}

/* ---------- Contact ---------- */

export function EditorialContact() {
	return (
		<>
			<CenteredHero
				eyebrow="Contact"
				lede="Ask about checks, results, or your account."
				sans
				title={
					<>
						Questions? <Muted>Send us a note.</Muted>
					</>
				}
			/>
			<section className="bg-[var(--ed-bg)] px-4 pb-10 sm:px-6 lg:px-8">
				<Stage className="mx-auto max-w-[1120px] py-14 sm:py-16">
					<ContactForm />
				</Stage>
			</section>
			<LinkCards
				eyebrow="Other ways to reach us"
				items={[
					{
						label: "Product questions",
						href: "/guides",
						description: "Read the guides first. Then email hello@oppulence.io.",
					},
					{
						label: "Integrations",
						href: "/integrations",
						description: "Each integration page has the setup steps.",
					},
					{
						label: "Security reports",
						href: "/responsible-disclosure",
						description: "Use the responsible disclosure page, not a public channel.",
					},
				]}
			/>
		</>
	);
}

/* ---------- Changelog ---------- */

export type ChangelogEntry = {
	version: string;
	date?: string;
	title: string;
	body: string;
};

function formatDate(value?: string) {
	if (!value) return "";
	const date = new Date(value);
	if (Number.isNaN(date.getTime())) return value;
	return new Intl.DateTimeFormat("en", {
		dateStyle: "medium",
		timeZone: "UTC",
	}).format(date);
}

export function EditorialChangelog({ entries }: { entries: ChangelogEntry[] }) {
	return (
		<>
			<CenteredHero
				eyebrow="Changelog"
				lede="Product changes, newest first."
				sans
				title={
					<>
						What changed. <Muted>And when.</Muted>
					</>
				}
			/>
			<Section className="pt-4 sm:pt-6 lg:pt-8">
				<Container>
					<ol className="border-t border-[var(--ed-line)]">
						{entries.map((entry) => (
							<li
								className="grid gap-4 border-b border-[var(--ed-line)] py-8 md:grid-cols-[200px_1fr]"
								key={entry.title}
							>
								<div>
									<time
										className="ed-mono text-[11px] tracking-[0.15em] text-[var(--ed-muted)] uppercase"
										dateTime={entry.date}
									>
										{formatDate(entry.date)}
									</time>
								</div>
								<div>
									<span className="ed-sans inline-flex rounded-full border border-[var(--ed-line)] px-2.5 py-0.5 text-[11.5px] text-[var(--ed-body)]">
										{entry.version}
									</span>
									<h2 className="ed-display mt-3 text-[24px] leading-[1.3] text-[var(--ed-ink)]">
										{entry.title}
									</h2>
									<p className="ed-sans mt-2 max-w-[62ch] text-[15px] leading-[1.6] font-[450] text-[var(--ed-body)]">
										{entry.body}
									</p>
								</div>
							</li>
						))}
					</ol>
				</Container>
			</Section>
		</>
	);
}

/* ---------- Resources ---------- */

export function EditorialResources() {
	return (
		<>
			<CenteredHero
				eyebrow="Resources"
				lede="Guides, integrations, security, and product updates."
				sans
				title={
					<>
						Resources. <Muted>For cleaner sending.</Muted>
					</>
				}
			/>
			<LinkCards
				eyebrow="Start here"
				items={[
					{
						label: "Guides",
						href: "/guides",
						description: "Read results, handle catch-all domains, and keep lists healthy.",
					},
					{
						label: "Blog",
						href: "/blog",
						description: "Notes on email verification and list hygiene.",
					},
					{
						label: "Changelog",
						href: "/changelog",
						description: "What changed in the product.",
					},
					{
						label: "Integrations",
						href: "/integrations",
						description: "SendGrid, Mailgun, Postmark, Amazon SES, CSV, and the API.",
					},
					{
						label: "API reference",
						href: "/developers",
						description: "Every endpoint, with request and response shapes.",
					},
					{
						label: "Security",
						href: "/security",
						description: "How we protect your lists and results.",
					},
				]}
			/>
			<LinkCards
				eyebrow="Guides"
				items={guidePages.map((page) => ({
					label: page.eyebrow,
					href: page.path,
					description: page.description,
				}))}
			/>
			<CtaBand lede="Start with 100 free checks a month." title="Ready to clean a list?" />
		</>
	);
}

/* ---------- 404 ---------- */

export function EditorialNotFound() {
	const cards = [
		{ label: "Home", href: "/", icon: SealCheck },
		{ label: "Features", href: "/features", icon: ListChecks },
		{ label: "Guides", href: "/guides", icon: BookOpen },
		{ label: "API reference", href: "/developers", icon: Code },
	];
	return (
		<Section>
			<Container>
				<div className="grid gap-6 md:grid-cols-2">
					<p className="ed-display text-[28px] text-[var(--ed-ink)]">404</p>
					<div>
						<h1 className="ed-display text-[28px] leading-[1.35] text-[var(--ed-ink)]">
							<Muted>Sorry.</Muted> We can’t find the page you’re looking for.
						</h1>
						<ButtonLink className="mt-6" href="/">
							Go to the home page
						</ButtonLink>
					</div>
				</div>
				<div className="mt-14 grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
					{cards.map((card) => (
						<Link
							className="group flex aspect-[4/3] flex-col justify-between rounded-2xl border border-[var(--ed-line)] bg-[var(--ed-subtle)] p-5 transition-colors hover:bg-[var(--ed-panel)]"
							href={card.href}
							key={card.href}
						>
							<card.icon aria-hidden="true" className="size-8 text-[var(--ed-faint)]" />
							<span className="ed-sans flex items-center justify-between text-[16px] text-[var(--ed-ink)]">
								{card.label}
								<ArrowRight
									aria-hidden="true"
									className="size-4 transition-transform group-hover:translate-x-0.5"
								/>
							</span>
						</Link>
					))}
				</div>
			</Container>
		</Section>
	);
}

/* ---------- Billing status ---------- */

export function EditorialBillingStatus({
	title,
	lede,
	actions,
}: {
	title: string;
	lede: string;
	actions: { href: string; label: string; variant?: "primary" | "outline" }[];
}) {
	return (
		<Section>
			<Container className="flex flex-col items-center text-center">
				<IconTile className="size-12" tone="accent">
					<ShieldCheck aria-hidden="true" />
				</IconTile>
				<Display as="h1" className="mt-6" size="md">
					{title}
				</Display>
				<Lead className="mx-auto mt-4">{lede}</Lead>
				<div className="mt-8 flex flex-wrap justify-center gap-2.5">
					{actions.map((action) => (
						<ButtonLink
							href={action.href}
							key={action.href}
							variant={action.variant === "outline" ? "secondary" : "primary"}
						>
							{action.label}
						</ButtonLink>
					))}
				</div>
			</Container>
		</Section>
	);
}

export const editorialClass = cn;
