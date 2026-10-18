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
import { NativeSelect, NativeSelectOption } from "@oppulence/ui/components/native-select";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";
import { ProviderDetails } from "@/components/features/verification/provider-details/provider-details";
import { RiskFactors } from "@/components/features/verification/risk-factors/risk-factors";
import { SignalChecklist } from "@/components/features/verification/signal-checklist/signal-checklist";
import { TechnicalDetails } from "@/components/features/verification/technical-details/technical-details";
import { VerdictSummary } from "@/components/features/verification/verdict-summary/verdict-summary";
import { V1CheckEmail200Response, V1GetFindEmail200Response } from "@/lib/api/generated/zod/v1/v1";
import {
	category,
	confidenceLevel,
	jobStatus,
	labelFor,
	reachability,
	REASON_LABELS,
} from "@/lib/verification/labels";

import { type FinderPanelPropsFields } from "./finder-panel.schema";

/** @oppulence-gen kind=component Owned by `finder-panel.lit.ts`. */
export type FinderPanelProps = FinderPanelPropsFields & ComponentPropsWithoutRef<"section">;

const api = new V1Api(new Configuration({ basePath: "/api/backend" }));
const createSchema = z.object({ job_id: z.number(), status: z.string() });
const candidateSchema = V1GetFindEmail200Response.shape.results.element;
// Each candidate result is parsed on its own, so one old result does not stop the page.
const resultSchema = z.object({
	...V1GetFindEmail200Response.shape,
	results: z.array(z.object({ ...candidateSchema.shape, result: z.unknown() })),
});

/** Poll until the job is finished. A failed job does not change again. */
export function finderRefetchInterval(status: string | undefined): number | false {
	return status === "completed" || status === "failed" ? false : 5_000;
}

function Stat({ label, children }: { label: string; children: React.ReactNode }) {
	return (
		<div>
			<dt className="text-xs text-[#858585]">{label}</dt>
			<dd className="mt-1 flex items-center gap-2 text-sm font-medium text-white">{children}</dd>
		</div>
	);
}

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
		refetchInterval: (query) => finderRefetchInterval(query.state.data?.status),
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
									<NativeSelect
										id="finder-strategy"
										className="h-10 w-full rounded-lg"
										value={strategy}
										onChange={(event) => {
											setStrategy(event.target.value === "waterfall" ? "waterfall" : "parallel");
										}}
									>
										<NativeSelectOption value="parallel">Check all patterns</NativeSelectOption>
										<NativeSelectOption value="waterfall">Best patterns first</NativeSelectOption>
									</NativeSelect>
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
							<>
								{result.data ? (
									<Link
										className="text-sm text-[#858585] hover:text-neutral-200"
										href={`/app/jobs?job=${String(result.data.bulk_job_id)}`}
									>
										Open job {result.data.bulk_job_id}
									</Link>
								) : null}
								<Link
									className="text-sm text-[#858585] hover:text-neutral-200"
									href={`/app/finder?job=${String(jobId)}`}
								>
									Share this search
								</Link>
							</>
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
										description={`${String(result.data.candidates_checked)} candidates checked · ${result.data.domain_has_mx ? "The domain accepts mail" : "The domain has no mail server"} · ${result.data.domain_is_catch_all ? "Catch-all domain" : "Not a catch-all domain"}`}
										label="Status"
										layout="inline"
									>
										<Pill tone={jobStatus.tone(result.data.status)}>
											{jobStatus.label(result.data.status)}
										</Pill>
									</Row>
									{result.data.status === "failed" ? (
										<Row>
											<p className="text-sm text-red-300" role="alert">
												This search failed. Check the job for details, or start a new search.
											</p>
										</Row>
									) : null}
									{result.data.best_match ? (
										<Row
											description={`Pattern ${result.data.best_match.pattern} · ${confidenceLevel.label(result.data.best_match.confidence)} · Score ${String(result.data.best_match.score)}/100`}
											label="Best match"
										>
											<span className="truncate text-sm font-medium text-white">
												{result.data.best_match.email}
											</span>
										</Row>
									) : result.data.status === "completed" ? (
										<Row
											description="No candidate scored high enough to be a match."
											label="Best match"
										/>
									) : null}
								</>
							) : null}
						</Group>
						{result.data?.results.length ? (
							<div className="flex flex-col gap-4">
								{result.data.results.map((row) => {
									const full =
										row.result == null ? null : V1CheckEmail200Response.safeParse(row.result);
									return (
										<Group key={row.email}>
											<Row>
												<div className="flex flex-wrap items-center gap-2">
													<span className="text-sm font-medium break-all text-white">
														{row.email}
													</span>
													<Pill tone={category.tone(row.category)}>
														{category.label(row.category)}
													</Pill>
												</div>
												<dl className="grid grid-cols-2 gap-5 sm:grid-cols-4">
													<Stat label="Pattern">{row.pattern}</Stat>
													<Stat label="Reachability">
														<Pill tone={reachability.tone(row.is_reachable)}>
															{reachability.label(row.is_reachable)}
														</Pill>
													</Stat>
													<Stat label="Confidence">
														{row.confidence ? (
															<>
																{row.confidence.score}/100
																<span className="text-[#858585]">
																	{confidenceLevel.label(row.confidence.level)}
																</span>
															</>
														) : (
															"Not available"
														)}
													</Stat>
													<Stat label="Score">{row.score}/100</Stat>
												</dl>
												<p className="text-xs text-[#858585]">
													{labelFor(REASON_LABELS, row.sub_reason)}
												</p>
												{full ? (
													<details className="group">
														<summary className="cursor-pointer list-none text-sm text-neutral-300 underline underline-offset-4 hover:text-white [&::-webkit-details-marker]:hidden">
															<span className="group-open:hidden">Show full result</span>
															<span className="hidden group-open:inline">Hide full result</span>
														</summary>
														{full.success ? (
															<div className="mt-4 flex flex-col gap-4">
																<VerdictSummary result={full.data} />
																<RiskFactors result={full.data} />
																<SignalChecklist result={full.data} />
																<ProviderDetails result={full.data} />
																<TechnicalDetails result={full.data} />
															</div>
														) : (
															<p className="mt-4 text-sm text-[#858585]">
																The full result for this address could not be read.
															</p>
														)}
													</details>
												) : null}
											</Row>
										</Group>
									);
								})}
							</div>
						) : null}
					</Section>
				) : null}
			</section>
		</Page>
	);
}
