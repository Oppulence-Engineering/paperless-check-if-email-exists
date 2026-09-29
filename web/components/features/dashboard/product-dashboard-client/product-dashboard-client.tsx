"use client";

import "client-only";

import Link from "next/link";
import { usePathname, useSearchParams } from "next/navigation";
import { useState, type CSSProperties, type ReactNode } from "react";

import { AuthGate } from "@/components/auth/auth-gate";
import { WorkspaceSwitcher } from "@/components/features/workspaces/workspace-switcher/workspace-switcher";
import { WorkspaceUrlScope } from "@/components/features/workspaces/workspace-url-scope/workspace-url-scope";
import { Avatar, AvatarFallback } from "@oppulence/ui/components/avatar";
import { Button } from "@oppulence/ui/components/button";
import { DropdownMenuItem } from "@oppulence/ui/components/dropdown-menu";
import { Input } from "@oppulence/ui/components/input";
import { authClient } from "@/lib/auth/auth-client";
import type { Branding } from "@/lib/auth/branding.schema";
import type { BrowserSessionResponse, WorkspaceSummary } from "@/lib/auth/schemas";
import {
	ArrowLeft,
	CaretUpDown,
	GearSix,
	MagnifyingGlass,
	SidebarSimple,
	SignOut,
	X,
} from "@/lib/icons";
import {
	settingsGroups,
	settingsHref,
	settingsSectionFromParam,
	settingsSections,
} from "@/lib/dashboard/settings-navigation";

export type ProductDashboardClientProps = {
	children: ReactNode;
	brand?: Branding;
	initialSession: Extract<BrowserSessionResponse, { authenticated: true }>;
	workspaces: WorkspaceSummary[];
};

const navigation = [
	{ href: "/app/check", label: "Check email" },
	{ href: "/app/finder", label: "Find email" },
	{ href: "/app/jobs", label: "Jobs" },
	{ href: "/app/lists", label: "Lists" },
	{ href: "/app/suppressions", label: "Suppressions" },
	{ href: "/app/pipelines", label: "Pipelines" },
	{ href: "/app/outcomes", label: "Outcomes" },
	{ href: "/app/domains", label: "Domains" },
	{ href: "/app/analytics", label: "Analytics" },
	{ href: "/app/history", label: "History" },
	{ href: "/app/integrations", label: "Integrations" },
	{ href: "/app/api", label: "API explorer" },
];

/** The template's framed product shell, with email verification routes in its rail. */
export function ProductDashboardClient({
	brand,
	children,
	initialSession,
	workspaces,
}: ProductDashboardClientProps) {
	const pathname = usePathname();
	const searchParams = useSearchParams();
	const [sidebarOpen, setSidebarOpen] = useState(true);
	const [mobileOpen, setMobileOpen] = useState(false);
	const [settingsFilter, setSettingsFilter] = useState("");
	const [overlayContainer, setOverlayContainer] = useState<HTMLElement | null>(null);
	const isSettings = pathname === "/app/settings";
	const settingsSection = settingsSectionFromParam(searchParams.get("settings"));
	const settingsRailGroups = settingsGroups
		.map((group) => ({
			...group,
			sections: settingsSections.filter(
				(section) =>
					section.group === group.key &&
					`${section.label} ${section.description}`
						.toLowerCase()
						.includes(settingsFilter.trim().toLowerCase()),
			),
		}))
		.filter((group) => group.sections.length > 0);
	const settingsMatchCount = settingsRailGroups.reduce(
		(total, group) => total + group.sections.length,
		0,
	);
	const activePage = [...navigation, { href: "/app/settings", label: "Settings" }].find(
		({ href }) => pathname === href || pathname.startsWith(`${href}/`),
	);
	const accountLabel = initialSession.user.name || initialSession.user.email;
	const brandStyle = brand
		? ({
				"--main-color": brand.primaryColor,
				"--oppulence-orange": brand.accentColor,
			} as CSSProperties)
		: undefined;
	const toggleSidebar = () => {
		if (window.matchMedia("(min-width: 768px)").matches) {
			setSidebarOpen((open) => !open);
		} else {
			setMobileOpen((open) => !open);
		}
	};
	const navClass =
		"flex h-[var(--shell-nav-row-height,30px)] w-full items-center rounded-[8px] px-2 text-[var(--text-small,13px)] text-[var(--text-body)] hover:bg-[var(--surface-hover)] hover:text-[var(--text-primary)] focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-[var(--border)]";

	return (
		<div className="contents" data-slot="product-dashboard-client" style={brandStyle}>
			<AuthGate initialSession={initialSession}>
				<WorkspaceUrlScope
					activeWorkspaceId={initialSession.user.organizationId}
					workspaces={workspaces}
				/>
				<section
					className="app-shell sim-product-shell sim-landing-root app-vh-shell flex w-full flex-col overflow-hidden bg-[var(--bg)] text-[var(--text-primary)]"
					data-product-shell
					data-slot="dashboard-shell"
					ref={setOverlayContainer}
				>
					<header
						className="flex h-14 shrink-0 items-center gap-3 px-4 md:px-6"
						data-slot="app-top-bar"
					>
						<Link
							className="flex min-w-0 items-center gap-2.5 text-sm font-semibold"
							href="/app/check"
						>
							<img
								alt=""
								className="size-6 object-contain"
								src={brand?.logoUrl || "/check-email-logo.svg"}
							/>
							<span className="truncate">{brand?.name || "Check If Email Exists"}</span>
						</Link>
					</header>
					<div className="min-h-0 flex-1 md:px-2.5 md:pb-2.5">
						<section
							className={`relative flex h-full overflow-clip border-[var(--border)] border-t bg-[var(--surface-2)] ${isSettings ? "settings-workspace border-0 md:border-0" : "md:border"}`}
							data-slot="dashboard-workspace"
						>
							{mobileOpen ? (
								<Button
									aria-label="Close sidebar"
									className="absolute inset-0 z-20 size-full bg-black/20 p-0 hover:bg-black/20 md:hidden"
									onClick={() => setMobileOpen(false)}
									type="button"
									variant="ghost"
								/>
							) : null}
							<aside
								className={`absolute inset-y-0 left-0 z-30 flex min-h-0 shrink-0 overflow-hidden border-[var(--border)] border-r shadow-xl transition-all duration-200 md:relative md:shadow-none ${isSettings ? "settings-rail" : ""} ${mobileOpen ? "visible w-[var(--shell-sidebar-width,252px)]" : "invisible w-0 border-r-0"} ${sidebarOpen ? "md:visible md:w-[var(--shell-sidebar-width,252px)] md:border-r" : "md:invisible md:w-0 md:border-r-0"}`}
								data-slot="app-sidebar"
								id="product-sidebar"
							>
								<div
									className={`flex h-full min-h-0 w-[var(--shell-sidebar-width,252px)] shrink-0 flex-col bg-[var(--surface-1)] ${isSettings ? "settings-rail" : ""}`}
								>
									<div className="flex h-10 shrink-0 items-center justify-end px-2.5 md:hidden">
										<Button
											aria-label="Close sidebar"
											onClick={() => setMobileOpen(false)}
											size="icon-sm"
											type="button"
											variant="ghost"
										>
											<SidebarSimple className="size-4" />
										</Button>
									</div>
									{isSettings ? (
										<nav
											aria-label="Settings"
											className="settings-rail-scroll flex min-h-0 flex-1 flex-col items-stretch overflow-y-auto px-2 pb-3 pt-2"
										>
											<Link
												className="settings-back"
												href="/app/check"
												onClick={() => setMobileOpen(false)}
											>
												<ArrowLeft className="size-3.5" />
												Back to app
											</Link>
											<div className="settings-rail-search">
												<MagnifyingGlass aria-hidden className="size-3.5 shrink-0 opacity-60" />
												<Input
													aria-label="Filter settings"
													className="settings-rail-search-input"
													onChange={(event) => setSettingsFilter(event.target.value)}
													onKeyDown={(event) => {
														if (event.key === "Escape") setSettingsFilter("");
													}}
													placeholder="Filter settings"
													type="search"
													value={settingsFilter}
												/>
												{settingsFilter ? (
													<Button
														aria-label="Clear settings filter"
														className="size-5 p-0"
														onClick={() => setSettingsFilter("")}
														size="icon-xs"
														type="button"
														variant="ghost"
													>
														<X className="size-3" />
													</Button>
												) : null}
											</div>
											{settingsFilter.trim() ? null : (
												<Link
													aria-current={settingsSection.key === "overview" ? "page" : undefined}
													className="settings-nav-item"
													data-active={settingsSection.key === "overview"}
													href={settingsHref("overview")}
													onClick={() => setMobileOpen(false)}
												>
													<GearSix /> Settings
												</Link>
											)}
											{settingsRailGroups.map((group) => (
												<div
													aria-labelledby={`settings-rail-${group.key}`}
													key={group.key}
													role="group"
												>
													<h2 className="settings-rail-heading" id={`settings-rail-${group.key}`}>
														{group.label}
													</h2>
													<div className="space-y-0.5">
														{group.sections.map((section) => (
															<Link
																aria-current={
																	settingsSection.key === section.key ? "page" : undefined
																}
																className="settings-nav-item"
																data-active={settingsSection.key === section.key}
																href={settingsHref(section.key)}
																key={section.key}
																onClick={() => setMobileOpen(false)}
															>
																<section.icon />
																<span className="truncate">{section.label}</span>
															</Link>
														))}
													</div>
												</div>
											))}
											{settingsRailGroups.length === 0 ? (
												<p className="settings-rail-empty">
													No settings match “{settingsFilter.trim()}”.
												</p>
											) : null}
											{settingsFilter.trim() ? (
												<p className="sr-only" role="status">
													{settingsMatchCount === 0
														? `No settings match ${settingsFilter.trim()}.`
														: `${settingsMatchCount} ${settingsMatchCount === 1 ? "section matches" : "sections match"} ${settingsFilter.trim()}.`}
												</p>
											) : null}
										</nav>
									) : (
										<nav
											aria-label="Main navigation"
											className="flex min-h-0 flex-1 flex-col gap-0.5 overflow-y-auto px-2 pb-2 pt-3"
											data-sidebar-nav
										>
											<p
												className="px-2 pb-1 text-[var(--text-caption,12px)] text-[var(--text-secondary)]"
												data-sidebar-section
											>
												Verification
											</p>
											{navigation.map(({ href, label }) => (
												<Link
													aria-current={activePage?.href === href ? "page" : undefined}
													className={navClass}
													data-active={activePage?.href === href ? "true" : undefined}
													data-sidebar-row
													href={href}
													key={href}
													onClick={() => setMobileOpen(false)}
												>
													{label}
												</Link>
											))}
										</nav>
									)}
									<div
										className={`border-[var(--border)] border-t px-2 py-2 ${isSettings ? "settings-rail-footer" : ""}`}
										data-sidebar-footer
									>
										<Link
											aria-current={activePage?.href === "/app/settings" ? "page" : undefined}
											className={navClass}
											data-active={activePage?.href === "/app/settings" ? "true" : undefined}
											data-sidebar-row
											href="/app/settings"
											onClick={() => setMobileOpen(false)}
										>
											Settings
										</Link>
									</div>
									<div
										className="mx-2 flex h-14 shrink-0 items-center border-[var(--border)] border-t"
										data-sidebar-account
									>
										<WorkspaceSwitcher
											activeWorkspaceId={initialSession.user.organizationId}
											aria-label="Account and workspace menu"
											className="min-w-0 flex-1"
											menuContainer={overlayContainer}
											trigger={
												<Button
													aria-label={`Open account and workspace menu for ${accountLabel}`}
													className="h-10 w-full min-w-0 justify-start gap-2.5 px-2 text-left hover:bg-[var(--surface-hover)]"
													type="button"
													variant="ghost"
												>
													<Avatar aria-hidden="true" className="size-6 rounded-none" size="sm">
														<AvatarFallback className="rounded-none bg-[var(--surface-3)] text-[11px] uppercase">
															{accountLabel.slice(0, 1)}
														</AvatarFallback>
													</Avatar>
													<span className="truncate text-sm">{accountLabel}</span>
													<CaretUpDown className="ml-auto size-3.5 shrink-0 text-[var(--text-muted)]" />
												</Button>
											}
											workspaces={workspaces}
										>
											<DropdownMenuItem
												onSelect={() =>
													void authClient.signOut({
														fetchOptions: {
															onSuccess: () => window.location.assign("/sign-in"),
														},
													})
												}
												variant="destructive"
											>
												<SignOut /> Sign out
											</DropdownMenuItem>
										</WorkspaceSwitcher>
									</div>
								</div>
							</aside>
							<main
								className={`flex min-w-0 flex-1 flex-col ${isSettings ? "settings-stage" : ""}`}
							>
								<header
									className={
										isSettings
											? "settings-stage-header"
											: "flex h-12 shrink-0 items-center gap-2 border-[var(--border)] border-b px-5"
									}
									data-slot="app-stage-header"
								>
									<Button
										aria-controls="product-sidebar"
										aria-label="Toggle sidebar"
										onClick={toggleSidebar}
										size="icon"
										title="Toggle sidebar"
										type="button"
										variant="ghost"
									>
										<SidebarSimple className="size-4" />
									</Button>
									<span className={isSettings ? "settings-stage-header-title" : undefined}>
										{isSettings ? settingsSection.label : activePage?.label || "Workspace"}
									</span>
								</header>
								<div
									className={
										isSettings
											? "flex min-h-0 flex-1 flex-col overflow-hidden"
											: "min-h-0 flex-1 overflow-y-auto px-5 py-8"
									}
								>
									{children}
								</div>
							</main>
						</section>
					</div>
				</section>
			</AuthGate>
		</div>
	);
}
