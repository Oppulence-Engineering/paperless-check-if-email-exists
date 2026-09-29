"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery } from "@tanstack/react-query";
import { Configuration, EventsApi, QueryApi, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
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
	const [domain, setDomain] = useState("");
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
	const reputation = useMutation({
		mutationFn: async () =>
			(await v1Api.v1CheckReputation({ reputationCheckRequest: { domain: domain.trim() } })).data,
	});
	const error = [results, events, sources, reputation].find((request) => request.isError)?.error;
	return (
		<section data-slot="analytics-panel" className={cn("space-y-6", className)} {...props}>
			<header>
				<h1 className="text-3xl font-semibold">Verification analytics</h1>
				<p className="mt-2 text-muted-foreground">
					Explore results, job activity, source quality, and domain risk.
				</p>
			</header>
			{error ? (
				<p role="alert" className="text-destructive">
					{error.message}
				</p>
			) : null}
			<Card>
				<CardHeader>
					<CardTitle>Result explorer</CardTitle>
				</CardHeader>
				<CardContent className="space-y-4">
					<div className="flex flex-wrap gap-3">
						<div>
							<Label htmlFor="analytics-category">Category</Label>
							<select
								id="analytics-category"
								className="h-9 rounded border bg-background px-3"
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
						<div>
							<Label htmlFor="analytics-safe">Safe to send</Label>
							<select
								id="analytics-safe"
								className="h-9 rounded border bg-background px-3"
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
					{results.data?.results.length ? (
						<div className="overflow-x-auto">
							<table className="w-full text-sm">
								<thead>
									<tr>
										<th className="p-2 text-left">Address</th>
										<th className="p-2 text-left">Category</th>
										<th className="p-2 text-left">Score</th>
										<th className="p-2 text-left">Job</th>
									</tr>
								</thead>
								<tbody>
									{results.data.results.map((row) => (
										<tr className="border-t" key={row.id}>
											<td className="p-2">{row.email || "—"}</td>
											<td className="p-2">{row.category || row.task_state}</td>
											<td className="p-2">{row.score ?? "—"}</td>
											<td className="p-2">
												{row.job_id ? (
													<Link className="underline" href={`/app/jobs?job=${String(row.job_id)}`}>
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
						<p>No results match these filters.</p>
					)}
					<div className="flex gap-2">
						<Button
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
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Job activity</CardTitle>
				</CardHeader>
				<CardContent className="space-y-4">
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
							id="analytics-job"
							inputMode="numeric"
							placeholder="Filter by job ID"
							value={jobInput}
							onChange={(event) => {
								setJobInput(event.target.value);
							}}
						/>
						<Button type="submit" variant="outline">
							Filter
						</Button>
					</form>
					{events.data?.events.length ? (
						<ol className="space-y-2">
							{events.data.events.map((event) => (
								<li className="rounded border p-3 text-sm" key={event.id}>
									{event.event_type} ·{" "}
									<Link className="underline" href={`/app/jobs?job=${String(event.job_id)}`}>
										Job {String(event.job_id)}
									</Link>{" "}
									· {new Date(event.created_at).toLocaleString()}
								</li>
							))}
						</ol>
					) : (
						<p>No activity found.</p>
					)}
					<div className="flex gap-2">
						<Button
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
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Source quality</CardTitle>
				</CardHeader>
				<CardContent>
					{sources.data?.sources.length ? (
						<ul className="space-y-2">
							{sources.data.sources.map((source) => (
								<li className="rounded border p-3" key={source.source_key}>
									<p className="font-medium">
										{source.source_key} · Grade {source.quality_grade}
									</p>
									<p className="text-sm">
										{source.safe_to_send_count} safe of {source.total_records} · Negative outcomes{" "}
										{source.negative_outcome_pct}%
									</p>
									<p className="text-sm text-muted-foreground">{source.summary}</p>
								</li>
							))}
						</ul>
					) : (
						<p>No source quality data yet.</p>
					)}
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Domain reputation</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					<form
						className="flex gap-2"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							reputation.mutate();
						}}
					>
						<Label className="sr-only" htmlFor="reputation-domain">
							Domain
						</Label>
						<Input
							id="reputation-domain"
							required
							placeholder="example.com"
							value={domain}
							onChange={(event) => {
								setDomain(event.target.value);
							}}
						/>
						<Button type="submit" disabled={reputation.isPending}>
							Check domain
						</Button>
					</form>
					{reputation.data ? (
						<div className="rounded border p-4">
							<p className="font-semibold">
								{reputation.data.domain} · {reputation.data.risk_level}
							</p>
							<p>
								Reputation score {reputation.data.score} ·{" "}
								{reputation.data.cached ? "Cached" : "Fresh"}
							</p>
							<p>{reputation.data.blacklist_results.length} blacklist checks</p>
						</div>
					) : null}
				</CardContent>
			</Card>
		</section>
	);
}
