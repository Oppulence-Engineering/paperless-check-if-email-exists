import type { ComponentPropsWithoutRef } from "react";
import Link from "next/link";

import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Row } from "@/components/platform/page-kit";

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
		<Page
			actions={
				<Link href="/app/admin/api" className="text-sm text-[#858585] hover:text-neutral-200">
					Open the Rust platform API console
				</Link>
			}
			description="Every workspace on this deployment. Access is limited to the platform admin allowlist and every visit is written to the identity audit trail."
			title="Tenants"
		>
			<section className={cn("flex flex-col gap-10", className)} data-slot="admin-panel" {...props}>
				<Group>
					<div className="px-5 py-3 text-xs font-medium text-[#858585]">
						{tenants.length} {tenants.length === 1 ? "workspace" : "workspaces"}
					</div>
					{tenants.length === 0 ? (
						<Row>
							<p className="text-sm text-[#858585]">No workspaces yet.</p>
						</Row>
					) : (
						tenants.map((tenant) => (
							<div
								className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
								key={tenant.id}
							>
								<div className="min-w-0 flex-1">
									<p className="truncate text-sm text-white">{tenant.name}</p>
									<p className="truncate font-mono text-xs text-[#858585]">{tenant.slug}</p>
								</div>
								<span className="text-xs text-[#858585]">
									{tenant.memberCount} {tenant.memberCount === 1 ? "member" : "members"}
								</span>
								<span className="hidden font-mono text-xs text-[#858585] md:block">
									{dateFormat.format(tenant.createdAt)}
								</span>
							</div>
						))
					)}
				</Group>
			</section>
		</Page>
	);
}
