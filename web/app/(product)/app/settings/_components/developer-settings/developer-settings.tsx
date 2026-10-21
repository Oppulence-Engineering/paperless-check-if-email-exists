"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import { useEntitlements } from "@/components/auth/auth-gate";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { denial } from "@/lib/entitlements/entitlements";

import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { AccountApi, Configuration } from "@oppulence/reacher-sdk";
import { z } from "zod";

import { Alert, AlertDescription } from "@oppulence/ui/components/alert";
import { Button } from "@oppulence/ui/components/button";
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
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Pill, Row, Section } from "@/components/platform/page-kit";
import {
	CreateTenantApiKey201Response,
	GetTenantApiKey200Response,
	ListTenantApiKeys200Response,
	RevokeTenantApiKey200Response,
	UpdateTenantApiKey200Response,
} from "@/lib/api/generated/zod/account/account";

import {
	ApiKeyListSchema,
	ApiKeySchema,
	ApiKeyScopeSchema,
	CreateApiKeySchema,
	CreatedApiKeySchema,
	type DeveloperSettingsPropsFields,
} from "./developer-settings.schema";

/**
 * @oppulence-gen kind=component
 * Tenant API key management through the session-protected backend proxy.
 * Owned by `developer-settings.lit.ts`.
 */
export type DeveloperSettingsProps = DeveloperSettingsPropsFields &
	ComponentPropsWithoutRef<"section">;

const sdk = new AccountApi(new Configuration({ basePath: "/api/backend" }));
const scopeOptions = [
	["verify", "Single checks"],
	["bulk", "Bulk checks"],
	["find", "Email finder"],
	["lists", "Lists"],
	["suppressions", "Suppressions"],
	["reputation", "Reputation"],
	["settings", "Settings"],
	["pipelines.read", "Read pipelines"],
	["pipelines.write", "Edit pipelines"],
	["pipelines.trigger", "Trigger pipelines"],
	["outcomes.read", "Read campaign results"],
	["outcomes.write", "Send campaign results"],
	// "admin" stays out of the picker: a key with it can create more keys.
] as const;

export function DeveloperSettings({
	className,
	organizationId,
	organizationRole,
	...props
}: DeveloperSettingsProps) {
	const queryClient = useQueryClient();
	const canManage = organizationRole
		.split(",")
		.some((role) => ["owner", "admin"].includes(role.trim()));
	const queryKey = ["tenant-api-keys", organizationId];
	const [name, setName] = useState("");
	const [scopes, setScopes] = useState<z.infer<typeof ApiKeyScopeSchema>[]>(["verify"]);
	const [lifetimeDays, setLifetimeDays] = useState(30);
	const [revealedKey, setRevealedKey] = useState<string | null>(null);
	const [copied, setCopied] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [notice, setNotice] = useState<string | null>(null);
	const [selectedKeyId, setSelectedKeyId] = useState<string | null>(null);
	const [editName, setEditName] = useState("");
	const [editScopes, setEditScopes] = useState<z.infer<typeof ApiKeyScopeSchema>[]>([]);

	const keys = useQuery({
		queryKey,
		enabled: canManage,
		queryFn: async ({ signal }) =>
			ApiKeyListSchema.parse(
				ListTenantApiKeys200Response.parse((await sdk.listTenantApiKeys({ signal })).data),
			),
	});
	// The published plans include the API on Professional and Enterprise only.
	const apiDenial = denial(useEntitlements(), "api_access");
	const create = useMutation({
		mutationFn: async (body: z.infer<typeof CreateApiKeySchema>) =>
			CreatedApiKeySchema.parse(
				CreateTenantApiKey201Response.parse(
					(await sdk.createTenantApiKey({ createApiKeyRequest: body })).data,
				),
			),
		onSuccess: async () => {
			await queryClient.invalidateQueries({ queryKey });
		},
	});
	const selectedKey = useQuery({
		queryKey: workflowQueryKey(...queryKey, selectedKeyId),
		enabled: canManage && Boolean(selectedKeyId),
		queryFn: async () => {
			if (!selectedKeyId) throw new Error("Select a key first.");
			return ApiKeySchema.parse(
				GetTenantApiKey200Response.parse(
					(await sdk.getTenantApiKey({ keyId: selectedKeyId })).data,
				),
			);
		},
	});
	const update = useMutation({
		mutationFn: async () => {
			if (!selectedKeyId) throw new Error("Select a key first.");
			if (!editName.trim() || !editScopes.length)
				throw new Error("Enter a name and choose at least one scope.");
			return ApiKeySchema.parse(
				UpdateTenantApiKey200Response.parse(
					(
						await sdk.updateTenantApiKey({
							keyId: selectedKeyId,
							updateApiKeyRequest: { name: editName.trim(), scopes: editScopes },
						})
					).data,
				),
			);
		},
		onSuccess: async () => {
			setNotice("API key updated.");
			await queryClient.invalidateQueries({ queryKey });
		},
	});
	const revoke = useMutation({
		mutationFn: async (keyId: string) => {
			RevokeTenantApiKey200Response.parse((await sdk.revokeTenantApiKey({ keyId })).data);
		},
		onSuccess: async () => {
			await queryClient.invalidateQueries({ queryKey });
		},
	});

	async function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		setError(null);
		setNotice(null);
		const body = CreateApiKeySchema.safeParse({
			name,
			scopes,
			expires_at: new Date(Date.now() + lifetimeDays * 86_400_000).toISOString(),
		});
		if (!body.success) {
			setError("Enter a name and select at least one scope.");
			return;
		}
		try {
			const result = await create.mutateAsync(body.data);
			setRevealedKey(result.key);
			setCopied(false);
			setName("");
			setScopes(["verify"]);
			setNotice("API key created. Copy it now; it will not be shown again.");
		} catch {
			// The form shows the error next to the create button.
		}
	}

	async function revokeKey(keyId: string, keyName: string) {
		if (!window.confirm(`Revoke ${keyName}? Requests using this key will stop working.`)) return;
		setError(null);
		setNotice(null);
		try {
			await revoke.mutateAsync(keyId);
			setNotice(`${keyName} revoked.`);
		} catch (cause) {
			setError(errorMessage(cause));
		}
	}

	const fieldLabel = "text-xs font-medium text-neutral-400";
	return (
		<section
			className={cn("flex flex-col gap-10", className)}
			data-slot="developer-settings"
			{...props}
		>
			<Section
				description="Create scoped credentials for the backend API and review their status and recent use."
				title="API keys"
			>
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
					<Section
						description={
							<>
								Call <code>/v1/*</code> on this app&apos;s host with the key as a Bearer token. Keys
								belong to this workspace.
							</>
						}
						title="API access"
					>
						<Group>
							<Row
								description="Browse backend endpoints and operation details in the OpenAPI schema."
								label="API reference"
								layout="inline"
							>
								<Link
									className="text-sm text-neutral-300 underline underline-offset-4 hover:text-white"
									href="/api/backend/openapi.json"
									rel="noopener noreferrer"
									target="_blank"
								>
									Open schema
								</Link>
							</Row>
						</Group>
					</Section>
					{!canManage ? (
						<Alert>
							<AlertDescription>
								Only workspace owners and administrators can manage API keys.
							</AlertDescription>
						</Alert>
					) : (
						<>
							{revealedKey ? (
								<div
									className="flex flex-col gap-3 rounded-2xl bg-emerald-500/5 p-5 ring-1 ring-emerald-500/20"
									role="status"
								>
									<Label className="text-sm font-medium text-white" htmlFor="new-api-key">
										New API key — copy it now
									</Label>
									<Input
										className="h-9 rounded-lg font-mono text-xs"
										id="new-api-key"
										readOnly
										value={revealedKey}
									/>
									<div className="flex gap-2">
										<Button
											className="rounded-lg"
											onClick={() =>
												void navigator.clipboard.writeText(revealedKey).then(
													() => {
														setCopied(true);
													},
													() => {
														setError("Could not copy the key.");
													},
												)
											}
											size="sm"
											type="button"
										>
											{copied ? "Copied" : "Copy key"}
										</Button>
										<Button
											onClick={() => {
												setRevealedKey(null);
											}}
											size="sm"
											type="button"
											variant="ghost"
										>
											Done
										</Button>
									</div>
								</div>
							) : null}
							<Section
								description="Only prefixes are visible after creation."
								title="Workspace keys"
							>
								<Group>
									{keys.isPending ? (
										<Row>
											<p className="text-sm text-[#858585]">Loading API keys…</p>
										</Row>
									) : null}
									{keys.isError ? (
										<Row
											label={<span role="alert">{errorMessage(keys.error)}</span>}
											layout="inline"
										>
											<Button
												className="rounded-lg"
												onClick={() => void keys.refetch()}
												size="sm"
												type="button"
												variant="outline"
											>
												Retry
											</Button>
										</Row>
									) : null}
									{keys.data?.api_keys.length === 0 ? (
										<Row>
											<p className="text-sm text-[#858585]">No API keys yet.</p>
										</Row>
									) : null}
									{keys.data?.api_keys.map((key) => (
										<div
											className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
											key={key.id}
										>
											<div className="min-w-0 flex-1">
												<p className="truncate text-sm text-white">{key.name}</p>
												<p className="truncate text-xs text-[#858585]">
													<code className="font-mono">{key.key_prefix}…</code> ·{" "}
													{key.scopes.length ? key.scopes.join(", ") : "Full access"}
												</p>
												<p className="truncate text-xs text-[#858585]">
													Created {new Date(key.created_at).toLocaleDateString()} · Last used{" "}
													{key.last_used_at
														? new Date(key.last_used_at).toLocaleDateString()
														: "never"}{" "}
													· Expires{" "}
													{key.expires_at ? new Date(key.expires_at).toLocaleDateString() : "never"}
												</p>
											</div>
											<Pill tone={key.status === "active" ? "green" : "gray"}>{key.status}</Pill>
											{key.status === "active" ? (
												<div className="flex items-center gap-2">
													<Button
														onClick={() => {
															setSelectedKeyId(key.id);
															setEditName(key.name);
															setEditScopes(
																key.scopes.filter(
																	(scope): scope is z.infer<typeof ApiKeyScopeSchema> =>
																		ApiKeyScopeSchema.safeParse(scope).success,
																),
															);
														}}
														size="sm"
														type="button"
														variant="secondary"
													>
														Manage
													</Button>
													<Button
														className="text-red-400 hover:text-red-300"
														disabled={revoke.isPending}
														onClick={() => void revokeKey(key.id, key.name)}
														size="sm"
														type="button"
														variant="ghost"
													>
														Revoke {key.name}
													</Button>
												</div>
											) : null}
										</div>
									))}
									{selectedKeyId ? (
										<Row>
											<div>
												<h3 className="text-sm font-medium text-white">Manage key</h3>
												{selectedKey.isPending ? (
													<p className="mt-0.5 text-xs text-[#858585]" role="status">
														Loading key details…
													</p>
												) : null}
												{selectedKey.isError ? (
													<p className="mt-0.5 text-xs text-red-300" role="alert">
														{errorMessage(selectedKey.error)}
													</p>
												) : null}
												{selectedKey.data ? (
													<p className="mt-0.5 text-xs text-[#858585]">
														{selectedKey.data.key_prefix}… · {selectedKey.data.status} · Expires{" "}
														{selectedKey.data.expires_at
															? new Date(selectedKey.data.expires_at).toLocaleDateString()
															: "never"}
													</p>
												) : null}
											</div>
											<form
												className="flex flex-col gap-4"
												onSubmit={(event) => {
													event.preventDefault();
													void update.mutateAsync().catch((cause: unknown) => {
														setError(errorMessage(cause));
													});
												}}
											>
												<div className="flex flex-col gap-1.5">
													<Label className={fieldLabel} htmlFor="edit-api-key-name">
														Name
													</Label>
													<Input
														className="h-10 rounded-lg"
														id="edit-api-key-name"
														value={editName}
														onChange={(event) => {
															setEditName(event.target.value);
														}}
													/>
												</div>
												<fieldset className="flex flex-col gap-2">
													<legend className={`mb-2 ${fieldLabel}`}>Scopes</legend>
													<div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
														{scopeOptions.map(([scope, label]) => (
															<Label
																className="flex items-center gap-2 text-xs text-neutral-300"
																htmlFor={`edit-api-key-scope-${scope}`}
																key={scope}
															>
																<Checkbox
																	id={`edit-api-key-scope-${scope}`}
																	checked={editScopes.includes(scope)}
																	onCheckedChange={(checked) => {
																		setEditScopes((current) =>
																			checked === true
																				? [...current, scope]
																				: current.filter((item) => item !== scope),
																		);
																	}}
																/>
																{label}
															</Label>
														))}
													</div>
												</fieldset>
												<div className="flex justify-end gap-2">
													<Button
														type="button"
														variant="ghost"
														onClick={() => {
															setSelectedKeyId(null);
														}}
													>
														Close
													</Button>
													<Button className="rounded-lg" type="submit" disabled={update.isPending}>
														Save key
													</Button>
												</div>
											</form>
										</Row>
									) : null}
									{apiDenial ? (
										<Row
											description="API access needs the Professional or Enterprise plan."
											label="Upgrade for API access"
										>
											<Link
												className="text-sm text-neutral-300 underline underline-offset-2 hover:text-white"
												href="/app/settings?settings=usage"
											>
												See plans in Usage
											</Link>
										</Row>
									) : null}
									<Row>
										<form className="flex flex-col gap-4" onSubmit={(event) => void submit(event)}>
											<div>
												<h3 className="text-sm font-medium text-white">Create a key</h3>
												<p className="mt-0.5 text-xs text-[#858585]">
													Choose only the operations this integration needs. New keys expire by
													default.
												</p>
											</div>
											<div className="grid gap-3 sm:grid-cols-[1fr_10rem]">
												<div className="flex flex-col gap-1.5">
													<Label className={fieldLabel} htmlFor="api-key-name">
														Key name
													</Label>
													<Input
														className="h-10 rounded-lg"
														id="api-key-name"
														maxLength={80}
														onChange={(event) => {
															setName(event.target.value);
														}}
														placeholder="Production integration"
														required
														value={name}
													/>
												</div>
												<div className="flex flex-col gap-1.5">
													<Label className={fieldLabel} htmlFor="api-key-lifetime">
														Expires after
													</Label>
													<Select
														onValueChange={(value) => {
															setLifetimeDays(Number(value));
														}}
														value={String(lifetimeDays)}
													>
														<SelectTrigger className="h-10 w-full rounded-lg" id="api-key-lifetime">
															<SelectValue />
														</SelectTrigger>
														<SelectContent>
															<SelectItem value="30">30 days</SelectItem>
															<SelectItem value="90">90 days</SelectItem>
															<SelectItem value="365">1 year</SelectItem>
														</SelectContent>
													</Select>
												</div>
											</div>
											<fieldset className="flex flex-col gap-2">
												<legend className={`mb-2 ${fieldLabel}`}>Scopes</legend>
												<div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
													{scopeOptions.map(([scope, label]) => (
														<Label
															className="flex items-center gap-2 text-xs text-neutral-300"
															htmlFor={`api-key-scope-${scope}`}
															key={scope}
														>
															<Checkbox
																aria-label={label}
																checked={scopes.includes(scope)}
																id={`api-key-scope-${scope}`}
																onCheckedChange={(checked) => {
																	setScopes((current) =>
																		checked === true
																			? [...current, scope]
																			: current.filter((item) => item !== scope),
																	);
																}}
															/>
															{label}
														</Label>
													))}
												</div>
											</fieldset>
											{create.isError ? (
												<p
													className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
													role="alert"
												>
													{errorMessage(create.error)}
												</p>
											) : null}
											<div className="flex justify-end">
												<Button
													className="rounded-lg"
													disabled={create.isPending || Boolean(revealedKey)}
													type="submit"
												>
													{create.isPending ? "Creating…" : "Create API key"}
												</Button>
											</div>
										</form>
									</Row>
								</Group>
							</Section>
						</>
					)}
				</div>
			</Section>
		</section>
	);
}
