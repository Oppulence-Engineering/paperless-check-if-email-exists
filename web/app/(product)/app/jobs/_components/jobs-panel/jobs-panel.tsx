"use client";
import { useJobStatus } from "@/hooks/queries/use-job-status";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, JobsApi, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useEffect, useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@oppulence/ui/components/tabs";
import { Textarea } from "@oppulence/ui/components/textarea";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";
import { JobProblems } from "@/components/features/lists/job-problems/job-problems";
import { JobProgress } from "@/components/features/lists/job-progress/job-progress";
import { JobReview, Stat } from "@/components/features/lists/job-review/job-review";
import { JobRows } from "@/components/features/lists/job-rows/job-rows";
import { fetchV1EmailHistory } from "@/hooks/queries/utils/fetch-v1-email-history";
import { saveFile } from "@/lib/downloads";
import { V1ListJobs200Response } from "@/lib/api/generated/zod/jobs/jobs";
import { V1GetJobResults200Response } from "@/lib/api/generated/zod/jobs/jobs";
import { ACTIVE_JOB_STATES } from "@/lib/api/schemas/jobs";
import { jobStatus, taskState } from "@/lib/verification/labels";

import { type JobsPanelPropsFields } from "./jobs-panel.schema";

/** @oppulence-gen kind=component Owned by `jobs-panel.lit.ts`. */
export type JobsPanelProps = JobsPanelPropsFields & ComponentPropsWithoutRef<"section">;

const jobsApi = new JobsApi(new Configuration({ basePath: "/api/backend" }));
const v1Api = new V1Api(new Configuration({ basePath: "/api/backend" }));

/** The segmented control style of the product pages, applied to the tabs. */
const TAB_LIST_CLASS =
	"h-auto w-full flex-wrap justify-start rounded-lg bg-white/[0.03] p-0.5 ring-1 ring-white/[0.08] sm:w-fit";
const TAB_CLASS =
	"flex-none rounded-md border-0 px-3 py-1 text-[#858585] after:hidden hover:text-neutral-200 data-[state=active]:bg-white/[0.08] data-[state=active]:text-white";

export function parseEmailBatch(value: string): string[] {
	return value
		.split(/[\s,;]+/)
		.map((email) => email.trim())
		.filter(Boolean);
}

export function JobsPanel({ initialJobId, className, ...props }: JobsPanelProps) {
	const queryClient = useQueryClient();
	const router = useRouter();
	const [offset, setOffset] = useState(0);
	const recent = useQuery({
		queryKey: workflowQueryKey("recent-jobs", offset),
		queryFn: async ({ signal }) =>
			V1ListJobs200Response.parse(
				(await jobsApi.v1ListJobs({ limit: 20, offset }, { signal })).data,
			),
	});
	const [input, setInput] = useState("");
	const [source, setSource] = useState("");
	const [lookup, setLookup] = useState(initialJobId ? String(initialJobId) : "");
	const [jobId, setJobId] = useState(initialJobId);
	useEffect(() => {
		setJobId(initialJobId);
		setLookup(initialJobId ? String(initialJobId) : "");
	}, [initialJobId]);
	const [notice, setNotice] = useState("");
	const [validation, setValidation] = useState("");
	const key = ["jobs", jobId];
	function selectedJobId(): number {
		if (!jobId) throw new Error("Select a job first.");
		return jobId;
	}

	const create = useMutation({
		onMutate: () => {
			setNotice("");
		},
		mutationFn: async () => {
			const emails = parseEmailBatch(input);
			if (!emails.length) throw new Error("Enter at least one email address.");
			return (
				await v1Api.v1CreateBulkJob({
					bulkCreateRequest: {
						input: emails,
						source_key: source.trim() || undefined,
					},
				})
			).data;
		},
		onSuccess: (data) => {
			router.replace(`/app/jobs?job=${String(data.job_id)}`);
			setJobId(data.job_id);
			setLookup(String(data.job_id));
			setInput("");
			setNotice(`Job ${String(data.job_id)} created.`);
			void queryClient.invalidateQueries({ queryKey: workflowQueryKey("recent-jobs") });
		},
	});
	const status = useJobStatus(jobId ?? undefined);
	// ponytail: the job API has no list ID. The history of one checked row has it.
	// Use a list_id field on the job status when the backend adds one.
	const listId = useQuery({
		queryKey: workflowQueryKey(...key, "list"),
		enabled: !!jobId,
		queryFn: async ({ signal }) => {
			const page = V1GetJobResults200Response.parse(
				(await jobsApi.v1GetJobResults({ jobId: selectedJobId(), limit: 10 }, { signal })).data,
			);
			const row = page.results.find((item) => item.result?.input);
			if (!row?.result) return null;
			const history = await fetchV1EmailHistory(row.result.input);
			return history.history.find((entry) => entry.task_id === row.id)?.list_id ?? null;
		},
	});
	const action = useMutation({
		onMutate: () => {
			setNotice("");
		},
		mutationFn: async () => {
			await jobsApi.v1CancelJob({ jobId: selectedJobId() });
		},
		onSuccess: async () => {
			setNotice("Cancellation requested.");
			await queryClient.invalidateQueries({ queryKey: key });
		},
	});
	const download = useMutation({
		onMutate: () => {
			setNotice("");
		},
		mutationFn: async () => {
			const response = await jobsApi.v1DownloadJobResults(
				{ jobId: selectedJobId(), format: "csv" },
				{ responseType: "blob" },
			);
			saveFile(response.data, `job-${String(jobId)}-results.csv`);
		},
	});

	function openJob(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const id = Number(lookup);
		setNotice("");
		create.reset();
		action.reset();
		download.reset();
		if (!/^\d+$/.test(lookup) || !Number.isSafeInteger(id) || id < 1 || id > 2147483647) {
			setJobId(undefined);
			setValidation("Enter a valid job ID.");
			router.replace("/app/jobs");
			return;
		}
		setValidation("");
		setJobId(id);
		router.replace(`/app/jobs?job=${String(id)}`);
	}
	const requestError = [create, status, action, download].find((request) => request.isError)?.error;

	return (
		<Page
			description="Create a batch, then review its rows, failures, and send readiness."
			title="Bulk jobs"
		>
			<section data-slot="jobs-panel" className={cn("flex flex-col gap-10", className)} {...props}>
				<Section title="Recent jobs">
					<Group>
						{recent.data?.jobs.map((job) => (
							<Row
								key={job.id}
								label={
									<Link
										href={`/app/jobs?job=${String(job.id)}`}
										onClick={() => {
											setJobId(job.id);
											setLookup(String(job.id));
										}}
									>
										Job {job.id}
									</Link>
								}
								description={`${String(job.total_records)} rows · ${new Date(job.created_at).toLocaleString()}`}
							>
								<Pill>{jobStatus.label(job.status)}</Pill>
							</Row>
						))}
					</Group>
					{recent.isError && !requestError ? (
						<p role="alert">
							{errorMessage(recent.error)}{" "}
							<Button onClick={() => void recent.refetch()}>Retry</Button>
						</p>
					) : null}
					{recent.data?.total === 0 ? <p>No jobs yet.</p> : null}
					<div className="flex gap-2">
						<Button
							disabled={offset === 0}
							onClick={() => {
								setOffset(Math.max(0, offset - 20));
							}}
						>
							Previous
						</Button>
						<Button
							disabled={offset + 20 >= (recent.data?.total ?? 0)}
							onClick={() => {
								setOffset(offset + 20);
							}}
						>
							Next
						</Button>
					</div>
				</Section>
				<form
					onSubmit={(event) => {
						event.preventDefault();
						create.mutate();
					}}
				>
					<Group>
						<Row>
							<h3 className="text-sm font-medium text-white">Create a job</h3>
							<div className="flex flex-col gap-1.5">
								<Label className="text-xs font-medium text-neutral-400" htmlFor="bulk-emails">
									Email addresses
								</Label>
								<Textarea
									id="bulk-emails"
									aria-label="Email addresses"
									className="min-h-32 rounded-lg"
									placeholder="One address per line"
									required
									value={input}
									onChange={(event) => {
										setInput(event.target.value);
									}}
								/>
							</div>
							<div className="grid gap-3 sm:grid-cols-[1fr_auto] sm:items-end">
								<div className="flex flex-col gap-1.5">
									<Label className="text-xs font-medium text-neutral-400" htmlFor="bulk-source">
										Source label (optional)
									</Label>
									<Input
										className="h-10 rounded-lg"
										id="bulk-source"
										value={source}
										onChange={(event) => {
											setSource(event.target.value);
										}}
									/>
								</div>
								<Button className="h-10 rounded-lg" type="submit" disabled={create.isPending}>
									Create job
								</Button>
							</div>
						</Row>
					</Group>
				</form>
				<form onSubmit={openJob}>
					<Group>
						<Row
							description={
								<>
									List uploads also create jobs.{" "}
									<Link href="/app/lists" className="underline">
										View lists
									</Link>
								</>
							}
							label="Find a job"
							layout="inline"
						>
							<Input
								aria-label="Job ID"
								className="h-10 rounded-lg md:w-40"
								inputMode="numeric"
								value={lookup}
								onChange={(event) => {
									setLookup(event.target.value);
								}}
							/>
							<Button className="h-10 rounded-lg" variant="outline" type="submit">
								Open
							</Button>
						</Row>
					</Group>
				</form>
				{validation || requestError ? (
					<p
						role="alert"
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					>
						{validation || errorMessage(requestError)}
						{status.isError && jobId ? (
							<Button
								variant="outline"
								onClick={() => {
									void status.refetch();
								}}
							>
								Retry
							</Button>
						) : null}
					</p>
				) : null}
				{notice && !requestError ? (
					<p
						role="status"
						className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
					>
						{notice}{" "}
						<Link href={`/app/jobs?job=${String(jobId)}`} className="underline">
							Share this job
						</Link>
					</p>
				) : null}
				{jobId && !status.isError ? (
					<Section
						actions={
							<Button
								className="rounded-lg"
								size="sm"
								variant="outline"
								onClick={() => {
									void queryClient.invalidateQueries({
										queryKey: key,
									});
								}}
							>
								Refresh
							</Button>
						}
						title={`Job ${String(jobId)}`}
					>
						<Group>
							{status.isPending ? (
								<Row>
									<p className="text-sm text-[#858585]" role="status">
										Loading job…
									</p>
								</Row>
							) : null}
							{status.data ? (
								<Row>
									<div className="flex flex-wrap items-center gap-x-4 gap-y-3">
										<Pill tone={jobStatus.tone(status.data.status)}>
											{jobStatus.label(status.data.status)}
										</Pill>
										<span className="min-w-0 flex-1 text-sm text-white">
											{status.data.total_records} records
											<span className="block text-xs text-[#858585]">
												Created {new Date(status.data.created_at).toLocaleString()}
												{status.data.completed_at
													? ` · Done ${new Date(status.data.completed_at).toLocaleString()}`
													: ""}
											</span>
										</span>
										{listId.data ? (
											<Link
												className="text-sm text-white underline underline-offset-4"
												href={`/app/lists/detail?list=${String(listId.data)}`}
											>
												Open list workspace
											</Link>
										) : null}
										<Button
											className="rounded-lg"
											disabled={download.isPending}
											onClick={() => {
												download.mutate();
											}}
											size="sm"
											variant="outline"
										>
											Download results CSV
										</Button>
										<Button
											className="rounded-lg"
											size="sm"
											variant="secondary"
											disabled={action.isPending || !ACTIVE_JOB_STATES.includes(status.data.status)}
											onClick={() => {
												if (window.confirm(`Cancel job ${String(jobId)}?`)) action.mutate();
											}}
										>
											Cancel
										</Button>
									</div>
									<div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
										{Object.entries(status.data.task_summary).map(([state, count]) => (
											<Stat key={state} label={taskState.label(state)} value={count} />
										))}
									</div>
									<JobProgress jobId={jobId} />
								</Row>
							) : null}
						</Group>
						<Tabs defaultValue="review" key={jobId}>
							<TabsList aria-label="Job views" className={TAB_LIST_CLASS}>
								<TabsTrigger className={TAB_CLASS} value="review">
									Review
								</TabsTrigger>
								<TabsTrigger className={TAB_CLASS} value="rows">
									Rows
								</TabsTrigger>
								<TabsTrigger className={TAB_CLASS} value="problems">
									Problems
								</TabsTrigger>
							</TabsList>
							<TabsContent className="mt-6" value="review">
								<JobReview jobId={jobId} />
							</TabsContent>
							<TabsContent className="mt-6" value="rows">
								<JobRows jobId={jobId} />
							</TabsContent>
							<TabsContent className="mt-6" value="problems">
								<JobProblems jobId={jobId} />
							</TabsContent>
						</Tabs>
					</Section>
				) : null}
			</section>
		</Page>
	);
}
