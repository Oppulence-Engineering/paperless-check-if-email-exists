"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, OutcomesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from "@oppulence/ui/components/sheet";
import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import {
	CallbackUrl,
	ProviderEndpointForm,
} from "@/components/features/deliverability/provider-endpoint-form/provider-endpoint-form";
import { type ProviderEndpoint } from "@/components/features/deliverability/provider-endpoint-form/provider-endpoint-form.schema";
import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	V1DeleteProviderEndpoint200Response,
	V1ListProviderEndpoints200Response,
	V1UpdateProviderEndpoint200Response,
} from "@/lib/api/generated/zod/outcomes/outcomes";

import { type ProviderEndpointListPropsFields } from "./provider-endpoint-list.schema";

/**
 * @oppulence-gen kind=component
 * ProviderEndpointList is a client data component.
 * Lists the provider endpoints. Each row can pause, activate, rotate the
 * delivery token, edit the label and the IP allow list, or delete.
 * Owned by `provider-endpoint-list.lit.ts`.
 */
export type ProviderEndpointListProps = ProviderEndpointListPropsFields &
	ComponentPropsWithoutRef<"section">;

const api = new OutcomesApi(new Configuration({ basePath: "/api/backend" }));

export function ProviderEndpointList({ className, ...props }: ProviderEndpointListProps) {
	const client = useQueryClient();
	const [notice, setNotice] = useState("");
	const [revealedPath, setRevealedPath] = useState("");
	const [editing, setEditing] = useState<ProviderEndpoint>();
	const endpoints = useQuery({
		queryKey: workflowQueryKey("provider-endpoints"),
		queryFn: async ({ signal }) =>
			V1ListProviderEndpoints200Response.parse(
				(await api.v1ListProviderEndpoints({ signal })).data,
			),
	});
	const update = useMutation({
		mutationFn: async (input: { endpointId: string; status?: string; rotate?: boolean }) =>
			V1UpdateProviderEndpoint200Response.parse(
				(
					await api.v1UpdateProviderEndpoint({
						endpointId: input.endpointId,
						updateProviderEndpointInput: {
							status: input.status,
							rotate_delivery_token: input.rotate ?? false,
						},
					})
				).data,
			),
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
		mutationFn: async (endpointId: string) =>
			V1DeleteProviderEndpoint200Response.parse(
				(await api.v1DeleteProviderEndpoint({ endpointId })).data,
			),
		onSuccess: async () => {
			setNotice("Provider endpoint removed.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("provider-endpoints") });
		},
	});
	const error = [endpoints, update, remove].find((request) => request.isError)?.error;

	return (
		<section
			data-slot="provider-endpoint-list"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			{error ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(error)}
				</p>
			) : null}
			{notice ? (
				<p
					className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
					role="status"
				>
					{notice}
				</p>
			) : null}
			{revealedPath ? (
				<CallbackUrl
					onDone={() => {
						setRevealedPath("");
					}}
					path={revealedPath}
				/>
			) : null}
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
								<p className="text-xs break-words text-[#858585]">
									Signature configuration:{" "}
									{item.provider === "postmark"
										? "not signed by Postmark; private URL and IP allow list"
										: item.provider_configured
											? "present"
											: "missing"}{" "}
									· Endpoint {item.endpoint_id}
								</p>
								<p className="text-xs break-words text-[#858585]">
									{item.allowed_ips.length
										? `Allowed IPs: ${item.allowed_ips.join(", ")}`
										: "Accepts calls from all IP addresses"}
								</p>
							</div>
							<Pill tone={item.status === "active" ? "green" : "gray"}>{item.status}</Pill>
							<div className="flex flex-wrap items-center gap-2">
								<Button
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
									size="sm"
									type="button"
									variant="secondary"
								>
									{item.status === "active" ? "Pause" : "Activate"}
								</Button>
								<Button
									disabled={update.isPending}
									onClick={() => {
										if (
											window.confirm(
												"Rotate the delivery token? The current provider URL will stop working.",
											)
										)
											update.mutate({ endpointId: item.endpoint_id, rotate: true });
									}}
									size="sm"
									type="button"
									variant="secondary"
								>
									Rotate token
								</Button>
								<Button
									aria-label={`Edit ${item.label}`}
									onClick={() => {
										setEditing(item);
									}}
									size="sm"
									type="button"
									variant="secondary"
								>
									Edit
								</Button>
								<Button
									className="text-red-400 hover:text-red-300"
									disabled={remove.isPending}
									onClick={() => {
										if (window.confirm("Delete this provider endpoint?"))
											remove.mutate(item.endpoint_id);
									}}
									size="sm"
									type="button"
									variant="ghost"
								>
									Delete
								</Button>
							</div>
						</div>
					))
				) : (
					<Row>
						<p className="text-sm text-[#858585]">
							{endpoints.isPending ? "Loading endpoints…" : "No provider endpoints yet."}
						</p>
					</Row>
				)}
			</Group>
			<Sheet
				onOpenChange={(next) => {
					if (!next) setEditing(undefined);
				}}
				open={!!editing}
			>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-lg">
					<SheetHeader>
						<SheetTitle>Edit {editing?.label}</SheetTitle>
						<SheetDescription>Change the label and the IP allow list.</SheetDescription>
					</SheetHeader>
					{editing ? (
						<ProviderEndpointForm
							className="px-4"
							endpoint={editing}
							onSaved={() => {
								setEditing(undefined);
								setNotice("Endpoint updated.");
							}}
						/>
					) : null}
				</SheetContent>
			</Sheet>
		</section>
	);
}
