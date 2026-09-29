"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
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
		<section data-slot="finder-panel" className={cn("space-y-6", className)} {...props}>
			<header>
				<h1 className="text-3xl font-semibold">Find a work email</h1>
				<p className="mt-2 text-muted-foreground">
					Generate likely addresses, verify them, and review the best match.
				</p>
			</header>
			<Card>
				<CardHeader>
					<CardTitle>Start a search</CardTitle>
				</CardHeader>
				<CardContent>
					<form
						className="grid gap-4 sm:grid-cols-2"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							create.mutate();
						}}
					>
						<div>
							<Label htmlFor="finder-first">First name</Label>
							<Input
								id="finder-first"
								required
								value={first}
								onChange={(event) => {
									setFirst(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="finder-last">Last name</Label>
							<Input
								id="finder-last"
								required
								value={last}
								onChange={(event) => {
									setLast(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="finder-domain">Company domain</Label>
							<Input
								id="finder-domain"
								required
								value={domain}
								onChange={(event) => {
									setDomain(event.target.value);
								}}
								placeholder="example.com"
							/>
						</div>
						<div>
							<Label htmlFor="finder-strategy">Search strategy</Label>
							<select
								id="finder-strategy"
								className="h-9 w-full rounded border bg-background px-3"
								value={strategy}
								onChange={(event) => {
									setStrategy(event.target.value === "waterfall" ? "waterfall" : "parallel");
								}}
							>
								<option value="parallel">Check all patterns</option>
								<option value="waterfall">Best patterns first</option>
							</select>
						</div>
						<Button type="submit" disabled={create.isPending}>
							Find email
						</Button>
					</form>
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Open a finder job</CardTitle>
				</CardHeader>
				<CardContent>
					<form className="flex gap-2" onSubmit={openJob}>
						<Input
							aria-label="Finder job ID"
							inputMode="numeric"
							value={lookup}
							onChange={(event) => {
								setLookup(event.target.value);
							}}
						/>
						<Button type="submit" variant="outline">
							Open
						</Button>
					</form>
				</CardContent>
			</Card>
			{validation || create.isError || result.isError ? (
				<p role="alert" className="text-destructive">
					{validation || create.error?.message || result.error?.message}
				</p>
			) : null}
			{jobId ? (
				<Card>
					<CardHeader>
						<CardTitle>Finder job {String(jobId)}</CardTitle>
					</CardHeader>
					<CardContent className="space-y-4">
						<Link className="underline" href={`/app/finder?job=${String(jobId)}`}>
							Share this search
						</Link>
						{result.isPending ? <p role="status">Checking candidates…</p> : null}
						{result.data ? (
							<>
								<p>
									Status: {result.data.status} · {result.data.candidates_checked} candidates checked
								</p>
								<p>
									Domain: {result.data.domain_has_mx ? "Accepts mail" : "No MX record"} ·{" "}
									{result.data.domain_is_catch_all ? "Catch-all" : "Not catch-all"}
								</p>
								{result.data.best_match ? (
									<div className="rounded border p-4">
										<p className="text-sm">Best match</p>
										<p className="text-xl font-semibold">{result.data.best_match.email}</p>
										<p>
											{result.data.best_match.confidence} confidence · Score{" "}
											{result.data.best_match.score}
										</p>
									</div>
								) : null}
								{result.data.results.length ? (
									<div className="overflow-x-auto">
										<table className="w-full text-sm">
											<thead>
												<tr>
													<th className="p-2 text-left">Candidate</th>
													<th className="p-2 text-left">Pattern</th>
													<th className="p-2 text-left">Score</th>
													<th className="p-2 text-left">Result</th>
												</tr>
											</thead>
											<tbody>
												{result.data.results.map((row) => (
													<tr className="border-t" key={row.email}>
														<td className="p-2">{row.email}</td>
														<td className="p-2">{row.pattern}</td>
														<td className="p-2">{row.score}</td>
														<td className="p-2">
															{row.category} · {row.sub_reason}
														</td>
													</tr>
												))}
											</tbody>
										</table>
									</div>
								) : null}
							</>
						) : null}
					</CardContent>
				</Card>
			) : null}
		</section>
	);
}
