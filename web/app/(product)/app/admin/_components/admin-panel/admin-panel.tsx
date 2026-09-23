import type { ComponentPropsWithoutRef } from "react";
import Link from "next/link";

import { cn } from "@oppulence/ui/lib/utils";

import { type AdminPanelPropsFields } from "./admin-panel.schema";

/**
 * @oppulence-gen kind=component
 * AdminPanel is a server presentation component.
 *
 * What support can see without joining a workspace: that it exists, when it
 * started, and how many people are in it. Deliberately no tenant content —
 * reading a customer's data is a different power, and it needs impersonation
 * with a banner and a kill switch, which this template does not ship yet.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `admin-panel.lit.ts`.
 */
export type AdminPanelProps = AdminPanelPropsFields & ComponentPropsWithoutRef<"section">;

const dateFormat = new Intl.DateTimeFormat("en", { dateStyle: "medium" });

export function AdminPanel({ tenants, className, ...props }: AdminPanelProps) {
	return (
		<section className={cn("settings-page", className)} data-slot="admin-panel" {...props}>
			<header className="settings-page-intro">
				<h1 className="settings-page-title">Tenants</h1>
				<p className="settings-page-description">
					Every workspace on this deployment. Access is limited to the platform admin allowlist and
					every visit is written to the identity audit trail.
				</p>
				<Link href="/app/admin/api" className="text-sm underline">
					Open the Rust platform API console
				</Link>
			</header>

			<div className="settings-panel">
				<div className="settings-row" data-header>
					<p className="settings-row-label">
						{tenants.length} {tenants.length === 1 ? "workspace" : "workspaces"}
					</p>
				</div>
				{tenants.length === 0 ? (
					<div className="settings-row">
						<p className="settings-row-description">No workspaces yet.</p>
					</div>
				) : (
					tenants.map((tenant) => (
						<div className="settings-row" key={tenant.id}>
							<div className="settings-row-copy">
								<p className="settings-row-label">{tenant.name}</p>
								<p className="settings-row-description font-mono">{tenant.slug}</p>
							</div>
							<div className="flex shrink-0 items-center gap-4">
								<span className="settings-row-description">
									{tenant.memberCount} {tenant.memberCount === 1 ? "member" : "members"}
								</span>
								<span className="settings-row-description font-mono">
									{dateFormat.format(tenant.createdAt)}
								</span>
							</div>
						</div>
					))
				)}
			</div>
		</section>
	);
}
