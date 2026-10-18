"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Configuration, OutcomesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { Textarea } from "@oppulence/ui/components/textarea";
import { cn } from "@oppulence/ui/lib/utils";

import {
	apiMessage,
	FILTER_CLASS,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	V1CreateProviderEndpoint201Response,
	V1UpdateProviderEndpoint200Response,
} from "@/lib/api/generated/zod/outcomes/outcomes";

import { type ProviderEndpointFormPropsFields } from "./provider-endpoint-form.schema";

/**
 * @oppulence-gen kind=component
 * ProviderEndpointForm is a client data component.
 * Creates a paused provider endpoint, or changes the label and the IP allow
 * list of one endpoint. After a create, it shows the one-time callback URL.
 * Owned by `provider-endpoint-form.lit.ts`.
 */
export type ProviderEndpointFormProps = ProviderEndpointFormPropsFields & {
	onSaved?: () => void;
} & ComponentPropsWithoutRef<"section">;

const api = new OutcomesApi(new Configuration({ basePath: "/api/backend" }));
const FIELD_LABEL = "text-xs font-medium text-neutral-400";
type Provider = "postmark" | "mailgun" | "sendgrid" | "ses";
const PROVIDERS: Record<Provider, string> = {
	postmark: "Postmark",
	mailgun: "Mailgun",
	sendgrid: "SendGrid",
	ses: "Amazon SES",
};
const VERIFICATION_LABELS: Record<Exclude<Provider, "postmark">, string> = {
	mailgun: "Mailgun signing key",
	sendgrid: "SendGrid public verification key (PEM)",
	ses: "Amazon SNS topic ARN",
};

/** IP addresses typed one on each line, or with commas or spaces between them. */
function ipList(value: string): string[] {
	return value.split(/[\s,]+/).filter(Boolean);
}

/** The one-time callback URL. The URL holds the delivery token, so it shows once only. */
export function CallbackUrl({ path, onDone }: { path: string; onDone: () => void }) {
	const url = typeof window === "undefined" ? path : `${window.location.origin}${path}`;
	return (
		<div className="flex flex-col gap-3 rounded-2xl bg-emerald-500/5 p-5 ring-1 ring-emerald-500/20">
			<p className="text-sm font-medium text-white">Copy the callback URL now</p>
			<p className="text-sm text-neutral-400">
				This one-time URL contains the delivery token. Store it in the provider dashboard. It will
				not be shown again after leaving this page.
			</p>
			<code className="overflow-x-auto rounded-lg bg-black/30 p-3 font-mono text-xs break-all text-neutral-300 ring-1 ring-white/[0.08]">
				{url}
			</code>
			<div className="flex gap-2">
				<Button
					className="rounded-lg"
					onClick={() => {
						void navigator.clipboard.writeText(url);
					}}
					size="sm"
					type="button"
				>
					Copy URL
				</Button>
				<Button onClick={onDone} size="sm" type="button" variant="ghost">
					Done
				</Button>
			</div>
		</div>
	);
}

export function ProviderEndpointForm({
	className,
	endpoint,
	onSaved,
	...props
}: ProviderEndpointFormProps) {
	const client = useQueryClient();
	const [provider, setProvider] = useState<Provider>("postmark");
	const [label, setLabel] = useState(endpoint?.label ?? "");
	const [signatureConfig, setSignatureConfig] = useState("");
	const [allowedIps, setAllowedIps] = useState(endpoint?.allowed_ips.join("\n") ?? "");
	const [revealedPath, setRevealedPath] = useState("");
	const kind = endpoint?.provider ?? provider;
	const save = useMutation({
		mutationFn: async () => {
			if (endpoint)
				return V1UpdateProviderEndpoint200Response.parse(
					(
						await api.v1UpdateProviderEndpoint({
							endpointId: endpoint.endpoint_id,
							updateProviderEndpointInput: {
								label: label.trim(),
								allowed_ips: ipList(allowedIps),
							},
						})
					).data,
				);
			const value = signatureConfig.trim();
			if (!label.trim() || (provider !== "postmark" && !value))
				throw new Error("Enter a label and the provider verification value.");
			const providerConfig =
				provider === "mailgun"
					? { signing_key: value }
					: provider === "sendgrid"
						? { public_key_pem: value }
						: provider === "ses"
							? { topic_arns: [value] }
							: {};
			return V1CreateProviderEndpoint201Response.parse(
				(
					await api.v1CreateProviderEndpoint({
						createProviderEndpointInput: {
							provider,
							label: label.trim(),
							status: "paused",
							provider_config: providerConfig,
							allowed_ips: ipList(allowedIps),
						},
					})
				).data,
			);
		},
		onSuccess: async (data) => {
			if (!endpoint) {
				setRevealedPath(data.webhook_path);
				setSignatureConfig("");
				setLabel("");
				setAllowedIps("");
			}
			await client.invalidateQueries({ queryKey: workflowQueryKey("provider-endpoints") });
			if (endpoint) onSaved?.();
		},
	});

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		save.mutate();
	}

	return (
		<section
			data-slot="provider-endpoint-form"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			{save.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(save.error)}
				</p>
			) : null}
			{revealedPath ? (
				<>
					<p
						className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
						role="status"
					>
						Endpoint created paused. Copy the callback URL now, then activate after provider setup.
					</p>
					<CallbackUrl
						onDone={() => {
							setRevealedPath("");
							onSaved?.();
						}}
						path={revealedPath}
					/>
				</>
			) : null}
			<form className="flex flex-col gap-4" onSubmit={submit}>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="endpoint-label">
						Label
					</Label>
					<Input
						className="h-10 rounded-lg"
						id="endpoint-label"
						onChange={(event) => {
							setLabel(event.target.value);
						}}
						required
						value={label}
					/>
				</div>
				{endpoint ? null : (
					<div className="flex flex-col gap-1.5">
						<Label className={FIELD_LABEL} htmlFor="endpoint-provider">
							Provider
						</Label>
						<select
							className={FILTER_CLASS}
							id="endpoint-provider"
							onChange={(event) => {
								setProvider(event.target.value as Provider);
								setSignatureConfig("");
							}}
							value={provider}
						>
							{Object.entries(PROVIDERS).map(([value, name]) => (
								<option key={value} value={value}>
									{name}
								</option>
							))}
						</select>
					</div>
				)}
				{!endpoint && provider !== "postmark" ? (
					<div className="flex flex-col gap-1.5">
						<Label className={FIELD_LABEL} htmlFor="endpoint-verification">
							{VERIFICATION_LABELS[provider]}
						</Label>
						<Input
							autoComplete="off"
							className="h-10 rounded-lg"
							id="endpoint-verification"
							onChange={(event) => {
								setSignatureConfig(event.target.value);
							}}
							type={provider === "mailgun" ? "password" : "text"}
							value={signatureConfig}
						/>
					</div>
				) : null}
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="endpoint-allowed-ips">
						Allowed IP addresses
					</Label>
					<Textarea
						className="min-h-20 rounded-lg font-mono text-xs"
						id="endpoint-allowed-ips"
						onChange={(event) => {
							setAllowedIps(event.target.value);
						}}
						placeholder={"192.0.2.10\n192.0.2.11"}
						value={allowedIps}
					/>
					<p className="text-xs text-[#858585]">
						{kind === "postmark"
							? "Postmark does not sign its calls. Enter the IP addresses that Postmark sends webhooks from, one on each line. "
							: "Optional. Enter one IP address on each line. "}
						Leave this empty to accept calls from all addresses.
					</p>
				</div>
				<div className="flex justify-end">
					<Button className="rounded-lg" disabled={save.isPending} type="submit">
						{endpoint ? "Save endpoint" : "Create paused endpoint"}
					</Button>
				</div>
			</form>
		</section>
	);
}
