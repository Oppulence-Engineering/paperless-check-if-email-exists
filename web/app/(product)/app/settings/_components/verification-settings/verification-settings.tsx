"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, JobsApi, TenantApi, V1Api } from "@oppulence/reacher-sdk";
import { z } from "zod";

import { Alert, AlertDescription } from "@oppulence/ui/components/alert";
import { Button } from "@oppulence/ui/components/button";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@oppulence/ui/components/select";
import { cn } from "@oppulence/ui/lib/utils";
import { ScorePolicyList } from "@/components/features/automation/score-policy-list/score-policy-list";
import { Group, Pill, Row, Section } from "@/components/platform/page-kit";

import {
	V1GetTenantSettings200Response,
	V1GetTenantUsage200Response,
	V1GetTenantWebhook200Response,
	V1UpdateTenantSettings200Response,
	V1UpdateTenantSettingsBody,
	V1UpdateTenantWebhook200Response,
	V1UpdateTenantWebhookBody,
} from "@/lib/api/generated/zod/tenant/tenant";
import { V1ReverificationStatus200Response } from "@/lib/api/generated/zod/v1/v1";
import { JobStatusSchema } from "@/lib/api/schemas/jobs";
import { jobStatus } from "@/lib/verification/labels";
import { useAuthSession, useEntitlements } from "@/components/auth/auth-gate";
import { authClient } from "@/lib/auth/auth-client";
import { BillingPlan } from "../billing-plan/billing-plan";
import { type VerificationSettingsPropsFields } from "./verification-settings.schema";

/** @oppulence-gen kind=component */
export type VerificationSettingsProps = VerificationSettingsPropsFields &
	ComponentPropsWithoutRef<"section">;

const sdk = new TenantApi(new Configuration({ basePath: "/api/backend" }));
const v1Api = new V1Api(new Configuration({ basePath: "/api/backend" }));
const jobsApi = new JobsApi(new Configuration({ basePath: "/api/backend" }));
const policyModes = [
	["growth", "Growth"],
	["deliverability", "Deliverability"],
	["signup_protection", "Signup protection"],
	["enterprise_strict", "Enterprise strict"],
	["custom", "Custom (review only)"],
] as const;

function field(form: FormData, name: string) {
	const value = form.get(name);
	return typeof value === "string" ? value : "";
}

export function VerificationSettings({
	className,
	organizationId,
	organizationRole,
	scope = "all",
	chosenPlan,
	...props
}: VerificationSettingsProps) {
	const queryClient = useQueryClient();
	const canManage = organizationRole
		.split(",")
		.some((role) => ["owner", "admin"].includes(role.trim()));
	const showUsage = scope === "all" || scope === "usage";
	const { plan } = useEntitlements();
	const billingEnabled = useAuthSession().billing?.enabled ?? false;
	const [billingError, setBillingError] = useState<string>();
	const [billingPending, setBillingPending] = useState(false);
	// Checkout and the billing portal redirect to Stripe; an error keeps the person here.
	async function billingRedirect(run: () => Promise<{ error?: { message?: string } | null }>) {
		setBillingPending(true);
		setBillingError(undefined);
		const result = await run();
		if (result.error) {
			setBillingError(result.error.message || "Billing is unavailable. Try again.");
			setBillingPending(false);
		}
	}
	const showDefaults = scope === "all" || scope === "verification";
	const showWebhook = scope === "all" || scope === "webhook";
	const heading = {
		all: ["Settings", "Manage verification defaults, webhooks, and usage."],
		verification: ["Verification", "Choose the default check policy and result retention."],
		usage: ["Usage", "Review the current verification allowance for this workspace."],
		webhook: ["Webhooks", "Configure pipeline delivery and its signing secret."],
	}[scope];
	const settingsKey = ["tenant-settings", organizationId];
	const usageKey = ["tenant-usage", organizationId];
	const webhookKey = workflowQueryKey("tenant-webhook");
	const [notice, setNotice] = useState<string>();
	const [error, setError] = useState<string>();
	const settings = useQuery({
		queryKey: settingsKey,
		enabled: canManage && showDefaults,
		queryFn: async ({ signal }) =>
			V1GetTenantSettings200Response.parse((await sdk.v1GetTenantSettings({ signal })).data),
	});
	const usage = useQuery({
		queryKey: usageKey,
		enabled: showUsage,
		queryFn: async ({ signal }) =>
			V1GetTenantUsage200Response.parse((await sdk.v1GetTenantUsage({ signal })).data),
	});
	const webhook = useQuery({
		queryKey: webhookKey,
		enabled: canManage && showWebhook,
		queryFn: async ({ signal }) =>
			V1GetTenantWebhook200Response.parse((await sdk.v1GetTenantWebhook({ signal })).data),
	});
	const reverification = useQuery({
		queryKey: workflowQueryKey("reverification-status", organizationId),
		enabled: showDefaults,
		queryFn: async ({ signal }) =>
			V1ReverificationStatus200Response.parse(
				(await v1Api.v1ReverificationStatus({ signal })).data,
			),
	});
	const lastJobId = reverification.data?.last_job_id;
	const lastJob = useQuery({
		queryKey: workflowQueryKey("reverification-status", organizationId, "job", lastJobId),
		enabled: lastJobId != null,
		queryFn: async ({ signal }) =>
			JobStatusSchema.parse(
				(await jobsApi.v1GetJobStatus({ jobId: Number(lastJobId) }, { signal })).data,
			),
	});
	const saveReverification = useMutation({
		mutationFn: async (body: { enabled: boolean; staleness_days: number; batch_size: number }) =>
			V1ReverificationStatus200Response.parse(
				(
					await v1Api.v1UpdateReverification({
						updateReverificationRequest: body,
					})
				).data,
			),
		onSuccess: (data) => {
			queryClient.setQueryData(workflowQueryKey("reverification-status", organizationId), data);
			setNotice(data.enabled ? "Scheduled re-checks are on." : "Scheduled re-checks are off.");
		},
	});
	function submitReverification(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		saveReverification.mutate({
			enabled: form.get("reverification-enabled") === "on",
			staleness_days: Number(field(form, "staleness")),
			batch_size: Number(field(form, "batch")),
		});
	}
	const saveSettings = useMutation({
		mutationFn: async (body: z.input<typeof V1UpdateTenantSettingsBody>) =>
			V1UpdateTenantSettings200Response.parse(
				(
					await sdk.v1UpdateTenantSettings({
						updateTenantSettingsRequest: V1UpdateTenantSettingsBody.parse(body),
					})
				).data,
			),
		onSuccess: async () => {
			await queryClient.invalidateQueries({ queryKey: settingsKey });
		},
	});
	const saveWebhook = useMutation({
		mutationFn: async (body: z.input<typeof V1UpdateTenantWebhookBody>) =>
			V1UpdateTenantWebhook200Response.parse(
				(
					await sdk.v1UpdateTenantWebhook({
						updateWebhookRequest: V1UpdateTenantWebhookBody.parse(body),
					})
				).data,
			),
		onSuccess: async () => {
			await Promise.all([
				queryClient.invalidateQueries({ queryKey: webhookKey }),
				queryClient.invalidateQueries({ queryKey: settingsKey }),
			]);
		},
	});
	const clearWebhook = useMutation({
		mutationFn: async () => {
			await sdk.v1ClearTenantWebhook();
		},
		onSuccess: async () => {
			await Promise.all([
				queryClient.invalidateQueries({ queryKey: webhookKey }),
				queryClient.invalidateQueries({ queryKey: settingsKey }),
			]);
		},
	});
	const busy = saveSettings.isPending || saveWebhook.isPending || clearWebhook.isPending;

	async function run(action: () => Promise<unknown>, message: string) {
		setError(undefined);
		setNotice(undefined);
		try {
			await action();
			setNotice(message);
		} catch (cause) {
			setError(errorMessage(cause));
		}
	}

	async function submitSettings(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		await run(
			() =>
				saveSettings.mutateAsync({
					result_retention_days: Number(field(form, "retention")),
					default_policy_mode: field(form, "policy"),
					approval_excellent_threshold: Number(field(form, "excellent")),
					approval_good_threshold: Number(field(form, "good")),
				}),
			"Verification settings saved.",
		);
	}

	async function submitWebhook(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const formElement = event.currentTarget;
		const form = new FormData(formElement);
		const secret = field(form, "secret");
		await run(async () => {
			await saveWebhook.mutateAsync({
				default_webhook_url: field(form, "url").trim() || null,
				...(secret ? { webhook_signing_secret: secret } : {}),
			});
			formElement.reset();
		}, "Webhook saved.");
	}

	const fieldClass = "h-10 rounded-lg md:w-72";
	const value = "text-sm font-medium text-white tabular-nums";
	return (
		<section
			data-slot="verification-settings"
			className={cn("flex flex-col gap-10", className)}
			{...props}
		>
			<Section description={heading[1]} title={heading[0]}>
				<div className="flex flex-col gap-10">
					{error ? (
						<Alert variant="destructive">
							<AlertDescription>{error}</AlertDescription>
						</Alert>
					) : null}
					{notice ? (
						<Alert role="status">
							<AlertDescription>{notice}</AlertDescription>
						</Alert>
					) : null}
					{showUsage ? (
						<>
							{usage.isError ? (
								<Alert variant="destructive">
									<AlertDescription>
										Usage is unavailable.{" "}
										<Button size="sm" variant="ghost" onClick={() => void usage.refetch()}>
											Retry
										</Button>
									</AlertDescription>
								</Alert>
							) : null}
							<Group>
								{usage.isPending ? (
									<Row>
										<p className="text-sm text-[#858585]">Loading usage…</p>
									</Row>
								) : usage.data ? (
									<>
										<Row label="Plan" layout="inline">
											<span className={value}>{usage.data.plan_tier}</span>
										</Row>
										<Row label="Used this period" layout="inline">
											<span className={value}>{usage.data.used_this_period.toLocaleString()}</span>
										</Row>
										<Row label="Monthly limit" layout="inline">
											<span className={value}>
												{usage.data.quota_unlimited
													? "Unlimited"
													: usage.data.monthly_email_limit?.toLocaleString()}
											</span>
										</Row>
										<Row label="Remaining" layout="inline">
											<span className={value}>
												{usage.data.quota_unlimited
													? "Unlimited"
													: usage.data.quota_remaining?.toLocaleString()}
											</span>
										</Row>
										<Row label="Resets" layout="inline">
											<time className={value} dateTime={usage.data.period_reset_at}>
												{new Date(usage.data.period_reset_at).toLocaleString()}
											</time>
										</Row>
									</>
								) : null}
							</Group>
							<Section
								description="Change the plan or manage payment for this workspace."
								title="Plan"
							>
								<Group>
									<BillingPlan
										billingEnabled={billingEnabled}
										canManage={canManage}
										chosen={chosenPlan}
										error={billingError}
										onManage={() =>
											void billingRedirect(() =>
												authClient.subscription.billingPortal({
													referenceId: organizationId,
													customerType: "organization",
													returnUrl: "/app/settings?settings=usage",
												}),
											)
										}
										onUpgrade={(next) =>
											void billingRedirect(() =>
												authClient.subscription.upgrade({
													plan: next,
													referenceId: organizationId,
													customerType: "organization",
													successUrl: "/app/settings?settings=usage&billing=success",
													cancelUrl: "/app/settings?settings=usage",
												}),
											)
										}
										pending={billingPending}
										plan={plan}
									/>
								</Group>
							</Section>
						</>
					) : null}
					{canManage && showDefaults ? (
						<Section
							description="Completed check and finder results are removed hourly after this period. Choose the default policy for checks."
							title="Verification defaults"
						>
							{settings.isError ? (
								<Alert variant="destructive">
									<AlertDescription>
										Settings are unavailable.{" "}
										<Button size="sm" variant="ghost" onClick={() => void settings.refetch()}>
											Retry
										</Button>
									</AlertDescription>
								</Alert>
							) : null}
							{settings.isPending ? (
								<Group>
									<Row>
										<p className="text-sm text-[#858585]">Loading settings…</p>
									</Row>
								</Group>
							) : settings.data ? (
								<form
									className="flex flex-col gap-4"
									key={[
										settings.data.result_retention_days,
										settings.data.default_policy_mode,
										settings.data.approval_excellent_threshold,
										settings.data.approval_good_threshold,
									].join(":")}
									onSubmit={(event) => void submitSettings(event)}
								>
									<Group>
										<Row label={<Label htmlFor="result-retention">Result retention (days)</Label>}>
											<Input
												className={fieldClass}
												id="result-retention"
												name="retention"
												type="number"
												min={1}
												step={1}
												defaultValue={settings.data.result_retention_days}
												required
											/>
										</Row>
										<Row label={<Label htmlFor="default-policy">Default policy</Label>}>
											<Select
												defaultValue={settings.data.default_policy_mode}
												name="policy"
												required
											>
												<SelectTrigger
													className="h-10 w-full rounded-lg md:w-72"
													id="default-policy"
												>
													<SelectValue />
												</SelectTrigger>
												<SelectContent>
													{policyModes.map(([mode, label]) => (
														<SelectItem key={mode} value={mode}>
															{label}
														</SelectItem>
													))}
												</SelectContent>
											</Select>
										</Row>
										<Row
											label={
												<Label htmlFor="approval-excellent">Excellent list (% safe to send)</Label>
											}
										>
											<Input
												className={fieldClass}
												id="approval-excellent"
												name="excellent"
												type="number"
												min={0}
												max={100}
												step={0.1}
												defaultValue={settings.data.approval_excellent_threshold}
												required
											/>
										</Row>
										<Row
											description="A job below this value is not ready to send."
											label={<Label htmlFor="approval-good">Good list (% safe to send)</Label>}
										>
											<Input
												className={fieldClass}
												id="approval-good"
												name="good"
												type="number"
												min={0}
												max={100}
												step={0.1}
												defaultValue={settings.data.approval_good_threshold}
												required
											/>
										</Row>
									</Group>
									<div className="flex justify-end">
										<Button className="rounded-lg" disabled={busy} type="submit">
											Save verification defaults
										</Button>
									</div>
								</form>
							) : null}
						</Section>
					) : null}
					{showDefaults ? (
						<Section
							id="reverification"
							description="Re-check stale addresses across this workspace. Each submitted address uses one verification credit; the batch size caps usage per run. Suppressed addresses are excluded."
							title="Reverification schedule"
						>
							<form
								className="flex flex-col gap-4"
								key={[
									reverification.data?.enabled,
									reverification.data?.staleness_days,
									reverification.data?.batch_size,
								].join(":")}
								onSubmit={submitReverification}
							>
								<Group>
									{reverification.isPending ? (
										<Row>
											<p className="text-sm text-[#858585]" role="status">
												Loading reverification status…
											</p>
										</Row>
									) : null}
									{reverification.isError ? (
										<Row>
											<p className="text-sm text-red-300" role="alert">
												{errorMessage(reverification.error)}
											</p>
										</Row>
									) : null}
									{reverification.data ? (
										<Row
											description={[
												reverification.data.next_run_at
													? `Next run: ${new Date(reverification.data.next_run_at).toLocaleString()}`
													: null,
												!canManage && reverification.data.staleness_days
													? `Staleness threshold: ${String(reverification.data.staleness_days)} days`
													: null,
											]
												.filter(Boolean)
												.join(" · ")}
											label="Status"
											layout="inline"
										>
											<Pill tone={reverification.data.enabled ? "green" : "gray"}>
												{reverification.data.enabled ? "Enabled" : "Disabled"}
											</Pill>
										</Row>
									) : null}
									{reverification.data ? (
										<Row
											description={
												reverification.data.last_run_at
													? [
															lastJob.data
																? `${lastJob.data.total_records.toLocaleString()} addresses in this run`
																: null,
															`${(reverification.data.emails_requeued ?? 0).toLocaleString()} addresses sent for a new check in all runs`,
														]
															.filter(Boolean)
															.join(" · ")
													: "No run yet."
											}
											label="Last run"
										>
											<div className="flex flex-wrap items-center gap-2">
												{reverification.data.last_run_at ? (
													<time className={value} dateTime={reverification.data.last_run_at}>
														{new Date(reverification.data.last_run_at).toLocaleString()}
													</time>
												) : null}
												{lastJob.data ? (
													<Pill tone={jobStatus.tone(lastJob.data.status)}>
														{jobStatus.label(lastJob.data.status)}
													</Pill>
												) : null}
												{lastJobId != null ? (
													<Link
														className="text-sm text-neutral-300 underline underline-offset-4 hover:text-white"
														href={`/app/jobs?job=${String(lastJobId)}`}
													>
														Open job {lastJobId}
													</Link>
												) : null}
											</div>
										</Row>
									) : null}
									{reverification.data && canManage ? (
										<>
											<Row
												label={
													<Label htmlFor="reverification-enabled">
														Re-check results on a schedule
													</Label>
												}
												layout="inline"
											>
												<Checkbox
													defaultChecked={reverification.data.enabled}
													id="reverification-enabled"
													name="reverification-enabled"
													value="on"
												/>
											</Row>
											<Row
												label={
													<Label htmlFor="reverification-staleness">Re-check after (days)</Label>
												}
											>
												<Input
													className={fieldClass}
													defaultValue={reverification.data.staleness_days ?? 30}
													id="reverification-staleness"
													max={365}
													min={1}
													name="staleness"
													required
													type="number"
												/>
											</Row>
											<Row label={<Label htmlFor="reverification-batch">Addresses per run</Label>}>
												<Input
													className={fieldClass}
													defaultValue={reverification.data.batch_size ?? 100}
													id="reverification-batch"
													max={1000}
													min={1}
													name="batch"
													required
													type="number"
												/>
											</Row>
										</>
									) : null}
								</Group>
								{reverification.data && canManage ? (
									<>
										{saveReverification.isError ? (
											<p className="text-sm text-red-300" role="alert">
												{errorMessage(saveReverification.error)}
											</p>
										) : null}
										<div className="flex justify-end">
											<Button
												className="rounded-lg"
												disabled={saveReverification.isPending}
												type="submit"
											>
												Save re-check schedule
											</Button>
										</div>
									</>
								) : null}
							</form>
						</Section>
					) : null}
					{canManage && showDefaults ? (
						<Section
							description="A score policy decides send, review, or suppress for each address in a list. A list or a pipeline can name a policy. Other lists use the default policy."
							title="Score policies"
						>
							<ScorePolicyList />
						</Section>
					) : null}
					{canManage && showWebhook ? (
						<Section
							description="Send pipeline run summaries to an HTTPS endpoint. The signing secret is never shown again."
							title="Webhook"
						>
							{webhook.isError ? (
								<Alert variant="destructive">
									<AlertDescription>
										Webhook settings are unavailable.{" "}
										<Button size="sm" variant="ghost" onClick={() => void webhook.refetch()}>
											Retry
										</Button>
									</AlertDescription>
								</Alert>
							) : null}
							{webhook.isPending ? (
								<Group>
									<Row>
										<p className="text-sm text-[#858585]">Loading webhook…</p>
									</Row>
								</Group>
							) : webhook.data ? (
								<>
									<form
										className="flex flex-col gap-4"
										key={webhook.data.default_webhook_url ?? ""}
										onSubmit={(event) => void submitWebhook(event)}
									>
										<Group>
											<Row label={<Label htmlFor="webhook-url">Webhook URL</Label>}>
												<Input
													className={fieldClass}
													id="webhook-url"
													name="url"
													type="url"
													pattern="https://.*"
													placeholder="https://example.com/webhook"
													defaultValue={webhook.data.default_webhook_url ?? ""}
												/>
											</Row>
											<Row
												description={
													webhook.data.webhook_signing_secret_configured
														? "A signing secret is configured."
														: "No signing secret is configured."
												}
												label={<Label htmlFor="webhook-secret">Signing secret</Label>}
											>
												<Input
													className={fieldClass}
													id="webhook-secret"
													name="secret"
													type="password"
													autoComplete="new-password"
													placeholder={
														webhook.data.webhook_signing_secret_configured
															? "Configured — leave blank to keep"
															: "Optional"
													}
												/>
											</Row>
										</Group>
										<div className="flex flex-wrap justify-end gap-2">
											{webhook.data.default_webhook_url ||
											webhook.data.webhook_signing_secret_configured ? (
												<Button
													className="rounded-lg"
													disabled={busy}
													type="button"
													variant="destructive"
													onClick={() => {
														if (window.confirm("Clear the webhook URL and signing secret?"))
															void run(() => clearWebhook.mutateAsync(), "Webhook cleared.");
													}}
												>
													Clear webhook
												</Button>
											) : null}
											{webhook.data.webhook_signing_secret_configured ? (
												<Button
													className="rounded-lg"
													disabled={busy}
													type="button"
													variant="outline"
													onClick={() =>
														void run(
															() =>
																saveWebhook.mutateAsync({
																	webhook_signing_secret: null,
																}),
															"Signing secret removed.",
														)
													}
												>
													Remove signing secret
												</Button>
											) : null}
											<Button className="rounded-lg" disabled={busy} type="submit">
												Save webhook
											</Button>
										</div>
									</form>
								</>
							) : null}
						</Section>
					) : null}
					{!canManage && (showDefaults || showWebhook) ? (
						<Alert>
							<AlertDescription>
								An organization owner or administrator can change verification defaults and
								webhooks. Usage is available to all members.
							</AlertDescription>
						</Alert>
					) : null}
				</div>
			</Section>
		</section>
	);
}
