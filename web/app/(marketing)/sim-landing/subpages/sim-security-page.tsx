import Link from "next/link";

import { cn } from "@/lib/sim/cn";

import { JsonLd, RelatedPages } from "../../marketing-primitives";
import { faqJsonLd } from "../../metadata";
import { SimCtaLink } from "../primitives";
import { HOME_TYPE, LANDING_CONTENT_WIDTH, LANDING_GUTTER } from "../tokens";
import { SimSubpageFaqSection } from "./sim-subpage-faq";

const faqs = [
	{
		question: "Where is verification history stored?",
		answer:
			"The application uses the configured PostgreSQL database. Self-hosted operators control their own database and retention.",
	},
	{
		question: "Does the browser receive backend credentials?",
		answer:
			"No. The web app keeps the backend JWT server-side and sends browser traffic through the same-origin API route.",
	},
	{
		question: "Does a check send an email?",
		answer: "No. Verification checks available signals without sending a message to the recipient.",
	},
] as const;

const rows = [
	{
		title: "Organization scope",
		body: "Checks, lists, history, and backend requests are scoped to the signed-in organization.",
	},
	{
		title: "Server-side backend access",
		body: "The Next.js route sends a short-lived JWT to the Rust API. The browser uses its Better Auth session.",
	},
	{
		title: "Self-hosted data",
		body: "Run the web app, Rust API, and worker in your environment with PostgreSQL and RabbitMQ.",
	},
	{
		title: "Verification limits",
		body: "Some mail servers hide mailbox status or accept all addresses. Review uncertain outcomes.",
	},
	{
		title: "No outbound message",
		body: "A verification check does not send an email to the recipient.",
	},
	{
		title: "Documented API",
		body: "The API contract and generated SDKs live in this repository for review.",
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
		title: "Self-hosting guide",
		href: "/guides/what-stays-on-device",
		body: "Understand where your data lives.",
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
							Workspace access and verification data.
						</h1>
					</div>
					<div className="flex w-[min(28rem,40%)] flex-col items-start max-xl:w-full">
						<p className={cn("max-w-[40ch] text-pretty text-[var(--text-body)]", HOME_TYPE.lead)}>
							The web app keeps backend credentials server-side and scopes requests to your
							organization. Read <Link href="/privacy">Privacy</Link> and{" "}
							<Link href="/responsible-disclosure">responsible disclosure</Link>.
						</p>
						<SimCtaLink className="mt-5" href="/guides/approval-before-send" variant="outline">
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
