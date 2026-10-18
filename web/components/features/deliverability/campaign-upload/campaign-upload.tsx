"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Configuration, OutcomesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import { Group } from "@/components/platform/page-kit";
import { CAMPAIGN_TYPES } from "@/components/features/deliverability/campaign-outcome-list/campaign-outcome-list";
import { Stat } from "@/components/features/lists/job-review/job-review";
import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1UploadCampaignOutcomes202Response } from "@/lib/api/generated/zod/outcomes/outcomes";

import { type CampaignUploadPropsFields } from "./campaign-upload.schema";

/**
 * @oppulence-gen kind=component
 * CampaignUpload is a client data component.
 * Uploads a CSV file of campaign results to POST /v1/campaign-outcomes/upload.
 *
 * It shows the accepted, rejected, and suppressed counts and the error of
 * each rejected row. Owned by `campaign-upload.lit.ts`.
 */
export type CampaignUploadProps = CampaignUploadPropsFields & ComponentPropsWithoutRef<"section">;

const api = new OutcomesApi(new Configuration({ basePath: "/api/backend" }));

export function CampaignUpload({ className, ...props }: CampaignUploadProps) {
	const client = useQueryClient();
	const [file, setFile] = useState<File>();
	const upload = useMutation({
		mutationFn: async (csv: File) => {
			return V1UploadCampaignOutcomes202Response.parse(
				(await api.v1UploadCampaignOutcomes({ file: csv })).data,
			);
		},
		onSuccess: async () => {
			// The first upload creates the default outcome policy.
			await Promise.all([
				client.invalidateQueries({ queryKey: workflowQueryKey("campaign-outcomes") }),
				client.invalidateQueries({ queryKey: workflowQueryKey("outcome-policies") }),
			]);
		},
	});
	const result = upload.data;

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		if (file) upload.mutate(file);
	}

	return (
		<section
			data-slot="campaign-upload"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			<form className="flex flex-col gap-4" onSubmit={submit}>
				<div className="flex flex-col gap-1.5">
					<Label className="text-xs font-medium text-neutral-400" htmlFor="campaign-file">
						CSV file
					</Label>
					<Input
						accept=".csv,text/csv"
						className="h-10 rounded-lg"
						id="campaign-file"
						onChange={(event) => {
							setFile(event.target.files?.[0]);
						}}
						type="file"
					/>
				</div>
				<ul className="flex list-disc flex-col gap-1 pl-5 text-xs text-[#858585]">
					<li>
						Use the columns email, type, and occurred_at. The columns source and campaign_id are
						optional.
					</li>
					<li>Use one of these types: {CAMPAIGN_TYPES.join(", ")}.</li>
					<li>
						Write occurred_at as a date and time with a time zone, for example 2026-10-01T09:30:00Z.
					</li>
					<li>The default outcome policy decides which addresses go on the suppression list.</li>
				</ul>
				<div className="flex justify-end">
					<Button className="rounded-lg" disabled={upload.isPending || !file} type="submit">
						{upload.isPending ? "Uploading…" : "Upload results"}
					</Button>
				</div>
			</form>
			{upload.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(upload.error)}
				</p>
			) : null}
			{result ? (
				<div className="flex flex-col gap-4" role="status">
					<div className="grid grid-cols-3 gap-3">
						<Stat label="Accepted" value={result.accepted} />
						<Stat label="Rejected" value={result.rejected} />
						<Stat label="Suppressed" value={result.suppressed} />
					</div>
					<p className="text-xs text-[#858585]">Applied outcome policy {result.policy_id}.</p>
					{result.errors.length ? (
						<Group>
							{result.errors.map((item) => (
								<div
									className="flex flex-col gap-1 px-5 py-3 text-sm"
									key={`${String(item.index)}-${item.email}-${item.message}`}
								>
									<span className="text-white">
										Row {item.index + 1}
										{item.email ? ` · ${item.email}` : ""}
									</span>
									<span className="text-xs text-red-300">{item.message}</span>
								</div>
							))}
						</Group>
					) : null}
				</div>
			) : null}
		</section>
	);
}
