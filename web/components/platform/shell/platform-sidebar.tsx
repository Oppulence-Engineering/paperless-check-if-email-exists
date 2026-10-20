"use client";

import Link from "next/link";
import { usePathname, useSearchParams } from "next/navigation";
import { useState } from "react";

import { WorkspaceSwitcher } from "@/components/features/workspaces/workspace-switcher/workspace-switcher";
import {
	Collapsible,
	CollapsibleContent,
	CollapsibleTrigger,
} from "@oppulence/ui/components/collapsible";
import { DropdownMenuItem } from "@oppulence/ui/components/dropdown-menu";
import { Sheet, SheetContent, SheetTitle, SheetTrigger } from "@oppulence/ui/components/sheet";
import {
	Tooltip,
	TooltipContent,
	TooltipProvider,
	TooltipTrigger,
} from "@oppulence/ui/components/tooltip";
import { authClient } from "@/lib/auth/auth-client";
import type { Branding } from "@/lib/auth/branding.schema";
import type { BrowserSessionResponse, WorkspaceSummary } from "@/lib/auth/schemas";
import {
	settingsHref,
	settingsSectionFromParam,
	settingsSections,
} from "@/lib/dashboard/settings-navigation";
import {
	Analytics,
	Api,
	CaretDown,
	CaretUpDown,
	ClockCounterClockwise,
	FlowArrow,
	Internet,
	List,
	ListBullets,
	MailBlock,
	MailSearch,
	MailValidation,
	Plugs,
	SignOut,
	SlidersHorizontal,
	SquaresFour,
	Target,
	Task,
	type Icon,
} from "@/lib/icons";
import { cn } from "@/lib/utils";

import { surface } from "./shell-config";

type SubItem = { href: string; icon: Icon; label: string };
type NavItem = { children?: SubItem[]; href: string; icon: Icon; label: string };

/** Navigation groups follow the customer flows, not the API resources. */
const NAV_GROUPS: { items: NavItem[]; label?: string }[] = [
	{ items: [{ href: "/app", icon: SquaresFour, label: "Overview" }] },
	{
		items: [
			{ href: "/app/check", icon: MailValidation, label: "Check an address" },
			{ href: "/app/finder", icon: MailSearch, label: "Find an address" },
			{ href: "/app/history", icon: ClockCounterClockwise, label: "Address history" },
		],
		label: "Verify",
	},
	{
		items: [
			{ href: "/app/lists", icon: ListBullets, label: "Lists" },
			{ href: "/app/jobs", icon: Task, label: "Bulk jobs" },
		],
		label: "Lists",
	},
	{
		items: [
			{ href: "/app/domains", icon: Internet, label: "Sender domains" },
			{ href: "/app/suppressions", icon: MailBlock, label: "Suppressions" },
			{ href: "/app/outcomes", icon: Target, label: "Delivery feedback" },
		],
		label: "Deliverability",
	},
	{
		items: [
			{ href: "/app/pipelines", icon: FlowArrow, label: "Pipelines" },
			{ href: "/app/integrations", icon: Plugs, label: "Integrations" },
			{ href: "/app/api", icon: Api, label: "API explorer" },
		],
		label: "Automation",
	},
	{ items: [{ href: "/app/analytics", icon: Analytics, label: "Insights" }], label: "Insights" },
	{
		items: [
			{
				children: settingsSections.map((section) => ({
					href: settingsHref(section.key),
					icon: section.icon,
					label: section.label,
				})),
				href: "/app/settings",
				icon: SlidersHorizontal,
				label: "Settings",
			},
		],
		label: "Workspace",
	},
];

function isActive(pathname: string, href: string): boolean {
	// Every product route starts with /app, so Overview needs an exact match.
	if (href === "/app") return pathname === href;
	return pathname === href || pathname.startsWith(`${href}/`);
}

/** Up to two initials for a workspace badge: "Acme Launch Team" → "AL". */
function workspaceInitials(name: string): string {
	const words = name.trim().split(/\s+/).filter(Boolean);
	return (
		words.length > 1 ? (words[0]?.[0] ?? "") + (words[1]?.[0] ?? "") : (words[0]?.slice(0, 2) ?? "")
	).toUpperCase();
}

function WorkspaceBadge({ name }: { name: string }) {
	return (
		<span
			aria-hidden
			className="grid size-6 shrink-0 place-items-center rounded-md bg-gradient-to-br from-fuchsia-500 via-violet-500 to-indigo-500 text-[10px] font-semibold text-white"
		>
			{workspaceInitials(name) || "W"}
		</span>
	);
}

/** The link of the current settings section, or null when the page is not settings. */
function useCurrentSettingsHref(): string | null {
	const section = settingsSectionFromParam(useSearchParams().get("settings"));
	// The section lists can be open on any page, so only mark a section on the settings page.
	return usePathname() === "/app/settings" ? settingsHref(section.key) : null;
}

/** Polar-style sub-links under the current section, on a hairline rail. */
function SubNav({
	items,
	label,
	onNavigate,
}: {
	items: SubItem[];
	label: string;
	onNavigate?: () => void;
}) {
	const current = useCurrentSettingsHref();
	return (
		<nav
			aria-label={`Workspace ${label.toLowerCase()}`}
			className="my-1.5 ml-[17px] flex flex-col gap-0.5 border-l border-white/[0.08] pl-3"
		>
			{items.map((item) => {
				const active = current === item.href;
				return (
					<Link
						aria-current={active ? "page" : undefined}
						className={cn(
							"rounded-md py-1 text-sm font-medium transition-colors",
							active ? "text-white" : "text-[#858585] hover:text-neutral-200",
						)}
						href={item.href}
						key={item.href}
						onClick={onNavigate}
					>
						{item.label}
					</Link>
				);
			})}
		</nav>
	);
}

/** The icon rail's version of SubNav: section icons under the parent, with tooltips. */
function RailSubNav({
	items,
	label,
	onNavigate,
}: {
	items: SubItem[];
	label: string;
	onNavigate?: () => void;
}) {
	const current = useCurrentSettingsHref();
	return (
		<nav
			aria-label={`Workspace ${label.toLowerCase()}`}
			className="mt-1 grid gap-0.5 border-y border-white/[0.06] py-1"
		>
			{items.map(({ href, icon: Icon, label: itemLabel }) => {
				const active = current === href;
				return (
					<Tooltip key={href}>
						<TooltipTrigger asChild>
							<Link
								aria-current={active ? "page" : undefined}
								aria-label={itemLabel}
								className={cn(
									"grid size-8 place-items-center rounded-lg border transition-colors",
									active
										? cn(surface.panel, surface.border, "text-white shadow-xs")
										: "border-transparent text-neutral-500 hover:text-neutral-200",
								)}
								href={href}
								onClick={onNavigate}
							>
								<Icon className="shrink-0" size={14} strokeWidth={1.8} />
							</Link>
						</TooltipTrigger>
						<TooltipContent side="right">{itemLabel}</TooltipContent>
					</Tooltip>
				);
			})}
		</nav>
	);
}

function NavLink({
	collapsed,
	item: { children, href, icon: Icon, label },
	onNavigate,
}: {
	collapsed: boolean;
	item: NavItem;
	onNavigate?: () => void;
}) {
	const pathname = usePathname();
	const active = isActive(pathname, href);
	const rowClassName = cn(
		// Polar-style: quiet text that brightens on hover, and a raised pill
		// one surface lighter for the current page instead of an accent color.
		"flex h-8 items-center gap-2.5 rounded-lg border px-2 text-sm font-medium transition-colors",
		active
			? cn(surface.panel, surface.border, "text-white shadow-xs")
			: cn(
					"border-transparent hover:text-neutral-200",
					// Icon-only rail items keep the Shots grey; visible labels use the AA grey.
					collapsed ? "text-neutral-500" : "text-[#858585]",
				),
		collapsed && "size-8 justify-center px-0",
	);
	const link = (
		<Link
			aria-current={active ? "page" : undefined}
			aria-label={collapsed ? label : undefined}
			className={rowClassName}
			href={href}
			onClick={onNavigate}
		>
			<Icon className="shrink-0" size={16} strokeWidth={1.8} />
			{collapsed ? null : <span className="truncate">{label}</span>}
		</Link>
	);
	// Section lists start open on their own page. The pathname key resets them on
	// navigation, so an open list closes when you go to another page.
	if (!collapsed) {
		if (!children) return link;
		return (
			<Collapsible defaultOpen={active} key={pathname}>
				<CollapsibleTrigger className={cn(rowClassName, "group w-full cursor-pointer")}>
					<Icon className="shrink-0" size={16} strokeWidth={1.8} />
					<span className="truncate">{label}</span>
					<CaretDown
						className="ml-auto shrink-0 transition-transform group-data-[state=closed]:-rotate-90"
						size={14}
					/>
				</CollapsibleTrigger>
				<CollapsibleContent>
					<SubNav items={children} label={label} onNavigate={onNavigate} />
				</CollapsibleContent>
			</Collapsible>
		);
	}
	if (children) {
		// The rail has no room for labels, so the section icons expand below.
		return (
			<Collapsible defaultOpen={active} key={pathname}>
				<Tooltip>
					<TooltipTrigger asChild>
						<CollapsibleTrigger asChild>
							<button
								aria-label={label}
								className={cn(
									"grid size-8 cursor-pointer place-items-center rounded-lg border transition-colors",
									active
										? cn(surface.panel, surface.border, "text-white shadow-xs")
										: "border-transparent text-neutral-500 hover:text-neutral-200 data-[state=open]:text-neutral-200",
								)}
								type="button"
							>
								<Icon className="shrink-0" size={16} strokeWidth={1.8} />
							</button>
						</CollapsibleTrigger>
					</TooltipTrigger>
					<TooltipContent side="right">{label}</TooltipContent>
				</Tooltip>
				<CollapsibleContent>
					<RailSubNav items={children} label={label} onNavigate={onNavigate} />
				</CollapsibleContent>
			</Collapsible>
		);
	}
	return (
		<Tooltip>
			<TooltipTrigger asChild>{link}</TooltipTrigger>
			<TooltipContent side="right">{label}</TooltipContent>
		</Tooltip>
	);
}

function Navigation({ collapsed, onNavigate }: { collapsed: boolean; onNavigate?: () => void }) {
	return (
		<nav aria-label="Main navigation" className={cn("grid", collapsed ? "gap-3" : "gap-6")}>
			{NAV_GROUPS.map((group, index) => (
				<div className="grid gap-0.5" key={index}>
					{group.label && !collapsed ? (
						<p className="mb-1.5 px-2 text-xs font-medium text-[#858585]">{group.label}</p>
					) : null}
					{group.items.map((item) => (
						<NavLink collapsed={collapsed} item={item} key={item.href} onNavigate={onNavigate} />
					))}
				</div>
			))}
		</nav>
	);
}

type AuthenticatedSession = Extract<BrowserSessionResponse, { authenticated: true }>;

/** Workspace and account in one row, like a team switcher; opens the account menu. */
function AccountSwitcher({
	collapsed,
	session,
	workspaces,
}: {
	collapsed: boolean;
	session: AuthenticatedSession;
	workspaces: WorkspaceSummary[];
}) {
	const organizationName =
		workspaces.find((workspace) => workspace.id === session.user.organizationId)?.name ??
		"Workspace";
	return (
		<WorkspaceSwitcher
			activeWorkspaceId={session.user.organizationId}
			align={collapsed ? "end" : "start"}
			side={collapsed ? "right" : "top"}
			trigger={
				<button
					aria-label={`Open account and workspace menu for ${session.user.name || session.user.email}`}
					className={cn(
						"flex h-11 w-full items-center gap-2.5 rounded-lg px-2 text-left transition-colors hover:bg-white/[0.04]",
						collapsed && "size-8 justify-center px-0",
					)}
					type="button"
				>
					<WorkspaceBadge name={organizationName} />
					{collapsed ? null : (
						<>
							<span className="min-w-0 flex-1">
								<span className="block truncate text-sm font-medium text-neutral-100">
									{organizationName}
								</span>
								<span className="block truncate text-xs text-[#858585]">{session.user.email}</span>
							</span>
							<CaretUpDown className="shrink-0 text-neutral-500" size={14} />
						</>
					)}
				</button>
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
	);
}

function Brand({ brand, collapsed }: { brand?: Branding; collapsed: boolean }) {
	const name = brand?.name || "Verifier";
	return (
		<Link
			aria-label={collapsed ? name : undefined}
			className="flex min-w-0 items-center gap-2.5"
			href="/app"
		>
			<img
				alt=""
				className="size-6 rounded-md object-contain"
				src={brand?.logoUrl || "/check-email-logo.svg"}
			/>
			{collapsed ? null : (
				<span className="truncate text-sm font-semibold tracking-tight text-neutral-100">
					{name}
				</span>
			)}
		</Link>
	);
}

/**
 * Desktop: a fixed icon rail that never expands; labels show as tooltips and
 * Settings expands its section icons below it. Mobile: a full drawer.
 */
export function PlatformSidebar({
	brand,
	devBadgeOffset = false,
	session,
	workspaces,
}: {
	brand?: Branding;
	/** Leaves room for the Next.js dev badge, which otherwise covers the switcher. */
	devBadgeOffset?: boolean;
	session: AuthenticatedSession;
	workspaces: WorkspaceSummary[];
}) {
	const [mobileOpen, setMobileOpen] = useState(false);

	return (
		<TooltipProvider>
			<aside
				className={cn(
					"sticky top-0 hidden h-[var(--app-vh)] w-14 shrink-0 flex-col items-center border-r border-white/[0.04] py-3 md:flex",
					surface.base,
				)}
				data-slot="app-sidebar"
			>
				<Brand brand={brand} collapsed />

				{/* The nav scrolls on short screens, so the switcher stays visible. */}
				<div className="scrollbar-hide mt-5 min-h-0 flex-1 overflow-y-auto">
					<Navigation collapsed />
				</div>

				{/* In development, room for the developer toolkit button and the
				    Next.js dev badge. They sit on the switcher and take its clicks. */}
				<div
					className={cn(
						"flex w-full justify-center border-t border-white/[0.06] pt-2",
						devBadgeOffset && "mb-24",
					)}
				>
					<AccountSwitcher collapsed session={session} workspaces={workspaces} />
				</div>
			</aside>

			<header
				className={cn(
					"sticky top-0 z-30 flex h-14 items-center gap-3 border-b border-white/[0.06] px-4 md:hidden",
					surface.base,
				)}
				data-slot="app-top-bar"
			>
				<Sheet onOpenChange={setMobileOpen} open={mobileOpen}>
					<SheetTrigger asChild>
						<button
							aria-label="Open navigation"
							className="grid size-9 place-items-center rounded-lg text-neutral-400 hover:bg-white/[0.04] hover:text-neutral-100"
							type="button"
						>
							<List size={18} strokeWidth={1.8} />
						</button>
					</SheetTrigger>
					<SheetContent
						className={cn("w-72 gap-0 border-white/[0.06] p-3", surface.base)}
						side="left"
					>
						<SheetTitle className="sr-only">Navigation</SheetTitle>
						<div className="px-2 py-1">
							<Brand brand={brand} collapsed={false} />
						</div>
						<div className="mt-6 min-h-0 flex-1 overflow-y-auto">
							<Navigation collapsed={false} onNavigate={() => setMobileOpen(false)} />
						</div>
						<div className={cn("border-t border-white/[0.06] pt-2", devBadgeOffset && "mb-24")}>
							<AccountSwitcher collapsed={false} session={session} workspaces={workspaces} />
						</div>
					</SheetContent>
				</Sheet>
				<Brand brand={brand} collapsed={false} />
			</header>
		</TooltipProvider>
	);
}
