"use client";

import "client-only";

import {
	useCallback,
	useEffect,
	useState,
	type ComponentPropsWithoutRef,
	type SyntheticEvent,
} from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { z } from "zod";

import { Alert, AlertDescription } from "@oppulence/ui/components/alert";
import { Avatar, AvatarFallback, AvatarImage } from "@oppulence/ui/components/avatar";
import { Button } from "@oppulence/ui/components/button";
import {
	Card,
	CardContent,
	CardDescription,
	CardHeader,
	CardTitle,
} from "@oppulence/ui/components/card";
import { useEntitlements } from "@/components/auth/auth-gate";
import { denial } from "@/lib/entitlements/entitlements";
import { Badge } from "@oppulence/ui/components/badge";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@oppulence/ui/components/select";
import {
	Table,
	TableBody,
	TableCell,
	TableHead,
	TableHeader,
	TableRow,
} from "@oppulence/ui/components/table";
import { Textarea } from "@oppulence/ui/components/textarea";
import { cn } from "@oppulence/ui/lib/utils";

import { authClient } from "@/lib/auth/auth-client";
import { switchBrowserWorkspace } from "@/lib/auth/client";
import { dashboardRequest } from "@/lib/auth/dashboard-fetch";
import {
	IdentityActionResultSchema,
	IdentityAuditListSchema,
	IdentityDomainVerificationSchema,
	IdentityFullOrganizationSchema,
	IdentityOrganizationListSchema,
	IdentityPasskeyListSchema,
	IdentitySCIMMappingsSchema,
	IdentitySCIMProvidersSchema,
	IdentitySCIMTokenSchema,
	IdentitySessionListSchema,
	IdentitySSOProvidersSchema,
	IdentityTOTPEnrollmentSchema,
	type IdentitySettingsPropsFields,
} from "./identity-settings.schema";

/**
 * @oppulence-gen kind=component
 * Better Auth organization and enterprise identity administration.
 * Owned by the sibling `identity-settings.lit.ts`.
 */

export type IdentitySettingsProps = IdentitySettingsPropsFields &
	ComponentPropsWithoutRef<"section">;

async function authRequest<T>(
	path: string,
	schema: z.ZodType<T>,
	body?: unknown,
	method = body === undefined ? "GET" : "POST",
): Promise<T> {
	const response = await dashboardRequest(path.startsWith("/api/") ? path : `/api/auth${path}`, {
		method,
		credentials: "include",
		cache: "no-store",
		headers:
			body === undefined ? { Accept: "application/json" } : { "Content-Type": "application/json" },
		body: body === undefined ? undefined : JSON.stringify(body),
	});
	if (!response.ok) {
		const payload = z
			.object({ code: z.string().optional(), error: z.string().optional() })
			.loose()
			.safeParse(await response.json().catch(() => null));
		if (payload.success && payload.data.code === "step_up_required") {
			throw new Error(
				"Verify with a passkey or TOTP in Authentication & sessions, then retry this change.",
			);
		}
		throw new Error(
			payload.success ? payload.data.error : `Request failed (${String(response.status)})`,
		);
	}
	return schema.parse(response.status === 204 ? {} : await response.json());
}

function SettingsCard({
	title,
	description,
	children,
	visible = true,
}: {
	title: string;
	description: string;
	children: React.ReactNode;
	visible?: boolean;
}) {
	if (!visible) return null;
	return (
		<Card className="gap-5 rounded-none border-primary/10 bg-background/80 py-5 shadow-sm shadow-black/[0.02]">
			<CardHeader className="gap-1.5 px-5">
				<CardTitle className="text-sm">
					<h2>{title}</h2>
				</CardTitle>
				<CardDescription className="text-xs">{description}</CardDescription>
			</CardHeader>
			<CardContent className="space-y-4 px-5">{children}</CardContent>
		</Card>
	);
}

/** Two letters for a member avatar: initials when there is a name, else the address. */
function memberInitials(name: string | null | undefined, email: string): string {
	const source = (name ?? "").trim() || email;
	const parts = source.split(/[\s@._-]+/).filter(Boolean);
	const letters = parts.length > 1 ? `${parts[0][0]}${parts[1][0]}` : source.slice(0, 2);
	return letters.toUpperCase();
}

function field(form: FormData, name: string): string {
	const value = form.get(name);
	return typeof value === "string" ? value.trim() : "";
}

const WORKSPACE_LOGO_TYPES = new Set(["image/jpeg", "image/png", "image/webp"]);
const MAX_WORKSPACE_LOGO_BYTES = 256 * 1024;

async function workspaceLogo(form: FormData): Promise<string | null> {
	const uploaded = form.get("logoFile");
	if (uploaded instanceof File && uploaded.size > 0) {
		if (!WORKSPACE_LOGO_TYPES.has(uploaded.type)) {
			throw new Error("Workspace logos must be PNG, JPEG, or WebP images.");
		}
		if (uploaded.size > MAX_WORKSPACE_LOGO_BYTES) {
			throw new Error("Workspace logos must be 256 KB or smaller.");
		}
		return await new Promise<string>((resolve, reject) => {
			const reader = new FileReader();
			reader.onerror = () => {
				reject(new Error("The workspace logo could not be read."));
			};
			reader.onload = () => {
				if (typeof reader.result === "string") resolve(reader.result);
				else reject(new Error("The workspace logo could not be read."));
			};
			reader.readAsDataURL(uploaded);
		});
	}
	return field(form, "logo") || null;
}

export function IdentitySettings({
	className,
	organizationId,
	organizationRole,
	scope = "all",
	userId,
	...props
}: IdentitySettingsProps) {
	const router = useRouter();
	const isAdministrator = organizationRole
		.split(",")
		.some((value) => value === "owner" || value === "admin");
	const isOwner = organizationRole.split(",").some((value) => value === "owner");
	const showWorkspace = scope === "all" || scope === "workspace";
	const showOrganization = scope === "all" || scope === "organization";
	const showSecurity = scope === "all" || scope === "security";
	const showEnterprise = scope === "all" || scope === "enterprise";
	const showBranding = scope === "all" || scope === "branding";
	const heading = {
		all: {
			title: "Identity and organization",
			description:
				"Manage members, authentication, enterprise provisioning, sessions, and branding.",
		},
		workspace: {
			title: "Workspace details",
			description: "Manage the active tenant identity and the context used by backend requests.",
		},
		organization: {
			title: "Members and access",
			description: "Manage organizations, invitations, membership, and workspace roles.",
		},
		security: {
			title: "Security",
			description: "Manage passkeys, administrator step-up, TOTP, and active sessions.",
		},
		enterprise: {
			title: "Security and compliance",
			description:
				"Configure administrator policy, verified-domain SSO, SCIM provisioning, and identity audit history.",
		},
		branding: {
			title: "Organization branding",
			description: "Control brand assets, colors, support links, and identity policies.",
		},
	}[scope];
	const [organizations, setOrganizations] = useState<
		z.infer<typeof IdentityOrganizationListSchema>
	>([]);
	const [organization, setOrganization] = useState<z.infer<
		typeof IdentityFullOrganizationSchema
	> | null>(null);
	const entitlements = useEntitlements();
	// Seats are counted from the members this panel already loaded.
	const seatDenial = denial(
		{
			...entitlements,
			seats: { ...entitlements.seats, used: organization?.members.length ?? 0 },
		},
		"invite_members",
	);
	const [passkeys, setPasskeys] = useState<z.infer<typeof IdentityPasskeyListSchema>>([]);
	const [sessions, setSessions] = useState<z.infer<typeof IdentitySessionListSchema>["sessions"]>(
		[],
	);
	const [ssoProviders, setSsoProviders] = useState<
		z.infer<typeof IdentitySSOProvidersSchema>["providers"]
	>([]);
	const [scimProviders, setScimProviders] = useState<
		z.infer<typeof IdentitySCIMProvidersSchema>["providers"]
	>([]);
	const [scimMappings, setScimMappings] = useState<
		z.infer<typeof IdentitySCIMMappingsSchema>["mappings"]
	>([]);
	const [auditEvents, setAuditEvents] = useState<z.infer<typeof IdentityAuditListSchema>["events"]>(
		[],
	);
	const [scimToken, setScimToken] = useState<string>();
	const [domainVerificationToken, setDomainVerificationToken] = useState<string>();
	const [totpEnrollment, setTotpEnrollment] = useState<z.infer<
		typeof IdentityTOTPEnrollmentSchema
	> | null>(null);
	const [ssoType, setSsoType] = useState<"oidc" | "saml">("oidc");
	const [busy, setBusy] = useState(false);
	const [notice, setNotice] = useState<string>();
	const [error, setError] = useState<string>();

	const refresh = useCallback(async () => {
		const requests: Promise<void>[] = [];

		if (showWorkspace) {
			requests.push(
				authRequest("/organization/list", IdentityOrganizationListSchema).then(setOrganizations),
			);
		}

		if (showWorkspace || showOrganization || showEnterprise || showBranding) {
			requests.push(
				authRequest("/organization/get-full-organization", IdentityFullOrganizationSchema).then(
					setOrganization,
				),
			);
		}

		if (showSecurity) {
			requests.push(
				authRequest("/passkey/list-user-passkeys", IdentityPasskeyListSchema).then(setPasskeys),
				authRequest("/api/auth/sessions", IdentitySessionListSchema).then((value) => {
					setSessions(value.sessions);
				}),
			);
		}

		if (isAdministrator && showEnterprise) {
			requests.push(
				authRequest("/sso/providers", IdentitySSOProvidersSchema).then((value) => {
					setSsoProviders(value.providers);
				}),
				authRequest("/api/auth/scim-admin?view=providers", IdentitySCIMProvidersSchema).then(
					(value) => {
						setScimProviders(value.providers);
					},
				),
				authRequest("/api/auth/scim-admin?view=mappings", IdentitySCIMMappingsSchema).then(
					(value) => {
						setScimMappings(value.mappings);
					},
				),
				authRequest("/api/auth/audit", IdentityAuditListSchema).then((value) => {
					setAuditEvents(value.events);
				}),
			);
		}

		await Promise.all(requests);
	}, [
		isAdministrator,
		showBranding,
		showEnterprise,
		showOrganization,
		showSecurity,
		showWorkspace,
	]);

	useEffect(() => {
		const timeout = window.setTimeout(() => {
			void refresh().catch(() => {
				setError("Identity settings are temporarily unavailable.");
			});
		}, 0);
		return () => {
			window.clearTimeout(timeout);
		};
	}, [refresh]);

	async function run(message: string, action: () => Promise<unknown>, reload = true) {
		setBusy(true);
		setError(undefined);
		setNotice(undefined);
		try {
			await action();
			setNotice(message);
			if (reload) await refresh();
		} catch (cause) {
			setError(cause instanceof Error ? cause.message : "The identity request failed.");
		} finally {
			setBusy(false);
		}
	}

	async function submitInvitation(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		await run("Invitation sent.", () =>
			authRequest("/organization/invite-member", IdentityActionResultSchema, {
				email: field(form, "email"),
				role: field(form, "role"),
				organizationId,
			}),
		);
		event.currentTarget.reset();
	}

	async function submitOwnershipTransfer(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		if (field(form, "confirmation") !== organization?.slug) {
			setError(`Type ${organization?.slug ?? "the workspace slug"} to confirm ownership transfer.`);
			return;
		}
		await run("Workspace ownership transferred. Your role is now administrator.", () =>
			authRequest("/api/auth/workspace-admin", IdentityActionResultSchema, {
				action: "transfer_ownership",
				memberId: field(form, "memberId"),
			}),
		);
	}

	async function submitWorkspaceLifecycle(
		event: SyntheticEvent<HTMLFormElement>,
		action: "archive_workspace" | "delete_workspace",
	) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		if (field(form, "confirmation") !== organization?.slug) {
			setError(`Type ${organization?.slug ?? "the workspace slug"} to confirm this change.`);
			return;
		}
		await run(
			action === "archive_workspace" ? "Workspace archived." : "Workspace deleted.",
			async () => {
				await authRequest("/api/auth/workspace-admin", IdentityActionResultSchema, { action });
				router.replace("/app");
				router.refresh();
			},
			false,
		);
	}

	async function submitWorkspace(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		await run("Workspace details saved.", async () => {
			await authRequest("/api/auth/workspace-admin", IdentityActionResultSchema, {
				action: "update_workspace",
				name: field(form, "name"),
				slug: field(form, "slug"),
				logo: await workspaceLogo(form),
			});
		});
	}

	async function submitSso(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		const providerId = field(form, "providerId");
		const base = {
			providerId,
			issuer: field(form, "issuer"),
			domain: field(form, "domain"),
			organizationId,
		};
		const body =
			ssoType === "oidc"
				? {
						...base,
						oidcConfig: {
							clientId: field(form, "clientId"),
							clientSecret: field(form, "clientSecret"),
							discoveryEndpoint: field(form, "discoveryEndpoint") || undefined,
							pkce: true,
						},
					}
				: {
						...base,
						samlConfig: {
							entryPoint: field(form, "entryPoint"),
							cert: field(form, "certificate"),
							callbackUrl: `${window.location.origin}/api/auth/sso/saml2/sp/acs/${encodeURIComponent(providerId)}`,
							spMetadata: {},
							wantAssertionsSigned: true,
						},
					};
		await run("Identity provider registered. Verify its domain before use.", () =>
			authRequest("/sso/register", IdentityActionResultSchema, body),
		);
		event.currentTarget.reset();
	}

	async function submitBranding(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		const optional = (name: string) => field(form, name) || undefined;
		await run("Organization identity policy saved.", () =>
			authRequest("/organization/update", IdentityActionResultSchema, {
				organizationId,
				data: {
					brandName: optional("brandName"),
					brandLogoUrl: optional("brandLogoUrl"),
					brandWordmarkUrl: optional("brandWordmarkUrl"),
					brandFaviconUrl: optional("brandFaviconUrl"),
					brandPrimaryColor: optional("brandPrimaryColor"),
					brandAccentColor: optional("brandAccentColor"),
					supportEmail: optional("supportEmail"),
					documentationUrl: optional("documentationUrl"),
					termsUrl: optional("termsUrl"),
					privacyUrl: optional("privacyUrl"),
					requireSso: form.get("requireSso") === "on",
					maxSessionAgeSeconds: Number(field(form, "maxSessionAgeSeconds")),
					idleTimeoutSeconds: optional("idleTimeoutSeconds")
						? Number(field(form, "idleTimeoutSeconds"))
						: undefined,
				},
			}),
		);
	}

	async function submitAdminPolicy(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		await run("Administrator security policy saved.", () =>
			authRequest("/api/auth/workspace-admin", IdentityActionResultSchema, {
				action: "update_policy",
				requireAdminStepUp: form.get("requireAdminStepUp") === "on",
			}),
		);
	}

	return (
		<section className={cn("space-y-4 pb-10", className)} data-slot="identity-settings" {...props}>
			<header className="settings-page-intro">
				{scope === "all" ? (
					<h2 className="settings-page-title">{heading.title}</h2>
				) : (
					<h1 className="settings-page-title">{heading.title}</h1>
				)}
				<p className="settings-page-description">{heading.description}</p>
			</header>

			{error ? (
				<Alert variant="destructive">
					<AlertDescription>{error}</AlertDescription>
				</Alert>
			) : null}
			{notice ? (
				<Alert role="status">
					<AlertDescription>{notice}</AlertDescription>
				</Alert>
			) : null}

			<SettingsCard
				title="Active workspace"
				description="Switch the tenant context used for this server session and its backend JWT."
				visible={showWorkspace}
			>
				<div className="grid gap-2">
					<Label htmlFor="active-organization">Active organization</Label>
					<Select
						disabled={busy}
						onValueChange={(value) =>
							void run(
								"Organization switched.",
								async () => {
									await switchBrowserWorkspace(value);
									window.location.reload();
								},
								false,
							)
						}
						value={organizationId}
					>
						<SelectTrigger className="w-full" id="active-organization">
							<SelectValue placeholder="Select an organization" />
						</SelectTrigger>
						<SelectContent>
							{organizations.map((item) => (
								<SelectItem key={item.id} value={item.id}>
									{item.name}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
			</SettingsCard>

			<SettingsCard
				title="Workspace identity"
				description="The name and stable slug shown across tenant-scoped product surfaces."
				visible={showWorkspace}
			>
				<form
					className="grid gap-4"
					key={`${organization?.id ?? organizationId}:${organization?.name ?? ""}:${organization?.slug ?? ""}:${organization?.logo ?? ""}`}
					onSubmit={(event) => void submitWorkspace(event)}
				>
					<div className="grid gap-3 sm:grid-cols-[auto_1fr] sm:items-center">
						<Avatar className="size-14 rounded-none">
							<AvatarImage alt="" src={organization?.logo ?? undefined} />
							<AvatarFallback className="rounded-none">
								{(organization?.name || "W").slice(0, 2).toUpperCase()}
							</AvatarFallback>
						</Avatar>
						<div className="grid gap-2">
							<div className="grid gap-1.5">
								<Label htmlFor="workspace-logo-url">Logo URL</Label>
								<Input
									defaultValue={organization?.logo ?? ""}
									disabled={busy || !isAdministrator}
									id="workspace-logo-url"
									name="logo"
									placeholder="https://example.com/logo.png"
								/>
							</div>
							<div className="grid gap-1.5">
								<Label htmlFor="workspace-logo-file">Upload a logo</Label>
								<Input
									accept="image/png,image/jpeg,image/webp"
									disabled={busy || !isAdministrator}
									id="workspace-logo-file"
									name="logoFile"
									type="file"
								/>
								<p className="text-xs text-muted-foreground">
									PNG, JPEG, or WebP. Maximum 256 KB. An upload replaces the URL above.
								</p>
							</div>
						</div>
					</div>
					<div className="grid gap-4 sm:grid-cols-2">
						<div className="grid gap-1.5">
							<Label htmlFor="workspace-name">Workspace name</Label>
							<Input
								defaultValue={organization?.name ?? ""}
								disabled={busy || !isAdministrator}
								id="workspace-name"
								name="name"
								required
							/>
						</div>
						<div className="grid gap-1.5">
							<Label htmlFor="workspace-slug">Workspace slug</Label>
							<Input
								defaultValue={organization?.slug ?? ""}
								disabled={busy || !isAdministrator}
								id="workspace-slug"
								name="slug"
								pattern="[a-z0-9]+(?:-[a-z0-9]+)*"
								required
							/>
						</div>
					</div>
					<div className="flex flex-wrap items-center justify-between gap-3 border-t pt-3">
						<div className="text-xs text-muted-foreground">
							Workspace ID: <span className="font-mono">{organizationId}</span>
						</div>
						{isAdministrator ? (
							<Button disabled={busy || !organization} size="sm" type="submit">
								Save workspace
							</Button>
						) : (
							<span className="text-xs text-muted-foreground">
								An owner or administrator can edit these details.
							</span>
						)}
					</div>
				</form>
			</SettingsCard>

			<SettingsCard
				title="Workspace lifecycle"
				description="Archive suspends access without deleting identity records. Deletion permanently removes the workspace and its membership records."
				visible={showWorkspace && isOwner}
			>
				<div className="grid gap-4 lg:grid-cols-2">
					<form
						className="grid gap-3 rounded-none border border-amber-500/30 bg-amber-500/5 p-4"
						onSubmit={(event) => void submitWorkspaceLifecycle(event, "archive_workspace")}
					>
						<div>
							<h3 className="text-sm font-medium">Archive workspace</h3>
							<p className="mt-1 text-xs text-muted-foreground">
								Blocks sessions, backend requests, and workspace switching while retaining records.
							</p>
						</div>
						<Label htmlFor="archive-confirmation">
							Type {organization?.slug ?? "the workspace slug"} to confirm
						</Label>
						<Input autoComplete="off" id="archive-confirmation" name="confirmation" required />
						<Button disabled={busy || !organization} type="submit" variant="outline">
							Archive workspace
						</Button>
					</form>
					<form
						className="grid gap-3 rounded-none border border-destructive/30 bg-destructive/5 p-4"
						onSubmit={(event) => void submitWorkspaceLifecycle(event, "delete_workspace")}
					>
						<div>
							<h3 className="text-sm font-medium text-destructive">Delete workspace</h3>
							<p className="mt-1 text-xs text-muted-foreground">
								Permanently deletes the workspace. This action cannot be undone.
							</p>
						</div>
						<Label htmlFor="delete-workspace-confirmation">
							Type {organization?.slug ?? "the workspace slug"} to confirm
						</Label>
						<Input
							autoComplete="off"
							id="delete-workspace-confirmation"
							name="confirmation"
							required
						/>
						<Button disabled={busy || !organization} type="submit" variant="destructive">
							Delete workspace
						</Button>
					</form>
				</div>
			</SettingsCard>

			<SettingsCard
				title={`Members${organization ? ` (${String(organization.members.length)})` : ""}`}
				description="Better Auth owns organization membership and the owner, admin, and member roles."
				visible={showOrganization}
			>
				<ul className="settings-panel">
					{organization?.members.map((member) => (
						<li className="settings-row" key={member.id}>
							<div className="flex min-w-0 items-center gap-3">
								<span aria-hidden className="settings-avatar">
									{memberInitials(member.user.name, member.user.email)}
								</span>
								<div className="settings-row-copy">
									<p className="settings-row-label truncate">
										{member.user.name || member.user.email}
									</p>
									{member.user.name && member.user.name !== member.user.email ? (
										<p className="settings-row-description truncate">{member.user.email}</p>
									) : null}
								</div>
							</div>
							<div className="flex shrink-0 items-center gap-2">
								{isAdministrator && !member.role.split(",").includes("owner") ? (
									<Select
										disabled={busy || member.userId === userId}
										onValueChange={(value) =>
											void run("Member role updated.", () =>
												authRequest("/api/auth/workspace-admin", IdentityActionResultSchema, {
													action: "change_member_role",
													memberId: member.id,
													role: value,
												}),
											)
										}
										value={member.role}
									>
										<SelectTrigger
											aria-label={`Role for ${member.user.email}`}
											className="w-28"
											size="sm"
										>
											<SelectValue />
										</SelectTrigger>
										<SelectContent>
											<SelectItem value="member">Member</SelectItem>
											<SelectItem value="admin">Admin</SelectItem>
										</SelectContent>
									</Select>
								) : (
									<Badge className="capitalize" variant="secondary">
										{member.role}
									</Badge>
								)}
								{isAdministrator &&
								member.userId !== userId &&
								!member.role.split(",").includes("owner") ? (
									<Button
										className="text-destructive hover:text-destructive"
										disabled={busy}
										onClick={() =>
											void run("Member removed.", () =>
												authRequest("/api/auth/workspace-admin", IdentityActionResultSchema, {
													action: "remove_member",
													memberId: member.id,
												}),
											)
										}
										size="sm"
										type="button"
										variant="ghost"
									>
										Remove
									</Button>
								) : null}
							</div>
						</li>
					))}
				</ul>
				{isAdministrator ? (
					<form
						className="grid gap-2 sm:grid-cols-[1fr_8rem_auto]"
						onSubmit={(event) => void submitInvitation(event)}
					>
						<Input
							aria-label="Invite email"
							name="email"
							placeholder="person@example.com"
							required
							type="email"
						/>
						<Select defaultValue="member" name="role">
							<SelectTrigger aria-label="Invite role" className="w-full">
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value="member">Member</SelectItem>
								<SelectItem value="admin">Admin</SelectItem>
							</SelectContent>
						</Select>
						<Button disabled={busy || seatDenial !== null} type="submit">
							Invite
						</Button>
					</form>
				) : null}
				{isAdministrator && seatDenial ? (
					<p className="settings-empty-line" role="status">
						{seatDenial.message}
					</p>
				) : null}
				<h3 className="settings-subheading">
					Pending invitations
					{organization
						? ` (${String(
								organization.invitations.filter((item) => item.status === "pending").length,
							)})`
						: ""}
				</h3>
				{organization?.invitations.some((item) => item.status === "pending") ? (
					<ul className="text-xs text-muted-foreground">
						{organization.invitations
							.filter((item) => item.status === "pending")
							.map((item) => (
								<li className="flex items-center gap-2 py-1" key={item.id}>
									<span className="flex-1">
										{item.email} · expires {item.expiresAt.toLocaleDateString()}
									</span>
									{isAdministrator ? (
										<div className="flex items-center gap-1">
											<Button
												disabled={busy}
												onClick={() =>
													void run("Invitation resent.", () =>
														authRequest("/api/auth/workspace-admin", IdentityActionResultSchema, {
															action: "resend_invitation",
															invitationId: item.id,
														}),
													)
												}
												size="sm"
												type="button"
												variant="ghost"
											>
												Resend
											</Button>
											<Button
												className="text-destructive hover:text-destructive"
												disabled={busy}
												onClick={() =>
													void run("Invitation revoked.", () =>
														authRequest("/api/auth/workspace-admin", IdentityActionResultSchema, {
															action: "revoke_invitation",
															invitationId: item.id,
														}),
													)
												}
												size="sm"
												type="button"
												variant="ghost"
											>
												Revoke
											</Button>
										</div>
									) : null}
								</li>
							))}
					</ul>
				) : (
					<p className="settings-empty-line">No invitations are waiting.</p>
				)}
			</SettingsCard>

			<SettingsCard
				title="Transfer ownership"
				description="Promote another member to owner and move your account to the administrator role. This is atomic and requires recent identity verification."
				visible={showOrganization && isOwner}
			>
				{organization?.members.some(
					(member) => member.userId !== userId && !member.role.split(",").includes("owner"),
				) ? (
					<form
						className="grid gap-3 sm:grid-cols-[1fr_1fr_auto] sm:items-end"
						onSubmit={(event) => void submitOwnershipTransfer(event)}
					>
						<div className="grid gap-1.5">
							<Label htmlFor="ownership-member">New owner</Label>
							<Select name="memberId" required>
								<SelectTrigger id="ownership-member">
									<SelectValue placeholder="Select a member" />
								</SelectTrigger>
								<SelectContent>
									{organization.members
										.filter(
											(member) =>
												member.userId !== userId && !member.role.split(",").includes("owner"),
										)
										.map((member) => (
											<SelectItem key={member.id} value={member.id}>
												{member.user.name} · {member.user.email}
											</SelectItem>
										))}
								</SelectContent>
							</Select>
						</div>
						<div className="grid gap-1.5">
							<Label htmlFor="ownership-confirmation">Type {organization.slug} to confirm</Label>
							<Input autoComplete="off" id="ownership-confirmation" name="confirmation" required />
						</div>
						<Button disabled={busy} type="submit" variant="destructive">
							Transfer ownership
						</Button>
					</form>
				) : (
					<p className="text-sm text-muted-foreground">
						Invite another member before transferring ownership.
					</p>
				)}
			</SettingsCard>

			<SettingsCard
				title="Passkeys and administrator step-up"
				description="A successful passkey or TOTP check authorizes protected identity changes for 15 minutes."
				visible={showSecurity}
			>
				<div className="flex flex-wrap gap-2">
					<Button
						disabled={busy}
						onClick={() =>
							void run("Passkey verification complete.", async () => {
								const result = await authClient.signIn.passkey({});
								if (result.error)
									throw new Error(result.error.message || "Passkey verification failed.");
							})
						}
						type="button"
						variant="outline"
					>
						Verify with passkey
					</Button>
					<Button
						disabled={busy}
						onClick={() =>
							void run("Passkey added.", async () => {
								const result = await authClient.passkey.addPasskey({
									name: "Security key",
								});
								if (result.error)
									throw new Error(result.error.message || "Passkey enrollment failed.");
							})
						}
						type="button"
					>
						Add passkey
					</Button>
				</div>
				<ul className="divide-y divide-primary/10 text-sm">
					{passkeys.map((passkey) => (
						<li className="flex items-center gap-2 py-2" key={passkey.id}>
							<span className="flex-1">
								{passkey.name || "Passkey"} · {passkey.deviceType} ·{" "}
								{passkey.createdAt.toLocaleDateString()}
							</span>
							<Button
								className="text-destructive hover:text-destructive"
								disabled={busy}
								onClick={() =>
									void run("Passkey removed.", () =>
										authRequest("/passkey/delete-passkey", IdentityActionResultSchema, {
											id: passkey.id,
										}),
									)
								}
								size="sm"
								type="button"
								variant="ghost"
							>
								Remove
							</Button>
						</li>
					))}
				</ul>
				<div className="flex flex-wrap gap-2">
					<Button
						disabled={busy}
						onClick={() =>
							void run(
								"Scan the TOTP URI, then verify a code.",
								async () => {
									setTotpEnrollment(
										await authRequest("/two-factor/enable", IdentityTOTPEnrollmentSchema, {}),
									);
								},
								false,
							)
						}
						type="button"
						variant="outline"
					>
						Set up TOTP
					</Button>
					<form
						className="flex gap-2"
						onSubmit={(event) => {
							event.preventDefault();
							const form = new FormData(event.currentTarget);
							void run("TOTP verification complete.", () =>
								authRequest("/two-factor/verify-totp", IdentityActionResultSchema, {
									code: field(form, "code"),
								}),
							);
						}}
					>
						<Input
							aria-label="TOTP code"
							inputMode="numeric"
							name="code"
							placeholder="123456"
							required
						/>
						<Button disabled={busy} type="submit">
							Verify TOTP
						</Button>
					</form>
				</div>
				{totpEnrollment ? (
					<Alert role="status">
						<AlertDescription className="text-xs">
							<p className="break-all font-mono">{totpEnrollment.totpURI}</p>
							<p className="mt-2">
								Backup codes (shown once): {totpEnrollment.backupCodes.join(" · ")}
							</p>
						</AlertDescription>
					</Alert>
				) : null}
			</SettingsCard>

			<SettingsCard
				title="Sessions"
				description="Review and revoke sessions without exposing their bearer tokens to browser code."
				visible={showSecurity}
			>
				<ul className="divide-y divide-primary/10 text-sm">
					{sessions.map((session) => (
						<li className="flex items-center gap-2 py-2" key={session.id}>
							<span className="min-w-0 flex-1 truncate">
								{session.current ? "Current · " : ""}
								{session.userAgent || "Unknown device"} · {session.updatedAt.toLocaleString()}
							</span>
							<Button
								className="text-destructive hover:text-destructive"
								disabled={busy}
								onClick={() =>
									void run("Session revoked.", () =>
										authRequest(
											"/api/auth/sessions",
											IdentityActionResultSchema,
											{ sessionId: session.id },
											"DELETE",
										),
									)
								}
								size="sm"
								type="button"
								variant="ghost"
							>
								Revoke
							</Button>
						</li>
					))}
				</ul>
			</SettingsCard>

			{isAdministrator ? (
				<>
					<SettingsCard
						title="Administrator mutation policy"
						description="Require a passkey or TOTP verification completed within the last 15 minutes before workspace, membership, SSO, SCIM, or lifecycle changes."
						visible={showEnterprise}
					>
						<form className="grid gap-4" onSubmit={(event) => void submitAdminPolicy(event)}>
							<div className="flex items-start gap-3 rounded-none border p-4">
								<Checkbox
									defaultChecked={organization?.requireAdminStepUp !== false}
									id="require-admin-step-up"
									name="requireAdminStepUp"
									value="on"
								/>
								<div className="grid gap-1">
									<Label htmlFor="require-admin-step-up">Require fresh administrator step-up</Label>
									<p className="text-xs text-muted-foreground">
										Disabling this policy removes the extra passkey or TOTP gate but does not change
										owner and administrator authorization checks.
									</p>
								</div>
							</div>
							<div className="flex justify-end">
								<Button disabled={busy || !organization} type="submit">
									Save administrator policy
								</Button>
							</div>
						</form>
					</SettingsCard>

					<SettingsCard
						title="OIDC and SAML SSO"
						description="Provider registration is restricted to owners and admins; domains must be proven through DNS before use."
						visible={showEnterprise}
					>
						<form className="grid gap-2 sm:grid-cols-2" onSubmit={(event) => void submitSso(event)}>
							<Select
								onValueChange={(value) => {
									setSsoType(value === "saml" ? "saml" : "oidc");
								}}
								value={ssoType}
							>
								<SelectTrigger aria-label="Provider type" className="w-full">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="oidc">OIDC</SelectItem>
									<SelectItem value="saml">SAML</SelectItem>
								</SelectContent>
							</Select>
							<Input name="providerId" placeholder="Provider ID" required />
							<Input name="domain" placeholder="example.com" required />
							<Input name="issuer" placeholder="Issuer URL or entity ID" required />
							{ssoType === "oidc" ? (
								<>
									<Input name="clientId" placeholder="Client ID" required />
									<Input name="clientSecret" placeholder="Client secret" required type="password" />
									<Input
										className="sm:col-span-2"
										name="discoveryEndpoint"
										placeholder="Discovery endpoint (optional)"
										type="url"
									/>
								</>
							) : (
								<>
									<Input name="entryPoint" placeholder="IdP entry point" required type="url" />
									<Textarea
										className="min-h-24 sm:col-span-2"
										name="certificate"
										placeholder="IdP signing certificate"
										required
									/>
								</>
							)}
							<Button disabled={busy} type="submit">
								Register provider
							</Button>
						</form>
						<ul className="divide-y divide-primary/10 text-sm">
							{ssoProviders.map((provider) => (
								<li className="flex flex-wrap items-center gap-2 py-2" key={provider.providerId}>
									<span className="flex-1">
										{provider.providerId} · {provider.type} · {provider.domain} ·{" "}
										{provider.domainVerified ? "verified" : "unverified"}
									</span>
									{!provider.domainVerified ? (
										<>
											<Button
												disabled={busy}
												onClick={() =>
													void run(
														"DNS verification value created. Copy it now.",
														async () => {
															const result = await authRequest(
																"/sso/request-domain-verification",
																IdentityDomainVerificationSchema,
																{ providerId: provider.providerId },
															);
															setDomainVerificationToken(result.domainVerificationToken);
														},
														false,
													)
												}
												size="sm"
												type="button"
												variant="outline"
											>
												Get DNS value
											</Button>
											<Button
												disabled={busy}
												onClick={() =>
													void run("Domain verified.", () =>
														authRequest("/sso/verify-domain", IdentityActionResultSchema, {
															providerId: provider.providerId,
														}),
													)
												}
												size="sm"
												type="button"
												variant="outline"
											>
												Verify DNS
											</Button>
										</>
									) : null}
									<Button
										className="text-destructive hover:text-destructive"
										disabled={busy}
										onClick={() =>
											void run("Provider removed.", () =>
												authRequest("/sso/delete-provider", IdentityActionResultSchema, {
													providerId: provider.providerId,
												}),
											)
										}
										size="sm"
										type="button"
										variant="ghost"
									>
										Remove
									</Button>
								</li>
							))}
						</ul>
						{domainVerificationToken ? (
							<Alert role="status">
								<AlertDescription className="break-all font-mono text-xs">
									{domainVerificationToken}
								</AlertDescription>
							</Alert>
						) : null}
					</SettingsCard>

					<SettingsCard
						title="SCIM directory provisioning"
						description="Tokens are shown once. Rotation issues a replacement, then revokes prior credentials."
						visible={showEnterprise}
					>
						<form
							className="flex gap-2"
							onSubmit={(event) => {
								event.preventDefault();
								const form = new FormData(event.currentTarget);
								void run("SCIM token created. Copy it now.", async () => {
									setScimToken(undefined);
									const result = await authRequest(
										"/api/auth/scim-admin",
										IdentitySCIMTokenSchema,
										{ action: "create", providerId: field(form, "providerId") },
									);
									setScimToken(result.scimToken);
								});
							}}
						>
							<Input name="providerId" placeholder="Directory name" required />
							<Button disabled={busy} type="submit">
								Create token
							</Button>
						</form>
						{scimToken ? (
							<Alert role="status">
								<AlertDescription className="break-all font-mono text-xs">
									{scimToken}
								</AlertDescription>
							</Alert>
						) : null}
						<ul className="divide-y divide-primary/10 text-sm">
							{scimProviders.map((provider) => (
								<li className="flex items-center gap-2 py-2" key={provider.id}>
									<span className="min-w-0 flex-1">
										<span className="block">{provider.providerId}</span>
										<code className="block truncate text-xs text-muted-foreground">
											{provider.connectionId}
										</code>
									</span>
									<Button
										disabled={busy}
										onClick={() =>
											void run("SCIM token rotated. Copy it now.", async () => {
												setScimToken(undefined);
												const result = await authRequest(
													"/api/auth/scim-admin",
													IdentitySCIMTokenSchema,
													{
														action: "rotate",
														connectionId: provider.connectionId,
													},
												);
												setScimToken(result.scimToken);
											})
										}
										size="sm"
										type="button"
										variant="outline"
									>
										Rotate
									</Button>
									<Button
										className="text-destructive hover:text-destructive"
										disabled={busy}
										onClick={() =>
											void run("SCIM connection revoked.", () =>
												authRequest("/api/auth/scim-admin", IdentityActionResultSchema, {
													action: "revoke",
													connectionId: provider.connectionId,
												}),
											)
										}
										size="sm"
										type="button"
										variant="ghost"
									>
										Revoke
									</Button>
								</li>
							))}
						</ul>
						<form
							className="grid gap-2 sm:grid-cols-[1fr_1fr_8rem_auto]"
							onSubmit={(event) => {
								event.preventDefault();
								const form = new FormData(event.currentTarget);
								void run("SCIM group role mapping saved for the next directory sync.", () =>
									authRequest("/api/auth/scim-admin", IdentityActionResultSchema, {
										action: "mapping",
										connectionId: field(form, "providerId"),
										group: field(form, "group"),
										role: field(form, "role"),
									}),
								);
							}}
						>
							<Input name="providerId" placeholder="Connection ID" required />
							<Input name="group" placeholder="Directory group ID" required />
							<Select defaultValue="member" name="role">
								<SelectTrigger aria-label="Mapped role" className="w-full">
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									<SelectItem value="member">Member</SelectItem>
									<SelectItem value="admin">Admin</SelectItem>
								</SelectContent>
							</Select>
							<Button disabled={busy} type="submit">
								Map group
							</Button>
						</form>
						<ul className="divide-y divide-primary/10 text-sm">
							{scimMappings.map((mapping) => (
								<li
									className="flex items-center gap-2 py-2"
									key={`${mapping.providerId}:${mapping.group}`}
								>
									<span className="flex-1">
										{mapping.providerId} · {mapping.group} → {mapping.role}
									</span>
									<Button
										className="text-destructive hover:text-destructive"
										disabled={busy}
										onClick={() =>
											void run("SCIM group role mapping removed.", () =>
												authRequest("/api/auth/scim-admin", IdentityActionResultSchema, {
													action: "mapping",
													connectionId: mapping.providerId,
													group: mapping.group,
													role: null,
												}),
											)
										}
										size="sm"
										type="button"
										variant="ghost"
									>
										Remove
									</Button>
								</li>
							))}
						</ul>
						<p className="text-xs text-muted-foreground">
							SCIM endpoint: <code>/api/auth/scim/v2</code>
						</p>
					</SettingsCard>

					<SettingsCard
						title="Organization policy and branding"
						description="Only validated colors and same-origin or HTTPS links are accepted. Arbitrary CSS is not supported."
						visible={showBranding}
					>
						<form
							className="grid gap-2 sm:grid-cols-2"
							key={organization?.id}
							onSubmit={(event) => void submitBranding(event)}
						>
							<Input
								defaultValue={organization?.brandName || ""}
								name="brandName"
								placeholder="Brand name"
							/>
							<Input
								defaultValue={organization?.supportEmail || ""}
								name="supportEmail"
								placeholder="Support email"
								type="email"
							/>
							{(
								[
									["brandLogoUrl", "Logo URL"],
									["brandWordmarkUrl", "Wordmark URL"],
									["brandFaviconUrl", "Favicon URL"],
									["documentationUrl", "Documentation URL"],
									["termsUrl", "Terms URL"],
									["privacyUrl", "Privacy URL"],
								] as const
							).map(([name, placeholder]) => (
								<Input
									defaultValue={organization?.[name] || ""}
									key={name}
									name={name}
									placeholder={placeholder}
								/>
							))}
							<Input
								defaultValue={organization?.brandPrimaryColor || ""}
								name="brandPrimaryColor"
								pattern="#[0-9A-Fa-f]{6}"
								placeholder="#112233"
							/>
							<Input
								defaultValue={organization?.brandAccentColor || ""}
								name="brandAccentColor"
								pattern="#[0-9A-Fa-f]{6}"
								placeholder="#445566"
							/>
							<Input
								defaultValue={organization?.maxSessionAgeSeconds || 2592000}
								min="900"
								name="maxSessionAgeSeconds"
								type="number"
							/>
							<Input
								defaultValue={organization?.idleTimeoutSeconds || ""}
								min="300"
								name="idleTimeoutSeconds"
								placeholder="Idle timeout seconds"
								type="number"
							/>
							<div className="flex items-center gap-2">
								<Checkbox
									defaultChecked={organization?.requireSso === true}
									id="require-sso"
									name="requireSso"
									value="on"
								/>
								<Label htmlFor="require-sso">Require SSO</Label>
							</div>
							<Button disabled={busy} type="submit">
								Save policy and brand
							</Button>
						</form>
					</SettingsCard>

					<SettingsCard
						title="Identity audit history"
						description="Append-only identity events; credentials, assertions, OTPs, and tokens are never recorded."
						visible={showEnterprise}
					>
						<Table>
							<TableHeader>
								<TableRow>
									<TableHead>Time</TableHead>
									<TableHead>Action</TableHead>
									<TableHead>Result</TableHead>
								</TableRow>
							</TableHeader>
							<TableBody>
								{auditEvents.map((event) => (
									<TableRow key={event.id}>
										<TableCell>
											<time>{event.createdAt.toLocaleString()}</time>
										</TableCell>
										<TableCell className="max-w-72 truncate">
											{event.action}
											{event.targetId ? ` · ${event.targetId}` : ""}
										</TableCell>
										<TableCell>{event.result}</TableCell>
									</TableRow>
								))}
							</TableBody>
						</Table>
						<div className="flex justify-end border-t pt-3">
							<Button asChild size="sm" variant="outline">
								<Link href="/app/audit-log">Open full audit log</Link>
							</Button>
						</div>
					</SettingsCard>
				</>
			) : null}
			{!isAdministrator && (showEnterprise || showBranding) ? (
				<Alert>
					<AlertDescription>
						An organization owner or administrator must manage these settings.
					</AlertDescription>
				</Alert>
			) : null}
		</section>
	);
}
