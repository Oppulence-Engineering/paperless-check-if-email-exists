"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { AccountApi, Configuration, TenantApi } from "@oppulence/reacher-sdk";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { DomainHealth } from "@/components/features/deliverability/domain-health/domain-health";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";

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
	const [lookup, setLookup] = useState("");
	const [checked, setChecked] = useState("");
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
	const field = "h-10 rounded-lg md:w-72";
	return (
		<Page
			description="Check the health of the domains this workspace sends from, and manage them."
			title="Sender domains"
		>
			<section
				data-slot="domains-panel"
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
				<Section
					description={
						<>
							{me.data
								? `${me.data.tenant_name} · ${me.data.plan_tier} · ${me.data.status}. `
								: null}
							The health check reads public DNS records and blocklists. It does not verify
							ownership.
						</>
					}
					title="Domains"
				>
					{domains.data?.domains.length ? (
						domains.data.domains.map((item) => (
							<Group key={item.domain}>
								<div className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4">
									<div className="min-w-0 flex-1 basis-40">
										<h3 className="truncate text-sm font-medium text-white">{item.domain}</h3>
									</div>
									<Pill tone={item.is_verified ? "green" : "yellow"}>
										{item.is_verified ? "Verified" : "Unverified"}
									</Pill>
									<Pill tone={item.is_active ? "blue" : "gray"}>
										{item.is_active ? "Active" : "Inactive"}
									</Pill>
									<Button
										variant="secondary"
										size="sm"
										onClick={() => {
											setSelected(item.domain);
											setEditNotes(item.notes || "");
										}}
									>
										Manage
									</Button>
								</div>
								<DomainHealth aria-label={`Health of ${item.domain}`} domain={item.domain} />
							</Group>
						))
					) : (
						<Group>
							<Row>
								<p className="text-sm text-[#858585]">No domains added.</p>
							</Row>
						</Group>
					)}
				</Section>
				{detail.data ? (
					<Section
						description={`${detail.data.is_verified ? "Verified" : "Unverified"} · ${detail.data.is_active ? "Active" : "Inactive"} · Added ${new Date(detail.data.created_at).toLocaleDateString()}`}
						title={detail.data.domain}
					>
						<Group>
							<Row>
								<div className="flex flex-wrap gap-2">
									<Button
										className="rounded-lg"
										variant="outline"
										disabled={
											update.isPending || (!detail.data.is_verified && !detail.data.is_active)
										}
										onClick={() => {
											update.mutate({ isActive: !detail.data.is_active });
										}}
									>
										{detail.data.is_active ? "Deactivate" : "Activate verified domain"}
									</Button>
									<Button
										className="rounded-lg"
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
							</Row>
							<Row label={<Label htmlFor="domain-notes-edit">Notes</Label>}>
								<form
									className="flex w-full gap-2 md:justify-end"
									onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
										event.preventDefault();
										update.mutate({ notes: editNotes.trim() });
									}}
								>
									<Input
										className={field}
										id="domain-notes-edit"
										value={editNotes}
										onChange={(event) => {
											setEditNotes(event.target.value);
										}}
									/>
									<Button
										className="h-10 rounded-lg"
										type="submit"
										variant="outline"
										disabled={update.isPending}
									>
										Save notes
									</Button>
								</form>
							</Row>
						</Group>
					</Section>
				) : null}
				<Section
					description="New domains start inactive. Verify ownership before activating."
					title="Add a domain"
				>
					<form
						className="flex flex-col gap-4"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							create.mutate();
						}}
					>
						<Group>
							<Row label={<Label htmlFor="new-domain">Domain</Label>}>
								<Input
									className={field}
									id="new-domain"
									required
									placeholder="example.com"
									value={domain}
									onChange={(event) => {
										setDomain(event.target.value);
									}}
								/>
							</Row>
							<Row label={<Label htmlFor="new-domain-notes">Notes</Label>}>
								<Input
									className={field}
									id="new-domain-notes"
									value={notes}
									onChange={(event) => {
										setNotes(event.target.value);
									}}
								/>
							</Row>
						</Group>
						<div className="flex justify-end">
							<Button className="rounded-lg" type="submit" disabled={create.isPending}>
								Add inactive domain
							</Button>
						</div>
					</form>
				</Section>
				<Section
					description="Check the health of any domain, also one that is not in this workspace."
					title="Check any domain"
				>
					<form
						className="flex flex-col gap-2 sm:flex-row"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							setChecked(lookup.trim().toLowerCase());
						}}
					>
						<Label className="sr-only" htmlFor="reputation-domain">
							Domain to check
						</Label>
						<Input
							className={field}
							id="reputation-domain"
							required
							placeholder="example.com"
							value={lookup}
							onChange={(event) => {
								setLookup(event.target.value);
							}}
						/>
						<Button className="h-10 rounded-lg" type="submit">
							Check domain
						</Button>
					</form>
					{checked ? (
						<Group>
							<DomainHealth aria-label={`Health of ${checked}`} domain={checked} />
						</Group>
					) : null}
				</Section>
			</section>
		</Page>
	);
}
