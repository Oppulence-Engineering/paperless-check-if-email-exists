"use client";

import "client-only";

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
import {
	CreateTenantApiKey201Response,
	ListTenantApiKeys200Response,
	RevokeTenantApiKey200Response,
} from "@/lib/api/generated/zod/account/account";

import {
	ApiKeyListSchema,
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
] as const;

function requestError(cause: unknown) {
	const envelope = z
		.object({
			response: z.object({ data: z.object({ error: z.string() }) }),
		})
		.safeParse(cause);
	return envelope.success
		? envelope.data.response.data.error
		: cause instanceof Error
			? cause.message
			: "Request failed.";
}

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

	const keys = useQuery({
		queryKey,
		enabled: canManage,
		queryFn: async ({ signal }) =>
			ApiKeyListSchema.parse(
				ListTenantApiKeys200Response.parse((await sdk.listTenantApiKeys({ signal })).data),
			),
	});
	const create = useMutation({
		mutationFn: async (body: z.infer<typeof CreateApiKeySchema>) =>
			CreatedApiKeySchema.parse(
				CreateTenantApiKey201Response.parse((await sdk.createTenantApiKey({ data: body })).data),
			),
		onSuccess: async () => {
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
		} catch (cause) {
			setError(requestError(cause));
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
			setError(requestError(cause));
		}
	}

	return (
		<section className={cn("pb-10", className)} data-slot="developer-settings" {...props}>
			<header className="settings-page-intro">
				<h1 className="settings-page-title">API keys</h1>
				<p className="settings-page-description">
					Create scoped credentials for the backend API and review their status and recent use.
				</p>
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
			<section className="settings-section-block">
				<div className="settings-section-heading">
					<div>
						<h2 className="settings-section-title">API access</h2>
						<p className="settings-section-description">
							Call <code>/v1/*</code> on this app&apos;s host with the key as a Bearer token. Keys
							belong to this workspace.
						</p>
					</div>
				</div>
				<div className="settings-panel">
					<div className="settings-row">
						<div className="settings-row-copy">
							<p className="settings-row-label">API reference</p>
							<p className="settings-row-description">
								Browse backend endpoints and operation details in the OpenAPI schema.
							</p>
						</div>
						<Link
							className="text-sm underline underline-offset-4"
							href="/api/backend/openapi.json"
							rel="noopener noreferrer"
							target="_blank"
						>
							Open schema
						</Link>
					</div>
				</div>
			</section>
			{!canManage ? (
				<Alert className="mt-6">
					<AlertDescription>
						Only workspace owners and administrators can manage API keys.
					</AlertDescription>
				</Alert>
			) : (
				<>
					<section className="settings-section-block">
						<div className="settings-section-heading">
							<div>
								<h2 className="settings-section-title">Workspace keys</h2>
								<p className="settings-section-description">
									Only prefixes are visible after creation.
								</p>
							</div>
						</div>
						<div className="settings-panel">
							{keys.isPending ? <p className="p-4 text-sm">Loading API keys…</p> : null}
							{keys.isError ? (
								<div className="settings-row">
									<p role="alert">{requestError(keys.error)}</p>
									<Button
										onClick={() => void keys.refetch()}
										size="sm"
										type="button"
										variant="outline"
									>
										Retry
									</Button>
								</div>
							) : null}
							{keys.data?.api_keys.length === 0 ? (
								<p className="p-4 text-sm text-muted-foreground">No API keys yet.</p>
							) : null}
							{keys.data?.api_keys.map((key) => (
								<div className="settings-row" key={key.id}>
									<div className="settings-row-copy">
										<p className="settings-row-label">{key.name}</p>
										<p className="settings-row-description">
											<code>{key.key_prefix}…</code> · {key.status} ·{" "}
											{key.scopes.length ? key.scopes.join(", ") : "Full access"}
										</p>
										<p className="settings-row-description">
											Created {new Date(key.created_at).toLocaleDateString()} · Last used{" "}
											{key.last_used_at ? new Date(key.last_used_at).toLocaleDateString() : "never"}{" "}
											· Expires{" "}
											{key.expires_at ? new Date(key.expires_at).toLocaleDateString() : "never"}
										</p>
									</div>
									{key.status === "active" ? (
										<Button
											disabled={revoke.isPending}
											onClick={() => void revokeKey(key.id, key.name)}
											size="sm"
											type="button"
											variant="destructive"
										>
											Revoke {key.name}
										</Button>
									) : null}
								</div>
							))}
						</div>
					</section>
					<section className="settings-section-block">
						<div className="settings-section-heading">
							<div>
								<h2 className="settings-section-title">Create a key</h2>
								<p className="settings-section-description">
									Choose only the operations this integration needs. New keys expire by default.
								</p>
							</div>
						</div>
						<div className="settings-panel p-5">
							<form className="space-y-5" onSubmit={(event) => void submit(event)}>
								<div className="space-y-2">
									<Label htmlFor="api-key-name">Key name</Label>
									<Input
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
								<fieldset className="space-y-2">
									<legend className="text-sm font-medium">Scopes</legend>
									<div className="grid gap-2 sm:grid-cols-2 lg:grid-cols-3">
										{scopeOptions.map(([scope, label]) => (
											<Label
												className="flex items-center gap-2 text-sm"
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
								<div className="space-y-2">
									<Label htmlFor="api-key-lifetime">Expires after</Label>
									<Select
										onValueChange={(value) => {
											setLifetimeDays(Number(value));
										}}
										value={String(lifetimeDays)}
									>
										<SelectTrigger className="w-full" id="api-key-lifetime">
											<SelectValue />
										</SelectTrigger>
										<SelectContent>
											<SelectItem value="30">30 days</SelectItem>
											<SelectItem value="90">90 days</SelectItem>
											<SelectItem value="365">1 year</SelectItem>
										</SelectContent>
									</Select>
								</div>
								<Button disabled={create.isPending || Boolean(revealedKey)} type="submit">
									{create.isPending ? "Creating…" : "Create API key"}
								</Button>
							</form>
							{revealedKey ? (
								<div className="mt-5 space-y-2 border-t pt-5" role="status">
									<Label htmlFor="new-api-key">New API key — copy it now</Label>
									<div className="flex gap-2">
										<Input id="new-api-key" readOnly value={revealedKey} />
										<Button
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
											type="button"
											variant="outline"
										>
											{copied ? "Copied" : "Copy key"}
										</Button>
									</div>
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
							) : null}
						</div>
					</section>
				</>
			)}
		</section>
	);
}
