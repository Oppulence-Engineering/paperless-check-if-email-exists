import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";

import { ApiReference } from "@/components/features/developer-portal/api-reference/api-reference";
import { Page } from "@/components/platform/page-kit";
import { referenceDocument } from "@/lib/developer-portal/operations";

import { type ApiExplorerPanelPropsFields } from "./api-explorer-panel.schema";

/**
 * @oppulence-gen kind=component
 * ApiExplorerPanel is a server presentation component.
 * API explorer route-private panel. It builds the workspace OpenAPI document
 * on the server and shows it in the shared Scalar reference.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `api-explorer-panel.lit.ts`.
 */
export type ApiExplorerPanelProps = ApiExplorerPanelPropsFields &
	ComponentPropsWithoutRef<"section">;

export function ApiExplorerPanel({ className, ...props }: ApiExplorerPanelProps) {
	return (
		<Page
			description="Send any workspace operation from the browser. The first server uses your session. The second server is the public API and needs an API key."
			title="API explorer"
		>
			<section
				data-slot="api-explorer-panel"
				className={cn(
					"overflow-hidden rounded-2xl bg-[#111111] ring-1 ring-white/[0.08]",
					className,
				)}
				{...props}
			>
				<ApiReference document={referenceDocument("workspace")} surface="workspace" />
			</section>
		</Page>
	);
}
