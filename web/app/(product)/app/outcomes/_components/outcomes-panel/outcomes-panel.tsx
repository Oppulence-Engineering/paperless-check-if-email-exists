"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, OutcomesApi } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { type OutcomesPanelPropsFields } from "./outcomes-panel.schema";

/** @oppulence-gen kind=component Owned by `outcomes-panel.lit.ts`. */
export type OutcomesPanelProps = OutcomesPanelPropsFields & ComponentPropsWithoutRef<"section">;

const api = new OutcomesApi(new Configuration({ basePath: "/api/backend" }));
type Provider = "postmark" | "mailgun" | "sendgrid" | "ses";
export function OutcomesPanel({ className, ...props }: OutcomesPanelProps) {
	const client = useQueryClient();
	const [offset, setOffset] = useState(0);
	const [emailFilter, setEmailFilter] = useState("");
	const [filter, setFilter] = useState("");
	const [provider, setProvider] = useState<Provider>("postmark");
	const [label, setLabel] = useState("");
	const [signatureConfig, setSignatureConfig] = useState("");
	const [revealedPath, setRevealedPath] = useState("");
	const [eventEmail, setEventEmail] = useState("");
	const [eventType, setEventType] = useState("delivered");
	const [eventProvider, setEventProvider] = useState("manual");
	const [notice, setNotice] = useState("");
	const outcomes = useQuery({
		queryKey: workflowQueryKey("outcomes", offset, filter),
		queryFn: async () =>
			(await api.v1ListOutcomes({ limit: 20, offset, email: filter || undefined })).data,
	});
	const endpoints = useQuery({
		queryKey: workflowQueryKey("provider-endpoints"),
		queryFn: async () => (await api.v1ListProviderEndpoints()).data,
	});
	const create = useMutation({
		mutationFn: async () => {
			const providerConfig =
				provider === "mailgun"
					? { signing_key: signatureConfig.trim() }
					: provider === "sendgrid"
						? { public_key_pem: signatureConfig.trim() }
						: {};
			if (
				!label.trim() ||
				((provider === "mailgun" || provider === "sendgrid") && !signatureConfig.trim())
			)
				throw new Error("Enter a label and provider verification key.");
			return (
				await api.v1CreateProviderEndpoint({
					createProviderEndpointInput: {
						provider,
						label: label.trim(),
						status: "paused",
						provider_config: providerConfig,
					},
				})
			).data;
		},
		onSuccess: async (data) => {
			setRevealedPath(data.webhook_path);
			setSignatureConfig("");
			setLabel("");
			setNotice(
				"Endpoint created paused. Copy the callback URL now, then activate after provider setup.",
			);
			await client.invalidateQueries({ queryKey: workflowQueryKey("provider-endpoints") });
		},
	});
	const update = useMutation({
		mutationFn: async (input: { endpointId: string; status?: string; rotate?: boolean }) =>
			(
				await api.v1UpdateProviderEndpoint({
					endpointId: input.endpointId,
					updateProviderEndpointInput: {
						status: input.status,
						rotate_delivery_token: input.rotate ?? false,
					},
				})
			).data,
		onSuccess: async (data) => {
			if (data.delivery_token) setRevealedPath(data.webhook_path);
			setNotice(
				data.delivery_token
					? "Delivery token rotated. Replace the provider callback URL now."
					: "Endpoint status updated.",
			);
			await client.invalidateQueries({ queryKey: workflowQueryKey("provider-endpoints") });
		},
	});
	const remove = useMutation({
		mutationFn: async (endpointId: string) => {
			await api.v1DeleteProviderEndpoint({ endpointId });
		},
		onSuccess: async () => {
			setNotice("Provider endpoint removed.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("provider-endpoints") });
		},
	});
	const ingest = useMutation({
		mutationFn: async () =>
			(
				await api.v1IngestOutcomes({
					outcomeIngestRequest: {
						provider: eventProvider.trim(),
						outcomes: [{ email: eventEmail.trim(), event_type: eventType }],
					},
				})
			).data,
		onSuccess: async (data) => {
			setNotice(
				`Recorded ${String(data.ingested)} outcome(s); ${String(data.auto_suppressed)} auto-suppressed.`,
			);
			setEventEmail("");
			await client.invalidateQueries({ queryKey: workflowQueryKey("outcomes") });
		},
	});
	const error = [outcomes, endpoints, create, update, remove, ingest].find(
		(request) => request.isError,
	)?.error;
	return (
		<section data-slot="outcomes-panel" className={cn("space-y-6", className)} {...props}>
			<header>
				<h1 className="text-3xl font-semibold">Delivery outcomes</h1>
				<p className="mt-2 text-muted-foreground">
					Review feedback and configure trusted provider deliveries.
				</p>
			</header>
			{error ? (
				<p role="alert" className="text-destructive">
					{error.message}
				</p>
			) : null}
			{notice ? <p role="status">{notice}</p> : null}
			{revealedPath ? (
				<Card>
					<CardHeader>
						<CardTitle>Copy the callback URL now</CardTitle>
					</CardHeader>
					<CardContent className="space-y-2">
						<p className="text-sm">
							This one-time URL contains the delivery token. Store it in the provider dashboard. It
							will not be shown again after leaving this page.
						</p>
						<code className="block break-all rounded bg-muted p-3">
							{typeof window === "undefined"
								? revealedPath
								: `${window.location.origin}${revealedPath}`}
						</code>
						<Button
							variant="outline"
							onClick={() => {
								void navigator.clipboard.writeText(`${window.location.origin}${revealedPath}`);
							}}
						>
							Copy URL
						</Button>
						<Button
							variant="ghost"
							onClick={() => {
								setRevealedPath("");
							}}
						>
							Done
						</Button>
					</CardContent>
				</Card>
			) : null}
			<Card>
				<CardHeader>
					<CardTitle>Recent outcomes</CardTitle>
				</CardHeader>
				<CardContent className="space-y-4">
					<form
						className="flex gap-2"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							setOffset(0);
							setFilter(emailFilter.trim());
						}}
					>
						<Label className="sr-only" htmlFor="outcome-filter">
							Filter by email
						</Label>
						<Input
							id="outcome-filter"
							type="email"
							placeholder="Filter by email"
							value={emailFilter}
							onChange={(event) => {
								setEmailFilter(event.target.value);
							}}
						/>
						<Button type="submit">Search</Button>
					</form>
					{outcomes.data?.outcomes.length ? (
						<div className="overflow-x-auto">
							<table className="w-full text-sm">
								<thead>
									<tr>
										<th className="p-2 text-left">Address</th>
										<th className="p-2 text-left">Event</th>
										<th className="p-2 text-left">Provider</th>
										<th className="p-2 text-left">When</th>
									</tr>
								</thead>
								<tbody>
									{outcomes.data.outcomes.map((item) => (
										<tr className="border-t" key={item.id}>
											<td className="p-2">{item.email}</td>
											<td className="p-2">{item.event_type}</td>
											<td className="p-2">{item.provider}</td>
											<td className="p-2">{new Date(item.occurred_at).toLocaleString()}</td>
										</tr>
									))}
								</tbody>
							</table>
						</div>
					) : (
						<p>No outcomes found.</p>
					)}
					<div className="flex gap-2">
						<Button
							size="sm"
							variant="outline"
							disabled={offset === 0}
							onClick={() => {
								setOffset(Math.max(0, offset - 20));
							}}
						>
							Previous
						</Button>
						<Button
							size="sm"
							variant="outline"
							disabled={!outcomes.data || outcomes.data.outcomes.length < 20}
							onClick={() => {
								setOffset(offset + 20);
							}}
						>
							Next
						</Button>
					</div>
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Provider endpoints</CardTitle>
				</CardHeader>
				<CardContent className="space-y-4">
					{endpoints.data?.provider_endpoints.length ? (
						<ul className="space-y-2">
							{endpoints.data.provider_endpoints.map((item) => (
								<li className="rounded border p-3" key={item.endpoint_id}>
									<p className="font-medium">
										{item.label} · {item.provider} · {item.status}
									</p>
									<p className="text-sm text-muted-foreground">
										Signature configuration:{" "}
										{item.provider === "postmark" || item.provider === "ses"
											? "provider-managed"
											: item.provider_configured
												? "present"
												: "missing"}{" "}
										· Endpoint {item.endpoint_id}
									</p>
									<div className="mt-2 flex flex-wrap gap-2">
										<Button
											size="sm"
											variant="outline"
											disabled={
												update.isPending ||
												(item.status !== "active" &&
													!item.provider_configured &&
													(item.provider === "mailgun" || item.provider === "sendgrid"))
											}
											onClick={() => {
												update.mutate({
													endpointId: item.endpoint_id,
													status: item.status === "active" ? "paused" : "active",
												});
											}}
										>
											{item.status === "active" ? "Pause" : "Activate"}
										</Button>
										<Button
											size="sm"
											variant="outline"
											disabled={update.isPending}
											onClick={() => {
												if (
													window.confirm(
														"Rotate the delivery token? The current provider URL will stop working.",
													)
												)
													update.mutate({ endpointId: item.endpoint_id, rotate: true });
											}}
										>
											Rotate token
										</Button>
										<Button
											size="sm"
											variant="ghost"
											disabled={remove.isPending}
											onClick={() => {
												if (window.confirm("Delete this provider endpoint?"))
													remove.mutate(item.endpoint_id);
											}}
										>
											Delete
										</Button>
									</div>
								</li>
							))}
						</ul>
					) : (
						<p>No provider endpoints yet.</p>
					)}
					<p className="text-sm">
						<Link className="underline" href="/app/integrations">
							Provider setup and callback guide
						</Link>
					</p>
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Register a provider</CardTitle>
				</CardHeader>
				<CardContent>
					<form
						className="grid gap-3 sm:grid-cols-2"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							create.mutate();
						}}
					>
						<div>
							<Label htmlFor="endpoint-label">Label</Label>
							<Input
								id="endpoint-label"
								required
								value={label}
								onChange={(event) => {
									setLabel(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="endpoint-provider">Provider</Label>
							<select
								id="endpoint-provider"
								className="h-9 w-full rounded border bg-background px-3"
								value={provider}
								onChange={(event) => {
									const value = event.target.value;
									if (
										value === "postmark" ||
										value === "mailgun" ||
										value === "sendgrid" ||
										value === "ses"
									) {
										setProvider(value);
										setSignatureConfig("");
									}
								}}
							>
								<option value="postmark">Postmark</option>
								<option value="mailgun">Mailgun</option>
								<option value="sendgrid">SendGrid</option>
								<option value="ses">Amazon SES</option>
							</select>
						</div>
						{provider === "mailgun" || provider === "sendgrid" ? (
							<div className="sm:col-span-2">
								<Label htmlFor="endpoint-verification">
									{provider === "mailgun"
										? "Mailgun signing key"
										: "SendGrid public verification key (PEM)"}
								</Label>
								<Input
									id="endpoint-verification"
									type={provider === "mailgun" ? "password" : "text"}
									autoComplete="off"
									value={signatureConfig}
									onChange={(event) => {
										setSignatureConfig(event.target.value);
									}}
								/>
							</div>
						) : null}
						<p className="text-sm text-muted-foreground sm:col-span-2">
							New endpoints start paused. Copy the one-time callback URL into your provider
							dashboard, send a provider test event, then activate.
						</p>
						<Button type="submit" disabled={create.isPending}>
							Create paused endpoint
						</Button>
					</form>
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Record a manual outcome</CardTitle>
				</CardHeader>
				<CardContent>
					<form
						className="grid gap-3 sm:grid-cols-3"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							if (
								window.confirm(
									"Record this delivery outcome? Bounce or complaint events may suppress the address.",
								)
							)
								ingest.mutate();
						}}
					>
						<div>
							<Label htmlFor="event-email">Email</Label>
							<Input
								id="event-email"
								type="email"
								required
								value={eventEmail}
								onChange={(event) => {
									setEventEmail(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="event-provider">Source</Label>
							<Input
								id="event-provider"
								required
								value={eventProvider}
								onChange={(event) => {
									setEventProvider(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="event-type">Event</Label>
							<select
								id="event-type"
								className="h-9 w-full rounded border bg-background px-3"
								value={eventType}
								onChange={(event) => {
									setEventType(event.target.value);
								}}
							>
								<option value="delivered">Delivered</option>
								<option value="bounced">Bounce</option>
								<option value="complained">Complaint</option>
								<option value="opened">Opened</option>
								<option value="clicked">Clicked</option>
								<option value="unsubscribed">Unsubscribed</option>
							</select>
						</div>
						<Button type="submit" disabled={ingest.isPending}>
							Record outcome
						</Button>
					</form>
				</CardContent>
			</Card>
		</section>
	);
}
