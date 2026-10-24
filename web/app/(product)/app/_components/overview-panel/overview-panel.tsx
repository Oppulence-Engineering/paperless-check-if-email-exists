"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import Form from "next/form";
import Link from "next/link";
import {
	createContext,
	useContext,
	useState,
	type ComponentPropsWithoutRef,
	type ReactNode,
} from "react";
import { useMutation, useQuery, useQueryClient, type UseQueryResult } from "@tanstack/react-query";
import {
	AccountApi,
	Configuration,
	OutcomesApi,
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
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1ListProviderEndpoints200Response } from "@/lib/api/generated/zod/outcomes/outcomes";
import { ListTenantApiKeys200Response } from "@/lib/api/generated/zod/account/account";
import { V1Overview200Response } from "@/lib/api/generated/zod/v1/v1";
import { ArrowRight, ListBullets } from "@/lib/icons";
import { alertType, humanize, listStatus, pipelineRunStatus } from "@/lib/verification/labels";

import { type OverviewPanelPropsFields } from "./overview-panel.schema";

/** @oppulence-gen kind=component Owned by `overview-panel.lit.ts`. */
export type OverviewPanelProps = OverviewPanelPropsFields & ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const accountApi = new AccountApi(config);
const outcomesApi = new OutcomesApi(config);
const v1Api = new V1Api(config);
const verificationApi = new VerificationApi(config);

const POLL_MS = 5_000;

/** Poll a list every 5 s while the backend checks its rows. Stop when the list is finished. */
export function listPollInterval(status: string | undefined): number | false {
	return status === "uploading" || status === "processing" ? POLL_MS : false;
}

type Overview = z.infer<typeof V1Overview200Response>;
type SectionQuery<T> = Pick<UseQueryResult<T>, "data" | "isPending" | "isError" | "refetch">;
const OverviewContext = createContext<UseQueryResult<Overview> | null>(null);
function useOverview() {
	const query = useContext(OverviewContext);
	if (!query) throw new Error("Overview provider is required");
	return query;
}
function useSection<K extends Exclude<keyof Overview, "usage">>(
	key: K,
): SectionQuery<NonNullable<Overview[K]["data"]>> {
	const query = useOverview();
	const section = query.data?.[key];
	return {
		data: section?.data ?? undefined,
		isPending: query.isPending,
		isError: query.isError || (!!section && section.status !== "ready"),
		refetch: query.refetch,
	} as SectionQuery<NonNullable<Overview[K]["data"]>>;
}

const number = (value: number) => value.toLocaleString();
const dateTime = (value: string) => new Date(value).toLocaleString();
const linkRow = "flex items-center gap-4 p-5 transition-colors hover:bg-white/[0.02]";

export function OverviewPanel({ className, ...props }: OverviewPanelProps) {
	const overview = useQuery({
		queryKey: workflowQueryKey("overview"),
		queryFn: async ({ signal }) =>
			V1Overview200Response.parse((await v1Api.v1Overview({ signal })).data),
		staleTime: 30_000,
		refetchInterval: (query) =>
			query.state.status === "error"
				? false
				: query.state.data?.lists.data?.some((list) => listPollInterval(list.status)) ||
					  query.state.data?.pipelines.data?.some((pipeline) =>
							["queued", "running", "verifying", "delivering", "pending"].includes(
								pipeline.last_run_status ?? "",
							),
					  )
					? POLL_MS
					: false,
		refetchOnWindowFocus: true,
	});
	return (
		<OverviewContext.Provider value={overview}>
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
		</OverviewContext.Provider>
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
	query: SectionQuery<T>;
}) {
	if (query.isPending) return <span className="text-sm text-[#858585]">Loading…</span>;
	if (query.isError) return <span className="text-sm text-red-300">Not available</span>;
	return (
		<span className="text-sm text-neutral-300 tabular-nums">
			{query.data === undefined ? "Not available" : children(query.data)}
		</span>
	);
}

function AlertsSection() {
	const client = useQueryClient();
	const alertsKey = workflowQueryKey("overview");
	const alerts = useSection("alerts");
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
	const overview = useOverview();
	const usage = { ...overview, data: overview.data?.usage };
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
				) : usage.isError || !usage.data ? (
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

type ListItem = NonNullable<Overview["lists"]["data"]>[number];

function ListsSection() {
	const lists = useSection("lists");
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
			) : lists.data?.length ? (
				<Group>
					{lists.data.map((list) => (
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
	const status = list.status;
	return (
		<LinkRow
			description={`${number(list.total_rows)} rows`}
			href={`/app/lists/detail?list=${String(list.id)}`}
			label={list.name}
		>
			{listPollInterval(status) ? <ListProgress list={list} /> : null}
			{list.ready_to_send != null && (status === "completed" || status === "failed") ? (
				<ListVerdict list={list} />
			) : null}
			<Pill tone={listStatus.tone(status)}>{listStatus.label(status)}</Pill>
		</LinkRow>
	);
}

function ListProgress({ list }: { list: ListItem }) {
	const { total_processed: done, total_rows: total, name } = list;
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

function ListVerdict({ list }: { list: ListItem }) {
	return (
		<span className="flex items-center gap-2">
			<span className="text-xs text-[#858585] tabular-nums">
				{number(list.safe_to_send_count ?? 0)} safe to send
			</span>
			<Pill tone={list.ready_to_send ? "green" : "yellow"}>
				{list.ready_to_send ? "Ready to send" : "Not ready to send"}
			</Pill>
		</span>
	);
}

function DeliverabilitySection() {
	const suppressions = useSection("suppressions");
	const outcomes = useSection("outcomes");
	const domains = useSection("domains");
	return (
		<Section title="Deliverability">
			<Group>
				<LinkRow href="/app/suppressions" label="Suppressed addresses">
					<Figure query={suppressions}>{number}</Figure>
				</LinkRow>
				<LinkRow href="/app/outcomes" label="Bounces in the last 7 days">
					<Figure query={outcomes}>{(data) => number(data.bounced)}</Figure>
				</LinkRow>
				<LinkRow href="/app/outcomes" label="Complaints in the last 7 days">
					<Figure query={outcomes}>{(data) => number(data.complained)}</Figure>
				</LinkRow>
				<LinkRow href="/app/domains" label="Sender domains">
					<Figure query={domains}>
						{(data) =>
							`${number(data.verified)} verified · ${number(data.active)} active of ${number(data.total)}`
						}
					</Figure>
				</LinkRow>
			</Group>
		</Section>
	);
}

type Pipeline = NonNullable<Overview["pipelines"]["data"]>[number];

function AutomationSection() {
	const pipelines = useSection("pipelines");
	const reverification = useSection("reverification");
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
				) : pipelines.data?.length ? (
					pipelines.data.map((pipeline) => <PipelineRow key={pipeline.id} pipeline={pipeline} />)
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
				<span className="flex items-center gap-2 text-xs text-[#858585]">
					Last run{" "}
					<Pill tone={pipelineRunStatus.tone(pipeline.last_run_status ?? "unknown")}>
						{pipelineRunStatus.label(pipeline.last_run_status ?? "unknown")}
					</Pill>
				</span>
			)}
		</LinkRow>
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
	const lists = useSection("lists");
	const domains = useSection("domains");
	const reverification = useSection("reverification");
	const endpoints = useQuery({
		queryKey: workflowQueryKey("provider-endpoints"),
		queryFn: async ({ signal }) =>
			V1ListProviderEndpoints200Response.parse(
				(await outcomesApi.v1ListProviderEndpoints({ signal })).data,
			),
	});
	const keys = useQuery({
		queryKey: workflowQueryKey("tenant-api-keys"),
		queryFn: async ({ signal }) =>
			ListTenantApiKeys200Response.parse((await accountApi.listTenantApiKeys({ signal })).data),
	});
	if ([lists, domains, reverification, endpoints, keys].some((query) => query.isPending)) {
		return null;
	}
	// A step shows only when its request succeeds. Members who cannot read API keys do not see that step.
	const steps = [
		{ href: "/app/lists", label: "Upload a list", done: lists.data && lists.data.length > 0 },
		{
			href: "/app/domains",
			label: "Add a sender domain",
			done: domains.data && domains.data.total > 0,
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
