"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, HealthApi, SystemApi } from "@oppulence/reacher-sdk";
import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { cn } from "@oppulence/ui/lib/utils";
import { IntegrationStack } from "@/components/features/automation/integration-stack/integration-stack";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";

import { type IntegrationsPanelPropsFields } from "./integrations-panel.schema";

/** @oppulence-gen kind=component Owned by `integrations-panel.lit.ts`. */
export type IntegrationsPanelProps = IntegrationsPanelPropsFields &
	ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const healthApi = new HealthApi(config);
const systemApi = new SystemApi(config);

export function IntegrationsPanel({ className, ...props }: IntegrationsPanelProps) {
	const client = useQueryClient();
	const health = useQuery({
		queryKey: workflowQueryKey("integration-status", "health"),
		queryFn: async () => (await healthApi.healthz()).status,
		refetchInterval: 30_000,
	});
	const readiness = useQuery({
		queryKey: workflowQueryKey("integration-status", "readiness"),
		queryFn: async () => (await healthApi.readyz()).status,
		refetchInterval: 30_000,
	});
	const version = useQuery({
		queryKey: workflowQueryKey("integration-status", "version"),
		queryFn: async () =>
			z.object({ version: z.string() }).parse((await systemApi.getVersion()).data),
	});
	const link = "text-sm text-neutral-300 underline hover:text-white";
	return (
		<Page
			actions={
				<Button
					className="rounded-lg"
					size="sm"
					type="button"
					variant="outline"
					onClick={() => {
						void client.invalidateQueries();
					}}
				>
					Check again
				</Button>
			}
			description="Connect delivery feedback, the webhook, API keys, and pipelines. Each row shows if it is set up and opens its setup page."
			title="Connect your stack"
			width="md"
		>
			<section
				data-slot="integrations-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				<Section title="Connections">
					<IntegrationStack />
				</Section>
				<Section title="Service status">
					<Group>
						<Row label="API liveness" layout="inline">
							<Pill tone={health.isPending ? "gray" : health.isSuccess ? "green" : "red"}>
								{health.isPending ? "Checking…" : health.isSuccess ? "Healthy" : "Unavailable"}
							</Pill>
						</Row>
						<Row label="API readiness" layout="inline">
							<Pill tone={readiness.isPending ? "gray" : readiness.isSuccess ? "green" : "red"}>
								{readiness.isPending ? "Checking…" : readiness.isSuccess ? "Ready" : "Unavailable"}
							</Pill>
						</Row>
						{version.isSuccess ? (
							<Row label="Backend version" layout="inline">
								<code className="font-mono text-xs text-neutral-300">{version.data.version}</code>
							</Row>
						) : null}
						<Row>
							<Link className={link} href="/openapi.json">
								Download current OpenAPI contract
							</Link>
						</Row>
					</Group>
					{health.isError || readiness.isError ? (
						<p
							role="alert"
							className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
						>
							The status request failed. Check the deployment and retry.
						</p>
					) : null}
				</Section>
				<Section
					description="The legacy onboarding check creates a tenant and API key without authentication. The hosted app blocks this endpoint; use the app sign-up flow for new workspaces."
					title="Onboarding checks"
				>
					<Group>
						<Row>
							<ol className="list-decimal space-y-1 pl-5 text-sm text-neutral-300">
								<li>Create the workspace through the hosted sign-up flow.</li>
								<li>Create a scoped API key after the workspace exists.</li>
								<li>Test verification and signup failure states in staging.</li>
							</ol>
						</Row>
						<Row>
							<div className="flex flex-wrap gap-4">
								<Link className={link} href="/app/settings?settings=developer">
									Manage API keys
								</Link>
								<Link className={link} href="/developers/reference/v1_check_email_with_onboard">
									Onboarding API contract
								</Link>
							</div>
						</Row>
					</Group>
				</Section>
				<Section
					description="Register a provider endpoint, configure the provider to send signed deliveries, then watch outcomes. The callback URL is shown once during setup; keep it out of code and logs."
					title="Provider callbacks"
				>
					<Group>
						<Row>
							<ol className="list-decimal space-y-1 pl-5 text-sm text-neutral-300">
								<li>Create the endpoint for the correct workspace and provider.</li>
								<li>Store its delivery token and signing secret in your provider configuration.</li>
								<li>Send a provider test delivery, then confirm an outcome appears.</li>
								<li>Rotate credentials and update the provider together if a token is exposed.</li>
							</ol>
						</Row>
						<Row>
							<div className="flex flex-wrap gap-4">
								<Link className={link} href="/app/outcomes">
									Manage endpoints and outcomes
								</Link>
								<Link className={link} href="/developers/reference/v1_create_provider_endpoint">
									Endpoint setup contract
								</Link>
								<Link className={link} href="/developers/reference/v1_ingest_provider_outcomes">
									Callback contract
								</Link>
							</div>
						</Row>
					</Group>
				</Section>
				<Section
					description="The hosted app uses workspace-scoped v1 keys. Legacy v0 routes are not available."
					title="Legacy v0 migration"
				>
					<Group>
						<Row>
							<ol className="list-decimal space-y-1 pl-5 text-sm text-neutral-300">
								<li>Inventory each v0 check and bulk caller in the old installation.</li>
								<li>
									Create a workspace key and replace v0 check with v1 check and v0 bulk with v1
									bulk.
								</li>
								<li>Change status and result polling to the matching v1 job routes.</li>
								<li>
									Verify parity in staging, switch callers, then retire the old global secret.
								</li>
							</ol>
						</Row>
						<Row>
							<div className="flex flex-wrap gap-4">
								<Link className={link} href="/developers/reference/post_check_email">
									Legacy check
								</Link>
								<Link className={link} href="/developers/reference/v1_check_email">
									Current check
								</Link>
								<Link className={link} href="/app/jobs">
									Current bulk jobs
								</Link>
							</div>
						</Row>
					</Group>
				</Section>
			</section>
		</Page>
	);
}
