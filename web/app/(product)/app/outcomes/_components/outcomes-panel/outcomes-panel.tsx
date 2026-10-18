"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, OutcomesApi } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";

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
						: provider === "ses"
							? { topic_arns: [signatureConfig.trim()] }
							: {};
			if (!label.trim() || (provider !== "postmark" && !signatureConfig.trim()))
				throw new Error("Enter a label and the provider verification value.");
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
	const field = "h-10 rounded-lg md:w-72";
	const select =
		"h-10 w-full rounded-lg bg-white/[0.03] px-2 text-sm text-neutral-200 ring-1 ring-white/[0.08] outline-none md:w-72";
	return (
		<Page
			description="Review feedback and configure trusted provider deliveries."
			title="Delivery outcomes"
		>
			<section
				data-slot="outcomes-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				{error ? (
					<p
						role="alert"
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					>
						{error.message}
					</p>
				) : null}
				{notice ? (
					<p
						role="status"
						className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
					>
						{notice}
					</p>
				) : null}
				{revealedPath ? (
					<div className="flex flex-col gap-3 rounded-2xl bg-emerald-500/5 p-5 ring-1 ring-emerald-500/20">
						<p className="text-sm font-medium text-white">Copy the callback URL now</p>
						<p className="text-sm text-neutral-400">
							This one-time URL contains the delivery token. Store it in the provider dashboard. It
							will not be shown again after leaving this page.
						</p>
						<code className="overflow-x-auto rounded-lg bg-black/30 p-3 font-mono text-xs break-all text-neutral-300 ring-1 ring-white/[0.08]">
							{typeof window === "undefined"
								? revealedPath
								: `${window.location.origin}${revealedPath}`}
						</code>
						<div className="flex gap-2">
							<Button
								className="rounded-lg"
								size="sm"
								onClick={() => {
									void navigator.clipboard.writeText(`${window.location.origin}${revealedPath}`);
								}}
							>
								Copy URL
							</Button>
							<Button
								size="sm"
								variant="ghost"
								onClick={() => {
									setRevealedPath("");
								}}
							>
								Done
							</Button>
						</div>
					</div>
				) : null}
				<Section title="Recent outcomes">
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
							className="h-10 rounded-lg"
							id="outcome-filter"
							type="email"
							placeholder="Filter by email"
							value={emailFilter}
							onChange={(event) => {
								setEmailFilter(event.target.value);
							}}
						/>
						<Button className="h-10 rounded-lg" type="submit">
							Search
						</Button>
					</form>
					<Group>
						{outcomes.data?.outcomes.length ? (
							<Row>
								<div className="overflow-x-auto">
									<table className="w-full text-left text-sm">
										<thead>
											<tr className="border-b border-border">
												<th className="py-2 pr-4 font-medium text-foreground">Address</th>
												<th className="py-2 pr-4 font-medium text-foreground">Event</th>
												<th className="py-2 pr-4 font-medium text-foreground">Provider</th>
												<th className="py-2 font-medium text-foreground">When</th>
											</tr>
										</thead>
										<tbody>
											{outcomes.data.outcomes.map((item) => (
												<tr className="border-b border-border/60" key={item.id}>
													<td className="py-2 pr-4 text-foreground">{item.email}</td>
													<td className="py-2 pr-4 text-muted-foreground">{item.event_type}</td>
													<td className="py-2 pr-4 text-muted-foreground">{item.provider}</td>
													<td className="py-2 text-muted-foreground">
														{new Date(item.occurred_at).toLocaleString()}
													</td>
												</tr>
											))}
										</tbody>
									</table>
								</div>
							</Row>
						) : (
							<Row>
								<p className="text-sm text-[#858585]">No outcomes found.</p>
							</Row>
						)}
					</Group>
					<div className="flex justify-end gap-2">
						<Button
							className="rounded-lg"
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
							className="rounded-lg"
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
				</Section>
				<Section
					actions={
						<Link
							className="text-sm text-[#858585] hover:text-neutral-200"
							href="/app/integrations"
						>
							Provider setup and callback guide
						</Link>
					}
					title="Provider endpoints"
				>
					<Group>
						{endpoints.data?.provider_endpoints.length ? (
							endpoints.data.provider_endpoints.map((item) => (
								<div
									className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
									key={item.endpoint_id}
								>
									<div className="min-w-0 flex-1">
										<p className="truncate text-sm text-white">
											{item.label} · {item.provider}
										</p>
										<p className="text-xs text-[#858585]">
											Signature configuration:{" "}
											{item.provider === "postmark"
												? "not signed by Postmark; private URL and IP allow list"
												: item.provider_configured
													? "present"
													: "missing"}{" "}
											· Endpoint {item.endpoint_id}
										</p>
									</div>
									<Pill tone={item.status === "active" ? "green" : "gray"}>{item.status}</Pill>
									<div className="flex flex-wrap items-center gap-2">
										<Button
											size="sm"
											variant="secondary"
											disabled={
												update.isPending ||
												(item.status !== "active" &&
													!item.provider_configured &&
													item.provider !== "postmark")
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
											variant="secondary"
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
											className="text-red-400 hover:text-red-300"
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
								</div>
							))
						) : (
							<Row>
								<p className="text-sm text-[#858585]">No provider endpoints yet.</p>
							</Row>
						)}
					</Group>
				</Section>
				<Section
					description="New endpoints start paused. Copy the one-time callback URL into your provider dashboard, send a provider test event, then activate."
					title="Register a provider"
				>
					<form
						className="flex flex-col gap-4"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							create.mutate();
						}}
					>
						<Group>
							<Row label={<Label htmlFor="endpoint-label">Label</Label>}>
								<Input
									className={field}
									id="endpoint-label"
									required
									value={label}
									onChange={(event) => {
										setLabel(event.target.value);
									}}
								/>
							</Row>
							<Row label={<Label htmlFor="endpoint-provider">Provider</Label>}>
								<select
									id="endpoint-provider"
									className={select}
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
							</Row>
							{provider !== "postmark" ? (
								<Row
									label={
										<Label htmlFor="endpoint-verification">
											{provider === "mailgun"
												? "Mailgun signing key"
												: provider === "ses"
													? "Amazon SNS topic ARN"
													: "SendGrid public verification key (PEM)"}
										</Label>
									}
								>
									<Input
										className={field}
										id="endpoint-verification"
										type={provider === "mailgun" ? "password" : "text"}
										autoComplete="off"
										value={signatureConfig}
										onChange={(event) => {
											setSignatureConfig(event.target.value);
										}}
									/>
								</Row>
							) : null}
						</Group>
						<div className="flex justify-end">
							<Button className="rounded-lg" type="submit" disabled={create.isPending}>
								Create paused endpoint
							</Button>
						</div>
					</form>
				</Section>
				<Section title="Record a manual outcome">
					<form
						className="flex flex-col gap-4"
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
						<Group>
							<Row label={<Label htmlFor="event-email">Email</Label>}>
								<Input
									className={field}
									id="event-email"
									type="email"
									required
									value={eventEmail}
									onChange={(event) => {
										setEventEmail(event.target.value);
									}}
								/>
							</Row>
							<Row label={<Label htmlFor="event-provider">Source</Label>}>
								<Input
									className={field}
									id="event-provider"
									required
									value={eventProvider}
									onChange={(event) => {
										setEventProvider(event.target.value);
									}}
								/>
							</Row>
							<Row label={<Label htmlFor="event-type">Event</Label>}>
								<select
									id="event-type"
									className={select}
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
							</Row>
						</Group>
						<div className="flex justify-end">
							<Button className="rounded-lg" type="submit" disabled={ingest.isPending}>
								Record outcome
							</Button>
						</div>
					</form>
				</Section>
			</section>
		</Page>
	);
}
