"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import Form from "next/form";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type ReactNode } from "react";
import {
	queryOptions,
	useMutation,
	useQuery,
	useQueryClient,
	type UseQueryResult,
} from "@tanstack/react-query";
import {
	AccountApi,
	Configuration,
	JobsApi,
	OutcomesApi,
	PipelinesApi,
	TenantApi,
	V1Api,
	VerificationApi,
} from "@oppulence/reacher-sdk";
import type { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { Progress } from "@oppulence/ui/components/progress";
import { cn } from "@oppulence/ui/lib/utils";

import { EmptyState, Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";
import { fetchV1ListLists } from "@/hooks/queries/utils/fetch-v1-list-lists";
import { v1ListListsKeys } from "@/hooks/queries/utils/v1-list-lists-keys";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1JobApprovalChecklist200Response } from "@/lib/api/generated/zod/jobs/jobs";
import { V1ListProviderEndpoints200Response } from "@/lib/api/generated/zod/outcomes/outcomes";
import { V1ListPipelines200Response } from "@/lib/api/generated/zod/pipelines/pipelines";
import { V1GetTenantUsage200Response } from "@/lib/api/generated/zod/tenant/tenant";
import {
	V1GetList200Response,
	V1ListLists200Response,
	V1ListSuppressions200Response,
	V1ReverificationStatus200Response,
} from "@/lib/api/generated/zod/v1/v1";
import { V1ListAlerts200Response } from "@/lib/api/generated/zod/verification/verification";
import { ArrowRight, ListBullets } from "@/lib/icons";
import { alertType, humanize, listStatus, pipelineRunStatus } from "@/lib/verification/labels";

import {
	ApiKeysSchema,
	BulkProgressSchema,
	OutcomePageSchema,
	RunStatusSchema,
	TenantDomainsSchema,
	type OverviewPanelPropsFields,
} from "./overview-panel.schema";

/** @oppulence-gen kind=component Owned by `overview-panel.lit.ts`. */
export type OverviewPanelProps = OverviewPanelPropsFields & ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const accountApi = new AccountApi(config);
const jobsApi = new JobsApi(config);
const outcomesApi = new OutcomesApi(config);
const pipelinesApi = new PipelinesApi(config);
const tenantApi = new TenantApi(config);
const v1Api = new V1Api(config);
const verificationApi = new VerificationApi(config);

const POLL_MS = 5_000;
const OUTCOME_PAGE = 500;
const WEEK_MS = 7 * 24 * 60 * 60 * 1_000;

/** Poll a list every 5 s while the backend checks its rows. Stop when the list is finished. */
export function listPollInterval(status: string | undefined): number | false {
	return status === "uploading" || status === "processing" ? POLL_MS : false;
}

// Queries that more than one section reads. TanStack Query sends one request for each key.
const listsQuery = queryOptions({
	queryKey: v1ListListsKeys.list({ limit: 5 }),
	queryFn: ({ signal }) => fetchV1ListLists({ limit: 5 }, signal),
});
const domainsQuery = queryOptions({
	queryKey: workflowQueryKey("overview", "domains"),
	queryFn: async ({ signal }) =>
		TenantDomainsSchema.parse((await tenantApi.v1ListTenantDomains({ signal })).data),
});
const reverificationQuery = queryOptions({
	queryKey: workflowQueryKey("overview", "reverification"),
	queryFn: async ({ signal }) =>
		V1ReverificationStatus200Response.parse((await v1Api.v1ReverificationStatus({ signal })).data),
});

const number = (value: number) => value.toLocaleString();
const dateTime = (value: string) => new Date(value).toLocaleString();
const linkRow = "flex items-center gap-4 p-5 transition-colors hover:bg-white/[0.02]";

export function OverviewPanel({ className, ...props }: OverviewPanelProps) {
	return (
		<Page description="What needs your attention in this workspace." title="Overview">
			<section
				className={cn("flex flex-col gap-10", className)}
				data-slot="overview-panel"
				{...props}
			>
				<AlertsSection />
				<UsageSection />
				<ListsSection />
				<DeliverabilitySection />
				<AutomationSection />
				<QuickCheckSection />
				<DeferredSetup />
			</section>
		</Page>
	);
}

function LinkRow({
	children,
	description,
	href,
	label,
}: {
	children?: ReactNode;
	description?: ReactNode;
	href: string;
	label: ReactNode;
}) {
	return (
		<Link className={linkRow} href={href}>
			{/* The figures wrap below the label on a narrow screen; the arrow stays on the right. */}
			<span className="flex min-w-0 flex-1 flex-wrap items-center gap-x-4 gap-y-2">
				<span className="min-w-0 flex-1 basis-40">
					<span className="block truncate text-sm font-medium text-white">{label}</span>
					{description ? (
						<span className="mt-0.5 block text-xs text-[#858585]">{description}</span>
					) : null}
				</span>
				{children}
			</span>
			<ArrowRight className="shrink-0 text-neutral-600" size={14} />
		</Link>
	);
}

function ActionLink({ children, href }: { children: ReactNode; href: string }) {
	return (
		<Button asChild className="rounded-lg" size="sm" variant="outline">
			<Link href={href}>{children}</Link>
		</Button>
	);
}

function Loading({ children }: { children: ReactNode }) {
	return (
		<Row>
			<p className="text-sm text-[#858585]">{children}</p>
		</Row>
	);
}

function Failed({ children, retry }: { children: ReactNode; retry: () => unknown }) {
	return (
		<Row>
			<p className="text-sm text-red-300" role="alert">
				{children}{" "}
				<Button size="sm" variant="ghost" onClick={() => void retry()}>
					Retry
				</Button>
			</p>
		</Row>
	);
}

/** One figure in a row. A failed request shows in its own row and does not stop the other rows. */
function Figure<T>({
	children,
	query,
}: {
	children: (data: T) => ReactNode;
	query: UseQueryResult<T>;
}) {
	if (query.isPending) return <span className="text-sm text-[#858585]">Loading…</span>;
	if (query.isError) return <span className="text-sm text-red-300">Not available</span>;
	return <span className="text-sm text-neutral-300 tabular-nums">{children(query.data)}</span>;
}

function AlertsSection() {
	const client = useQueryClient();
	const alertsKey = workflowQueryKey("overview", "alerts");
	const alerts = useQuery({
		queryKey: alertsKey,
		queryFn: async ({ signal }) =>
			V1ListAlerts200Response.parse(
				(await verificationApi.v1ListAlerts({ status: "unread", limit: 10 }, { signal })).data,
			),
	});
	// The API accepts "read" and "dismissed" (backend/src/http/v1/alerts.rs). Both close the alert.
	const update = useMutation({
		mutationFn: (input: { id: number; status: "read" | "dismissed" }) =>
			verificationApi.v1UpdateAlert({
				alertId: input.id,
				updateAlertRequest: { status: input.status },
			}),
		onSuccess: () => client.invalidateQueries({ queryKey: alertsKey }),
	});
	if (alerts.isError) {
		return (
			<Section title="Alerts">
				<Group>
					<Failed retry={alerts.refetch}>Alerts could not be loaded.</Failed>
				</Group>
			</Section>
		);
	}
	if (!alerts.data?.alerts.length) return null;
	return (
		<Section description={`${number(alerts.data.total)} unread`} title="Alerts">
			{update.isError ? (
				<p className="text-sm text-red-300" role="alert">
					{errorMessage(update.error)}
				</p>
			) : null}
			<Group>
				{alerts.data.alerts.map((alert) => (
					<div
						className="flex flex-col gap-3 p-5 md:flex-row md:items-start md:justify-between"
						key={alert.id}
					>
						<div className="min-w-0">
							<div className="flex flex-wrap items-center gap-2">
								<h3 className="text-sm font-medium text-white">
									{alert.title
										.replace("changed to became_safe", "is now safe to send")
										.replace("changed to became_invalid", "is now invalid")
										.replace("changed to became_risky", "now needs review")}
								</h3>
								<Pill tone={alertType.tone(alert.type)}>{alertType.label(alert.type)}</Pill>
							</div>
							{alert.body ? <p className="mt-1 text-sm text-[#858585]">{alert.body}</p> : null}
							<p className="mt-1 text-xs break-words text-[#858585]">
								{alert.canonical_email} ·{" "}
								<time dateTime={alert.created_at}>{dateTime(alert.created_at)}</time>
							</p>
						</div>
						<div className="flex shrink-0 gap-2">
							<Button
								disabled={update.isPending}
								size="sm"
								variant="secondary"
								onClick={() => {
									update.mutate({ id: alert.id, status: "read" });
								}}
							>
								Mark read
							</Button>
							<Button
								disabled={update.isPending}
								size="sm"
								variant="ghost"
								onClick={() => {
									update.mutate({ id: alert.id, status: "dismissed" });
								}}
							>
								Dismiss
							</Button>
						</div>
					</div>
				))}
			</Group>
		</Section>
	);
}

function UsageSection() {
	const usage = useQuery({
		queryKey: workflowQueryKey("overview", "usage"),
		queryFn: async ({ signal }) =>
			V1GetTenantUsage200Response.parse((await tenantApi.v1GetTenantUsage({ signal })).data),
	});
	const value = "text-sm text-neutral-300 tabular-nums";
	const limit = usage.data?.quota_unlimited ? null : usage.data?.monthly_email_limit;
	const remaining = usage.data?.quota_unlimited ? null : usage.data?.quota_remaining;
	return (
		<Section
			actions={<ActionLink href="/app/settings?settings=usage">Usage details</ActionLink>}
			title="Usage this period"
		>
			<Group>
				{usage.isPending ? (
					<Loading>Loading usage…</Loading>
				) : usage.isError ? (
					<Failed retry={usage.refetch}>Usage could not be loaded.</Failed>
				) : (
					<>
						<Row label="Plan" layout="inline">
							<span className={value}>{humanize(usage.data.plan_tier)}</span>
						</Row>
						<Row label="Checks used" layout="inline">
							<span className={value}>{number(usage.data.used_this_period)}</span>
						</Row>
						<Row label="Monthly limit" layout="inline">
							<span className={value}>{limit == null ? "Unlimited" : number(limit)}</span>
						</Row>
						<Row label="Remaining" layout="inline">
							<span className={value}>{remaining == null ? "Unlimited" : number(remaining)}</span>
						</Row>
						<Row label="Resets" layout="inline">
							<time className={value} dateTime={usage.data.period_reset_at}>
								{dateTime(usage.data.period_reset_at)}
							</time>
						</Row>
					</>
				)}
			</Group>
		</Section>
	);
}

type ListItem = z.infer<typeof V1ListLists200Response>["lists"][number];

function ListsSection() {
	const lists = useQuery(listsQuery);
	return (
		<Section
			actions={<ActionLink href="/app/lists">All lists</ActionLink>}
			description="The five newest lists."
			title="Lists that need attention"
		>
			{lists.isPending ? (
				<Group>
					<Loading>Loading lists…</Loading>
				</Group>
			) : lists.isError ? (
				<Group>
					<Failed retry={lists.refetch}>Lists could not be loaded.</Failed>
				</Group>
			) : lists.data.lists.length ? (
				<Group>
					{lists.data.lists.map((list) => (
						<ListRow key={list.id} list={list} />
					))}
				</Group>
			) : (
				<EmptyState
					action={
						<Button asChild className="rounded-lg">
							<Link href="/app/lists">Upload a list</Link>
						</Button>
					}
					description="Upload a CSV file to check every address in it."
					icon={ListBullets}
					title="No lists yet"
				/>
			)}
		</Section>
	);
}

function ListRow({ list }: { list: ListItem }) {
	// The list detail gives the job ID and the current status. The backend
	// marks a list finished only when this detail request runs.
	const detail = useQuery({
		queryKey: workflowQueryKey("overview", "list", list.id),
		queryFn: async ({ signal }) =>
			V1GetList200Response.parse((await v1Api.v1GetList({ listId: list.id }, { signal })).data),
		refetchInterval: (query) => listPollInterval(query.state.data?.status ?? list.status),
	});
	const status = detail.data?.status ?? list.status;
	return (
		<LinkRow
			description={`${number(list.total_rows)} rows`}
			href={`/app/lists/detail?list=${String(list.id)}`}
			label={list.name}
		>
			{detail.data && listPollInterval(status) ? (
				<ListProgress jobId={detail.data.job_id} name={list.name} />
			) : null}
			{detail.data && (status === "completed" || status === "failed") ? (
				<ListVerdict jobId={detail.data.job_id} />
			) : null}
			<Pill tone={listStatus.tone(status)}>{listStatus.label(status)}</Pill>
		</LinkRow>
	);
}

function ListProgress({ jobId, name }: { jobId: number; name: string }) {
	// The parent unmounts this part when the list is finished, so the polling stops.
	const progress = useQuery({
		queryKey: workflowQueryKey("overview", "progress", jobId),
		queryFn: async ({ signal }) =>
			BulkProgressSchema.parse((await jobsApi.v1GetBulkJobProgress({ jobId }, { signal })).data),
		refetchInterval: POLL_MS,
	});
	if (progress.isPending) return null;
	if (progress.isError) return <span className="text-xs text-red-300">Progress not available</span>;
	const { total_processed: done, total_records: total } = progress.data;
	return (
		<span className="flex w-40 flex-col gap-1">
			<Progress
				aria-label={`${name} progress`}
				className="h-1.5 bg-white/[0.08]"
				value={total ? Math.round((done / total) * 100) : 0}
			/>
			<span className="text-xs text-[#858585] tabular-nums">
				{number(done)} of {number(total)} checked
			</span>
		</span>
	);
}

function ListVerdict({ jobId }: { jobId: number }) {
	const approval = useQuery({
		queryKey: workflowQueryKey("overview", "approval", jobId),
		queryFn: async ({ signal }) =>
			V1JobApprovalChecklist200Response.parse(
				(await jobsApi.v1JobApprovalChecklist({ jobId }, { signal })).data,
			),
	});
	if (approval.isPending) return null;
	if (approval.isError) return <span className="text-xs text-red-300">Verdict not available</span>;
	return (
		<span className="flex items-center gap-2">
			<span className="text-xs text-[#858585] tabular-nums">
				{number(approval.data.safe_to_send_count)} safe to send
			</span>
			<Pill tone={approval.data.ready_to_send ? "green" : "yellow"}>
				{approval.data.ready_to_send ? "Ready to send" : "Not ready to send"}
			</Pill>
		</span>
	);
}

function DeliverabilitySection() {
	// Keep one start time for each visit, so the query key stays the same between renders.
	const [since] = useState(() => new Date(Date.now() - WEEK_MS).toISOString());
	// A limit of 0 returns only the total.
	const suppressions = useQuery({
		queryKey: workflowQueryKey("overview", "suppressions"),
		queryFn: async ({ signal }) =>
			V1ListSuppressions200Response.parse(
				(await v1Api.v1ListSuppressions({ limit: 0 }, { signal })).data,
			),
	});
	// ponytail: /v1/outcomes returns no total, so this counts one page of 500 rows and shows "500+" above it.
	const outcomeCount = (eventType: string) => ({
		queryKey: workflowQueryKey("overview", "outcomes", eventType, since),
		queryFn: async ({ signal }: { signal: AbortSignal }) =>
			OutcomePageSchema.parse(
				(await outcomesApi.v1ListOutcomes({ eventType, since, limit: OUTCOME_PAGE }, { signal }))
					.data,
			).outcomes.length,
	});
	const bounces = useQuery(outcomeCount("bounced"));
	const complaints = useQuery(outcomeCount("complained"));
	const domains = useQuery(domainsQuery);
	const count = (value: number) =>
		value >= OUTCOME_PAGE ? `${number(OUTCOME_PAGE)}+` : number(value);
	return (
		<Section title="Deliverability">
			<Group>
				<LinkRow href="/app/suppressions" label="Suppressed addresses">
					<Figure query={suppressions}>{(data) => number(data.total)}</Figure>
				</LinkRow>
				<LinkRow href="/app/outcomes" label="Bounces in the last 7 days">
					<Figure query={bounces}>{count}</Figure>
				</LinkRow>
				<LinkRow href="/app/outcomes" label="Complaints in the last 7 days">
					<Figure query={complaints}>{count}</Figure>
				</LinkRow>
				<LinkRow href="/app/domains" label="Sender domains">
					<Figure query={domains}>
						{(data) =>
							`${number(data.domains.filter((domain) => domain.is_verified).length)} verified · ${number(
								data.domains.filter((domain) => domain.is_active).length,
							)} active of ${number(data.domains.length)}`
						}
					</Figure>
				</LinkRow>
			</Group>
		</Section>
	);
}

type Pipeline = z.infer<typeof V1ListPipelines200Response>["pipelines"][number];

function AutomationSection() {
	const pipelines = useQuery({
		queryKey: workflowQueryKey("overview", "pipelines"),
		queryFn: async ({ signal }) =>
			V1ListPipelines200Response.parse(
				(await pipelinesApi.v1ListPipelines({ limit: 5 }, { signal })).data,
			),
	});
	const reverification = useQuery(reverificationQuery);
	return (
		<Section
			actions={<ActionLink href="/app/pipelines">All pipelines</ActionLink>}
			title="Automation"
		>
			<Group>
				{pipelines.isPending ? (
					<Loading>Loading pipelines…</Loading>
				) : pipelines.isError ? (
					<Failed retry={pipelines.refetch}>Pipelines could not be loaded.</Failed>
				) : pipelines.data.pipelines.length ? (
					pipelines.data.pipelines.map((pipeline) => (
						<PipelineRow key={pipeline.id} pipeline={pipeline} />
					))
				) : (
					<LinkRow
						description="Check a list again on a schedule."
						href="/app/pipelines"
						label="Pipelines"
					>
						<span className="text-sm text-[#858585]">None yet</span>
					</LinkRow>
				)}
				<LinkRow href="/app/settings?settings=verification" label="Scheduled re-checks">
					<Figure query={reverification}>
						{(data) =>
							!data.enabled
								? "Off"
								: data.next_run_at
									? `Next run ${dateTime(data.next_run_at)}`
									: "On"
						}
					</Figure>
				</LinkRow>
			</Group>
		</Section>
	);
}

function PipelineRow({ pipeline }: { pipeline: Pipeline }) {
	return (
		<LinkRow
			description={
				pipeline.next_run_at
					? `Next run ${dateTime(pipeline.next_run_at)}`
					: humanize(pipeline.status)
			}
			href="/app/pipelines"
			label={pipeline.name}
		>
			{pipeline.last_run_id == null ? (
				<span className="text-xs text-[#858585]">No runs yet</span>
			) : (
				<LastRun pipelineId={pipeline.id} runId={pipeline.last_run_id} />
			)}
		</LinkRow>
	);
}

function LastRun({ pipelineId, runId }: { pipelineId: number; runId: number }) {
	const run = useQuery({
		queryKey: workflowQueryKey("overview", "pipeline-run", pipelineId, runId),
		queryFn: async ({ signal }) =>
			RunStatusSchema.parse(
				(await pipelinesApi.v1GetPipelineRun({ pipelineId, runId }, { signal })).data,
			),
	});
	return (
		<Figure query={run}>
			{(data) => (
				<span className="flex items-center gap-2 text-xs text-[#858585]">
					Last run
					<Pill tone={pipelineRunStatus.tone(data.status)}>
						{pipelineRunStatus.label(data.status)}
					</Pill>
				</span>
			)}
		</Figure>
	);
}

function QuickCheckSection() {
	return (
		<Section description="Open the full result for one address." title="Quick check">
			{/* A GET form: the browser adds ?email=<value> to /app/check. */}
			<Form action="/app/check">
				<Group>
					<Row label={<Label htmlFor="overview-email">Email address</Label>}>
						<Input
							className="h-10 min-w-0 flex-1 rounded-lg md:w-72 md:flex-none"
							id="overview-email"
							name="email"
							placeholder="name@example.com"
							required
							type="email"
						/>
						<Button className="h-10 rounded-lg" type="submit">
							Check
						</Button>
					</Row>
				</Group>
			</Form>
		</Section>
	);
}

function DeferredSetup() {
	const [open, setOpen] = useState(false);
	return (
		<details
			onToggle={(event) => {
				setOpen(event.currentTarget.open);
			}}
		>
			<summary className="cursor-pointer text-sm">Workspace setup checklist</summary>
			{open ? <SetupSection /> : null}
		</details>
	);
}

function SetupSection() {
	const lists = useQuery(listsQuery);
	const domains = useQuery(domainsQuery);
	const reverification = useQuery(reverificationQuery);
	const endpoints = useQuery({
		queryKey: workflowQueryKey("overview", "provider-endpoints"),
		queryFn: async ({ signal }) =>
			V1ListProviderEndpoints200Response.parse(
				(await outcomesApi.v1ListProviderEndpoints({ signal })).data,
			),
	});
	const keys = useQuery({
		queryKey: workflowQueryKey("overview", "api-keys"),
		queryFn: async ({ signal }) =>
			ApiKeysSchema.parse((await accountApi.listTenantApiKeys({ signal })).data),
	});
	if ([lists, domains, reverification, endpoints, keys].some((query) => query.isPending)) {
		return null;
	}
	// A step shows only when its request succeeds. Members who cannot read API keys do not see that step.
	const steps = [
		{ href: "/app/lists", label: "Upload a list", done: lists.data && lists.data.total > 0 },
		{
			href: "/app/domains",
			label: "Add a sender domain",
			done: domains.data && domains.data.domains.length > 0,
		},
		{
			href: "/app/outcomes",
			label: "Connect delivery feedback",
			done: endpoints.data && endpoints.data.provider_endpoints.length > 0,
		},
		{
			href: "/app/settings?settings=developer",
			label: "Create an API key",
			done: keys.data && keys.data.api_keys.length > 0,
		},
		{
			href: "/app/settings?settings=verification",
			label: "Turn on re-checks",
			done: reverification.data?.enabled,
		},
	].filter((step) => step.done !== undefined);
	const open = steps.filter((step) => !step.done).length;
	if (!open) return <p className="text-sm text-[#858585]">All setup steps are done.</p>;
	return (
		<Section
			description={`${String(steps.length - open)} of ${String(steps.length)} steps done.`}
			title="Set up your workspace"
		>
			<Group>
				{steps.map((step) => (
					<LinkRow href={step.href} key={step.label} label={step.label}>
						<Pill tone={step.done ? "green" : "gray"}>{step.done ? "Done" : "To do"}</Pill>
					</LinkRow>
				))}
			</Group>
		</Section>
	);
}
