"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, TenantApi } from "@oppulence/reacher-sdk";
import { z } from "zod";

import { Alert, AlertDescription } from "@oppulence/ui/components/alert";
import { Button } from "@oppulence/ui/components/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from "@oppulence/ui/components/card";
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

import {
	V1GetTenantSettings200Response,
	V1GetTenantUsage200Response,
	V1GetTenantWebhook200Response,
	V1UpdateTenantSettings200Response,
	V1UpdateTenantSettingsBody,
	V1UpdateTenantWebhook200Response,
	V1UpdateTenantWebhookBody,
} from "@/lib/api/generated/zod/tenant/tenant";
import { type VerificationSettingsPropsFields } from "./verification-settings.schema";

/** @oppulence-gen kind=component */
export type VerificationSettingsProps = VerificationSettingsPropsFields &
	ComponentPropsWithoutRef<"section">;

const sdk = new TenantApi(new Configuration({ basePath: "/api/backend" }));
const policyModes = [
	["growth", "Growth"],
	["deliverability", "Deliverability"],
	["signup_protection", "Signup protection"],
	["enterprise_strict", "Enterprise strict"],
	["custom", "Custom (review only)"],
] as const;

function requestError(cause: unknown) {
	const envelope = z
		.object({
			response: z.object({ data: z.object({ error: z.string() }) }),
		})
		.safeParse(cause);
	return envelope.success
		? envelope.data.response.data.error
		: cause instanceof Error
			? cause.message
			: "Request failed.";
}

function field(form: FormData, name: string) {
	const value = form.get(name);
	return typeof value === "string" ? value : "";
}

export function VerificationSettings({
	className,
	organizationId,
	organizationRole,
	scope = "all",
	...props
}: VerificationSettingsProps) {
	const queryClient = useQueryClient();
	const canManage = organizationRole
		.split(",")
		.some((role) => ["owner", "admin"].includes(role.trim()));
	const showUsage = scope === "all" || scope === "usage";
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
	const webhookKey = ["tenant-webhook", organizationId];
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
			setError(requestError(cause));
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

	return (
		<section
			data-slot="verification-settings"
			className={cn("space-y-4 pb-10", className)}
			{...props}
		>
			<header className="settings-page-intro">
				<h1 className="settings-page-title">{heading[0]}</h1>
				<p className="settings-page-description">{heading[1]}</p>
			</header>
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
					<Card className="gap-5 rounded-none border-primary/10 bg-background/80 py-5 shadow-none">
						<CardHeader className="gap-1.5 px-5">
							<CardTitle className="text-sm">Usage</CardTitle>
							<CardDescription className="text-xs">
								Current verification allowance for this workspace.
							</CardDescription>
						</CardHeader>
						<CardContent className="grid gap-2 px-5 text-sm sm:grid-cols-2">
							{usage.isPending ? (
								<p>Loading usage…</p>
							) : usage.data ? (
								<>
									<p>
										Plan: <strong>{usage.data.plan_tier}</strong>
									</p>
									<p>
										Used this period:{" "}
										<strong>{usage.data.used_this_period.toLocaleString()}</strong>
									</p>
									<p>
										Monthly limit:{" "}
										<strong>
											{usage.data.quota_unlimited
												? "Unlimited"
												: usage.data.monthly_email_limit?.toLocaleString()}
										</strong>
									</p>
									<p>
										Remaining:{" "}
										<strong>
											{usage.data.quota_unlimited
												? "Unlimited"
												: usage.data.quota_remaining?.toLocaleString()}
										</strong>
									</p>
									<p className="sm:col-span-2">
										Resets:{" "}
										<time dateTime={usage.data.period_reset_at}>
											{new Date(usage.data.period_reset_at).toLocaleString()}
										</time>
									</p>
								</>
							) : null}
						</CardContent>
					</Card>
				</>
			) : null}
			{canManage && showDefaults ? (
				<Card className="gap-5 rounded-none border-primary/10 bg-background/80 py-5 shadow-none">
					<CardHeader className="gap-1.5 px-5">
						<CardTitle className="text-sm">Verification defaults</CardTitle>
						<CardDescription className="text-xs">
							Completed check and finder results are removed hourly after this period. Choose the
							default policy for checks.
						</CardDescription>
					</CardHeader>
					<CardContent className="space-y-4 px-5">
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
							<p className="text-sm">Loading settings…</p>
						) : settings.data ? (
							<form
								className="grid gap-4 sm:grid-cols-2"
								key={[settings.data.result_retention_days, settings.data.default_policy_mode].join(
									":",
								)}
								onSubmit={(event) => void submitSettings(event)}
							>
								<div className="space-y-2">
									<Label htmlFor="result-retention">Result retention (days)</Label>
									<Input
										id="result-retention"
										name="retention"
										type="number"
										min={1}
										step={1}
										defaultValue={settings.data.result_retention_days}
										required
									/>
								</div>
								<div className="space-y-2">
									<Label htmlFor="default-policy">Default policy</Label>
									<Select defaultValue={settings.data.default_policy_mode} name="policy" required>
										<SelectTrigger className="w-full" id="default-policy">
											<SelectValue />
										</SelectTrigger>
										<SelectContent>
											{policyModes.map(([value, label]) => (
												<SelectItem key={value} value={value}>
													{label}
												</SelectItem>
											))}
										</SelectContent>
									</Select>
								</div>
								<Button
									className="sm:col-span-2 sm:justify-self-start"
									disabled={busy}
									type="submit"
								>
									Save verification defaults
								</Button>
							</form>
						) : null}
					</CardContent>
				</Card>
			) : null}
			{canManage && showWebhook ? (
				<Card className="gap-5 rounded-none border-primary/10 bg-background/80 py-5 shadow-none">
					<CardHeader className="gap-1.5 px-5">
						<CardTitle className="text-sm">Webhook</CardTitle>
						<CardDescription className="text-xs">
							Send pipeline run summaries to an HTTPS endpoint. The signing secret is never shown
							again.
						</CardDescription>
					</CardHeader>
					<CardContent className="space-y-4 px-5">
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
							<p className="text-sm">Loading webhook…</p>
						) : webhook.data ? (
							<>
								<form
									className="grid gap-4"
									key={webhook.data.default_webhook_url ?? ""}
									onSubmit={(event) => void submitWebhook(event)}
								>
									<div className="space-y-2">
										<Label htmlFor="webhook-url">Webhook URL</Label>
										<Input
											id="webhook-url"
											name="url"
											type="url"
											pattern="https://.*"
											placeholder="https://example.com/webhook"
											defaultValue={webhook.data.default_webhook_url ?? ""}
										/>
									</div>
									<div className="space-y-2">
										<Label htmlFor="webhook-secret">Signing secret</Label>
										<Input
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
										<p className="text-xs text-muted-foreground">
											{webhook.data.webhook_signing_secret_configured
												? "A signing secret is configured."
												: "No signing secret is configured."}
										</p>
									</div>
									<Button disabled={busy} type="submit" className="justify-self-start">
										Save webhook
									</Button>
								</form>
								{webhook.data.webhook_signing_secret_configured ? (
									<Button
										disabled={busy}
										size="sm"
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
								{webhook.data.default_webhook_url ||
								webhook.data.webhook_signing_secret_configured ? (
									<Button
										disabled={busy}
										size="sm"
										variant="destructive"
										onClick={() => {
											if (window.confirm("Clear the webhook URL and signing secret?"))
												void run(() => clearWebhook.mutateAsync(), "Webhook cleared.");
										}}
									>
										Clear webhook
									</Button>
								) : null}
							</>
						) : null}
					</CardContent>
				</Card>
			) : null}
			{!canManage && (showDefaults || showWebhook) ? (
				<Alert>
					<AlertDescription>
						An organization owner or administrator can change verification defaults and webhooks.
						Usage is available to all members.
					</AlertDescription>
				</Alert>
			) : null}
		</section>
	);
}
