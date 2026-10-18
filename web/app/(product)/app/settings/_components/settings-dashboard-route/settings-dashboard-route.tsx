"use client";

import "client-only";

import dynamic from "next/dynamic";
import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Row, Section } from "@/components/platform/page-kit";
import { ArrowRight } from "@/lib/icons";
import { settingsHref, settingsSectionFromParam } from "@/lib/dashboard/settings-navigation";

import type { IdentitySettingsPropsFields } from "../identity-settings/identity-settings.schema";
import type { VerificationSettingsPropsFields } from "../verification-settings/verification-settings.schema";
import { type SettingsDashboardRoutePropsFields } from "./settings-dashboard-route.schema";

const IdentitySettings = dynamic(
	() => import("../identity-settings/identity-settings").then((module) => module.IdentitySettings),
	{ loading: () => <p className="text-sm text-[#858585]">Loading settings…</p> },
);
const VerificationSettings = dynamic(
	() =>
		import("../verification-settings/verification-settings").then(
			(module) => module.VerificationSettings,
		),
	{ loading: () => <p className="text-sm text-[#858585]">Loading settings…</p> },
);
const DeveloperSettings = dynamic(
	() =>
		import("../developer-settings/developer-settings").then((module) => module.DeveloperSettings),
	{ loading: () => <p className="text-sm text-[#858585]">Loading API keys…</p> },
);

/**
 * @oppulence-gen kind=component
 * One settings section for this workspace, like the Shots settings page.
 * Owned by `settings-dashboard-route.lit.ts`.
 */
export type SettingsDashboardRouteProps = SettingsDashboardRoutePropsFields &
	ComponentPropsWithoutRef<"section">;

export function SettingsDashboardRoute({
	className,
	section,
	organizationId,
	organizationRole,
	userId,
	userName,
	userEmail,
	chosenPlan,
	...props
}: SettingsDashboardRouteProps) {
	const current = settingsSectionFromParam(section);
	const identityScopes: Record<string, IdentitySettingsPropsFields["scope"] | undefined> = {
		workspace: "workspace",
		branding: "branding",
		members: "organization",
		security: "security",
		compliance: "enterprise",
	};
	const identityScope = identityScopes[current.key];
	const verificationScopes: Record<string, VerificationSettingsPropsFields["scope"] | undefined> = {
		verification: "verification",
		usage: "usage",
		webhooks: "webhook",
	};
	const verificationScope = verificationScopes[current.key];

	const linkRow = "flex items-center gap-4 p-5 transition-colors hover:bg-white/[0.02]";
	return (
		<Page title="Settings" width="md">
			<section
				className={cn("flex flex-col gap-10", className)}
				data-slot="settings-dashboard-route"
				{...props}
			>
				{identityScope ? (
					<IdentitySettings
						organizationId={organizationId}
						organizationRole={organizationRole}
						scope={identityScope}
						userId={userId}
					/>
				) : null}
				{verificationScope ? (
					<VerificationSettings
						chosenPlan={chosenPlan}
						organizationId={organizationId}
						organizationRole={organizationRole}
						scope={verificationScope}
					/>
				) : null}
				{current.key === "developer" ? (
					<DeveloperSettings
						key={organizationId}
						organizationId={organizationId}
						organizationRole={organizationRole}
					/>
				) : null}
				{current.key === "account" ? (
					<Section
						description="Review your personal identity and current workspace access."
						title="Account"
					>
						<Section title="Profile">
							<Group>
								{[
									["Name", userName || "—"],
									["Email", userEmail],
									["Workspace role", organizationRole],
								].map(([label, value]) => (
									<Row key={label} label={label} layout="inline">
										<span className="text-sm text-neutral-300">{value}</span>
									</Row>
								))}
								<Link className={linkRow} href={settingsHref("security")}>
									<span className="min-w-0 flex-1 text-sm font-medium text-white">
										Manage authentication and sessions
									</span>
									<ArrowRight className="shrink-0 text-neutral-600" size={14} />
								</Link>
							</Group>
						</Section>
					</Section>
				) : null}
				{current.key === "help" ? (
					<Section description="Get help with your workspace and verification." title="Help">
						<Section title="Support">
							<nav aria-label="Support">
								<Group>
									<Link className={linkRow} href="/contact">
										<span className="min-w-0 flex-1 text-sm font-medium text-white">
											Contact support
										</span>
										<ArrowRight className="shrink-0 text-neutral-600" size={14} />
									</Link>
									<Link className={linkRow} href="/blog">
										<span className="min-w-0 flex-1 text-sm font-medium text-white">
											Product updates
										</span>
										<ArrowRight className="shrink-0 text-neutral-600" size={14} />
									</Link>
								</Group>
							</nav>
						</Section>
					</Section>
				) : null}
			</section>
		</Page>
	);
}
