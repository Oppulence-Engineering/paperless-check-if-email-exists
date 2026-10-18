"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useQuery } from "@tanstack/react-query";
import { Configuration, EventsApi, QueryApi, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";

import { type AnalyticsPanelPropsFields } from "./analytics-panel.schema";

/** @oppulence-gen kind=component Owned by `analytics-panel.lit.ts`. */
export type AnalyticsPanelProps = AnalyticsPanelPropsFields & ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const queryApi = new QueryApi(config);
const eventsApi = new EventsApi(config);
const v1Api = new V1Api(config);
const resultsSchema = z.object({
	results: z.array(
		z.object({
			id: z.number(),
			job_id: z.number().nullable().optional(),
			email: z.string().nullable().optional(),
			score: z.number().nullable().optional(),
			category: z.string().nullable().optional(),
			safe_to_send: z.boolean().nullable().optional(),
			task_state: z.string(),
		}),
	),
	total: z.number(),
});
const eventsSchema = z.object({
	events: z.array(
		z.object({
			id: z.number(),
			job_id: z.number(),
			event_type: z.string(),
			actor: z.string().nullable().optional(),
			created_at: z.string(),
		}),
	),
	total: z.number(),
});
const sourcesSchema = z.object({
	sources: z.array(
		z.object({
			source_key: z.string(),
			quality_grade: z.string(),
			total_records: z.number(),
			safe_to_send_count: z.number(),
			negative_outcome_pct: z.number(),
			summary: z.string(),
		}),
	),
});

export function AnalyticsPanel({ className, ...props }: AnalyticsPanelProps) {
	const [category, setCategory] = useState("");
	const [safe, setSafe] = useState("");
	const [resultOffset, setResultOffset] = useState(0);
	const [eventOffset, setEventOffset] = useState(0);
	const [jobInput, setJobInput] = useState("");
	const [jobFilter, setJobFilter] = useState<number>();
	const results = useQuery({
		queryKey: workflowQueryKey("analytics-results", category, safe, resultOffset),
		queryFn: async () =>
			resultsSchema.parse(
				(
					await queryApi.v1QueryResults({
						limit: 20,
						offset: resultOffset,
						category: category || undefined,
						safeToSend: safe ? safe === "yes" : undefined,
					})
				).data,
			),
	});
	const events = useQuery({
		queryKey: workflowQueryKey("analytics-events", jobFilter, eventOffset),
		queryFn: async () =>
			eventsSchema.parse(
				(await eventsApi.v1ListEvents({ limit: 20, offset: eventOffset, jobId: jobFilter })).data,
			),
	});
	const sources = useQuery({
		queryKey: workflowQueryKey("source-quality"),
		queryFn: async () => sourcesSchema.parse((await v1Api.v1SourceQuality()).data),
	});
	const error = [results, events, sources].find((request) => request.isError)?.error;
	const filter =
		"h-8 rounded-lg bg-white/[0.03] px-2 text-sm text-neutral-200 ring-1 ring-white/[0.08] outline-none";
	return (
		<Page
			description="Explore results, job activity, and source quality."
			title="Verification analytics"
		>
			<section
				data-slot="analytics-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				{error ? (
					<p
						role="alert"
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					>
						{error.message}
					</p>
				) : null}
				<Section title="Result explorer">
					<div className="flex flex-wrap items-center gap-4">
						<div className="flex items-center gap-2 text-sm text-[#858585]">
							<Label htmlFor="analytics-category">Category</Label>
							<select
								id="analytics-category"
								className={filter}
								value={category}
								onChange={(event) => {
									setResultOffset(0);
									setCategory(event.target.value);
								}}
							>
								<option value="">All</option>
								<option value="valid">Valid</option>
								<option value="risky">Risky</option>
								<option value="unknown">Unknown</option>
								<option value="invalid">Invalid</option>
							</select>
						</div>
						<div className="flex items-center gap-2 text-sm text-[#858585]">
							<Label htmlFor="analytics-safe">Safe to send</Label>
							<select
								id="analytics-safe"
								className={filter}
								value={safe}
								onChange={(event) => {
									setResultOffset(0);
									setSafe(event.target.value);
								}}
							>
								<option value="">Any</option>
								<option value="yes">Yes</option>
								<option value="no">No</option>
							</select>
						</div>
					</div>
					<Group>
						<Row>
							{results.data?.results.length ? (
								<div className="overflow-x-auto">
									<table className="w-full text-left text-sm">
										<thead>
											<tr className="border-b border-border">
												<th className="py-2 pr-4 font-medium text-foreground">Address</th>
												<th className="py-2 pr-4 font-medium text-foreground">Category</th>
												<th className="py-2 pr-4 font-medium text-foreground">Score</th>
												<th className="py-2 font-medium text-foreground">Job</th>
											</tr>
										</thead>
										<tbody>
											{results.data.results.map((row) => (
												<tr className="border-b border-border/60" key={row.id}>
													<td className="py-2 pr-4 text-foreground">{row.email || "—"}</td>
													<td className="py-2 pr-4 text-muted-foreground">
														{row.category || row.task_state}
													</td>
													<td className="py-2 pr-4 text-muted-foreground">{row.score ?? "—"}</td>
													<td className="py-2 text-muted-foreground">
														{row.job_id ? (
															<Link
																className="underline"
																href={`/app/jobs?job=${String(row.job_id)}`}
															>
																{row.job_id}
															</Link>
														) : (
															"—"
														)}
													</td>
												</tr>
											))}
										</tbody>
									</table>
								</div>
							) : (
								<p className="text-sm text-[#858585]">No results match these filters.</p>
							)}
						</Row>
					</Group>
					<div className="flex justify-end gap-2">
						<Button
							className="rounded-lg"
							size="sm"
							variant="outline"
							disabled={resultOffset === 0}
							onClick={() => {
								setResultOffset(Math.max(0, resultOffset - 20));
							}}
						>
							Previous
						</Button>
						<Button
							className="rounded-lg"
							size="sm"
							variant="outline"
							disabled={!results.data || resultOffset + 20 >= results.data.total}
							onClick={() => {
								setResultOffset(resultOffset + 20);
							}}
						>
							Next
						</Button>
					</div>
				</Section>
				<Section title="Job activity">
					<form
						className="flex gap-2"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							const id = Number(jobInput);
							setJobFilter(Number.isSafeInteger(id) && id > 0 ? id : undefined);
							setEventOffset(0);
						}}
					>
						<Label className="sr-only" htmlFor="analytics-job">
							Job ID
						</Label>
						<Input
							className="h-10 rounded-lg"
							id="analytics-job"
							inputMode="numeric"
							placeholder="Filter by job ID"
							value={jobInput}
							onChange={(event) => {
								setJobInput(event.target.value);
							}}
						/>
						<Button className="h-10 rounded-lg" type="submit" variant="outline">
							Filter
						</Button>
					</form>
					<Group>
						{events.data?.events.length ? (
							events.data.events.map((event) => (
								<div className="px-5 py-4 text-sm text-white" key={event.id}>
									{event.event_type}{" "}
									<span className="text-xs text-[#858585]">
										·{" "}
										<Link
											className="underline hover:text-neutral-200"
											href={`/app/jobs?job=${String(event.job_id)}`}
										>
											Job {String(event.job_id)}
										</Link>{" "}
										· {new Date(event.created_at).toLocaleString()}
									</span>
								</div>
							))
						) : (
							<Row>
								<p className="text-sm text-[#858585]">No activity found.</p>
							</Row>
						)}
					</Group>
					<div className="flex justify-end gap-2">
						<Button
							className="rounded-lg"
							size="sm"
							variant="outline"
							disabled={eventOffset === 0}
							onClick={() => {
								setEventOffset(Math.max(0, eventOffset - 20));
							}}
						>
							Previous
						</Button>
						<Button
							className="rounded-lg"
							size="sm"
							variant="outline"
							disabled={!events.data || eventOffset + 20 >= events.data.total}
							onClick={() => {
								setEventOffset(eventOffset + 20);
							}}
						>
							Next
						</Button>
					</div>
				</Section>
				<Section title="Source quality">
					<Group>
						{sources.data?.sources.length ? (
							sources.data.sources.map((source) => (
								<div
									className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
									key={source.source_key}
								>
									<div className="min-w-0 flex-1">
										<p className="truncate text-sm text-white">{source.source_key}</p>
										<p className="text-xs text-[#858585]">
											{source.safe_to_send_count} safe of {source.total_records} · Negative outcomes{" "}
											{source.negative_outcome_pct}%
										</p>
										<p className="mt-0.5 text-xs text-[#858585]">{source.summary}</p>
									</div>
									<Pill>Grade {source.quality_grade}</Pill>
								</div>
							))
						) : (
							<Row>
								<p className="text-sm text-[#858585]">No source quality data yet.</p>
							</Row>
						)}
					</Group>
				</Section>
			</section>
		</Page>
	);
}
