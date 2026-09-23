"use client";

import "client-only";

import dynamic from "next/dynamic";
import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";
import { ArrowRight } from "@/lib/icons";
import {
	settingsGroups,
	settingsHref,
	settingsSectionFromParam,
	settingsSections,
} from "@/lib/dashboard/settings-navigation";

import type { IdentitySettingsPropsFields } from "../identity-settings/identity-settings.schema";
import type { VerificationSettingsPropsFields } from "../verification-settings/verification-settings.schema";
import { type SettingsDashboardRoutePropsFields } from "./settings-dashboard-route.schema";

const IdentitySettings = dynamic(
	() => import("../identity-settings/identity-settings").then((module) => module.IdentitySettings),
	{ loading: () => <div className="settings-panel p-4">Loading settings…</div> },
);
const VerificationSettings = dynamic(
	() =>
		import("../verification-settings/verification-settings").then(
			(module) => module.VerificationSettings,
		),
	{ loading: () => <div className="settings-panel p-4">Loading settings…</div> },
);
const DeveloperSettings = dynamic(
	() =>
		import("../developer-settings/developer-settings").then((module) => module.DeveloperSettings),
	{ loading: () => <div className="settings-panel p-4">Loading API keys…</div> },
);

/**
 * @oppulence-gen kind=component
 * Template settings overview and focused panels for this workspace.
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

	return (
		<section
			className={cn("settings-page-scroll min-h-0 flex-1", className)}
			data-slot="settings-dashboard-route"
			{...props}
		>
			<div className={cn("settings-page", current.key === "overview" && "settings-page--wide")}>
				{current.key === "overview" ? (
					<>
						<header className="settings-page-intro">
							<h1 className="settings-page-title">Settings</h1>
							<p className="settings-page-description">
								Manage your workspace, verification, access, integrations, and account.
							</p>
						</header>
						<div className="settings-overview-grid">
							{settingsGroups.map((group) => (
								<section className="settings-overview-group" data-group={group.key} key={group.key}>
									<h2 className="settings-section-title mb-2">{group.label}</h2>
									<nav aria-label={`${group.label} settings`} className="settings-link-list">
										{settingsSections
											.filter((item) => item.group === group.key)
											.map((item) => (
												<Link
													aria-label={item.label}
													className="settings-link-row"
													href={settingsHref(item.key)}
													key={item.key}
												>
													<span className="settings-link-row-icon">
														<item.icon />
													</span>
													<span className="settings-link-row-copy min-w-0 flex-1">
														<span className="settings-link-row-title">{item.label}</span>
														<span className="settings-link-row-description">
															{item.description}
														</span>
													</span>
													<ArrowRight className="ml-2 size-3.5 shrink-0 opacity-30" />
												</Link>
											))}
									</nav>
								</section>
							))}
						</div>
					</>
				) : null}
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
					<>
						<header className="settings-page-intro">
							<h1 className="settings-page-title">Account</h1>
							<p className="settings-page-description">
								Review your personal identity and current workspace access.
							</p>
						</header>
						<section className="settings-section-block">
							<h2 className="settings-section-title mb-3">Profile</h2>
							<div className="settings-panel">
								{[
									["Name", userName || "—"],
									["Email", userEmail],
									["Workspace role", organizationRole],
								].map(([label, value]) => (
									<div className="settings-row" key={label}>
										<span className="settings-row-label">{label}</span>
										<span className="text-sm">{value}</span>
									</div>
								))}
							</div>
						</section>
						<Link
							className="mt-5 inline-flex text-sm underline underline-offset-4"
							href={settingsHref("security")}
						>
							Manage authentication and sessions
						</Link>
					</>
				) : null}
				{current.key === "help" ? (
					<>
						<header className="settings-page-intro">
							<h1 className="settings-page-title">Help</h1>
							<p className="settings-page-description">
								Get help with your workspace and verification.
							</p>
						</header>
						<section className="settings-section-block">
							<h2 className="settings-section-title mb-3">Support</h2>
							<nav aria-label="Support" className="settings-link-list">
								<Link className="settings-link-row" href="/contact">
									Contact support
								</Link>
								<Link className="settings-link-row" href="/blog">
									Product updates
								</Link>
							</nav>
						</section>
					</>
				) : null}
			</div>
		</section>
	);
}
