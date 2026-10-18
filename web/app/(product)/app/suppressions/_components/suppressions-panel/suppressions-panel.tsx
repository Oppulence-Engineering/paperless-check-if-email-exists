"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";

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
		<Page
			description="Keep addresses out of sends and inspect why each entry was added."
			title="Suppressions"
		>
			<section
				data-slot="suppressions-panel"
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
					actions={
						<Button
							className="rounded-lg"
							size="sm"
							variant="outline"
							disabled={exportCsv.isPending}
							onClick={() => {
								exportCsv.mutate();
							}}
						>
							Export CSV
						</Button>
					}
					title="Suppressed addresses"
				>
					<Group>
						{entries.isPending ? (
							<Row>
								<p className="text-sm text-[#858585]" role="status">
									Loading suppressions…
								</p>
							</Row>
						) : null}
						{entries.data?.entries.length ? (
							<>
								<div className="px-5 py-3 text-xs font-medium text-[#858585]">
									{entries.data.total} total
								</div>
								{entries.data.entries.map((entry) => (
									<div
										className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
										key={entry.id}
									>
										<div className="min-w-0 flex-1">
											<p className="truncate text-sm text-white">{entry.email}</p>
											<p className="text-xs text-[#858585] capitalize">{entry.reason}</p>
										</div>
										<Pill>{entry.status}</Pill>
										<div className="flex items-center gap-2">
											<Button
												size="sm"
												variant="secondary"
												onClick={() => {
													setSelectedId(entry.id);
												}}
											>
												Audit
											</Button>
											<Button
												className="text-red-400 hover:text-red-300"
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
										</div>
									</div>
								))}
							</>
						) : entries.isPending ? null : (
							<Row>
								<p className="text-sm text-[#858585]">No suppressions found.</p>
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
							disabled={!entries.data || offset + 20 >= entries.data.total}
							onClick={() => {
								setOffset(offset + 20);
							}}
						>
							Next
						</Button>
					</div>
				</Section>
				{selectedId ? (
					<Section title={`Decision history for entry ${String(selectedId)}`}>
						<Group>
							{events.data?.events.length ? (
								events.data.events.map((event) => (
									<div className="px-5 py-4 text-sm text-white" key={event.id}>
										{event.event_type}{" "}
										<span className="text-xs text-[#858585]">
											· {event.actor_type} · {new Date(event.created_at).toLocaleString()}
										</span>
									</div>
								))
							) : (
								<Row>
									<p className="text-sm text-[#858585]">No events recorded.</p>
								</Row>
							)}
						</Group>
					</Section>
				) : null}
				<form
					onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
						event.preventDefault();
						setCheckEmail(checkInput.trim());
					}}
				>
					<Group>
						<Row
							description={
								check.data ? (
									<span role="status">
										{check.data.suppressed ? "Suppressed" : "Not suppressed"}
										{check.data.reason ? ` · ${check.data.reason}` : ""}
									</span>
								) : undefined
							}
							label="Check an address"
						>
							<Label className="sr-only" htmlFor="suppression-email">
								Email address
							</Label>
							<Input
								className="h-10 rounded-lg"
								id="suppression-email"
								type="email"
								required
								value={checkInput}
								onChange={(event) => {
									setCheckInput(event.target.value);
								}}
							/>
							<Button className="h-10 rounded-lg" type="submit">
								Check
							</Button>
						</Row>
					</Group>
				</form>
				<Section title="Add or import">
					<Group>
						<Row
							label={<Label htmlFor="suppression-batch">Addresses, one per line</Label>}
							layout="stacked"
						>
							<textarea
								id="suppression-batch"
								aria-label="Addresses"
								className="min-h-32 w-full rounded-lg border border-input bg-transparent px-3 py-2 text-sm dark:bg-input/30"
								value={batch}
								onChange={(event) => {
									setBatch(event.target.value);
								}}
							/>
						</Row>
						<Row label={<Label htmlFor="suppression-reason">Reason</Label>}>
							<select
								id="suppression-reason"
								className="h-10 w-full rounded-lg bg-white/[0.03] px-2 text-sm text-neutral-200 ring-1 ring-white/[0.08] outline-none md:w-72"
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
						</Row>
					</Group>
					<div className="flex justify-end gap-2">
						<Button
							className="rounded-lg"
							variant="outline"
							disabled={save.isPending || !batch.trim()}
							onClick={() => {
								save.mutate("import");
							}}
						>
							Import batch
						</Button>
						<Button
							className="rounded-lg"
							disabled={save.isPending || !batch.trim()}
							onClick={() => {
								save.mutate("add");
							}}
						>
							Add
						</Button>
					</div>
				</Section>
			</section>
		</Page>
	);
}
