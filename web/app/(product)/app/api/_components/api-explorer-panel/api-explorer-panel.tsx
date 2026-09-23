import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";
import Link from "next/link";

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
		<section
			data-slot="api-explorer-panel"
			className={cn("mx-auto flex w-full max-w-6xl flex-col gap-6 p-6", className)}
			{...props}
		>
			<header>
				<h1 className="text-2xl font-semibold">API explorer</h1>
				<p className="mt-2 text-sm text-muted-foreground">
					Run workspace operations with your current session. Requests stay on this host and use
					your active workspace. See the{" "}
					<Link className="underline" href="/developers/reference">
						full API reference
					</Link>{" "}
					for schemas and integration examples.
				</p>
			</header>
			<OperationRunner operations={tenantOperations} />
		</section>
	);
}
