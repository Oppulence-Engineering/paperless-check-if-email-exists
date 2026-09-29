"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { AccountApi, Configuration, TenantApi } from "@oppulence/reacher-sdk";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { type DomainsPanelPropsFields } from "./domains-panel.schema";

/** @oppulence-gen kind=component Owned by `domains-panel.lit.ts`. */
export type DomainsPanelProps = DomainsPanelPropsFields & ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const tenantApi = new TenantApi(config);
const accountApi = new AccountApi(config);
const domainSchema = z.object({
	domain: z.string(),
	is_active: z.boolean(),
	is_verified: z.boolean(),
	notes: z.string().nullable().optional(),
	created_at: z.string(),
});
const listSchema = z.object({ domains: z.array(domainSchema) });
const meSchema = z.object({ tenant_name: z.string(), status: z.string(), plan_tier: z.string() });

export function DomainsPanel({ className, ...props }: DomainsPanelProps) {
	const client = useQueryClient();
	const [domain, setDomain] = useState("");
	const [notes, setNotes] = useState("");
	const [selected, setSelected] = useState("");
	const [editNotes, setEditNotes] = useState("");
	const [notice, setNotice] = useState("");
	const me = useQuery({
		queryKey: workflowQueryKey("tenant-profile"),
		queryFn: async () => meSchema.parse((await accountApi.v1Me()).data),
	});
	const domains = useQuery({
		queryKey: workflowQueryKey("tenant-domains"),
		queryFn: async () => listSchema.parse((await tenantApi.v1ListTenantDomains()).data),
	});
	const detail = useQuery({
		queryKey: workflowQueryKey("tenant-domain", selected),
		enabled: !!selected,
		queryFn: async () =>
			domainSchema.parse((await tenantApi.v1GetTenantDomain({ domain: selected })).data),
	});
	const create = useMutation({
		mutationFn: async () => {
			await tenantApi.v1CreateTenantDomain({
				createTenantDomainRequest: {
					domain: domain.trim(),
					notes: notes.trim() || undefined,
					is_active: false,
				},
			});
		},
		onSuccess: async () => {
			setDomain("");
			setNotes("");
			setNotice("Domain added inactive. Review verification before activation.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("tenant-domains") });
		},
	});
	const update = useMutation({
		mutationFn: async (input: { isActive?: boolean; notes?: string }) => {
			await tenantApi.v1UpdateTenantDomain({
				domain: selected,
				updateTenantDomainRequest: { is_active: input.isActive, notes: input.notes },
			});
		},
		onSuccess: async () => {
			setNotice("Domain updated.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("tenant-domains") });
			await client.invalidateQueries({ queryKey: workflowQueryKey("tenant-domain", selected) });
		},
	});
	const remove = useMutation({
		mutationFn: async () => {
			await tenantApi.v1DeleteTenantDomain({ domain: selected });
		},
		onSuccess: async () => {
			setSelected("");
			setNotice("Domain removed.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("tenant-domains") });
		},
	});
	const error = [me, domains, detail, create, update, remove].find(
		(request) => request.isError,
	)?.error;
	return (
		<section data-slot="domains-panel" className={cn("space-y-6", className)} {...props}>
			<header>
				<h1 className="text-3xl font-semibold">Workspace domains</h1>
				<p className="mt-2 text-muted-foreground">
					Manage domains used by this workspace and review their verification state.
				</p>
			</header>
			{me.data ? (
				<p className="text-sm">
					{me.data.tenant_name} · {me.data.plan_tier} · {me.data.status}
				</p>
			) : null}
			{error ? (
				<p role="alert" className="text-destructive">
					{error.message}
				</p>
			) : null}
			{notice ? <p role="status">{notice}</p> : null}
			<Card>
				<CardHeader>
					<CardTitle>Domains</CardTitle>
				</CardHeader>
				<CardContent>
					{domains.data?.domains.length ? (
						<ul className="divide-y">
							{domains.data.domains.map((item) => (
								<li className="flex items-center justify-between gap-3 py-3" key={item.domain}>
									<div>
										<p className="font-medium">{item.domain}</p>
										<p className="text-sm text-muted-foreground">
											{item.is_verified ? "Verified" : "Unverified"} ·{" "}
											{item.is_active ? "Active" : "Inactive"}
										</p>
									</div>
									<Button
										variant="outline"
										size="sm"
										onClick={() => {
											setSelected(item.domain);
											setEditNotes(item.notes || "");
										}}
									>
										Manage
									</Button>
								</li>
							))}
						</ul>
					) : (
						<p>No domains added.</p>
					)}
				</CardContent>
			</Card>
			{detail.data ? (
				<Card>
					<CardHeader>
						<CardTitle>{detail.data.domain}</CardTitle>
					</CardHeader>
					<CardContent className="space-y-4">
						<p>
							{detail.data.is_verified ? "Verified" : "Unverified"} ·{" "}
							{detail.data.is_active ? "Active" : "Inactive"} · Added{" "}
							{new Date(detail.data.created_at).toLocaleDateString()}
						</p>
						<div className="flex gap-2">
							<Button
								variant="outline"
								disabled={update.isPending || (!detail.data.is_verified && !detail.data.is_active)}
								onClick={() => {
									update.mutate({ isActive: !detail.data.is_active });
								}}
							>
								{detail.data.is_active ? "Deactivate" : "Activate verified domain"}
							</Button>
							<Button
								variant="destructive"
								disabled={remove.isPending}
								onClick={() => {
									if (window.confirm(`Remove ${detail.data.domain} from this workspace?`))
										remove.mutate();
								}}
							>
								Remove
							</Button>
						</div>
						<form
							className="flex gap-2"
							onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
								event.preventDefault();
								update.mutate({ notes: editNotes.trim() });
							}}
						>
							<Label className="sr-only" htmlFor="domain-notes-edit">
								Notes
							</Label>
							<Input
								id="domain-notes-edit"
								value={editNotes}
								onChange={(event) => {
									setEditNotes(event.target.value);
								}}
							/>
							<Button type="submit" variant="outline" disabled={update.isPending}>
								Save notes
							</Button>
						</form>
					</CardContent>
				</Card>
			) : null}
			<Card>
				<CardHeader>
					<CardTitle>Add a domain</CardTitle>
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
							<Label htmlFor="new-domain">Domain</Label>
							<Input
								id="new-domain"
								required
								placeholder="example.com"
								value={domain}
								onChange={(event) => {
									setDomain(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="new-domain-notes">Notes</Label>
							<Input
								id="new-domain-notes"
								value={notes}
								onChange={(event) => {
									setNotes(event.target.value);
								}}
							/>
						</div>
						<p className="text-sm text-muted-foreground sm:col-span-2">
							New domains start inactive. Verify ownership before activating.
						</p>
						<Button type="submit" disabled={create.isPending}>
							Add inactive domain
						</Button>
					</form>
				</CardContent>
			</Card>
		</section>
	);
}
