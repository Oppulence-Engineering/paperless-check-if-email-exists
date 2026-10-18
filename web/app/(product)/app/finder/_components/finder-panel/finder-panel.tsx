"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Row, Section } from "@/components/platform/page-kit";

import { type FinderPanelPropsFields } from "./finder-panel.schema";

/** @oppulence-gen kind=component Owned by `finder-panel.lit.ts`. */
export type FinderPanelProps = FinderPanelPropsFields & ComponentPropsWithoutRef<"section">;

const api = new V1Api(new Configuration({ basePath: "/api/backend" }));
const createSchema = z.object({ job_id: z.number(), status: z.string() });
const resultSchema = z.object({
	status: z.string(),
	domain_has_mx: z.boolean(),
	domain_is_catch_all: z.boolean(),
	candidates_checked: z.number(),
	best_match: z.object({ email: z.string(), score: z.number(), confidence: z.string() }).nullable(),
	results: z.array(
		z.object({
			email: z.string(),
			score: z.number(),
			category: z.string(),
			pattern: z.string(),
			sub_reason: z.string(),
		}),
	),
});

export function FinderPanel({ initialJobId, className, ...props }: FinderPanelProps) {
	const [first, setFirst] = useState("");
	const [last, setLast] = useState("");
	const [domain, setDomain] = useState("");
	const [strategy, setStrategy] = useState<"parallel" | "waterfall">("parallel");
	const [jobId, setJobId] = useState(initialJobId);
	const [lookup, setLookup] = useState(initialJobId ? String(initialJobId) : "");
	const [validation, setValidation] = useState("");
	const create = useMutation({
		mutationFn: async () =>
			createSchema.parse(
				(
					await api.v1FindEmail({
						findEmailRequest: {
							first_name: first.trim(),
							last_name: last.trim(),
							domain: domain.trim(),
							strategy,
						},
					})
				).data,
			),
		onSuccess: (data) => {
			setJobId(data.job_id);
			setLookup(String(data.job_id));
		},
	});
	const result = useQuery({
		queryKey: workflowQueryKey("finder", jobId),
		enabled: !!jobId,
		queryFn: async () => {
			if (!jobId) throw new Error("Select a finder job.");
			return resultSchema.parse((await api.v1GetFindEmail({ jobId })).data);
		},
		refetchInterval: (query) => (query.state.data?.status === "completed" ? false : 5_000),
	});
	function openJob(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const id = Number(lookup);
		if (!Number.isSafeInteger(id) || id < 1) {
			setValidation("Enter a valid finder job ID.");
			return;
		}
		setValidation("");
		setJobId(id);
	}
	return (
		<Page
			description="Generate likely addresses, verify them, and review the best match."
			title="Find a work email"
		>
			<section
				data-slot="finder-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				<form
					onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
						event.preventDefault();
						create.mutate();
					}}
				>
					<Group>
						<Row>
							<h3 className="text-sm font-medium text-white">Start a search</h3>
							<div className="grid gap-3 sm:grid-cols-2">
								<div className="flex flex-col gap-1.5">
									<Label className="text-xs font-medium text-neutral-400" htmlFor="finder-first">
										First name
									</Label>
									<Input
										className="h-10 rounded-lg"
										id="finder-first"
										required
										value={first}
										onChange={(event) => {
											setFirst(event.target.value);
										}}
									/>
								</div>
								<div className="flex flex-col gap-1.5">
									<Label className="text-xs font-medium text-neutral-400" htmlFor="finder-last">
										Last name
									</Label>
									<Input
										className="h-10 rounded-lg"
										id="finder-last"
										required
										value={last}
										onChange={(event) => {
											setLast(event.target.value);
										}}
									/>
								</div>
								<div className="flex flex-col gap-1.5">
									<Label className="text-xs font-medium text-neutral-400" htmlFor="finder-domain">
										Company domain
									</Label>
									<Input
										className="h-10 rounded-lg"
										id="finder-domain"
										required
										value={domain}
										onChange={(event) => {
											setDomain(event.target.value);
										}}
										placeholder="example.com"
									/>
								</div>
								<div className="flex flex-col gap-1.5">
									<Label className="text-xs font-medium text-neutral-400" htmlFor="finder-strategy">
										Search strategy
									</Label>
									<select
										id="finder-strategy"
										className="h-10 w-full rounded-lg bg-white/[0.03] px-2 text-sm text-neutral-200 ring-1 ring-white/[0.08] outline-none"
										value={strategy}
										onChange={(event) => {
											setStrategy(event.target.value === "waterfall" ? "waterfall" : "parallel");
										}}
									>
										<option value="parallel">Check all patterns</option>
										<option value="waterfall">Best patterns first</option>
									</select>
								</div>
							</div>
							<div className="flex justify-end">
								<Button className="rounded-lg" type="submit" disabled={create.isPending}>
									Find email
								</Button>
							</div>
						</Row>
					</Group>
				</form>
				<form onSubmit={openJob}>
					<Group>
						<Row label="Open a finder job" layout="inline">
							<Input
								aria-label="Finder job ID"
								className="h-10 rounded-lg md:w-40"
								inputMode="numeric"
								value={lookup}
								onChange={(event) => {
									setLookup(event.target.value);
								}}
							/>
							<Button className="h-10 rounded-lg" type="submit" variant="outline">
								Open
							</Button>
						</Row>
					</Group>
				</form>
				{validation || create.isError || result.isError ? (
					<p
						role="alert"
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					>
						{validation || create.error?.message || result.error?.message}
					</p>
				) : null}
				{jobId ? (
					<Section
						actions={
							<Link
								className="text-sm text-[#858585] hover:text-neutral-200"
								href={`/app/finder?job=${String(jobId)}`}
							>
								Share this search
							</Link>
						}
						title={`Finder job ${String(jobId)}`}
					>
						<Group>
							{result.isPending ? (
								<Row>
									<p className="text-sm text-[#858585]" role="status">
										Checking candidates…
									</p>
								</Row>
							) : null}
							{result.data ? (
								<>
									<Row
										description={`${result.data.domain_has_mx ? "Accepts mail" : "No MX record"} · ${result.data.domain_is_catch_all ? "Catch-all" : "Not catch-all"}`}
										label={`Status: ${result.data.status} · ${String(result.data.candidates_checked)} candidates checked`}
									/>
									{result.data.best_match ? (
										<Row
											description={`${result.data.best_match.confidence} confidence · Score ${String(result.data.best_match.score)}`}
											label="Best match"
										>
											<span className="truncate text-sm font-medium text-white">
												{result.data.best_match.email}
											</span>
										</Row>
									) : null}
									{result.data.results.length ? (
										<Row>
											<div className="overflow-x-auto">
												<table className="w-full text-left text-sm">
													<thead>
														<tr className="border-b border-border">
															<th className="py-2 pr-4 font-medium text-foreground">Candidate</th>
															<th className="py-2 pr-4 font-medium text-foreground">Pattern</th>
															<th className="py-2 pr-4 font-medium text-foreground">Score</th>
															<th className="py-2 font-medium text-foreground">Result</th>
														</tr>
													</thead>
													<tbody>
														{result.data.results.map((row) => (
															<tr className="border-b border-border/60" key={row.email}>
																<td className="py-2 pr-4 text-foreground">{row.email}</td>
																<td className="py-2 pr-4 text-muted-foreground">{row.pattern}</td>
																<td className="py-2 pr-4 text-muted-foreground">{row.score}</td>
																<td className="py-2 text-muted-foreground">
																	{row.category} · {row.sub_reason}
																</td>
															</tr>
														))}
													</tbody>
												</table>
											</div>
										</Row>
									) : null}
								</>
							) : null}
						</Group>
					</Section>
				) : null}
			</section>
		</Page>
	);
}
