"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { AdminApi, AdminJobsApi, Configuration } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { type PlatformOperationRunnerPropsFields } from "./platform-operation-runner.schema";

/** @oppulence-gen kind=component Owned by `platform-operation-runner.lit.ts`. */
export type PlatformOperationRunnerProps = PlatformOperationRunnerPropsFields &
	ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/platform/backend" });
const admin = new AdminApi(config);
const jobs = new AdminJobsApi(config);
const jobSchema = z.object({
	job_id: z.number(),
	status: z.string(),
	total_records: z.number(),
	tenant_name: z.string().nullable().optional(),
	tenant_id: z.string().nullable().optional(),
});
const jobsSchema = z.object({ jobs: z.array(jobSchema), total: z.number() });
const jobEventsSchema = z.object({
	events: z.array(z.object({ id: z.number(), event_type: z.string(), created_at: z.string() })),
	total: z.number(),
});
const jobResultsSchema = z.object({
	results: z.array(
		z.object({ id: z.number(), task_state: z.string(), error: z.string().nullable().optional() }),
	),
	total: z.number(),
});
type Action =
	| "createTenant"
	| "updateTenant"
	| "deleteTenant"
	| "updateQuota"
	| "resetQuota"
	| "createKey"
	| "updateKey"
	| "revokeKey"
	| "reactivateKey";

export function PlatformOperationRunner({
	operations,
	className,
	...props
}: PlatformOperationRunnerProps) {
	const client = useQueryClient();
	const [tenantId, setTenantId] = useState("");
	const [keyId, setKeyId] = useState("");
	const [jobId, setJobId] = useState<number>();
	const [reason, setReason] = useState("");
	const [name, setName] = useState("");
	const [slug, setSlug] = useState("");
	const [email, setEmail] = useState("");
	const [status, setStatus] = useState("active");
	const [quotaInput, setQuotaInput] = useState("");
	const [keyName, setKeyName] = useState("");
	const [keyScopes, setKeyScopes] = useState("verify");
	const [revealedKey, setRevealedKey] = useState("");
	const [notice, setNotice] = useState("");
	const [validation, setValidation] = useState("");
	const headers = { "x-admin-reason": reason.trim() };
	const tenants = useQuery({
		queryKey: workflowQueryKey("operator-tenants"),
		queryFn: async () => (await admin.listTenants({ limit: 100 })).data,
	});
	const tenant = useQuery({
		queryKey: workflowQueryKey("operator-tenant", tenantId),
		enabled: !!tenantId,
		queryFn: async () => (await admin.getTenant({ tenantId })).data,
	});
	const quota = useQuery({
		queryKey: workflowQueryKey("operator-quota", tenantId),
		enabled: !!tenantId,
		queryFn: async () => (await admin.getTenantQuota({ tenantId })).data,
	});
	const globalKeys = useQuery({
		queryKey: workflowQueryKey("operator-all-keys"),
		queryFn: async () => (await admin.listAllApiKeys({ limit: 100 })).data,
	});
	const tenantKeys = useQuery({
		queryKey: workflowQueryKey("operator-keys", tenantId),
		enabled: !!tenantId,
		queryFn: async () => (await admin.listApiKeys({ tenantId })).data,
	});
	const key = useQuery({
		queryKey: workflowQueryKey("operator-key", tenantId, keyId),
		enabled: !!tenantId && !!keyId,
		queryFn: async () => (await admin.getApiKey({ tenantId, keyId })).data,
	});
	const allJobs = useQuery({
		queryKey: workflowQueryKey("operator-jobs"),
		queryFn: async () => jobsSchema.parse((await jobs.listJobs({ limit: 50 })).data),
	});
	const tenantJobs = useQuery({
		queryKey: workflowQueryKey("operator-tenant-jobs", tenantId),
		enabled: !!tenantId,
		queryFn: async () =>
			jobsSchema.parse((await jobs.listTenantJobs({ tenantId, limit: 50 })).data),
	});
	const job = useQuery({
		queryKey: workflowQueryKey("operator-job", jobId),
		enabled: !!jobId,
		queryFn: async () => {
			if (!jobId) throw new Error("Select a job.");
			return jobSchema.parse((await jobs.getJob({ jobId })).data);
		},
	});
	const events = useQuery({
		queryKey: workflowQueryKey("operator-job-events", jobId),
		enabled: !!jobId,
		queryFn: async () => {
			if (!jobId) throw new Error("Select a job.");
			return jobEventsSchema.parse((await jobs.getJobEvents({ jobId, limit: 50 })).data);
		},
	});
	const results = useQuery({
		queryKey: workflowQueryKey("operator-job-results", jobId),
		enabled: !!jobId,
		queryFn: async () => {
			if (!jobId) throw new Error("Select a job.");
			return jobResultsSchema.parse((await jobs.getJobResults({ jobId, limit: 50 })).data);
		},
	});
	const action = useMutation({
		mutationFn: async (kind: Action) => {
			if (kind === "createTenant") {
				if (!name.trim() || !slug.trim() || !email.trim())
					throw new Error("Name, slug, and contact email are required.");
				const data = (
					await admin.createTenant(
						{
							adminCreateTenantRequest: {
								name: name.trim(),
								slug: slug.trim(),
								contact_email: email.trim(),
							},
						},
						{ headers },
					)
				).data;
				if (data.id) setTenantId(data.id);
			}
			if (kind === "updateTenant")
				await admin.updateTenant(
					{ tenantId, adminUpdateTenantRequest: { name: name.trim(), status } },
					{ headers },
				);
			if (kind === "deleteTenant") await admin.deleteTenant({ tenantId }, { headers });
			if (kind === "updateQuota") {
				const value = Number(quotaInput);
				if (!Number.isSafeInteger(value) || value < 0)
					throw new Error("Enter a nonnegative monthly limit.");
				await admin.updateTenantQuota(
					{ tenantId, adminUpdateQuotaRequest: { monthly_email_limit: value } },
					{ headers },
				);
			}
			if (kind === "resetQuota") await admin.resetTenantQuota({ tenantId }, { headers });
			const scopes = keyScopes
				.split(/[\s,]+/)
				.map((scope) => scope.trim())
				.filter(Boolean);
			if (kind === "createKey") {
				if (!keyName.trim() || !scopes.length)
					throw new Error("Enter a name and at least one scope.");
				const data = (
					await admin.createApiKey(
						{ tenantId, adminApiKeyWriteRequest: { name: keyName.trim(), scopes } },
						{ headers },
					)
				).data;
				setRevealedKey(z.object({ key: z.string() }).parse(data).key);
			}
			if (kind === "updateKey")
				await admin.updateApiKey(
					{ tenantId, keyId, adminApiKeyWriteRequest: { name: keyName.trim(), scopes } },
					{ headers },
				);
			if (kind === "revokeKey") await admin.revokeApiKey({ tenantId, keyId }, { headers });
			if (kind === "reactivateKey") await admin.reactivateApiKey({ tenantId, keyId }, { headers });
			return kind;
		},
		onSuccess: async (kind) => {
			setNotice(`Operator ${kind} action completed.`);
			if (kind === "deleteTenant") {
				setTenantId("");
				setKeyId("");
			}
			await Promise.all([
				client.invalidateQueries({ queryKey: workflowQueryKey("operator-tenants") }),
				client.invalidateQueries({ queryKey: workflowQueryKey("operator-tenant", tenantId) }),
				client.invalidateQueries({ queryKey: workflowQueryKey("operator-quota", tenantId) }),
				client.invalidateQueries({ queryKey: workflowQueryKey("operator-all-keys") }),
				client.invalidateQueries({ queryKey: workflowQueryKey("operator-keys", tenantId) }),
				client.invalidateQueries({ queryKey: workflowQueryKey("operator-key", tenantId, keyId) }),
			]);
		},
	});
	function run(kind: Action) {
		if (reason.trim().length < 10) {
			setValidation("Enter a reason of at least 10 characters.");
			return;
		}
		if (
			(kind !== "createTenant" && !tenantId) ||
			(["updateKey", "revokeKey", "reactivateKey"].includes(kind) && !keyId)
		) {
			setValidation("Select a tenant and key first.");
			return;
		}
		setValidation("");
		if (window.confirm(`Confirm ${kind} for ${tenantId || slug}? Reason: ${reason.trim()}`))
			action.mutate(kind);
	}
	const error = [
		tenants,
		tenant,
		quota,
		globalKeys,
		tenantKeys,
		key,
		allJobs,
		tenantJobs,
		job,
		events,
		results,
		action,
	].find((request) => request.isError)?.error;
	return (
		<section
			data-slot="platform-operation-runner"
			className={cn("space-y-6", className)}
			{...props}
		>
			<p className="text-sm text-muted-foreground">
				{operations.length} restricted operations are grouped into tenant, key, and job workflows.
				Reads and changes pass through the audited operator route.
			</p>
			{validation || error ? (
				<p role="alert" className="text-destructive">
					{validation || error?.message} If recent verification is required, complete it in{" "}
					<Link className="underline" href="/app/settings?settings=security">
						Security settings
					</Link>
					.
				</p>
			) : null}
			{notice ? <p role="status">{notice}</p> : null}
			<Card>
				<CardHeader>
					<CardTitle>Operator change reason</CardTitle>
				</CardHeader>
				<CardContent>
					<Label htmlFor="operator-reason">Reason for changes</Label>
					<Input
						id="operator-reason"
						minLength={10}
						maxLength={255}
						value={reason}
						onChange={(event) => {
							setReason(event.target.value);
						}}
						placeholder="Ticket and purpose"
					/>
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Tenant directory</CardTitle>
				</CardHeader>
				<CardContent className="space-y-4">
					{tenants.data?.tenants?.length ? (
						<ul className="divide-y">
							{tenants.data.tenants.map((item) => (
								<li className="flex items-center justify-between gap-3 py-2" key={item.id}>
									<div>
										<p className="font-medium">
											{item.name} · {item.status}
										</p>
										<p className="text-xs text-muted-foreground">
											{item.id} · {item.slug}
										</p>
									</div>
									<Button
										size="sm"
										variant="outline"
										onClick={() => {
											setTenantId(item.id || "");
											setKeyId("");
											setName(item.name || "");
											setStatus(item.status || "active");
											setQuotaInput(
												item.monthly_email_limit == null ? "" : String(item.monthly_email_limit),
											);
										}}
									>
										Inspect
									</Button>
								</li>
							))}
						</ul>
					) : (
						<p>No tenants found.</p>
					)}
					<form
						className="grid gap-3 sm:grid-cols-3"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							run("createTenant");
						}}
					>
						<div>
							<Label htmlFor="operator-tenant-name">Tenant name</Label>
							<Input
								id="operator-tenant-name"
								value={name}
								onChange={(event) => {
									setName(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="operator-tenant-slug">Slug</Label>
							<Input
								id="operator-tenant-slug"
								value={slug}
								onChange={(event) => {
									setSlug(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="operator-tenant-email">Contact email</Label>
							<Input
								id="operator-tenant-email"
								type="email"
								value={email}
								onChange={(event) => {
									setEmail(event.target.value);
								}}
							/>
						</div>
						<Button type="submit" disabled={action.isPending}>
							Create tenant
						</Button>
					</form>
				</CardContent>
			</Card>
			{tenantId ? (
				<Card>
					<CardHeader>
						<CardTitle>Tenant {tenant.data?.name || tenantId}</CardTitle>
					</CardHeader>
					<CardContent className="space-y-4">
						<p>
							{tenant.data?.status} · {tenant.data?.plan_tier} · {tenant.data?.contact_email}
						</p>
						<div className="flex flex-wrap gap-2">
							<Label htmlFor="operator-status">Status</Label>
							<select
								id="operator-status"
								className="h-9 rounded border bg-background px-3"
								value={status}
								onChange={(event) => {
									setStatus(event.target.value);
								}}
							>
								<option value="active">Active</option>
								<option value="suspended">Suspended</option>
							</select>
							<Button
								variant="outline"
								disabled={action.isPending}
								onClick={() => {
									run("updateTenant");
								}}
							>
								Save tenant
							</Button>
							<Button
								variant="destructive"
								disabled={action.isPending}
								onClick={() => {
									run("deleteTenant");
								}}
							>
								Delete tenant
							</Button>
						</div>
						<p>
							Quota: {quota.data?.used_this_period ?? 0} used ·{" "}
							{quota.data?.quota_unlimited
								? "Unlimited"
								: (quota.data?.monthly_email_limit ?? "Unset")}{" "}
							monthly limit
						</p>
						<div className="flex flex-wrap gap-2">
							<Label className="sr-only" htmlFor="operator-quota">
								Monthly email limit
							</Label>
							<Input
								id="operator-quota"
								inputMode="numeric"
								placeholder="Monthly limit"
								value={quotaInput}
								onChange={(event) => {
									setQuotaInput(event.target.value);
								}}
							/>
							<Button
								variant="outline"
								disabled={action.isPending}
								onClick={() => {
									run("updateQuota");
								}}
							>
								Set quota
							</Button>
							<Button
								variant="outline"
								disabled={action.isPending}
								onClick={() => {
									run("resetQuota");
								}}
							>
								Reset usage
							</Button>
						</div>
						<p className="text-sm">
							Tenant jobs: {tenantJobs.data?.total ?? 0}. Select a job below to inspect results and
							events.
						</p>
					</CardContent>
				</Card>
			) : null}
			<Card>
				<CardHeader>
					<CardTitle>API key oversight</CardTitle>
				</CardHeader>
				<CardContent className="space-y-4">
					<p>
						Platform keys: {globalKeys.data?.total ?? globalKeys.data?.api_keys?.length ?? 0} ·
						Selected tenant keys: {tenantKeys.data?.api_keys?.length ?? 0}
					</p>
					{tenantKeys.data?.api_keys?.length ? (
						<ul className="divide-y">
							{tenantKeys.data.api_keys.map((item) => (
								<li className="flex items-center justify-between gap-2 py-2" key={item.id}>
									<div>
										<p>
											{item.name} · {item.status}
										</p>
										<p className="text-xs">
											{item.key_prefix}… · {item.scopes?.join(", ")}
										</p>
									</div>
									<Button
										variant="outline"
										size="sm"
										onClick={() => {
											setKeyId(item.id || "");
											setKeyName(item.name || "");
											setKeyScopes(item.scopes?.join(", ") || "verify");
										}}
									>
										Inspect
									</Button>
								</li>
							))}
						</ul>
					) : (
						<p>Select a tenant to review its keys.</p>
					)}
					{tenantId ? (
						<div className="space-y-3">
							<div className="flex flex-wrap gap-2">
								<div>
									<Label htmlFor="operator-key-name">Key name</Label>
									<Input
										id="operator-key-name"
										value={keyName}
										onChange={(event) => {
											setKeyName(event.target.value);
										}}
									/>
								</div>
								<div>
									<Label htmlFor="operator-key-scopes">Scopes, comma separated</Label>
									<Input
										id="operator-key-scopes"
										value={keyScopes}
										onChange={(event) => {
											setKeyScopes(event.target.value);
										}}
									/>
								</div>
							</div>
							<div className="flex flex-wrap gap-2">
								<Button
									disabled={action.isPending || Boolean(revealedKey)}
									onClick={() => {
										run("createKey");
									}}
								>
									Create key
								</Button>
								{keyId ? (
									<>
										<Button
											variant="outline"
											disabled={action.isPending}
											onClick={() => {
												run("updateKey");
											}}
										>
											Update key
										</Button>
										<Button
											variant="outline"
											disabled={action.isPending}
											onClick={() => {
												run("reactivateKey");
											}}
										>
											Reactivate
										</Button>
										<Button
											variant="destructive"
											disabled={action.isPending}
											onClick={() => {
												run("revokeKey");
											}}
										>
											Revoke
										</Button>
									</>
								) : null}
							</div>
							{key.data ? (
								<p className="text-sm">
									Selected key {key.data.key_prefix}… · {key.data.status} · Last used{" "}
									{key.data.last_used_at || "never"}
								</p>
							) : null}
						</div>
					) : null}
					{revealedKey ? (
						<div role="status" className="rounded border p-3">
							<Label htmlFor="operator-new-key">New key — copy once</Label>
							<Input id="operator-new-key" readOnly value={revealedKey} />
							<Button
								variant="outline"
								onClick={() => {
									void navigator.clipboard.writeText(revealedKey);
								}}
							>
								Copy
							</Button>
							<Button
								variant="ghost"
								onClick={() => {
									setRevealedKey("");
								}}
							>
								Done
							</Button>
						</div>
					) : null}
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Job monitoring</CardTitle>
				</CardHeader>
				<CardContent className="space-y-4">
					<p>{allJobs.data?.total ?? 0} recent platform jobs</p>
					{allJobs.data?.jobs.length ? (
						<ul className="divide-y">
							{allJobs.data.jobs.map((item) => (
								<li className="flex items-center justify-between gap-2 py-2" key={item.job_id}>
									<span>
										Job {String(item.job_id)} · {item.tenant_name || item.tenant_id || "Legacy"} ·{" "}
										{item.status}
									</span>
									<Button
										size="sm"
										variant="outline"
										onClick={() => {
											setJobId(item.job_id);
										}}
									>
										Inspect
									</Button>
								</li>
							))}
						</ul>
					) : (
						<p>No jobs found.</p>
					)}
					{job.data ? (
						<div className="rounded border p-3">
							<p className="font-medium">
								Job {String(job.data.job_id)} · {job.data.status} · {job.data.total_records} records
							</p>
							<p>
								Events: {events.data?.total ?? 0} · Results: {results.data?.total ?? 0}
							</p>
							<ol className="mt-2 space-y-1 text-sm">
								{events.data?.events.map((item) => (
									<li key={item.id}>
										{item.event_type} · {new Date(item.created_at).toLocaleString()}
									</li>
								))}
							</ol>
							<ul className="mt-2 space-y-1 text-sm">
								{results.data?.results.map((item) => (
									<li key={item.id}>
										Task {String(item.id)} · {item.task_state}
										{item.error ? ` · ${item.error}` : ""}
									</li>
								))}
							</ul>
						</div>
					) : null}
				</CardContent>
			</Card>
		</section>
	);
}
