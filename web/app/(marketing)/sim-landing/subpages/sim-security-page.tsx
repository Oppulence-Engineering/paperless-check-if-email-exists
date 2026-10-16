import Link from "next/link";

import { cn } from "@/lib/sim/cn";

import { JsonLd, RelatedPages } from "../../marketing-primitives";
import { faqJsonLd } from "../../metadata";
import { SimCtaLink } from "../primitives";
import { HOME_TYPE, LANDING_CONTENT_WIDTH, LANDING_GUTTER } from "../tokens";
import { SimSubpageFaqSection } from "./sim-subpage-faq";

const faqs = [
	{
		question: "Where is my data stored?",
		answer:
			"In our hosted service, with access limited to your organization. If your data must stay on your own servers, you can self-host.",
	},
	{
		question: "Does the browser receive server credentials?",
		answer:
			"No. Requests from the browser go through our own servers, and server credentials never reach the browser.",
	},
	{
		question: "Does a check send an email?",
		answer: "No. A check never delivers a message to the recipient.",
	},
] as const;

const rows = [
	{
		title: "Organization scope",
		body: "Lists, results, history, and API keys belong to your organization. Other organizations cannot see them.",
	},
	{
		title: "No credentials in the browser",
		body: "The browser never receives the credentials our servers use to talk to each other.",
	},
	{
		title: "Scoped API keys",
		body: "Give each API key only the access it needs, and revoke it at any time.",
	},
	{
		title: "Signed webhooks",
		body: "Webhooks we send are signed with your secret. Delivery events from SendGrid, Mailgun, and Amazon SES are signature-checked.",
	},
	{
		title: "Retention you choose",
		body: "Choose how many days results are kept in your workspace.",
	},
	{
		title: "No email sent",
		body: "A check never delivers a message to the recipient.",
	},
] as const;

const trustLinks = [
	{ title: "Privacy", href: "/privacy", body: "How account and verification data are handled." },
	{
		title: "Responsible disclosure",
		href: "/responsible-disclosure",
		body: "Report a vulnerability through the security channel.",
	},
	{
		title: "Self-hosting",
		href: "/self-hosting",
		body: "Run it on your own servers.",
	},
] as const;

/** Full `/security` page — Sim split headline + card grid, no invented compliance marks. */
export function SimSecurityPage() {
	return (
		<>
			<JsonLd data={faqJsonLd([...faqs])} />
			<section
				aria-labelledby="security-page-heading"
				className={cn(
					"flex w-full flex-col gap-16 pb-16 max-sm:gap-10 max-sm:pb-12",
					LANDING_CONTENT_WIDTH,
					LANDING_GUTTER,
				)}
				id="security-page"
			>
				<div className="flex w-full items-start justify-between gap-10 max-sm:gap-5 max-xl:flex-col">
					<div className="flex flex-col gap-3 max-xl:w-full md:w-1/2">
						<p className="text-[12px] text-[var(--text-muted)] uppercase tracking-[0.08em]">
							[security]
						</p>
						<h1
							className={cn("max-w-[16ch] text-balance text-[var(--text-primary)]", HOME_TYPE.h2)}
							id="security-page-heading"
						>
							How we protect your lists.
						</h1>
					</div>
					<div className="flex w-[min(28rem,40%)] flex-col items-start max-xl:w-full">
						<p className={cn("max-w-[40ch] text-pretty text-[var(--text-body)]", HOME_TYPE.lead)}>
							Your lists and results stay inside your organization, and server credentials stay on
							our servers. Read <Link href="/privacy">Privacy</Link> and{" "}
							<Link href="/responsible-disclosure">responsible disclosure</Link>.
						</p>
						<SimCtaLink className="mt-5" href="/guides/reading-results" variant="outline">
							How results work
						</SimCtaLink>
					</div>
				</div>

				<ul className="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3">
					{rows.map((row) => (
						<li key={row.title}>
							<article className="flex h-full flex-col gap-3 rounded-none border border-[var(--border)] bg-[var(--surface-2)] px-6 py-8 max-sm:px-5">
								<h2 className="text-[18px] text-[var(--text-primary)] leading-[1.3]">
									{row.title}
								</h2>
								<p className="text-[15px] text-[var(--text-secondary)] leading-[1.45]">
									{row.body}
								</p>
							</article>
						</li>
					))}
				</ul>

				<div className="flex flex-col gap-6 border-t border-[var(--border)] pt-10">
					<h2 className="text-[20px] text-[var(--text-primary)] leading-[100%] tracking-[-0.02em] lg:text-[24px]">
						Docs to read next
					</h2>
					<ul className="grid grid-cols-3 gap-6 max-sm:grid-cols-1 max-sm:gap-4">
						{trustLinks.map((item) => (
							<li key={item.href}>
								<Link
									className={cn(
										"group flex h-full flex-col items-start bg-[var(--surface-2)] px-8 pt-10 pb-10",
										"rounded-[10px] border border-[var(--border)] max-sm:px-6 max-sm:pb-8",
										"transition-[border-color] duration-200 ease-out hover-hover:hover:border-[var(--text-muted)] motion-reduce:transition-none",
										"focus-visible:outline focus-visible:outline-1 focus-visible:outline-[var(--text-primary)] focus-visible:outline-offset-4",
									)}
									href={item.href}
								>
									<span className="text-[18px] text-[var(--text-primary)] leading-[1.3]">
										{item.title}
									</span>
									<span className="mt-2 max-w-[32ch] text-[15px] text-[var(--text-secondary)] leading-[1.45]">
										{item.body}
									</span>
								</Link>
							</li>
						))}
					</ul>
				</div>

				<SimSubpageFaqSection faqs={[...faqs]} heading="The short versions" />

				<RelatedPages
					items={[
						{
							label: "How results work",
							href: "/guides/approval-before-send",
							description: "The write gate.",
						},
						{
							label: "Responsible disclosure",
							href: "/responsible-disclosure",
							description: "How to report a vulnerability.",
						},
					]}
				/>
			</section>
		</>
	);
}
