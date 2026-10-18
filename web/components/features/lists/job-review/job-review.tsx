"use client";

import "client-only";

import type { ComponentPropsWithoutRef, ReactNode } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, JobsApi } from "@oppulence/reacher-sdk";

import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1JobApprovalChecklist200Response } from "@/lib/api/generated/zod/jobs/jobs";
import { category } from "@/lib/verification/labels";

import { type JobReviewPropsFields } from "./job-review.schema";

/**
 * @oppulence-gen kind=component
 * JobReview is a client data component.
 * Shows the send readiness checklist for one bulk job.
 *
 * It reads the approval checklist through the generated SDK and TanStack
 * Query. It polls every 5 seconds while rows are not processed yet.
 * Owned by `job-review.lit.ts`.
 */
export type JobReviewProps = JobReviewPropsFields & ComponentPropsWithoutRef<"section">;

const jobsApi = new JobsApi(new Configuration({ basePath: "/api/backend" }));

const RISK_FLAGS = {
	disposable_count: "Disposable",
	catch_all_count: "Catch-all",
	role_account_count: "Role address",
	spam_trap_count: "Spam trap",
	suppressed_count: "Suppressed",
} as const;

/** One number with its label. The panels use the same tile. */
export function Stat({ label, value }: { label: string; value: ReactNode }) {
	return (
		<div className="rounded-xl bg-white/[0.02] p-3 ring-1 ring-white/[0.06]">
			<p className="text-xs text-[#858585]">{label}</p>
			<p className="mt-1 text-xl font-medium text-white">{value}</p>
		</div>
	);
}

export function JobReview({ className, jobId, ...props }: JobReviewProps) {
	const approval = useQuery({
		queryKey: workflowQueryKey("jobs", jobId, "approval"),
		queryFn: async () =>
			V1JobApprovalChecklist200Response.parse(
				(await jobsApi.v1JobApprovalChecklist({ jobId })).data,
			),
		refetchInterval: (query) => (query.state.data?.categories.unprocessed ? 5_000 : false),
	});
	const data = approval.data;

	return (
		<section data-slot="job-review" className={cn("flex flex-col gap-4", className)} {...props}>
			<h2 className="text-lg font-medium text-white">Send readiness</h2>
			{approval.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{approval.error.message}
				</p>
			) : null}
			{data ? (
				<Group>
					<Row
						description={`${String(data.safe_to_send_count)} of ${String(data.total_records)} addresses are safe to send (${String(data.safe_to_send_pct)}%).`}
						label={data.recommendation}
					>
						<Pill tone={data.ready_to_send ? "green" : "yellow"}>
							{data.ready_to_send ? "Ready to send" : "Review before sending"}
						</Pill>
					</Row>
					<Row>
						<h3 className="text-sm font-medium text-white">Categories</h3>
						<div className="grid grid-cols-2 gap-3 sm:grid-cols-5">
							{(["valid", "risky", "unknown", "invalid"] as const).map((key) => (
								<Stat key={key} label={category.label(key)} value={data.categories[key]} />
							))}
							<Stat label="Not checked yet" value={data.categories.unprocessed} />
						</div>
					</Row>
					<Row>
						<h3 className="text-sm font-medium text-white">Risk flags</h3>
						<div className="grid grid-cols-2 gap-3 sm:grid-cols-5">
							{Object.entries(RISK_FLAGS).map(([key, label]) => (
								<Stat
									key={key}
									label={label}
									value={data.risk_flags[key as keyof typeof RISK_FLAGS]}
								/>
							))}
						</div>
					</Row>
				</Group>
			) : null}
		</section>
	);
}
