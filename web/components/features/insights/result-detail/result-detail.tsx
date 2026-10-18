"use client";

import "client-only";

import type { ComponentPropsWithoutRef } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, JobsApi } from "@oppulence/reacher-sdk";

import { cn } from "@oppulence/ui/lib/utils";

import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { AddressTimeline } from "@/components/features/verification/address-timeline/address-timeline";
import { ProviderDetails } from "@/components/features/verification/provider-details/provider-details";
import { RiskFactors } from "@/components/features/verification/risk-factors/risk-factors";
import { SignalChecklist } from "@/components/features/verification/signal-checklist/signal-checklist";
import { TechnicalDetails } from "@/components/features/verification/technical-details/technical-details";
import { VerdictSummary } from "@/components/features/verification/verdict-summary/verdict-summary";
import { Group, Pill, Row } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1GetJobResults200Response } from "@/lib/api/generated/zod/jobs/jobs";
import { parsePolicyEvaluation, parseRecommendation } from "@/lib/api/schemas/decision";
import {
	labelFor,
	POLICY_MODE_LABELS,
	policyDecision,
	reasonSeverity,
	recommendation,
	taskState,
} from "@/lib/verification/labels";

import { type ResultDetailPropsFields } from "./result-detail.schema";

/**
 * @oppulence-gen kind=component
 * ResultDetail is a client data component.
 * Shows the full verdict of one result from the result explorer.
 *
 * GET /v1/query does not send the full check result. For a job row, this part
 * reads that one row from GET /v1/jobs/{job_id}/results: the cursor is the
 * row before it. A single check has no job, so the part shows the decision
 * from the query row only. Owned by `result-detail.lit.ts`.
 */
export type ResultDetailProps = ResultDetailPropsFields & ComponentPropsWithoutRef<"section">;

const jobsApi = new JobsApi(new Configuration({ basePath: "/api/backend" }));

export function ResultDetail({ className, row, ...props }: ResultDetailProps) {
	const jobId = row.job_id ?? undefined;
	const full = useQuery({
		queryKey: workflowQueryKey("jobs", jobId, "result", row.id),
		enabled: jobId !== undefined,
		queryFn: async () => {
			const page = V1GetJobResults200Response.parse(
				(
					await jobsApi.v1GetJobResults({
						jobId: jobId ?? 0,
						cursor: row.id - 1,
						limit: 1,
						state: row.task_state,
					})
				).data,
			);
			return page.results.find((item) => item.id === row.id) ?? null;
		},
	});
	const result = full.data?.result;
	const decision = parseRecommendation(row.recommendation);
	const policy = parsePolicyEvaluation(row.policy_evaluation);

	return (
		<section data-slot="result-detail" className={cn("flex flex-col gap-6", className)} {...props}>
			{full.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(full.error)}
				</p>
			) : null}
			{result ? (
				<>
					<VerdictSummary result={result} />
					<RiskFactors result={result} />
					<SignalChecklist result={result} />
					<ProviderDetails result={result} />
					<TechnicalDetails result={result} />
				</>
			) : full.isLoading ? (
				<p className="text-sm text-[#858585]">Loading the full result…</p>
			) : (
				<Group>
					<Row>
						<p className="text-sm text-[#858585]">
							{jobId === undefined
								? "This was a single check, so only its decision is kept here. The history below shows each check of this address."
								: (full.data?.error ??
									`This row has no result. State: ${taskState.label(row.task_state)}.`)}
						</p>
					</Row>
					{decision || row.recommendation_action ? (
						<Row label="Recommendation" layout="stacked">
							<div className="flex flex-col gap-2">
								<Pill tone={recommendation.tone(decision?.action ?? row.recommendation_action)}>
									{recommendation.label(decision?.action ?? row.recommendation_action)}
								</Pill>
								{decision?.summary ? (
									<p className="text-sm text-[#858585]">{decision.summary}</p>
								) : null}
								<Reasons reasons={decision?.reasons ?? []} />
							</div>
						</Row>
					) : null}
					{policy || row.policy_decision ? (
						<Row label="Policy" layout="stacked">
							<div className="flex flex-col gap-2">
								<p className="flex flex-wrap items-center gap-2 text-sm text-white">
									<Pill tone={policyDecision.tone(policy?.decision ?? row.policy_decision)}>
										{policyDecision.label(policy?.decision ?? row.policy_decision)}
									</Pill>
									<span className="text-[#858585]">
										{labelFor(POLICY_MODE_LABELS, policy?.mode ?? row.policy_mode)}
									</span>
								</p>
								<Reasons reasons={policy?.reasons ?? []} />
							</div>
						</Row>
					) : null}
				</Group>
			)}
			{row.email ? <AddressTimeline email={row.email} /> : null}
		</section>
	);
}

function Reasons({ reasons }: { reasons: { code: string; severity: string; message: string }[] }) {
	if (!reasons.length) return null;
	return (
		<ul className="flex flex-col gap-2">
			{reasons.map((reason) => (
				<li className="flex items-start gap-2 text-sm text-neutral-300" key={reason.code}>
					<Pill tone={reasonSeverity.tone(reason.severity)}>
						{reasonSeverity.label(reason.severity)}
					</Pill>
					<span>{reason.message}</span>
				</li>
			))}
		</ul>
	);
}
