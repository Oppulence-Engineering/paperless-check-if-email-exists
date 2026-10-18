import {
	ControlGrid,
	HeroStage,
	LinkCards as LinkCardGrid,
	PageHero,
} from "@/components/platform/landing/blocks";
import {
	CONTAINER,
	MONO,
	MUTED,
	Section,
	SectionIntro,
	Stage,
	Tag,
	Window,
} from "@/components/platform/landing/primitives";
import { CheckCircle, Key, LinkSimple, LockSimple, ShieldCheck, UserCircle } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { guidePages, homepageFaqs } from "../catalog";
import { productPage, type PlatformPage } from "../marketing-data";
import { CONTACT_EMAIL, ContactForm } from "./contact-form";
import { CheckPipeline } from "./graphics";
import { FeatureBento, SortedList, Teams } from "./home";
import { ApiKeysMock, FixPlanTableMock, RetentionMock } from "./mocks";
import { ChecklistCard, FaqSection, LinkCards, NumberedPoints, StepCards } from "./page-kit";
import { AppWindow, ButtonLink, Display, Lead } from "./ui";

const PLATFORMS = ["SendGrid", "Mailgun", "Postmark", "Amazon SES"] as const;

/* ---------- Product ---------- */

export function EditorialProduct() {
	return (
		<>
			<PageHero
				body={productPage.description}
				eyebrow={{ label: productPage.eyebrow }}
				lead={productPage.title}
				primary={{
					href: productPage.ctaHref ?? "/sign-up",
					label: productPage.ctaLabel ?? "Start free",
				}}
			>
				<HeroStage>
					<div className="w-full min-w-0 max-w-[760px]">
						<CheckPipeline />
					</div>
				</HeroStage>
			</PageHero>
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
		</>
	);
}

/* ---------- Web workspace ---------- */

export function EditorialWeb({ page }: { page: PlatformPage }) {
	return (
		<>
			<PageHero
				body={page.lede}
				eyebrow={{ label: page.name, parent: "Product", href: "/product" }}
				lead={page.title}
				primary={page.cta ?? { label: "Start free", href: "/sign-up" }}
			>
				<HeroStage>
					<AppWindow className="max-w-[520px]" title="Fix plan · October newsletter">
						<FixPlanTableMock />
					</AppWindow>
				</HeroStage>
			</PageHero>
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
	{
		icon: Key,
		title: "Retention you choose",
		body: "Choose how many days results are kept in your workspace.",
	},
];

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
			<PageHero
				body="Every list, result, and key is scoped to your organization. You choose how long results are kept, and server credentials never reach the browser."
				eyebrow={{ label: "Security" }}
				lead="Your lists stay yours."
				primary={{ href: "/sign-up", label: "Start free" }}
				trail="Inside your organization."
			>
				<HeroStage>
					<div className="w-full max-w-[360px]">
						<RetentionMock />
					</div>
				</HeroStage>
				<div className="mt-10 flex flex-col gap-6 sm:flex-row sm:items-center sm:justify-between">
					<p className={cn("text-[15px]", MUTED)}>Works with your email platform</p>
					<ul className="flex flex-wrap items-center gap-x-7 gap-y-3">
						{PLATFORMS.map((name) => (
							<li className="text-[15px] text-white/55" key={name}>
								{name}
							</li>
						))}
					</ul>
				</div>
			</PageHero>

			<Section>
				<SectionIntro
					label="How it is built"
					lead="Protection built in."
					trail="Not added later."
				/>
				<div className="mt-24 grid gap-4 lg:grid-cols-[1.2fr_1fr]">
					<Stage className="min-h-[340px] items-center px-5 py-10 sm:p-12" variant={2}>
						<div className="w-full max-w-[460px]">
							<ApiKeysMock />
							<p className="mt-4 text-center text-[13px] text-white/90">
								Give each key only the scopes it needs. Revoke it at any time.
							</p>
						</div>
					</Stage>
					<ul className="grid gap-4">
						{SECURITY_CARDS.slice(0, 2).map(({ body, icon: Icon, title }) => (
							<li className="flex flex-col bg-[#111] p-8 sm:p-10" key={title}>
								<Icon aria-hidden="true" className="size-5 text-white/80" strokeWidth={1.5} />
								<h3 className="mt-12 text-[22px] tracking-[-0.015em] text-white">{title}</h3>
								<p className={cn("mt-3 text-base leading-[1.6]", MUTED)}>{body}</p>
							</li>
						))}
					</ul>
				</div>
				<div className="mt-4">
					<ControlGrid items={SECURITY_CARDS.slice(2)} />
				</div>
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
		</>
	);
}

/* ---------- Contact ---------- */

export function EditorialContact() {
	return (
		<>
			<PageHero
				body="Ask about checks, results, or your account."
				eyebrow={{ label: "Contact" }}
				lead="Questions?"
				secondary={null}
				trail="Send us a note."
			>
				<HeroStage>
					<Window className="w-full max-w-[520px]" title={CONTACT_EMAIL}>
						<ContactForm className="p-5 sm:p-6" />
					</Window>
				</HeroStage>
			</PageHero>
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
			<PageHero
				body="Product changes, newest first."
				eyebrow={{ label: "Changelog" }}
				lead="What changed."
				trail="And when."
			/>
			<div className={cn(CONTAINER, "pb-24 sm:pb-32")}>
				<ol className="border-t border-[#212121]">
					{entries.map((entry) => (
						<li
							className="grid gap-2 border-b border-[#212121] py-6 sm:grid-cols-[140px_1fr_1.4fr] sm:items-baseline sm:gap-8"
							key={entry.title}
						>
							<time
								className={cn(MONO, "text-[12px] tracking-[0.06em] uppercase", MUTED)}
								dateTime={entry.date}
							>
								{formatDate(entry.date)}
							</time>
							<div>
								<Tag>{entry.version}</Tag>
								<h2 className="mt-3 text-lg text-white">{entry.title}</h2>
							</div>
							<p className={cn("text-base", MUTED)}>{entry.body}</p>
						</li>
					))}
				</ol>
			</div>
		</>
	);
}

/* ---------- Resources ---------- */

export function EditorialResources() {
	return (
		<>
			<PageHero
				body="Guides, integrations, security, and product updates."
				eyebrow={{ label: "Resources" }}
				lead="Resources."
				trail="For cleaner sending."
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
		</>
	);
}

/* ---------- 404 ---------- */

export function EditorialNotFound() {
	return (
		<section className={cn(CONTAINER, "pt-16 pb-24 sm:pt-24 sm:pb-32")}>
			<div className="grid gap-6 md:grid-cols-2 md:gap-8">
				<p className="text-[26px] leading-[1.3] tracking-[-0.02em] text-white sm:text-[30px]">
					404
				</p>
				<div>
					<h1 className="text-[26px] leading-[1.35] tracking-[-0.02em] text-balance text-white sm:text-[30px]">
						<span className={MUTED}>Sorry.</span> We can’t find the page you’re looking for.
					</h1>
					<ButtonLink className="mt-10" href="/">
						Go to the home page
					</ButtonLink>
				</div>
			</div>
			<div className="mt-24">
				<LinkCardGrid
					columns={4}
					items={[
						{ label: "Home", href: "/" },
						{ label: "Features", href: "/features" },
						{ label: "Guides", href: "/guides" },
						{ label: "API reference", href: "/developers" },
					]}
				/>
			</div>
		</section>
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
		<section
			className={cn(
				CONTAINER,
				"flex flex-col items-center pt-16 pb-24 text-center sm:pt-24 sm:pb-32",
			)}
		>
			<ShieldCheck aria-hidden="true" className="size-6 text-white/80" strokeWidth={1.5} />
			<Display as="h1" className="mt-8" size="xl">
				{title}
			</Display>
			<Lead className="mx-auto mt-6">{lede}</Lead>
			<div className="mt-8 flex flex-wrap justify-center gap-3">
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
		</section>
	);
}
