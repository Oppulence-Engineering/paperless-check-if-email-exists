"use client";

import "client-only";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";
import type { ComponentPropsWithoutRef } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { cn } from "@oppulence/ui/lib/utils";

import { ReputationReport } from "@/components/features/deliverability/reputation-report/reputation-report";
import { Row } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1CheckReputation200Response } from "@/lib/api/generated/zod/v1/v1";

import { type DomainHealthPropsFields } from "./domain-health.schema";

/**
 * @oppulence-gen kind=component
 * DomainHealth is a client data component.
 * Checks the reputation of one domain and shows the result.
 *
 * It reads `POST /v1/reputation/check` through the generated SDK and TanStack
 * Query. The backend keeps a result for 24 hours. "Check again" asks for a new
 * result. It shows rows without a card frame, so put it in a `Group`.
 * Owned by `domain-health.lit.ts`.
 */
export type DomainHealthProps = DomainHealthPropsFields & ComponentPropsWithoutRef<"section">;

const v1Api = new V1Api(new Configuration({ basePath: "/api/backend" }));
const errorEnvelope = z.object({ response: z.object({ data: z.object({ error: z.string() }) }) });

async function checkReputation(domain: string, forceRefresh = false) {
	try {
		const response = await v1Api.v1CheckReputation({
			reputationCheckRequest: { domain, force_refresh: forceRefresh },
		});
		return V1CheckReputation200Response.parse(response.data);
	} catch (error) {
		// Show the backend message, for example "Invalid domain", not the HTTP status.
		const envelope = errorEnvelope.safeParse(error);
		throw envelope.success ? new Error(envelope.data.response.data.error) : error;
	}
}

export function DomainHealth({ className, domain, ...props }: DomainHealthProps) {
	const client = useQueryClient();
	const queryKey = workflowQueryKey("domain-reputation", domain);
	const report = useQuery({ queryKey, queryFn: () => checkReputation(domain) });
	const refresh = useMutation({
		mutationFn: () => checkReputation(domain, true),
		onSuccess: (data) => {
			client.setQueryData(queryKey, data);
		},
	});
	const busy = report.isPending || refresh.isPending;
	const error = refresh.error ?? report.error;
	const checkAgain = (
		<Button
			className="rounded-lg"
			disabled={busy}
			onClick={() => {
				refresh.mutate();
			}}
			size="sm"
			variant="outline"
		>
			Check again
		</Button>
	);

	return (
		<section
			aria-busy={busy}
			data-slot="domain-health"
			className={cn("flex flex-col divide-y divide-white/[0.07]", className)}
			{...props}
		>
			{report.data ? (
				<ReputationReport action={checkAgain} report={report.data} />
			) : (
				<Row
					description={report.isPending ? `Checking ${domain}…` : "The check did not finish."}
					label="Health score"
				>
					{report.isPending ? null : checkAgain}
				</Row>
			)}
			{error ? (
				<Row>
					<p
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
						role="alert"
					>
						{error.message}
					</p>
				</Row>
			) : null}
		</section>
	);
}
