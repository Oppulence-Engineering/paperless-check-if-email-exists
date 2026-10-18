import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import { primaryAction, recommendationOf, suggestedEmail } from "@/lib/verification/check-result";
import {
	bounceRisk,
	category,
	confidenceLevel,
	freshness,
	REASON_LABELS,
	labelFor,
	reachability,
	reasonSeverity,
	recommendation,
} from "@/lib/verification/labels";

import { type VerdictSummaryPropsFields } from "./verdict-summary.schema";

/**
 * @oppulence-gen kind=component
 * VerdictSummary is a server presentation component.
 * Shows the verdict for one checked address: the action, why, and the key numbers.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `verdict-summary.lit.ts`.
 */
export type VerdictSummaryProps = VerdictSummaryPropsFields & ComponentPropsWithoutRef<"section">;

function Stat({ label, children }: { label: string; children: React.ReactNode }) {
	return (
		<div>
			<dt className="text-xs text-[#858585]">{label}</dt>
			<dd className="mt-1 flex items-center gap-2 text-sm font-medium text-white">{children}</dd>
		</div>
	);
}

export function VerdictSummary({ className, result, ...props }: VerdictSummaryProps) {
	const decision = recommendationOf(result);
	const action = primaryAction(result);
	const suggestion = suggestedEmail(result);
	const { score } = result;
	const reasons = decision?.reasons.length
		? decision.reasons.map((reason) => ({
				key: reason.code,
				text: reason.message,
				tone: reasonSeverity.tone(reason.severity),
				tag: reasonSeverity.label(reason.severity),
			}))
		: score.reason_codes.map((code) => ({
				key: code,
				text: labelFor(REASON_LABELS, code),
				tone: undefined,
				tag: undefined,
			}));

	return (
		<section data-slot="verdict-summary" className={cn(className)} {...props}>
			<Group>
				<Row>
					<div className="flex flex-col items-start gap-3 sm:flex-row sm:justify-between">
						<div className="min-w-0">
							<p className="text-xs text-[#858585]">Recommendation</p>
							<p className="mt-1 text-lg font-medium text-white">{recommendation.label(action)}</p>
							{decision?.summary ? (
								<p className="mt-1 text-sm text-[#858585]">{decision.summary}</p>
							) : null}
							{suggestion ? (
								<p className="mt-2 text-sm text-neutral-300">
									Did you mean{" "}
									<Link
										className="font-medium text-white underline underline-offset-4"
										href={`/app/check?email=${encodeURIComponent(suggestion)}`}
									>
										{suggestion}
									</Link>
									?
								</p>
							) : null}
						</div>
						<Pill tone={recommendation.tone(action)}>{recommendation.label(action)}</Pill>
					</div>
				</Row>
				<Row>
					<dl className="grid grid-cols-2 gap-5 sm:grid-cols-3 lg:grid-cols-4">
						<Stat label="Quality score">
							{score.score}/100
							<Pill tone={category.tone(score.category)}>{category.label(score.category)}</Pill>
						</Stat>
						<Stat label="Safe to send">{score.safe_to_send ? "Yes" : "No"}</Stat>
						<Stat label="Reachability">
							<Pill tone={reachability.tone(result.is_reachable)}>
								{reachability.label(result.is_reachable)}
							</Pill>
						</Stat>
						{result.bounce_risk ? (
							<Stat label="Bounce risk">
								<Pill tone={bounceRisk.tone(result.bounce_risk.category)}>
									{bounceRisk.label(result.bounce_risk.category)}
								</Pill>
								<span className="text-[#858585]">{result.bounce_risk.score}/100</span>
							</Stat>
						) : null}
						{score.confidence !== undefined ? (
							<Stat label="Confidence">
								{score.confidence}/100
								{score.confidence_level ? (
									<span className="text-[#858585]">
										{confidenceLevel.label(score.confidence_level)}
									</span>
								) : null}
							</Stat>
						) : null}
						{score.freshness ? (
							<Stat label="Result age">
								<Pill tone={freshness.tone(score.freshness)}>
									{freshness.label(score.freshness)}
								</Pill>
								{score.age_days !== undefined ? (
									<span className="text-[#858585]">
										{score.age_days === 0 ? "today" : `${score.age_days} days`}
									</span>
								) : null}
							</Stat>
						) : null}
					</dl>
				</Row>
				{reasons.length > 0 ? (
					<Row label="Why" layout="stacked">
						<ul className="flex flex-col gap-2">
							{reasons.map((reason) => (
								<li className="flex items-start gap-2 text-sm text-neutral-300" key={reason.key}>
									{reason.tag ? <Pill tone={reason.tone}>{reason.tag}</Pill> : null}
									<span>{reason.text}</span>
								</li>
							))}
						</ul>
					</Row>
				) : null}
			</Group>
		</section>
	);
}
