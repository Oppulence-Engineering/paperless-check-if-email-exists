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
import { useEntitlements } from "@/components/auth/auth-gate";
import { denial } from "@/lib/entitlements/entitlements";
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
import { Textarea } from "@oppulence/ui/components/textarea";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Pill, Row, Section } from "@/components/platform/page-kit";

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
	// The published plans include SCIM on Enterprise only.
	const scimDenial = denial(entitlements, "enterprise_identity");
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

	const inputClass = "h-10 rounded-lg md:w-72";
	const fieldLabel = "text-xs font-medium text-neutral-400";
	const listRow = "flex items-center gap-4 px-5 py-4";
	const removeButton = "text-red-400 hover:text-red-300";
	const noticeBox =
		"flex flex-col gap-3 rounded-2xl bg-emerald-500/5 p-5 ring-1 ring-emerald-500/20";
	const codeBlock =
		"overflow-x-auto rounded-lg bg-black/30 p-3 font-mono text-xs text-neutral-300 ring-1 ring-white/[0.08]";
	const confirmLabel = `Type ${organization?.slug ?? "the workspace slug"} to confirm`;
	return (
		<section
			className={cn("flex flex-col gap-10", className)}
			data-slot="identity-settings"
			{...props}
		>
			<Section description={heading.description} title={heading.title}>
				<div className="flex flex-col gap-10">
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

					{showWorkspace ? (
						<Section
							description="Switch the tenant context used for this server session and its backend JWT."
							title="Active workspace"
						>
							<Group>
								<Row label={<Label htmlFor="active-organization">Active organization</Label>}>
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
										<SelectTrigger
											className="h-10 w-full rounded-lg md:w-72"
											id="active-organization"
										>
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
								</Row>
							</Group>
						</Section>
					) : null}

					{showWorkspace ? (
						<Section
							description="The name and stable slug shown across tenant-scoped product surfaces."
							title="Workspace identity"
						>
							<form
								className="flex flex-col gap-4"
								key={`${organization?.id ?? organizationId}:${organization?.name ?? ""}:${organization?.slug ?? ""}:${organization?.logo ?? ""}`}
								onSubmit={(event) => void submitWorkspace(event)}
							>
								<Group>
									<Row label={<Label htmlFor="workspace-name">Workspace name</Label>}>
										<Input
											className={inputClass}
											defaultValue={organization?.name ?? ""}
											disabled={busy || !isAdministrator}
											id="workspace-name"
											name="name"
											required
										/>
									</Row>
									<Row label={<Label htmlFor="workspace-slug">Workspace slug</Label>}>
										<Input
											className={inputClass}
											defaultValue={organization?.slug ?? ""}
											disabled={busy || !isAdministrator}
											id="workspace-slug"
											name="slug"
											pattern="[a-z0-9]+(?:-[a-z0-9]+)*"
											required
										/>
									</Row>
									<Row label={<Label htmlFor="workspace-logo-url">Logo URL</Label>}>
										<Input
											className={inputClass}
											defaultValue={organization?.logo ?? ""}
											disabled={busy || !isAdministrator}
											id="workspace-logo-url"
											name="logo"
											placeholder="https://example.com/logo.png"
										/>
									</Row>
									<Row
										description="PNG, JPEG, or WebP. Maximum 256 KB. An upload replaces the URL above."
										label={<Label htmlFor="workspace-logo-file">Upload a logo</Label>}
									>
										<div className="flex items-center gap-3">
											<Avatar className="size-10 rounded-lg bg-white/[0.03] ring-1 ring-white/[0.08]">
												<AvatarImage alt="" src={organization?.logo ?? undefined} />
												<AvatarFallback className="rounded-lg text-xs">
													{(organization?.name || "W").slice(0, 2).toUpperCase()}
												</AvatarFallback>
											</Avatar>
											<Input
												accept="image/png,image/jpeg,image/webp"
												className="h-10 rounded-lg md:w-60"
												disabled={busy || !isAdministrator}
												id="workspace-logo-file"
												name="logoFile"
												type="file"
											/>
										</div>
									</Row>
									<Row label="Workspace ID" layout="inline">
										<span className="font-mono text-xs text-[#858585]">{organizationId}</span>
									</Row>
								</Group>
								{isAdministrator ? (
									<div className="flex justify-end">
										<Button className="rounded-lg" disabled={busy || !organization} type="submit">
											Save workspace
										</Button>
									</div>
								) : (
									<p className="text-sm text-[#858585]">
										An owner or administrator can edit these details.
									</p>
								)}
							</form>
						</Section>
					) : null}

					{showWorkspace && isOwner ? (
						<Section
							description="Archive suspends access without deleting identity records. Deletion permanently removes the workspace and its membership records."
							title="Workspace lifecycle"
						>
							<Group>
								<Row
									description={`Blocks sessions, backend requests, and workspace switching while retaining records. ${confirmLabel}.`}
									label="Archive workspace"
									layout="stacked"
								>
									<form
										className="flex w-full flex-col gap-3 sm:flex-row"
										onSubmit={(event) => void submitWorkspaceLifecycle(event, "archive_workspace")}
									>
										<Input
											aria-label={confirmLabel}
											autoComplete="off"
											className="h-10 rounded-lg sm:max-w-xs"
											id="archive-confirmation"
											name="confirmation"
											placeholder={organization?.slug}
											required
										/>
										<Button
											className="rounded-lg"
											disabled={busy || !organization}
											type="submit"
											variant="outline"
										>
											Archive workspace
										</Button>
									</form>
								</Row>
								<Row
									description={`Permanently deletes the workspace. This action cannot be undone. ${confirmLabel}.`}
									label="Delete workspace"
									layout="stacked"
								>
									<form
										className="flex w-full flex-col gap-3 sm:flex-row"
										onSubmit={(event) => void submitWorkspaceLifecycle(event, "delete_workspace")}
									>
										<Input
											aria-label={confirmLabel}
											autoComplete="off"
											className="h-10 rounded-lg sm:max-w-xs"
											id="delete-workspace-confirmation"
											name="confirmation"
											placeholder={organization?.slug}
											required
										/>
										<Button
											className="rounded-lg"
											disabled={busy || !organization}
											type="submit"
											variant="destructive"
										>
											Delete workspace
										</Button>
									</form>
								</Row>
							</Group>
						</Section>
					) : null}

					{showOrganization ? (
						<Section
							description="Better Auth owns organization membership and the owner, admin, and member roles."
							title={`Members${organization ? ` (${String(organization.members.length)})` : ""}`}
						>
							{isAdministrator ? (
								<form onSubmit={(event) => void submitInvitation(event)}>
									<Group>
										<Row>
											<div>
												<h3 className="text-sm font-medium text-white">Invite a member</h3>
												<p className="mt-0.5 text-xs text-[#858585]">
													Send an email invitation and choose the role they join with.
												</p>
											</div>
											<div className="grid gap-3 sm:grid-cols-[1fr_9rem_auto]">
												<Input
													aria-label="Invite email"
													className="h-10 rounded-lg"
													name="email"
													placeholder="person@example.com"
													required
													type="email"
												/>
												<Select defaultValue="member" name="role">
													<SelectTrigger
														aria-label="Invite role"
														className="h-10 w-full rounded-lg"
													>
														<SelectValue />
													</SelectTrigger>
													<SelectContent>
														<SelectItem value="member">Member</SelectItem>
														<SelectItem value="admin">Admin</SelectItem>
													</SelectContent>
												</Select>
												<Button
													className="h-10 rounded-lg"
													disabled={busy || seatDenial !== null}
													type="submit"
												>
													Invite
												</Button>
											</div>
										</Row>
									</Group>
								</form>
							) : null}
							{isAdministrator && seatDenial ? (
								<p className="text-sm text-[#858585]" role="status">
									{seatDenial.message}
								</p>
							) : null}
							{organization?.members.length ? (
								<Group>
									<ul className="flex flex-col divide-y divide-white/[0.07]">
										{organization.members.map((member) => (
											<li
												className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
												key={member.id}
											>
												<div
													aria-hidden="true"
													className="flex size-8 shrink-0 items-center justify-center rounded-full bg-white/[0.06] text-xs font-medium text-neutral-300 uppercase ring-1 ring-white/10"
												>
													{memberInitials(member.user.name, member.user.email)}
												</div>
												<div className="min-w-0 flex-1">
													<p className="truncate text-sm text-white">
														{member.user.name || member.user.email}
													</p>
													{member.user.name && member.user.name !== member.user.email ? (
														<p className="truncate text-xs text-[#858585]">{member.user.email}</p>
													) : null}
												</div>
												<div className="flex shrink-0 items-center gap-2">
													{isAdministrator && !member.role.split(",").includes("owner") ? (
														<Select
															disabled={busy || member.userId === userId}
															onValueChange={(value) =>
																void run("Member role updated.", () =>
																	authRequest(
																		"/api/auth/workspace-admin",
																		IdentityActionResultSchema,
																		{
																			action: "change_member_role",
																			memberId: member.id,
																			role: value,
																		},
																	),
																)
															}
															value={member.role}
														>
															<SelectTrigger
																aria-label={`Role for ${member.user.email}`}
																className="w-32 rounded-lg"
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
														<Pill
															tone={member.role.split(",").includes("owner") ? "purple" : "gray"}
														>
															<span className="capitalize">{member.role}</span>
														</Pill>
													)}
													{isAdministrator &&
													member.userId !== userId &&
													!member.role.split(",").includes("owner") ? (
														<Button
															className={removeButton}
															disabled={busy}
															onClick={() =>
																void run("Member removed.", () =>
																	authRequest(
																		"/api/auth/workspace-admin",
																		IdentityActionResultSchema,
																		{
																			action: "remove_member",
																			memberId: member.id,
																		},
																	),
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
								</Group>
							) : null}
						</Section>
					) : null}

					{showOrganization ? (
						<Section
							title={`Pending invitations${
								organization
									? ` (${String(
											organization.invitations.filter((item) => item.status === "pending").length,
										)})`
									: ""
							}`}
						>
							<Group>
								{organization?.invitations.some((item) => item.status === "pending") ? (
									<ul className="flex flex-col divide-y divide-white/[0.07]">
										{organization.invitations
											.filter((item) => item.status === "pending")
											.map((item) => (
												<li
													className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
													key={item.id}
												>
													<div className="min-w-0 flex-1">
														<p className="truncate text-sm text-white">{item.email}</p>
														<p className="text-xs text-[#858585]">
															<span className="capitalize">{item.role ?? "member"}</span> · expires{" "}
															{item.expiresAt.toLocaleDateString()}
														</p>
													</div>
													<div className="flex items-center gap-2">
														<Pill tone="yellow">Pending</Pill>
														{isAdministrator ? (
															<>
																<Button
																	disabled={busy}
																	onClick={() =>
																		void run("Invitation resent.", () =>
																			authRequest(
																				"/api/auth/workspace-admin",
																				IdentityActionResultSchema,
																				{
																					action: "resend_invitation",
																					invitationId: item.id,
																				},
																			),
																		)
																	}
																	size="sm"
																	type="button"
																	variant="secondary"
																>
																	Resend
																</Button>
																<Button
																	className={removeButton}
																	disabled={busy}
																	onClick={() =>
																		void run("Invitation revoked.", () =>
																			authRequest(
																				"/api/auth/workspace-admin",
																				IdentityActionResultSchema,
																				{
																					action: "revoke_invitation",
																					invitationId: item.id,
																				},
																			),
																		)
																	}
																	size="sm"
																	type="button"
																	variant="ghost"
																>
																	Revoke
																</Button>
															</>
														) : null}
													</div>
												</li>
											))}
									</ul>
								) : (
									<Row>
										<p className="text-sm text-[#858585]">No invitations are waiting.</p>
									</Row>
								)}
							</Group>
						</Section>
					) : null}

					{showOrganization && isOwner ? (
						<Section
							description="Promote another member to owner and move your account to the administrator role. This is atomic and requires recent identity verification."
							title="Transfer ownership"
						>
							{organization?.members.some(
								(member) => member.userId !== userId && !member.role.split(",").includes("owner"),
							) ? (
								<form
									className="flex flex-col gap-4"
									onSubmit={(event) => void submitOwnershipTransfer(event)}
								>
									<Group>
										<Row label={<Label htmlFor="ownership-member">New owner</Label>}>
											<Select name="memberId" required>
												<SelectTrigger
													className="h-10 w-full rounded-lg md:w-72"
													id="ownership-member"
												>
													<SelectValue placeholder="Select a member" />
												</SelectTrigger>
												<SelectContent>
													{organization.members
														.filter(
															(member) =>
																member.userId !== userId &&
																!member.role.split(",").includes("owner"),
														)
														.map((member) => (
															<SelectItem key={member.id} value={member.id}>
																{member.user.name} · {member.user.email}
															</SelectItem>
														))}
												</SelectContent>
											</Select>
										</Row>
										<Row
											label={
												<Label htmlFor="ownership-confirmation">
													Type {organization.slug} to confirm
												</Label>
											}
										>
											<Input
												autoComplete="off"
												className={inputClass}
												id="ownership-confirmation"
												name="confirmation"
												required
											/>
										</Row>
									</Group>
									<div className="flex justify-end">
										<Button
											className="rounded-lg"
											disabled={busy}
											type="submit"
											variant="destructive"
										>
											Transfer ownership
										</Button>
									</div>
								</form>
							) : (
								<Group>
									<Row>
										<p className="text-sm text-[#858585]">
											Invite another member before transferring ownership.
										</p>
									</Row>
								</Group>
							)}
						</Section>
					) : null}

					{showSecurity ? (
						<Section
							description="A successful passkey or TOTP check authorizes protected identity changes for 15 minutes."
							title="Passkeys and administrator step-up"
						>
							<Group>
								<Row label="Passkeys">
									<Button
										className="rounded-lg"
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
										className="rounded-lg"
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
								</Row>
								{passkeys.length ? (
									<ul className="flex flex-col divide-y divide-white/[0.07]">
										{passkeys.map((passkey) => (
											<li className={listRow} key={passkey.id}>
												<div className="min-w-0 flex-1">
													<p className="truncate text-sm text-white">{passkey.name || "Passkey"}</p>
													<p className="truncate text-xs text-[#858585]">
														{passkey.deviceType} · {passkey.createdAt.toLocaleDateString()}
													</p>
												</div>
												<Button
													className={removeButton}
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
								) : null}
								<Row label="Authenticator app">
									<Button
										className="rounded-lg"
										disabled={busy}
										onClick={() =>
											void run(
												"Scan the TOTP URI, then verify a code.",
												async () => {
													setTotpEnrollment(
														await authRequest(
															"/two-factor/enable",
															IdentityTOTPEnrollmentSchema,
															{},
														),
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
								</Row>
								{totpEnrollment ? (
									<>
										<Row label="TOTP URI" layout="stacked">
											<code className={cn("w-full break-all", codeBlock)}>
												{totpEnrollment.totpURI}
											</code>
										</Row>
										<Row label="Backup codes (shown once)" layout="stacked">
											<div className="grid w-full grid-cols-2 gap-2 rounded-lg bg-black/30 p-3 font-mono text-xs text-neutral-300 ring-1 ring-white/[0.08]">
												{totpEnrollment.backupCodes.map((backupCode) => (
													<code key={backupCode}>{backupCode}</code>
												))}
											</div>
										</Row>
									</>
								) : null}
								<Row label={<Label htmlFor="totp-code">TOTP code</Label>}>
									<form
										className="flex w-full gap-3 md:w-auto"
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
											className="h-10 flex-1 rounded-lg md:w-40 md:flex-none"
											id="totp-code"
											inputMode="numeric"
											name="code"
											placeholder="123456"
											required
										/>
										<Button className="h-10 rounded-lg" disabled={busy} type="submit">
											Verify TOTP
										</Button>
									</form>
								</Row>
							</Group>
						</Section>
					) : null}

					{showSecurity ? (
						<Section
							description="Review and revoke sessions without exposing their bearer tokens to browser code."
							title="Sessions"
						>
							{sessions.length ? (
								<Group>
									<ul className="flex flex-col divide-y divide-white/[0.07]">
										{sessions.map((session) => (
											<li className={listRow} key={session.id}>
												<div className="min-w-0 flex-1">
													<p className="truncate text-sm text-white">
														{session.userAgent || "Unknown device"}
													</p>
													<p className="truncate text-xs text-[#858585]">
														{session.updatedAt.toLocaleString()}
													</p>
												</div>
												{session.current ? <Pill tone="green">Current</Pill> : null}
												<Button
													className={removeButton}
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
								</Group>
							) : null}
						</Section>
					) : null}

					{isAdministrator ? (
						<>
							{showEnterprise ? (
								<Section
									description="Require a passkey or TOTP verification completed within the last 15 minutes before workspace, membership, SSO, SCIM, or lifecycle changes."
									title="Administrator mutation policy"
								>
									<form
										className="flex flex-col gap-4"
										onSubmit={(event) => void submitAdminPolicy(event)}
									>
										<Group>
											<Row
												description="Disabling this policy removes the extra passkey or TOTP gate but does not change owner and administrator authorization checks."
												label={
													<Label htmlFor="require-admin-step-up">
														Require fresh administrator step-up
													</Label>
												}
												layout="inline"
											>
												<Checkbox
													defaultChecked={organization?.requireAdminStepUp !== false}
													id="require-admin-step-up"
													name="requireAdminStepUp"
													value="on"
												/>
											</Row>
										</Group>
										<div className="flex justify-end">
											<Button className="rounded-lg" disabled={busy || !organization} type="submit">
												Save administrator policy
											</Button>
										</div>
									</form>
								</Section>
							) : null}

							{showEnterprise ? (
								<Section
									description="Provider registration is restricted to owners and admins; domains must be proven through DNS before use."
									title="OIDC and SAML SSO"
								>
									{domainVerificationToken ? (
										<div className={noticeBox} role="status">
											<code className={cn("break-all", codeBlock)}>{domainVerificationToken}</code>
										</div>
									) : null}
									<Group>
										{ssoProviders.length ? (
											<ul className="flex flex-col divide-y divide-white/[0.07]">
												{ssoProviders.map((provider) => (
													<li
														className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
														key={provider.providerId}
													>
														<div className="min-w-0 flex-1">
															<p className="truncate font-mono text-sm text-white">
																{provider.providerId}
															</p>
															<p className="truncate text-xs text-[#858585]">
																{provider.type} · {provider.domain}
															</p>
														</div>
														<Pill tone={provider.domainVerified ? "green" : "yellow"}>
															{provider.domainVerified ? "verified" : "unverified"}
														</Pill>
														<div className="flex items-center gap-2">
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
																					setDomainVerificationToken(
																						result.domainVerificationToken,
																					);
																				},
																				false,
																			)
																		}
																		size="sm"
																		type="button"
																		variant="secondary"
																	>
																		Get DNS value
																	</Button>
																	<Button
																		disabled={busy}
																		onClick={() =>
																			void run("Domain verified.", () =>
																				authRequest(
																					"/sso/verify-domain",
																					IdentityActionResultSchema,
																					{
																						providerId: provider.providerId,
																					},
																				),
																			)
																		}
																		size="sm"
																		type="button"
																		variant="secondary"
																	>
																		Verify DNS
																	</Button>
																</>
															) : null}
															<Button
																className={removeButton}
																disabled={busy}
																onClick={() =>
																	void run("Provider removed.", () =>
																		authRequest(
																			"/sso/delete-provider",
																			IdentityActionResultSchema,
																			{
																				providerId: provider.providerId,
																			},
																		),
																	)
																}
																size="sm"
																type="button"
																variant="ghost"
															>
																Remove
															</Button>
														</div>
													</li>
												))}
											</ul>
										) : null}
										<Row>
											<form
												className="flex flex-col gap-4"
												onSubmit={(event) => void submitSso(event)}
											>
												<div className="grid gap-3 sm:grid-cols-2">
													<div className="flex flex-col gap-1.5">
														<Label className={fieldLabel} htmlFor="sso-provider-type">
															Provider type
														</Label>
														<Select
															onValueChange={(value) => {
																setSsoType(value === "saml" ? "saml" : "oidc");
															}}
															value={ssoType}
														>
															<SelectTrigger
																className="h-10 w-full rounded-lg"
																id="sso-provider-type"
															>
																<SelectValue />
															</SelectTrigger>
															<SelectContent>
																<SelectItem value="oidc">OIDC</SelectItem>
																<SelectItem value="saml">SAML</SelectItem>
															</SelectContent>
														</Select>
													</div>
													{(
														[
															["providerId", "Provider ID", "Provider ID"],
															["domain", "Domain", "example.com"],
															["issuer", "Issuer", "Issuer URL or entity ID"],
														] as const
													).map(([name, label, placeholder]) => (
														<div className="flex flex-col gap-1.5" key={name}>
															<Label className={fieldLabel} htmlFor={`sso-${name}`}>
																{label}
															</Label>
															<Input
																className="h-10 rounded-lg"
																id={`sso-${name}`}
																name={name}
																placeholder={placeholder}
																required
															/>
														</div>
													))}
													{ssoType === "oidc" ? (
														<>
															<div className="flex flex-col gap-1.5">
																<Label className={fieldLabel} htmlFor="sso-client-id">
																	Client ID
																</Label>
																<Input
																	className="h-10 rounded-lg"
																	id="sso-client-id"
																	name="clientId"
																	placeholder="Client ID"
																	required
																/>
															</div>
															<div className="flex flex-col gap-1.5">
																<Label className={fieldLabel} htmlFor="sso-client-secret">
																	Client secret
																</Label>
																<Input
																	className="h-10 rounded-lg"
																	id="sso-client-secret"
																	name="clientSecret"
																	placeholder="Client secret"
																	required
																	type="password"
																/>
															</div>
															<div className="flex flex-col gap-1.5 sm:col-span-2">
																<Label className={fieldLabel} htmlFor="sso-discovery-endpoint">
																	Discovery endpoint (optional)
																</Label>
																<Input
																	className="h-10 rounded-lg"
																	id="sso-discovery-endpoint"
																	name="discoveryEndpoint"
																	placeholder="Discovery endpoint (optional)"
																	type="url"
																/>
															</div>
														</>
													) : (
														<>
															<div className="flex flex-col gap-1.5 sm:col-span-2">
																<Label className={fieldLabel} htmlFor="sso-entry-point">
																	IdP entry point
																</Label>
																<Input
																	className="h-10 rounded-lg"
																	id="sso-entry-point"
																	name="entryPoint"
																	placeholder="IdP entry point"
																	required
																	type="url"
																/>
															</div>
															<div className="flex flex-col gap-1.5 sm:col-span-2">
																<Label className={fieldLabel} htmlFor="sso-certificate">
																	IdP signing certificate
																</Label>
																<Textarea
																	className="min-h-24 rounded-lg"
																	id="sso-certificate"
																	name="certificate"
																	placeholder="IdP signing certificate"
																	required
																/>
															</div>
														</>
													)}
												</div>
												<div className="flex justify-end">
													<Button className="rounded-lg" disabled={busy} type="submit">
														Register provider
													</Button>
												</div>
											</form>
										</Row>
									</Group>
								</Section>
							) : null}

							{showEnterprise ? (
								<Section
									description="Tokens are shown once. Rotation issues a replacement, then revokes prior credentials."
									title="SCIM directory provisioning"
								>
									{scimToken ? (
										<div className={noticeBox} role="status">
											<code className={cn("break-all", codeBlock)}>{scimToken}</code>
										</div>
									) : null}
									<Group>
										{scimProviders.length ? (
											<ul className="flex flex-col divide-y divide-white/[0.07]">
												{scimProviders.map((provider) => (
													<li className={listRow} key={provider.id}>
														<div className="min-w-0 flex-1">
															<p className="truncate text-sm text-white">{provider.providerId}</p>
															<code className="block truncate font-mono text-xs text-[#858585]">
																{provider.connectionId}
															</code>
														</div>
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
															variant="secondary"
														>
															Rotate
														</Button>
														<Button
															className={removeButton}
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
										) : null}
										{scimDenial ? (
											<Row>
												<p className="text-sm text-[#858585]">
													SCIM provisioning comes with the Enterprise plan.{" "}
													<Link
														className="text-neutral-300 underline underline-offset-2 hover:text-white"
														href="/contact"
													>
														Contact us
													</Link>
												</p>
											</Row>
										) : (
											<Row>
												<form
													className="flex flex-col gap-3"
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
													<Label className={fieldLabel} htmlFor="scim-provider-id">
														Directory name
													</Label>
													<div className="flex gap-3">
														<Input
															className="h-10 flex-1 rounded-lg"
															id="scim-provider-id"
															name="providerId"
															placeholder="Directory name"
															required
														/>
														<Button className="h-10 rounded-lg" disabled={busy} type="submit">
															Create token
														</Button>
													</div>
												</form>
											</Row>
										)}
										{scimMappings.length ? (
											<ul className="flex flex-col divide-y divide-white/[0.07]">
												{scimMappings.map((mapping) => (
													<li className={listRow} key={`${mapping.providerId}:${mapping.group}`}>
														<span className="min-w-0 flex-1 truncate text-sm text-white">
															{mapping.providerId} · {mapping.group} → {mapping.role}
														</span>
														<Button
															className={removeButton}
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
										) : null}
										<Row>
											<form
												className="grid gap-3 sm:grid-cols-[1fr_1fr_8rem_auto]"
												onSubmit={(event) => {
													event.preventDefault();
													const form = new FormData(event.currentTarget);
													void run(
														"SCIM group role mapping saved for the next directory sync.",
														() =>
															authRequest("/api/auth/scim-admin", IdentityActionResultSchema, {
																action: "mapping",
																connectionId: field(form, "providerId"),
																group: field(form, "group"),
																role: field(form, "role"),
															}),
													);
												}}
											>
												<Input
													className="h-10 rounded-lg"
													name="providerId"
													placeholder="Connection ID"
													required
												/>
												<Input
													className="h-10 rounded-lg"
													name="group"
													placeholder="Directory group ID"
													required
												/>
												<Select defaultValue="member" name="role">
													<SelectTrigger
														aria-label="Mapped role"
														className="h-10 w-full rounded-lg"
													>
														<SelectValue />
													</SelectTrigger>
													<SelectContent>
														<SelectItem value="member">Member</SelectItem>
														<SelectItem value="admin">Admin</SelectItem>
													</SelectContent>
												</Select>
												<Button className="h-10 rounded-lg" disabled={busy} type="submit">
													Map group
												</Button>
											</form>
										</Row>
										<Row label="SCIM endpoint" layout="inline">
											<code className="font-mono text-xs text-neutral-300">/api/auth/scim/v2</code>
										</Row>
									</Group>
								</Section>
							) : null}

							{showBranding ? (
								<Section
									description="Only validated colors and same-origin or HTTPS links are accepted. Arbitrary CSS is not supported."
									title="Organization policy and branding"
								>
									<form
										className="flex flex-col gap-4"
										key={organization?.id}
										onSubmit={(event) => void submitBranding(event)}
									>
										<Group>
											<Row label={<Label htmlFor="brand-name">Brand name</Label>}>
												<Input
													className={inputClass}
													defaultValue={organization?.brandName || ""}
													id="brand-name"
													name="brandName"
													placeholder="Brand name"
												/>
											</Row>
											<Row label={<Label htmlFor="support-email">Support email</Label>}>
												<Input
													className={inputClass}
													defaultValue={organization?.supportEmail || ""}
													id="support-email"
													name="supportEmail"
													placeholder="Support email"
													type="email"
												/>
											</Row>
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
												<Row key={name} label={<Label htmlFor={name}>{placeholder}</Label>}>
													<Input
														className={inputClass}
														defaultValue={organization?.[name] || ""}
														id={name}
														name={name}
														placeholder={placeholder}
													/>
												</Row>
											))}
											<Row label={<Label htmlFor="brand-primary-color">Primary color</Label>}>
												<Input
													className={inputClass}
													defaultValue={organization?.brandPrimaryColor || ""}
													id="brand-primary-color"
													name="brandPrimaryColor"
													pattern="#[0-9A-Fa-f]{6}"
													placeholder="#112233"
												/>
											</Row>
											<Row label={<Label htmlFor="brand-accent-color">Accent color</Label>}>
												<Input
													className={inputClass}
													defaultValue={organization?.brandAccentColor || ""}
													id="brand-accent-color"
													name="brandAccentColor"
													pattern="#[0-9A-Fa-f]{6}"
													placeholder="#445566"
												/>
											</Row>
											<Row
												label={
													<Label htmlFor="max-session-age">Maximum session age (seconds)</Label>
												}
											>
												<Input
													className={inputClass}
													defaultValue={organization?.maxSessionAgeSeconds || 2592000}
													id="max-session-age"
													min="900"
													name="maxSessionAgeSeconds"
													type="number"
												/>
											</Row>
											<Row label={<Label htmlFor="idle-timeout">Idle timeout seconds</Label>}>
												<Input
													className={inputClass}
													defaultValue={organization?.idleTimeoutSeconds || ""}
													id="idle-timeout"
													min="300"
													name="idleTimeoutSeconds"
													placeholder="Idle timeout seconds"
													type="number"
												/>
											</Row>
											<Row label={<Label htmlFor="require-sso">Require SSO</Label>} layout="inline">
												<Checkbox
													defaultChecked={organization?.requireSso === true}
													id="require-sso"
													name="requireSso"
													value="on"
												/>
											</Row>
										</Group>
										<div className="flex justify-end">
											<Button className="rounded-lg" disabled={busy} type="submit">
												Save policy and brand
											</Button>
										</div>
									</form>
								</Section>
							) : null}

							{showEnterprise ? (
								<Section
									actions={
										<Button asChild className="rounded-lg" size="sm" variant="outline">
											<Link href="/app/audit-log">Open full audit log</Link>
										</Button>
									}
									description="Append-only identity events; credentials, assertions, OTPs, and tokens are never recorded."
									title="Identity audit history"
								>
									<Group>
										{auditEvents.length ? (
											<ul className="flex flex-col divide-y divide-white/[0.07]">
												{auditEvents.map((event) => (
													<li className={listRow} key={event.id}>
														<div className="min-w-0 flex-1">
															<p className="truncate font-mono text-sm text-white">
																{event.action}
																{event.targetId ? ` · ${event.targetId}` : ""}
															</p>
															<p className="truncate text-xs text-[#858585]">
																<time>{event.createdAt.toLocaleString()}</time>
															</p>
														</div>
														<Pill tone={event.result === "success" ? "green" : "red"}>
															{event.result}
														</Pill>
													</li>
												))}
											</ul>
										) : (
											<Row>
												<p className="text-sm text-[#858585]">
													No identity events have been recorded.
												</p>
											</Row>
										)}
									</Group>
								</Section>
							) : null}
						</>
					) : null}
					{!isAdministrator && (showEnterprise || showBranding) ? (
						<Alert>
							<AlertDescription>
								An organization owner or administrator must manage these settings.
							</AlertDescription>
						</Alert>
					) : null}
				</div>
			</Section>
		</section>
	);
}
