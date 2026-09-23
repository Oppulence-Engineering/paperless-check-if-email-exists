"use client";

import "client-only";

import Link from "next/link";
import { useMemo, useState, type ComponentPropsWithoutRef, type ReactElement } from "react";

import { Avatar, AvatarFallback, AvatarImage } from "@oppulence/ui/components/avatar";
import { Badge } from "@oppulence/ui/components/badge";
import { Button } from "@oppulence/ui/components/button";
import {
	Dialog,
	DialogContent,
	DialogDescription,
	DialogFooter,
	DialogHeader,
	DialogTitle,
} from "@oppulence/ui/components/dialog";
import {
	DropdownMenu,
	DropdownMenuContent,
	DropdownMenuGroup,
	DropdownMenuItem,
	DropdownMenuLabel,
	DropdownMenuSeparator,
	DropdownMenuTrigger,
} from "@oppulence/ui/components/dropdown-menu";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import {
	Buildings,
	CaretUpDown,
	Check,
	CircleNotch,
	GearSix,
	Key,
	Plus,
	SlidersHorizontal,
	ChartLineUp,
	Plugs,
} from "@/lib/icons";
import { createBrowserWorkspace, switchBrowserWorkspace } from "@/lib/auth/client";
import { settingsHref } from "@/lib/dashboard/settings-navigation";
import { WORKSPACE_PARAM } from "@/components/features/workspaces/workspace-url-scope/workspace-url-scope";

import { type WorkspaceSwitcherPropsFields } from "./workspace-switcher.schema";

/**
 * @oppulence-gen kind=component
 * WorkspaceSwitcher keeps the active tenant visible and lets a member move
 * between authorized workspaces without exposing session credentials.
 *
 * Data arrives through validated props. The only mutation delegates to the
 * browser auth client. Owned by `workspace-switcher.lit.ts`.
 */
export type WorkspaceSwitcherProps = WorkspaceSwitcherPropsFields &
	ComponentPropsWithoutRef<"section"> & {
		menuContainer?: HTMLElement | null;
		trigger?: ReactElement;
	};

function monogram(name: string): string {
	return name
		.split(/\s+/)
		.slice(0, 2)
		.map((part) => part[0])
		.join("")
		.toUpperCase();
}

function roleLabel(role: string): string {
	return role
		.split(",")
		.map((value) => value.trim())
		.filter(Boolean)
		.join(" · ");
}

function slugify(value: string): string {
	return value
		.toLowerCase()
		.trim()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "")
		.slice(0, 64);
}

export function WorkspaceSwitcher({
	activeWorkspaceId,
	children,
	className,
	menuContainer,
	planLabel,
	trigger,
	workspaces,
	...props
}: WorkspaceSwitcherProps) {
	const [switchingTo, setSwitchingTo] = useState<string | null>(null);
	const [error, setError] = useState<string | null>(null);
	const [createOpen, setCreateOpen] = useState(false);
	const [createName, setCreateName] = useState("");
	const [createSlug, setCreateSlug] = useState("");
	const [slugEdited, setSlugEdited] = useState(false);
	const [creating, setCreating] = useState(false);
	const [createError, setCreateError] = useState<string | null>(null);
	const activeWorkspace = useMemo(
		() => workspaces.find((workspace) => workspace.id === activeWorkspaceId) ?? workspaces[0],
		[activeWorkspaceId, workspaces],
	);

	const switchWorkspace = async (organizationId: string) => {
		if (organizationId === activeWorkspaceId || switchingTo) return;
		setSwitchingTo(organizationId);
		setError(null);
		try {
			await switchBrowserWorkspace(organizationId);
			// Name the workspace in the URL so the address someone copies opens the
			// workspace they were looking at.
			const slug = workspaces.find((workspace) => workspace.id === organizationId)?.slug;
			if (slug && typeof window !== "undefined") {
				const url = new URL(window.location.href);
				url.searchParams.set(WORKSPACE_PARAM, slug);
				window.history.replaceState(null, "", url.toString());
			}
		} catch {
			setError("We could not switch workspaces. Try again.");
			setSwitchingTo(null);
		}
	};

	const handleCreateOpenChange = (open: boolean) => {
		setCreateOpen(open);
		if (open) {
			setCreateName("");
			setCreateSlug("");
			setSlugEdited(false);
			setCreateError(null);
		}
	};

	const createWorkspace = async () => {
		setCreating(true);
		setCreateError(null);
		try {
			await createBrowserWorkspace({
				name: createName.trim(),
				slug: createSlug.trim(),
			});
		} catch {
			setCreateError("We could not create that workspace. Check the workspace URL and try again.");
			setCreating(false);
		}
	};

	if (!activeWorkspace && !trigger) return null;

	const integrated = Boolean(trigger);

	return (
		<section
			className={cn(integrated ? "min-w-0" : "px-2 pb-2", className)}
			data-slot="workspace-switcher"
			{...props}
		>
			<DropdownMenu>
				<DropdownMenuTrigger asChild>
					{trigger ?? (
						<Button
							aria-label={`Switch workspace, current workspace ${activeWorkspace?.name ?? "unknown"}`}
							className="h-auto w-full justify-start gap-2.5 rounded-[6px] border border-border/70 bg-[var(--surface-2)] px-2.5 py-2 text-left shadow-[0_1px_2px_rgba(0,0,0,0.04)] transition-[transform,background-color,border-color] active:scale-[0.98] hover:border-primary/15 hover:bg-background-100 data-[state=open]:border-primary/20 data-[state=open]:bg-background-100"
							type="button"
							variant="ghost"
						>
							<Avatar className="size-8 rounded-[5px]" size="sm">
								{activeWorkspace?.logoUrl ? (
									<AvatarImage alt="" className="object-cover" src={activeWorkspace.logoUrl} />
								) : null}
								<AvatarFallback className="rounded-[5px] bg-primary font-mono text-[10px] font-semibold text-primary-foreground">
									{monogram(activeWorkspace?.name ?? "Workspace")}
								</AvatarFallback>
							</Avatar>
							<span className="min-w-0 flex-1">
								<span className="block truncate text-[13px] font-medium text-primary">
									{activeWorkspace?.name}
								</span>
								<span className="block truncate text-[11px] capitalize text-primary/45">
									{roleLabel(activeWorkspace?.role ?? "member")}
								</span>
							</span>
							{planLabel ? (
								<Badge
									className="rounded-[4px] px-1.5 text-[9px] uppercase tracking-[0.08em]"
									variant="outline"
								>
									{planLabel}
								</Badge>
							) : null}
							<CaretUpDown className="size-3.5 shrink-0 text-primary/35" />
						</Button>
					)}
				</DropdownMenuTrigger>
				<DropdownMenuContent
					align="start"
					aria-label={integrated ? "Account and workspace menu" : "Workspace menu"}
					className={cn(
						"p-1.5",
						integrated ? "app-shell w-[292px] rounded-none" : "w-[272px] rounded-[8px]",
					)}
					container={menuContainer}
					side={integrated ? "top" : "bottom"}
					sideOffset={integrated ? 8 : 6}
				>
					{activeWorkspace ? (
						<DropdownMenuGroup aria-label="Workspaces">
							<DropdownMenuLabel className="px-2 pb-1 pt-1.5 text-[10px] font-medium uppercase tracking-[0.12em] text-primary/40">
								Workspaces
							</DropdownMenuLabel>
							{workspaces.map((workspace) => {
								const active = workspace.id === activeWorkspaceId;
								const switching = workspace.id === switchingTo;
								return (
									<DropdownMenuItem
										aria-busy={switching || undefined}
										aria-current={active ? "true" : undefined}
										className="min-h-11 gap-2.5 rounded-[6px] px-2 active:scale-[0.98]"
										disabled={Boolean(switchingTo)}
										key={workspace.id}
										onSelect={() => void switchWorkspace(workspace.id)}
									>
										<Avatar className="size-7 rounded-[5px]" size="sm">
											{workspace.logoUrl ? <AvatarImage alt="" src={workspace.logoUrl} /> : null}
											<AvatarFallback className="rounded-[5px] bg-background-200 font-mono text-[9px] font-semibold text-primary/70">
												{monogram(workspace.name)}
											</AvatarFallback>
										</Avatar>
										<span className="min-w-0 flex-1">
											<Label className="block truncate text-[12px] font-medium text-primary">
												{workspace.name}
											</Label>
											<span
												aria-live="polite"
												className="block truncate text-[10px] capitalize text-primary/45"
											>
												{switching ? "Switching…" : roleLabel(workspace.role)}
											</span>
										</span>
										{active ? (
											<>
												<span className="sr-only">Current workspace</span>
												<Check className="size-3.5 text-[var(--oppulence-orange)]" />
											</>
										) : null}
									</DropdownMenuItem>
								);
							})}
							{error ? (
								<p className="px-2 py-1.5 text-[11px] text-destructive" role="alert">
									{error}
								</p>
							) : null}
							<DropdownMenuSeparator />
							<DropdownMenuItem
								className="rounded-[6px]"
								disabled={Boolean(switchingTo)}
								onSelect={() => setCreateOpen(true)}
							>
								<Plus />
								Create workspace
							</DropdownMenuItem>
							<DropdownMenuItem asChild className="rounded-[6px]">
								<Link href={settingsHref("workspace")}>
									<GearSix />
									Manage workspaces
								</Link>
							</DropdownMenuItem>
							<DropdownMenuItem asChild className="rounded-[6px]">
								<Link href={settingsHref("members")}>
									<Buildings />
									Members and access
								</Link>
							</DropdownMenuItem>
						</DropdownMenuGroup>
					) : null}
					{activeWorkspace ? (
						<>
							<DropdownMenuSeparator />
							<DropdownMenuGroup aria-label="Settings and developer tools">
								<DropdownMenuLabel className="px-2 pb-1 pt-1.5 text-[10px] font-medium uppercase tracking-[0.12em] text-primary/40">
									Settings
								</DropdownMenuLabel>
								{[
									{ key: "overview", label: "All settings", icon: GearSix },
									{ key: "developer", label: "API keys", icon: Key },
									{ key: "verification", label: "Verification defaults", icon: SlidersHorizontal },
									{ key: "usage", label: "Usage", icon: ChartLineUp },
									{ key: "webhooks", label: "Webhooks", icon: Plugs },
								].map((item) => (
									<DropdownMenuItem asChild className="rounded-[6px]" key={item.key}>
										<Link href={settingsHref(item.key)}>
											<item.icon />
											{item.label}
										</Link>
									</DropdownMenuItem>
								))}
							</DropdownMenuGroup>
						</>
					) : null}
					{children ? (
						<>
							{activeWorkspace ? <DropdownMenuSeparator /> : null}
							{children}
						</>
					) : null}
				</DropdownMenuContent>
			</DropdownMenu>
			<Dialog onOpenChange={handleCreateOpenChange} open={createOpen}>
				<DialogContent className="rounded-none sm:max-w-md">
					<DialogHeader>
						<DialogTitle>Create a workspace</DialogTitle>
						<DialogDescription>
							Create a separate tenant for a team, company, or environment. You will become its
							owner and switch to it automatically.
						</DialogDescription>
					</DialogHeader>
					<form
						className="space-y-4"
						onSubmit={(event) => {
							event.preventDefault();
							void createWorkspace();
						}}
					>
						<div className="space-y-2">
							<Label htmlFor="new-workspace-name">Workspace name</Label>
							<Input
								id="new-workspace-name"
								maxLength={80}
								onChange={(event) => {
									const nextName = event.target.value;
									setCreateName(nextName);
									if (!slugEdited) setCreateSlug(slugify(nextName));
								}}
								placeholder="Acme Operations"
								value={createName}
							/>
						</div>
						<div className="space-y-2">
							<Label htmlFor="new-workspace-slug">Workspace handle</Label>
							<div className="flex items-center rounded-none border border-input bg-background px-3 focus-within:ring-2 focus-within:ring-ring/50">
								<Input
									aria-describedby="new-workspace-slug-help"
									className="border-0 px-0 font-mono shadow-none focus-visible:ring-0"
									id="new-workspace-slug"
									maxLength={64}
									onChange={(event) => {
										setCreateSlug(slugify(event.target.value));
										setSlugEdited(true);
									}}
									placeholder="acme-operations"
									value={createSlug}
								/>
							</div>
							<p className="text-xs text-muted-foreground" id="new-workspace-slug-help">
								A unique identifier using lowercase letters, numbers, and hyphens.
							</p>
						</div>
						{createError ? (
							<p className="text-sm text-destructive" role="alert">
								{createError}
							</p>
						) : null}
						<DialogFooter>
							<Button
								disabled={creating}
								onClick={() => setCreateOpen(false)}
								type="button"
								variant="outline"
							>
								Cancel
							</Button>
							<Button
								disabled={creating || createName.trim().length < 2 || createSlug.length < 2}
								type="submit"
							>
								{creating ? <CircleNotch className="size-4 animate-spin" /> : <Plus />}
								{creating ? "Creating…" : "Create workspace"}
							</Button>
						</DialogFooter>
					</form>
				</DialogContent>
			</Dialog>
		</section>
	);
}
