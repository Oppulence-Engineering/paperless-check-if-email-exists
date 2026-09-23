"use client";

import { useEffect, useRef, useState } from "react";

import { cn } from "@/lib/sim/cn";

import { HOME_INSET, HOME_TYPE, LANDING_CONTENT_WIDTH, LANDING_GUTTER } from "./tokens";

const COUNT_DURATION_MS = 900;
const METRIC_STAGGER_MS = 120;

interface HonestMetric {
	value: number;
	prefix?: string;
	suffix?: string;
	description: string;
}

const METRICS: HonestMetric[] = [
	{
		value: 5,
		description: "signal groups in a result: syntax, domain, mail server, mailbox, and risk.",
	},
	{
		value: 2,
		description: "ways to work: check one address or upload a list.",
	},
	{
		value: 0,
		suffix: "",
		description: "messages sent to an address just to run a verification check.",
	},
];

/** Sim `AgentMomentumMetrics` with honest Oppulence product constraints instead of invented stats. */
function SimHonestMetrics() {
	const listRef = useRef<HTMLDListElement>(null);
	const hasAnimatedRef = useRef(false);
	const [counts, setCounts] = useState(() => METRICS.map(() => 0));

	useEffect(() => {
		const list = listRef.current;
		if (!list) return;

		const media = window.matchMedia("(prefers-reduced-motion: reduce)");
		let observer: IntersectionObserver | null = null;
		let frame = 0;

		const finishImmediately = () => {
			hasAnimatedRef.current = true;
			setCounts(METRICS.map((metric) => metric.value));
		};

		const animate = () => {
			if (hasAnimatedRef.current) return;
			hasAnimatedRef.current = true;
			const startedAt = performance.now();

			const tick = (now: number) => {
				let complete = true;
				const next = METRICS.map((metric, index) => {
					const elapsed = now - startedAt - index * METRIC_STAGGER_MS;
					const progress = Math.min(Math.max(elapsed / COUNT_DURATION_MS, 0), 1);
					if (progress < 1) complete = false;
					const eased = 1 - (1 - progress) ** 4;
					return Math.round(metric.value * eased);
				});
				setCounts(next);
				if (!complete) frame = requestAnimationFrame(tick);
			};

			frame = requestAnimationFrame(tick);
		};

		if (media.matches) {
			finishImmediately();
		} else if (typeof IntersectionObserver === "undefined") {
			animate();
		} else {
			observer = new IntersectionObserver(
				([entry]) => {
					if (!entry?.isIntersecting) return;
					observer?.disconnect();
					animate();
				},
				{ threshold: 0.25 },
			);
			observer.observe(list);
		}

		return () => {
			observer?.disconnect();
			cancelAnimationFrame(frame);
		};
	}, []);

	return (
		<dl className="max-lg:mt-14 max-lg:border-t max-lg:border-[var(--border)]" ref={listRef}>
			{METRICS.map((metric, index) => {
				const count = counts[index] ?? 0;
				const finalValue = `${metric.prefix ?? ""}${metric.value}${metric.suffix ?? ""}`;
				const formattedValue = `${metric.prefix ?? ""}${count}${metric.suffix ?? ""}`;

				return (
					<div
						className={cn(
							"grid min-h-[190px] grid-cols-[minmax(0,1.35fr)_minmax(10rem,0.65fr)] items-start gap-8 py-8 max-sm:min-h-0 max-sm:grid-cols-1 max-sm:gap-4 max-sm:py-6",
							index > 0 && "border-t border-[var(--border)]",
						)}
						key={metric.description}
					>
						<dt
							className={cn(
								"order-2 max-w-[19rem] text-balance pt-2 text-[var(--text-secondary)] max-sm:order-2 max-sm:pt-0",
								HOME_TYPE.body,
							)}
						>
							{metric.description}
						</dt>
						<dd className="order-1 whitespace-nowrap text-[84px] leading-[0.95] tracking-[-0.045em] text-[var(--text-primary)] max-sm:text-[52px] max-xl:text-[68px]">
							<span className="sr-only">{finalValue}</span>
							<span
								aria-hidden="true"
								className={cn(
									"inline-block origin-left tabular-nums transition-[opacity,transform] duration-500 ease-out motion-reduce:translate-y-0 motion-reduce:scale-100 motion-reduce:opacity-100 motion-reduce:transition-none",
									count > 0
										? "translate-y-0 scale-100 opacity-100"
										: "translate-y-3 scale-95 opacity-0",
								)}
							>
								{formattedValue}
							</span>
						</dd>
					</div>
				);
			})}
		</dl>
	);
}

const explainTerms = [
	{
		chip: "What was entered",
		title: "Address",
		body: "An address can look valid while its domain or mailbox cannot receive mail.",
	},
	{
		chip: "What the servers say",
		title: "Signals",
		body: "Syntax, DNS, MX, and SMTP checks explain what the verifier could observe.",
	},
	{
		chip: "What to do next",
		title: "Result",
		body: "A score and reason codes help you decide whether to keep, review, or remove an address.",
	},
] as const;

const problemRows = [
	{
		title: "The address looked right",
		body: "A typo or missing mail server can still make delivery impossible.",
	},
	{
		title: "The list aged quietly",
		body: "A contact file collected over time can contain addresses that need another review.",
	},
	{
		title: "A label alone is not enough",
		body: "Keep the underlying signals and reason codes with the result.",
	},
	{
		title: "One-off checks get lost",
		body: "A shared workspace keeps lists and verification history together.",
	},
] as const;

/** Sim `AgentMomentum` with honest Oppulence metrics instead of invented stats. */
export function SimAgentMomentum() {
	return (
		<section
			aria-label="Email verification at a glance"
			className="flex w-full flex-col border-b border-[var(--border)]"
			id="agent-momentum"
		>
			<div
				className={cn(
					"grid grid-cols-[minmax(0,0.95fr)_minmax(0,1.05fr)] gap-x-24 max-lg:grid-cols-1 max-lg:gap-x-0",
					LANDING_CONTENT_WIDTH,
					LANDING_GUTTER,
				)}
			>
				<div className="pt-8 pr-12 max-sm:pt-6 max-lg:pr-0">
					<p className="max-w-[40rem] text-pretty text-[30px] leading-[1.3] tracking-[-0.015em] text-[var(--text-primary)] max-sm:text-[22px] max-xl:text-[26px]">
						Check an address before you send. Keep the result and the signals that explain it in one
						workspace.
					</p>
				</div>
				<SimHonestMetrics />
			</div>
		</section>
	);
}

/** Oppulence-specific problem editorial — quiet Sim-style dividers, no card chrome. */
export function SimProblemEditorial() {
	return (
		<section
			className={cn(
				"flex w-full flex-col border-b border-[var(--border)]",
				LANDING_CONTENT_WIDTH,
				LANDING_GUTTER,
			)}
			id="problem"
		>
			<div className={cn(HOME_INSET, "py-16 max-sm:py-10")}>
				<h2 className={cn("max-w-[20ch] text-balance text-[var(--text-primary)]", HOME_TYPE.h2)}>
					A valid-looking address can still fail when you send.
				</h2>
				<p className={cn("mt-6 max-w-[42ch] text-pretty text-[var(--text-body)]", HOME_TYPE.lead)}>
					A typo, missing MX record, disposable domain, or mailbox response can change the result.
					Check If Email Exists makes those signals visible before you rely on the address.
				</p>

				<dl className="mt-12 border-t border-[var(--border)]">
					{problemRows.map((row, index) => (
						<div
							className={cn(
								"grid gap-2 py-6 max-sm:py-5",
								index > 0 && "border-t border-[var(--border)]",
							)}
							key={row.title}
						>
							<dt className="text-[18px] font-medium text-[var(--text-primary)]">{row.title}</dt>
							<dd className="max-w-[42ch] text-[15px] leading-[1.45] text-[var(--text-secondary)]">
								{row.body}
							</dd>
						</div>
					))}
				</dl>

				<div className="mt-12 grid gap-x-12 gap-y-8 border-t border-[var(--border)] pt-12 md:grid-cols-3">
					{explainTerms.map((item) => (
						<article key={item.title}>
							<p className="text-[13px] text-[var(--text-secondary)]">{item.chip}</p>
							<h3 className="mt-2 text-[18px] font-medium text-[var(--text-primary)]">
								{item.title}
							</h3>
							<p className="mt-2 text-[15px] leading-[1.45] text-[var(--text-secondary)]">
								{item.body}
							</p>
						</article>
					))}
				</div>
			</div>
		</section>
	);
}

/** @deprecated Use {@link SimAgentMomentum} + {@link SimProblemEditorial} */
export function SimEditorial() {
	return (
		<>
			<SimAgentMomentum />
			<SimProblemEditorial />
		</>
	);
}
