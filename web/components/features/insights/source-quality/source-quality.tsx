"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { reportCsv, saveFile } from "@/lib/downloads";
import { useQuery } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";

import { cn } from "@oppulence/ui/lib/utils";

import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { Group, Pill } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1SourceQuality200Response } from "@/lib/api/generated/zod/v1/v1";
import { category, outcomeType, qualityGrade, recommendation } from "@/lib/verification/labels";

import { type SourceQualityPropsFields } from "./source-quality.schema";

/**
 * @oppulence-gen kind=component
 * SourceQuality is a client data component.
 * Shows every quality counter for each list source.
 *
 * It reads GET /v1/sources/quality through the generated SDK and TanStack
 * Query. The API sends the 50 sources with the most invalid rows first.
 * Owned by `source-quality.lit.ts`.
 */
export type SourceQualityProps = SourceQualityPropsFields & ComponentPropsWithoutRef<"section">;

const api = new V1Api(new Configuration({ basePath: "/api/backend" }));

type Counter = keyof typeof V1SourceQuality200Response.shape.sources.element.shape;

/** All counters of source_quality.rs, in groups. The `_pct` values are percentages. */
const GROUPS: { title: string; items: [Counter, string][] }[] = [
	{
		title: "Records",
		items: [
			["total_records", "Total"],
			["processed_records", "Processed"],
			["safe_to_send_count", "Safe to send"],
		],
	},
	{
		title: "Categories",
		items: (["valid", "risky", "unknown", "invalid"] as const).map((value): [Counter, string] => [
			`${value}_count`,
			category.label(value),
		]),
	},
	{
		title: "Recommendations",
		items: (["send", "send_with_caution", "review", "suppress", "drop"] as const).map(
			(value): [Counter, string] => [`${value}_count`, recommendation.label(value)],
		),
	},
	{
		title: "Delivery feedback",
		items: [
			["outcome_count", "All events"],
			...(["delivered", "opened", "clicked", "bounced", "complained", "unsubscribed"] as const).map(
				(value): [Counter, string] => [`${value}_count`, outcomeType.label(value)],
			),
		],
	},
	{
		title: "Rates",
		items: [
			["risky_pct", "Risky"],
			["invalid_pct", "Invalid"],
			["unsafe_recommendation_pct", "Review, suppress, or drop"],
			["negative_outcome_pct", "Negative share of events"],
		],
	},
];

export function SourceQuality({ className, ...props }: SourceQualityProps) {
	const [sourceKey, setSourceKey] = useState("");
	const [minRecords, setMinRecords] = useState(1);
	const [filter, setFilter] = useState({ sourceKey: "", minRecords: 1 });
	const sources = useQuery({
		queryKey: workflowQueryKey("source-quality", filter.sourceKey, filter.minRecords),
		queryFn: async ({ signal }) =>
			V1SourceQuality200Response.parse(
				(
					await api.v1SourceQuality(
						{
							sourceKey: filter.sourceKey || undefined,
							minRecords: filter.minRecords,
							limit: 50,
						},
						{ signal },
					)
				).data,
			),
	});

	return (
		<section data-slot="source-quality" className={cn("flex flex-col gap-4", className)} {...props}>
			<form
				className="flex flex-wrap items-end gap-3"
				onSubmit={(event) => {
					event.preventDefault();
					setFilter({ sourceKey: sourceKey.trim(), minRecords });
				}}
			>
				<div className="flex flex-col gap-2">
					<Label htmlFor="quality-source">Lead source</Label>
					<Input
						id="quality-source"
						value={sourceKey}
						onChange={(event) => {
							setSourceKey(event.target.value);
						}}
						placeholder="All sources"
					/>
				</div>
				<div className="flex flex-col gap-2">
					<Label htmlFor="quality-minimum">Minimum records</Label>
					<Input
						id="quality-minimum"
						type="number"
						min={1}
						step={1}
						required
						value={minRecords}
						onChange={(event) => {
							setMinRecords(event.target.valueAsNumber);
						}}
					/>
				</div>
				<Button type="submit">Apply filters</Button>
				<Button
					type="button"
					disabled={!sources.data?.sources.length || sources.isFetching}
					onClick={() => {
						const rows = sources.data?.sources ?? [];
						const keys: Counter[] = [
							"source_key",
							"quality_grade",
							...GROUPS.flatMap((group) => group.items.map(([key]) => key)),
						];
						saveFile(
							reportCsv([keys, ...rows.map((row) => keys.map((key) => row[key]))]),
							"lead-source-quality.csv",
						);
					}}
				>
					Download report CSV
				</Button>
			</form>
			<p className="text-xs text-[#858585]">
				Showing up to 50 matching sources, with the most invalid records first. Duplicate import
				rows are excluded. Rates use checked records; delivery feedback counts events, not unique
				recipients. Campaign feedback uses the linked verification or latest tagged verification for
				that mailbox. Grades are rules-based summaries, not delivery predictions.
			</p>
			{sources.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(sources.error)}
				</p>
			) : null}
			<Group>
				{sources.data?.sources.length ? (
					sources.data.sources.map((source) => (
						<article className="flex flex-col gap-4 p-5" key={source.source_key}>
							<div className="flex items-start justify-between gap-3">
								<div className="min-w-0">
									<h3 className="text-sm font-medium break-all text-white">{source.source_key}</h3>
									<p className="mt-0.5 text-xs text-[#858585]">{source.summary}</p>
								</div>
								<Pill tone={qualityGrade.tone(source.quality_grade)}>
									{qualityGrade.label(source.quality_grade)}
								</Pill>
							</div>
							<div className="grid grid-cols-2 gap-x-6 gap-y-5 sm:grid-cols-3 lg:grid-cols-5">
								{GROUPS.map((group) => (
									<div key={group.title}>
										<h4 className="text-xs font-medium text-neutral-400">{group.title}</h4>
										<dl className="mt-2 flex flex-col gap-1">
											{group.items.map(([key, label]) => (
												<div className="flex justify-between gap-3 text-sm" key={key}>
													<dt className="text-[#858585]">{label}</dt>
													<dd className="text-white tabular-nums">
														{source[key]}
														{key.endsWith("_pct") ? "%" : ""}
													</dd>
												</div>
											))}
										</dl>
									</div>
								))}
							</div>
						</article>
					))
				) : (
					<p className="p-5 text-sm text-[#858585]">
						{sources.isPending
							? "Loading sources…"
							: "No source quality data yet. Add a source key when you upload a list."}
					</p>
				)}
			</Group>
		</section>
	);
}
