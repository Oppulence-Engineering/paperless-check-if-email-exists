"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { type SuppressionsPanelPropsFields } from "./suppressions-panel.schema";

/** @oppulence-gen kind=component Owned by `suppressions-panel.lit.ts`. */
export type SuppressionsPanelProps = SuppressionsPanelPropsFields &
	ComponentPropsWithoutRef<"section">;

const api = new V1Api(new Configuration({ basePath: "/api/backend" }));
const reasonSchema = z.enum(["manual", "bounce", "complaint", "unsubscribe"]);
const entriesSchema = z.object({
	entries: z.array(
		z.object({
			id: z.number(),
			email: z.string(),
			status: z.string(),
			reason: z.string(),
			source: z.string().nullable().optional(),
			created_at: z.string(),
		}),
	),
	total: z.number(),
});
const eventsSchema = z.object({
	events: z.array(
		z.object({
			id: z.number(),
			event_type: z.string(),
			actor_type: z.string(),
			created_at: z.string(),
		}),
	),
	total: z.number(),
});

export function SuppressionsPanel({ className, ...props }: SuppressionsPanelProps) {
	const client = useQueryClient();
	const [offset, setOffset] = useState(0);
	const [checkInput, setCheckInput] = useState("");
	const [checkEmail, setCheckEmail] = useState("");
	const [batch, setBatch] = useState("");
	const [reason, setReason] = useState<z.infer<typeof reasonSchema>>("manual");
	const [selectedId, setSelectedId] = useState<number>();
	const [notice, setNotice] = useState("");
	const entries = useQuery({
		queryKey: workflowQueryKey("suppressions", offset),
		queryFn: async () =>
			entriesSchema.parse((await api.v1ListSuppressions({ limit: 20, offset })).data),
	});
	const check = useQuery({
		queryKey: workflowQueryKey("suppression-check", checkEmail),
		enabled: !!checkEmail,
		queryFn: async () => (await api.v1CheckSuppression({ email: checkEmail })).data,
	});
	const events = useQuery({
		queryKey: workflowQueryKey("suppression-events", selectedId),
		enabled: !!selectedId,
		queryFn: async () => {
			if (!selectedId) throw new Error("Select an entry.");
			return eventsSchema.parse((await api.v1ListSuppressionEvents({ id: selectedId })).data);
		},
	});
	const save = useMutation({
		mutationFn: async (kind: "add" | "import") => {
			const emails = batch
				.split(/[\s,;]+/)
				.map((value) => value.trim())
				.filter(Boolean);
			if (!emails.length) throw new Error("Enter at least one email address.");
			if (kind === "import")
				return (await api.v1ImportSuppressions({ requestBody: { emails, reason } })).data;
			return (await api.v1AddSuppressions({ addSuppressionsRequest: { emails, reason } })).data;
		},
		onSuccess: async () => {
			setBatch("");
			setNotice("Suppression entries saved.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("suppressions") });
		},
	});
	const remove = useMutation({
		mutationFn: async (id: number) => {
			await api.v1DeleteSuppression({ id });
		},
		onSuccess: async () => {
			setSelectedId(undefined);
			setNotice("Suppression removed.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("suppressions") });
		},
	});
	const exportCsv = useMutation({
		mutationFn: async () => {
			const response = await api.v1ExportSuppressions({ responseType: "blob" });
			const blob = response.data instanceof Blob ? response.data : new Blob([response.data]);
			const url = URL.createObjectURL(blob);
			const link = document.createElement("a");
			link.href = url;
			link.download = "suppressions.csv";
			link.click();
			setTimeout(() => {
				URL.revokeObjectURL(url);
			}, 1_000);
		},
	});
	const error = [entries, check, events, save, remove, exportCsv].find(
		(request) => request.isError,
	)?.error;
	return (
		<section data-slot="suppressions-panel" className={cn("space-y-6", className)} {...props}>
			<header>
				<h1 className="text-3xl font-semibold">Suppressions</h1>
				<p className="mt-2 text-muted-foreground">
					Keep addresses out of sends and inspect why each entry was added.
				</p>
			</header>
			{error ? (
				<p role="alert" className="text-destructive">
					{error.message}
				</p>
			) : null}
			{notice ? <p role="status">{notice}</p> : null}
			<Card>
				<CardHeader>
					<CardTitle>Suppressed addresses</CardTitle>
				</CardHeader>
				<CardContent className="space-y-4">
					{entries.isPending ? <p role="status">Loading suppressions…</p> : null}
					{entries.data?.entries.length ? (
						<div className="overflow-x-auto">
							<table className="w-full text-sm">
								<thead>
									<tr>
										<th className="p-2 text-left">Address</th>
										<th className="p-2 text-left">Reason</th>
										<th className="p-2 text-left">Status</th>
										<th className="p-2 text-left">Actions</th>
									</tr>
								</thead>
								<tbody>
									{entries.data.entries.map((entry) => (
										<tr className="border-t" key={entry.id}>
											<td className="p-2">{entry.email}</td>
											<td className="p-2">{entry.reason}</td>
											<td className="p-2">{entry.status}</td>
											<td className="flex gap-2 p-2">
												<Button
													size="sm"
													variant="outline"
													onClick={() => {
														setSelectedId(entry.id);
													}}
												>
													Audit
												</Button>
												<Button
													size="sm"
													variant="ghost"
													disabled={remove.isPending}
													onClick={() => {
														if (window.confirm(`Remove ${entry.email} from suppressions?`))
															remove.mutate(entry.id);
													}}
												>
													Remove
												</Button>
											</td>
										</tr>
									))}
								</tbody>
							</table>
						</div>
					) : (
						<p>No suppressions found.</p>
					)}
					<div className="flex items-center gap-3">
						<span>{entries.data?.total ?? 0} total</span>
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
							disabled={!entries.data || offset + 20 >= entries.data.total}
							onClick={() => {
								setOffset(offset + 20);
							}}
						>
							Next
						</Button>
						<Button
							size="sm"
							variant="outline"
							disabled={exportCsv.isPending}
							onClick={() => {
								exportCsv.mutate();
							}}
						>
							Export CSV
						</Button>
					</div>
				</CardContent>
			</Card>
			{selectedId ? (
				<Card>
					<CardHeader>
						<CardTitle>Decision history for entry {String(selectedId)}</CardTitle>
					</CardHeader>
					<CardContent>
						{events.data?.events.length ? (
							<ol className="space-y-2">
								{events.data.events.map((event) => (
									<li className="rounded border p-2 text-sm" key={event.id}>
										{event.event_type} · {event.actor_type} ·{" "}
										{new Date(event.created_at).toLocaleString()}
									</li>
								))}
							</ol>
						) : (
							<p>No events recorded.</p>
						)}
					</CardContent>
				</Card>
			) : null}
			<Card>
				<CardHeader>
					<CardTitle>Check an address</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					<form
						className="flex gap-2"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							setCheckEmail(checkInput.trim());
						}}
					>
						<Label className="sr-only" htmlFor="suppression-email">
							Email address
						</Label>
						<Input
							id="suppression-email"
							type="email"
							required
							value={checkInput}
							onChange={(event) => {
								setCheckInput(event.target.value);
							}}
						/>
						<Button type="submit">Check</Button>
					</form>
					{check.data ? (
						<p role="status">
							{check.data.suppressed ? "Suppressed" : "Not suppressed"}
							{check.data.reason ? ` · ${check.data.reason}` : ""}
						</p>
					) : null}
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Add or import</CardTitle>
				</CardHeader>
				<CardContent className="space-y-3">
					<Label htmlFor="suppression-batch">Addresses, one per line</Label>
					<textarea
						id="suppression-batch"
						aria-label="Addresses"
						className="min-h-32 w-full rounded border bg-background p-3"
						value={batch}
						onChange={(event) => {
							setBatch(event.target.value);
						}}
					/>
					<Label htmlFor="suppression-reason">Reason</Label>
					<select
						id="suppression-reason"
						className="h-9 w-full rounded border bg-background px-3"
						value={reason}
						onChange={(event) => {
							const parsed = reasonSchema.safeParse(event.target.value);
							if (parsed.success) setReason(parsed.data);
						}}
					>
						<option value="manual">Manual</option>
						<option value="bounce">Bounce</option>
						<option value="complaint">Complaint</option>
						<option value="unsubscribe">Unsubscribe</option>
					</select>
					<div className="flex gap-2">
						<Button
							disabled={save.isPending || !batch.trim()}
							onClick={() => {
								save.mutate("add");
							}}
						>
							Add
						</Button>
						<Button
							variant="outline"
							disabled={save.isPending || !batch.trim()}
							onClick={() => {
								save.mutate("import");
							}}
						>
							Import batch
						</Button>
					</div>
				</CardContent>
			</Card>
		</section>
	);
}
