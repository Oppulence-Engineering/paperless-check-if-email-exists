import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";
import { operations } from "@/lib/developer-portal/operations";

import { PlatformOperationRunner } from "../platform-operation-runner/platform-operation-runner";

import { type ApiPanelPropsFields } from "./api-panel.schema";

/**
 * @oppulence-gen kind=component
 * ApiPanel is a server presentation component.
 * Platform API route-private panel. Presentation only.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `api-panel.lit.ts`.
 */
export type ApiPanelProps = ApiPanelPropsFields & ComponentPropsWithoutRef<"section">;

export function ApiPanel({ className, ...props }: ApiPanelProps) {
	const adminOperations = operations
		.filter((operation) => operation.audience === "admin")
		.map((operation) => ({
			id: operation.id,
			method: operation.method,
			path: operation.path,
			description: operation.description,
			requestBody: [
				"create_tenant",
				"update_tenant",
				"create_api_key",
				"update_api_key",
				"update_tenant_quota",
			].includes(operation.id),
		}));
	return (
		<section
			data-slot="api-panel"
			className={cn("mx-auto flex w-full max-w-6xl flex-col gap-6 p-6", className)}
			{...props}
		>
			<header>
				<h1 className="text-2xl font-semibold">Platform operations</h1>
				<p className="mt-2 text-sm text-muted-foreground">
					Restricted Rust control plane. Each request is audited. Mutations require a reason,
					confirmation, and recent TOTP or passkey verification.
				</p>
			</header>
			{process.env.RCH__HEADER_SECRET?.trim() ? (
				<PlatformOperationRunner operations={adminOperations} />
			) : (
				<p role="status">The platform API is not configured on this deployment.</p>
			)}
		</section>
	);
}
