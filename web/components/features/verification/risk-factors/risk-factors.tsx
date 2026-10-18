import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import { bounceRisk, humanize } from "@/lib/verification/labels";

import { type RiskFactorsPropsFields } from "./risk-factors.schema";

/**
 * @oppulence-gen kind=component
 * RiskFactors is a server presentation component.
 * Shows the bounce risk model's factors and the score's confidence factors.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `risk-factors.lit.ts`.
 */
export type RiskFactorsProps = RiskFactorsPropsFields & ComponentPropsWithoutRef<"section">;

export function RiskFactors({ className, result, ...props }: RiskFactorsProps) {
	const risk = result.bounce_risk;
	const factors = risk?.risk_factors ?? [];
	const largest = Math.max(1, ...factors.map((factor) => Math.abs(factor.contribution)));
	const confidenceFactors = result.score.confidence_factors ?? [];

	return (
		<section data-slot="risk-factors" className={cn(className)} {...props}>
			<Group>
				{risk ? (
					<Row
						description={`Model ${risk.model_version}, confidence ${Math.round(risk.confidence * 100)}%`}
						label="Bounce risk"
						layout="inline"
					>
						<Pill tone={bounceRisk.tone(risk.category)}>{bounceRisk.label(risk.category)}</Pill>
						<span className="text-sm text-white">{risk.score}/100</span>
					</Row>
				) : (
					<Row
						description="The bounce risk model gave no score for this check."
						label="Bounce risk"
					/>
				)}
				{factors.length > 0 ? (
					<Row label="What changed the risk" layout="stacked">
						<ul className="flex flex-col gap-3">
							{factors.map((factor) => {
								const lowers = factor.direction === "decreases_risk";
								return (
									<li
										className="flex flex-col gap-1.5"
										key={`${factor.signal}-${factor.description}`}
									>
										<div className="flex items-baseline justify-between gap-4 text-sm">
											<span className="text-neutral-200">{factor.description}</span>
											<span className={lowers ? "text-emerald-300" : "text-red-300"}>
												{lowers ? "Lowers risk" : "Raises risk"} · {Math.abs(factor.contribution)}
											</span>
										</div>
										<div className="h-1 rounded-full bg-white/[0.06]">
											<div
												className={cn(
													"h-1 rounded-full",
													lowers ? "bg-emerald-400/70" : "bg-red-400/70",
												)}
												style={{ width: `${(Math.abs(factor.contribution) / largest) * 100}%` }}
											/>
										</div>
										<span className="text-xs text-[#858585]">{humanize(factor.signal)}</span>
									</li>
								);
							})}
						</ul>
					</Row>
				) : null}
				{confidenceFactors.length > 0 ? (
					<Row label="Why we are this confident" layout="stacked">
						<ul className="list-disc space-y-1 pl-5 text-sm text-neutral-300">
							{confidenceFactors.map((factor) => (
								<li key={factor}>{factor}</li>
							))}
						</ul>
					</Row>
				) : null}
			</Group>
		</section>
	);
}
