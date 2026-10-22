"use client";

import "client-only";

import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";
import { useQuery, type UseQueryResult } from "@tanstack/react-query";
import {
	AccountApi,
	Configuration,
	OutcomesApi,
	PipelinesApi,
	TenantApi,
} from "@oppulence/reacher-sdk";

import { cn } from "@oppulence/ui/lib/utils";

import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { Group, Pill, type PillTone } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1ListProviderEndpoints200Response } from "@/lib/api/generated/zod/outcomes/outcomes";
import { V1ListPipelines200Response } from "@/lib/api/generated/zod/pipelines/pipelines";
import { V1GetTenantWebhook200Response } from "@/lib/api/generated/zod/tenant/tenant";
import { ListTenantApiKeys200Response } from "@/lib/api/generated/zod/account/account";
import { settingsHref } from "@/lib/dashboard/settings-navigation";
import { ArrowRight } from "@/lib/icons";

import { type IntegrationStackPropsFields } from "./integration-stack.schema";

/**
 * @oppulence-gen kind=component
 * IntegrationStack is a client data component.
 * Shows each part of the stack that a workspace can connect and whether it is
 * set up. Each row links to the page that sets it up.
 * Owned by `integration-stack.lit.ts`.
 */
export type IntegrationStackProps = IntegrationStackPropsFields &
	ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const outcomesApi = new OutcomesApi(config);
const tenantApi = new TenantApi(config);
const accountApi = new AccountApi(config);
const pipelinesApi = new PipelinesApi(config);

/** The providers that can send delivery feedback (provider-endpoint-form). */
const PROVIDERS = [
	["postmark", "Postmark"],
	["mailgun", "Mailgun"],
	["sendgrid", "SendGrid"],
	["ses", "Amazon SES"],
] as const;

type State = { tone: PillTone; status: string; detail: string };

const on = (detail: string): State => ({ tone: "green", status: "Connected", detail });
const off = (detail: string): State => ({ tone: "gray", status: "Not set up", detail });

/** The state of a row while its request runs or after it fails, else `ready`. */
function view<T>(query: UseQueryResult<T>, ready: (data: T) => State): State {
	if (query.isPending) return { tone: "gray", status: "Checking…", detail: "" };
	if (query.isError) return { tone: "red", status: "Unavailable", detail: apiMessage(query.error) };
	return ready(query.data);
}

export function IntegrationStack({ className, ...props }: IntegrationStackProps) {
	const endpoints = useQuery({
		queryKey: workflowQueryKey("provider-endpoints"),
		queryFn: async ({ signal }) =>
			V1ListProviderEndpoints200Response.parse(
				(await outcomesApi.v1ListProviderEndpoints({ signal })).data,
			),
	});
	const webhook = useQuery({
		queryKey: workflowQueryKey("tenant-webhook"),
		queryFn: async ({ signal }) =>
			V1GetTenantWebhook200Response.parse((await tenantApi.v1GetTenantWebhook({ signal })).data),
	});
	const keys = useQuery({
		queryKey: workflowQueryKey("tenant-api-keys"),
		queryFn: async ({ signal }) =>
			ListTenantApiKeys200Response.parse((await accountApi.listTenantApiKeys({ signal })).data),
	});
	const pipelines = useQuery({
		queryKey: workflowQueryKey("pipelines", "list", "all", 0, 200),
		// ponytail: the first 200 pipelines only; add paging if a workspace has more.
		queryFn: async ({ signal }) =>
			V1ListPipelines200Response.parse(
				(await pipelinesApi.v1ListPipelines({ limit: 200 }, { signal })).data,
			),
	});

	const rows: { label: string; kind: string; href: string; state: State }[] = [
		...PROVIDERS.map(([provider, label]) => ({
			label,
			kind: "Delivery feedback",
			href: "/app/outcomes",
			state: view(endpoints, (data) => {
				const mine = data.provider_endpoints.filter((item) => item.provider === provider);
				const active = mine.filter((item) => item.status === "active").length;
				if (!mine.length) return off("No endpoint for this provider.");
				return active
					? on(`${String(active)} of ${String(mine.length)} endpoints active.`)
					: {
							tone: "yellow",
							status: "Paused",
							detail: `${String(mine.length)} endpoints, none active.`,
						};
			}),
		})),
		{
			label: "Webhook",
			kind: "Pipeline run summaries",
			href: settingsHref("webhooks"),
			state: view(webhook, (data) => {
				if (!data.default_webhook_url) return off("No webhook URL.");
				const host = URL.canParse(data.default_webhook_url)
					? new URL(data.default_webhook_url).host
					: data.default_webhook_url;
				return data.webhook_signing_secret_configured
					? on(`Sends signed requests to ${host}.`)
					: {
							tone: "yellow",
							status: "Not signed",
							detail: `Sends to ${host} without a signing secret.`,
						};
			}),
		},
		{
			label: "API keys",
			kind: "Backend API",
			href: settingsHref("developer"),
			state: view(keys, (data) => {
				const active = data.api_keys.filter((key) => key.status === "active").length;
				return active
					? on(`${String(active)} active ${active === 1 ? "key" : "keys"}.`)
					: off("No active key.");
			}),
		},
		{
			label: "Pipelines",
			kind: "Scheduled list checks",
			href: "/app/pipelines",
			state: view(pipelines, (data) => {
				const active = data.pipelines.filter((pipeline) => pipeline.status === "active").length;
				if (!data.total) return off("No pipeline.");
				return active
					? on(`${String(active)} of ${String(data.total)} pipelines active.`)
					: {
							tone: "yellow",
							status: "Paused",
							detail: `${String(data.total)} pipelines, none active.`,
						};
			}),
		},
	];

	return (
		<section data-slot="integration-stack" className={cn(className)} {...props}>
			<Group>
				{rows.map((row) => (
					<Link
						className="flex items-center gap-4 p-5 transition-colors hover:bg-white/[0.02]"
						href={row.href}
						key={row.label}
					>
						<span className="flex min-w-0 flex-1 flex-wrap items-center gap-x-4 gap-y-2">
							<span className="min-w-0 flex-1 basis-48">
								<span className="block text-sm font-medium text-white">{row.label}</span>
								<span className="mt-0.5 block text-xs break-words text-[#858585]">
									{row.kind}
									{row.state.detail ? ` · ${row.state.detail}` : ""}
								</span>
							</span>
							<Pill tone={row.state.tone}>{row.state.status}</Pill>
						</span>
						<ArrowRight className="shrink-0 text-neutral-600" size={14} />
					</Link>
				))}
			</Group>
		</section>
	);
}
