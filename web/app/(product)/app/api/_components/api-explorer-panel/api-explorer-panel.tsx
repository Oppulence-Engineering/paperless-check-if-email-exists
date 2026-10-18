import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";
import Link from "next/link";

import { Page } from "@/components/platform/page-kit";
import { operations } from "@/lib/developer-portal/operations";

import { OperationRunner } from "../operation-runner/operation-runner";

import { type ApiExplorerPanelPropsFields } from "./api-explorer-panel.schema";

/**
 * @oppulence-gen kind=component
 * ApiExplorerPanel is a server presentation component.
 * API explorer route-private panel. Presentation only.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `api-explorer-panel.lit.ts`.
 */
export type ApiExplorerPanelProps = ApiExplorerPanelPropsFields &
	ComponentPropsWithoutRef<"section">;

export function ApiExplorerPanel({ className, ...props }: ApiExplorerPanelProps) {
	const tenantOperations = operations
		.filter((operation) => operation.audience === "tenant")
		.map((operation) => ({
			id: operation.id,
			method: operation.method,
			path: operation.path,
			family: operation.family,
			description: operation.description,
			scope: operation.scope,
			requestMedia: Object.keys(
				(operation.requestBody as { content?: Record<string, unknown> } | undefined)?.content ?? {},
			),
		}));
	return (
		<Page
			description={
				<>
					Run workspace operations with your current session. Requests stay on this host and use
					your active workspace. See the{" "}
					<Link className="underline hover:text-neutral-200" href="/developers/reference">
						full API reference
					</Link>{" "}
					for schemas and integration examples.
				</>
			}
			title="API explorer"
		>
			<section
				data-slot="api-explorer-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				<OperationRunner operations={tenantOperations} />
			</section>
		</Page>
	);
}
