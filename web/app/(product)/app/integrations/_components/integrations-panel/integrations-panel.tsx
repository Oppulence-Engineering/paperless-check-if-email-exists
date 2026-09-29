"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useQuery } from "@tanstack/react-query";
import { Configuration, HealthApi, SystemApi } from "@oppulence/reacher-sdk";
import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { cn } from "@oppulence/ui/lib/utils";
import { type IntegrationsPanelPropsFields } from "./integrations-panel.schema";

/** @oppulence-gen kind=component Owned by `integrations-panel.lit.ts`. */
export type IntegrationsPanelProps = IntegrationsPanelPropsFields &
	ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const healthApi = new HealthApi(config);
const systemApi = new SystemApi(config);

export function IntegrationsPanel({ className, ...props }: IntegrationsPanelProps) {
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
	return (
		<section data-slot="integrations-panel" className={cn("space-y-6", className)} {...props}>
			<header>
				<h1 className="text-3xl font-semibold">Integration setup</h1>
				<p className="mt-2 text-muted-foreground">
					Set up server integrations and check their readiness without sending provider callbacks
					from the browser.
				</p>
			</header>
			<Card>
				<CardHeader>
					<CardTitle>Service status</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					<p>
						API liveness:{" "}
						<strong>
							{health.isPending ? "Checking…" : health.isSuccess ? "Healthy" : "Unavailable"}
						</strong>
					</p>
					<p>
						API readiness:{" "}
						<strong>
							{readiness.isPending ? "Checking…" : readiness.isSuccess ? "Ready" : "Unavailable"}
						</strong>
					</p>
					{version.isSuccess ? (
						<p>
							Backend version: <code>{version.data.version}</code>
						</p>
					) : null}
					{health.isError || readiness.isError ? (
						<p role="alert" className="text-destructive">
							The status request failed. Check the deployment and retry.
						</p>
					) : null}
					<div className="flex flex-wrap gap-3">
						<Button
							type="button"
							variant="outline"
							onClick={() => {
								void health.refetch();
								void readiness.refetch();
								void version.refetch();
							}}
						>
							Check again
						</Button>
						<Link className="underline" href="/openapi.json">
							Download current OpenAPI contract
						</Link>
					</div>
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Onboarding checks</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					<p>
						The legacy onboarding check creates a tenant and API key without authentication. The
						hosted app blocks this endpoint; use the app sign-up flow for new workspaces. For a
						self-hosted migration, keep the endpoint private and apply signup abuse controls.
					</p>
					<ol className="list-decimal space-y-1 pl-5">
						<li>Create the workspace through the hosted sign-up flow.</li>
						<li>Create a scoped API key after the workspace exists.</li>
						<li>Test verification and signup failure states in staging.</li>
					</ol>
					<div className="flex gap-4">
						<Link className="underline" href="/app/settings?settings=developer">
							Manage API keys
						</Link>
						<Link className="underline" href="/developers/reference/v1_check_email_with_onboard">
							Onboarding API contract
						</Link>
					</div>
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Provider callbacks</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					<p>
						Register a provider endpoint, configure the provider to send signed deliveries, then
						watch outcomes. The callback URL is shown once during setup; keep it out of code and
						logs.
					</p>
					<ol className="list-decimal space-y-1 pl-5">
						<li>Create the endpoint for the correct workspace and provider.</li>
						<li>Store its delivery token and signing secret in your provider configuration.</li>
						<li>Send a provider test delivery, then confirm an outcome appears.</li>
						<li>Rotate credentials and update the provider together if a token is exposed.</li>
					</ol>
					<div className="flex gap-4">
						<Link className="underline" href="/app/outcomes">
							Manage endpoints and outcomes
						</Link>
						<Link className="underline" href="/developers/reference/v1_create_provider_endpoint">
							Endpoint setup contract
						</Link>
						<Link className="underline" href="/developers/reference/v1_ingest_provider_outcomes">
							Callback contract
						</Link>
					</div>
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Legacy v0 migration</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					<p>
						The hosted shared app uses workspace-scoped v1 keys. Legacy v0 routes and their global
						secret belong to self-hosted installations.
					</p>
					<ol className="list-decimal space-y-1 pl-5">
						<li>Inventory each v0 check and bulk caller in the old installation.</li>
						<li>
							Create a workspace key and replace v0 check with v1 check and v0 bulk with v1 bulk.
						</li>
						<li>Change status and result polling to the matching v1 job routes.</li>
						<li>Verify parity in staging, switch callers, then retire the old global secret.</li>
					</ol>
					<div className="flex gap-4">
						<Link className="underline" href="/developers/reference/post_check_email">
							Legacy check
						</Link>
						<Link className="underline" href="/developers/reference/v1_check_email">
							Current check
						</Link>
						<Link className="underline" href="/app/jobs">
							Current bulk jobs
						</Link>
					</div>
				</CardContent>
			</Card>
		</section>
	);
}
