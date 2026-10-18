import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import { policyEvaluationOf } from "@/lib/verification/check-result";
import {
	confidenceFactor,
	labelFor,
	POLICY_MODE_LABELS,
	policyDecision,
	PROVIDER_LABELS,
	providerConfidence,
	REASON_LABELS,
	reasonSeverity,
	SMTP_UNCERTAINTY_LABELS,
} from "@/lib/verification/labels";

import { type ProviderDetailsPropsFields } from "./provider-details.schema";

/**
 * @oppulence-gen kind=component
 * ProviderDetails is a server presentation component.
 * Shows the mailbox provider, the provider rules, the workspace policy result,
 * and why a check could not finish.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `provider-details.lit.ts`.
 */
export type ProviderDetailsProps = ProviderDetailsPropsFields & ComponentPropsWithoutRef<"section">;

export function ProviderDetails({ className, result, ...props }: ProviderDetailsProps) {
	const policy = policyEvaluationOf(result);
	const partial = result.score.partial_confidence;

	return (
		<section data-slot="provider-details" className={cn(className)} {...props}>
			<Group>
				<Row
					description={
						result.provider_rules_applied
							? "This provider's address rules were applied."
							: "No provider address rules were applied."
					}
					label="Mailbox provider"
					layout="inline"
				>
					<span className="text-sm text-white">
						{result.provider ? labelFor(PROVIDER_LABELS, result.provider) : "Not recognized"}
					</span>
					{result.provider_confidence ? (
						<Pill tone={providerConfidence.tone(result.provider_confidence)}>
							{providerConfidence.label(result.provider_confidence)} confidence
						</Pill>
					) : null}
				</Row>
				{result.provider_rejection_reason ? (
					<Row
						description={labelFor(REASON_LABELS, result.provider_rejection_reason)}
						label="Rejected by provider rules"
					/>
				) : null}
				{partial ? (
					<Row
						description={[
							`Confidence ${partial.confidence}/100`,
							...partial.factors.map(confidenceFactor),
						].join(". ")}
						label="Why the check could not finish"
						layout="stacked"
					>
						<span className="text-sm text-white">
							{labelFor(SMTP_UNCERTAINTY_LABELS, partial.classification)}
						</span>
					</Row>
				) : null}
				{policy ? (
					<Row
						description={`${labelFor(POLICY_MODE_LABELS, policy.mode)} policy${
							policy.result_age_days != null ? `, result ${policy.result_age_days} days old` : ""
						}`}
						label="Workspace policy"
						layout="stacked"
					>
						<div className="flex flex-col items-start gap-2">
							<Pill tone={policyDecision.tone(policy.decision)}>
								{policyDecision.label(policy.decision)}
							</Pill>
							{policy.reasons.length > 0 ? (
								<ul className="flex flex-col gap-1.5">
									{policy.reasons.map((reason) => (
										<li
											className="flex items-start gap-2 text-sm text-neutral-300"
											key={reason.code}
										>
											<Pill tone={reasonSeverity.tone(reason.severity)}>
												{reasonSeverity.label(reason.severity)}
											</Pill>
											<span>{reason.message}</span>
										</li>
									))}
								</ul>
							) : null}
						</div>
					</Row>
				) : null}
			</Group>
		</section>
	);
}
