"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { CommentsApi, Configuration, ListsApi, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";

import { type ListDetailPanelPropsFields } from "./list-detail-panel.schema";

/** @oppulence-gen kind=component Owned by `list-detail-panel.lit.ts`. */
/** Remediation export files the backend can build (backend/src/http/v1/lists/remediation.rs). */
const EXPORT_PARTITIONS = {
	safe_to_send: "Send-ready (safe and fixed)",
	safe: "Safe",
	fixed: "Fixed",
	review: "Review",
	drop: "Drop",
	all: "All rows",
} as const;

type ExportPartition = keyof typeof EXPORT_PARTITIONS;

export type ListDetailPanelProps = ListDetailPanelPropsFields & ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const listsApi = new ListsApi(config);
const commentsApi = new CommentsApi(config);
const v1Api = new V1Api(config);
const qualitySchema = z.object({
	processed: z.number(),
	quality_grade: z.string(),
	safe_to_send_count: z.number(),
	safe_to_send_pct: z.number(),
	categories: z.record(z.string(), z.number()),
});
const planSchema = z.object({
	id: z.number(),
	status: z.string(),
	summary_counts: z.record(z.string(), z.number()),
	preview_rows: z.array(
		z.object({
			id: z.number(),
			classification: z.string(),
			original_email: z.string(),
			effective_email: z.string(),
		}),
	),
});
const commentsSchema = z.object({
	comments: z.array(
		z.object({
			id: z.number(),
			body: z.string(),
			author: z.string().nullable().optional(),
			created_at: z.string(),
		}),
	),
});

function download(data: unknown, filename: string) {
	const blob = data instanceof Blob ? data : new Blob([data as BlobPart]);
	const url = URL.createObjectURL(blob);
	const link = document.createElement("a");
	link.href = url;
	link.download = filename;
	link.click();
	setTimeout(() => {
		URL.revokeObjectURL(url);
	}, 1_000);
}

export function ListDetailPanel({ listId, className, ...props }: ListDetailPanelProps) {
	const client = useQueryClient();
	const [comment, setComment] = useState("");
	const [notice, setNotice] = useState("");
	const [partition, setPartition] = useState<ExportPartition>("safe_to_send");
	const [exportIds, setExportIds] = useState<Partial<Record<ExportPartition, number>>>({});
	const key = ["list-detail", listId];
	function selectedListId(): number {
		if (!listId) throw new Error("Select a list first.");
		return listId;
	}
	const detail = useQuery({
		queryKey: workflowQueryKey(...key, "detail"),
		enabled: !!listId,
		queryFn: async () => (await v1Api.v1GetList({ listId: selectedListId() })).data,
	});
	const quality = useQuery({
		queryKey: workflowQueryKey(...key, "quality"),
		enabled: !!listId,
		queryFn: async () =>
			qualitySchema.parse((await listsApi.v1ListQuality({ listId: selectedListId() })).data),
	});
	const plan = useQuery({
		queryKey: workflowQueryKey(...key, "plan"),
		enabled: !!listId,
		retry: false,
		queryFn: async () =>
			planSchema.parse((await listsApi.v1GetRemediationPlan({ listId: selectedListId() })).data),
	});
	const comments = useQuery({
		queryKey: workflowQueryKey(...key, "comments"),
		enabled: !!listId,
		queryFn: async () =>
			commentsSchema.parse(
				(await commentsApi.v1ListComments({ listId: selectedListId(), limit: 50 })).data,
			),
	});
	const createPlan = useMutation({
		mutationFn: async () =>
			planSchema.parse(
				(await listsApi.v1CreateRemediationPlan({ listId: selectedListId(), requestBody: {} }))
					.data,
			),
		onSuccess: async () => {
			setNotice("Remediation plan created.");
			await client.invalidateQueries({ queryKey: workflowQueryKey(...key, "plan") });
		},
	});
	const createExport = useMutation({
		mutationFn: async (selected: ExportPartition) => ({
			selected,
			id: z.object({ id: z.number() }).parse(
				(
					await listsApi.v1CreateRemediationExport({
						listId: selectedListId(),
						requestBody: { plan_id: plan.data?.id, partitions: [selected], format: "csv" },
					})
				).data,
			).id,
		}),
		onSuccess: ({ selected, id }) => {
			setExportIds((current) => ({ ...current, [selected]: id }));
			setNotice(`${EXPORT_PARTITIONS[selected]} file is ready.`);
		},
	});
	const addComment = useMutation({
		mutationFn: async () => {
			await commentsApi.v1CreateComment({
				createCommentRequest: { list_id: selectedListId(), body: comment.trim() },
			});
		},
		onSuccess: async () => {
			setComment("");
			await client.invalidateQueries({ queryKey: workflowQueryKey(...key, "comments") });
		},
	});
	const removeComment = useMutation({
		mutationFn: async (commentId: number) => {
			await commentsApi.v1DeleteComment({ commentId });
		},
		onSuccess: async () => {
			await client.invalidateQueries({ queryKey: workflowQueryKey(...key, "comments") });
		},
	});
	const removeList = useMutation({
		mutationFn: async () => {
			await v1Api.v1DeleteList({ listId: selectedListId() });
		},
		onSuccess: async () => {
			setNotice("List deleted.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("lists") });
		},
	});
	const downloadFile = useMutation({
		mutationFn: async (kind: "cleaned" | ExportPartition) => {
			const id = selectedListId();
			const response =
				kind === "cleaned"
					? await v1Api.v1DownloadList({ listId: id, format: "csv" }, { responseType: "blob" })
					: await listsApi.v1DownloadRemediationExport(
							{ listId: id, exportId: exportIds[kind] ?? 0 },
							{ responseType: "blob" },
						);
			download(response.data, `list-${String(id)}-${kind}.csv`);
		},
	});
	const error = [
		detail,
		quality,
		comments,
		createPlan,
		createExport,
		addComment,
		removeComment,
		removeList,
		downloadFile,
	].find((request) => request.isError)?.error;

	return (
		<Page
			actions={
				<Link href="/app/lists" className="text-sm text-[#858585] hover:text-neutral-200">
					← All lists
				</Link>
			}
			description={
				detail.data ? (
					<>
						{detail.data.total_rows} rows · {detail.data.status} · Email column:{" "}
						{detail.data.email_column} ·{" "}
						<Link
							className="underline hover:text-neutral-200"
							href={`/app/jobs?job=${String(detail.data.job_id)}`}
						>
							Track verification job
						</Link>
					</>
				) : undefined
			}
			title={detail.data?.name ?? "List detail"}
		>
			<section
				data-slot="list-detail-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				{!listId ? (
					<p
						role="alert"
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					>
						Choose a list from the list index.
					</p>
				) : null}
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
				{detail.data ? (
					<>
						<Section
							actions={
								<Button
									className="rounded-lg"
									size="sm"
									type="button"
									variant="outline"
									disabled={downloadFile.isPending}
									onClick={() => {
										downloadFile.mutate("cleaned");
									}}
								>
									Download cleaned CSV
								</Button>
							}
							title="Quality and downloads"
						>
							{quality.data ? (
								<Group>
									<Row
										description={`${String(quality.data.safe_to_send_count)} safe to send (${String(quality.data.safe_to_send_pct)}%) · ${String(quality.data.processed)} processed`}
										label={`Grade ${quality.data.quality_grade}`}
									>
										<div className="flex flex-wrap gap-2 md:justify-end">
											{Object.entries(quality.data.categories)
												.filter(([label]) => !label.endsWith("_pct"))
												.map(([label, count]) => (
													<Pill key={label}>
														{label}: {count}
													</Pill>
												))}
										</div>
									</Row>
								</Group>
							) : null}
						</Section>
						<Section
							description="Review normalization, duplicate removal, suppression, and risky rows before export."
							title="Remediation"
						>
							<Group>
								{plan.data ? (
									<>
										<Row
											description={Object.entries(plan.data.summary_counts)
												.map(([label, count]) => `${label}: ${String(count)}`)
												.join(" · ")}
											label={`Plan ${plan.data.status}`}
										/>
										{plan.data.preview_rows.length ? (
											<Row>
												<div className="max-h-72 overflow-auto">
													<table className="w-full text-left text-sm">
														<thead>
															<tr className="border-b border-border">
																<th className="py-2 pr-4 font-medium text-foreground">Original</th>
																<th className="py-2 pr-4 font-medium text-foreground">Effective</th>
																<th className="py-2 font-medium text-foreground">Decision</th>
															</tr>
														</thead>
														<tbody>
															{plan.data.preview_rows.map((row) => (
																<tr className="border-b border-border/60" key={row.id}>
																	<td className="py-2 pr-4 text-foreground">
																		{row.original_email}
																	</td>
																	<td className="py-2 pr-4 text-muted-foreground">
																		{row.effective_email}
																	</td>
																	<td className="py-2 text-muted-foreground">
																		{row.classification}
																	</td>
																</tr>
															))}
														</tbody>
													</table>
												</div>
											</Row>
										) : null}
										<Row label={<Label htmlFor="export-partition">Export file</Label>}>
											<select
												className="h-10 w-full rounded-lg bg-white/[0.03] px-2 text-sm text-neutral-200 ring-1 ring-white/[0.08] outline-none md:w-72"
												id="export-partition"
												onChange={(event) => {
													setPartition(event.target.value as ExportPartition);
												}}
												value={partition}
											>
												{Object.entries(EXPORT_PARTITIONS).map(([value, label]) => (
													<option key={value} value={value}>
														{label}
													</option>
												))}
											</select>
										</Row>
										<Row>
											<div className="flex flex-wrap justify-end gap-2">
												{exportIds[partition] ? (
													<Button
														className="rounded-lg"
														type="button"
														variant="outline"
														disabled={downloadFile.isPending}
														onClick={() => {
															downloadFile.mutate(partition);
														}}
													>
														Download {EXPORT_PARTITIONS[partition].toLowerCase()} CSV
													</Button>
												) : null}
												<Button
													className="rounded-lg"
													type="button"
													disabled={createExport.isPending}
													onClick={() => {
														createExport.mutate(partition);
													}}
												>
													Prepare CSV
												</Button>
											</div>
										</Row>
									</>
								) : (
									<Row>
										<div className="flex justify-end">
											<Button
												className="rounded-lg"
												type="button"
												disabled={createPlan.isPending}
												onClick={() => {
													createPlan.mutate();
												}}
											>
												Create remediation plan
											</Button>
										</div>
									</Row>
								)}
							</Group>
						</Section>
						<Section title="Team comments">
							<form
								className="flex gap-2"
								onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
									event.preventDefault();
									if (comment.trim()) addComment.mutate();
								}}
							>
								<Label className="sr-only" htmlFor="list-comment">
									Comment
								</Label>
								<Input
									className="h-10 rounded-lg"
									id="list-comment"
									value={comment}
									onChange={(event) => {
										setComment(event.target.value);
									}}
									placeholder="Add a note about this list"
								/>
								<Button
									className="h-10 rounded-lg"
									type="submit"
									disabled={addComment.isPending || !comment.trim()}
								>
									Post
								</Button>
							</form>
							<Group>
								{comments.data?.comments.length ? (
									comments.data.comments.map((item) => (
										<div
											className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
											key={item.id}
										>
											<div className="min-w-0 flex-1">
												<p className="text-sm text-white">{item.body}</p>
												<p className="text-xs text-[#858585]">
													{item.author || "Team member"} ·{" "}
													{new Date(item.created_at).toLocaleString()}
												</p>
											</div>
											<Button
												className="text-red-400 hover:text-red-300"
												type="button"
												size="sm"
												variant="ghost"
												disabled={removeComment.isPending}
												onClick={() => {
													if (window.confirm("Delete this comment?")) removeComment.mutate(item.id);
												}}
											>
												Delete
											</Button>
										</div>
									))
								) : (
									<Row>
										<p className="text-sm text-[#858585]">No comments yet.</p>
									</Row>
								)}
							</Group>
						</Section>
						<div className="flex justify-end">
							<Button
								className="rounded-lg"
								type="button"
								variant="destructive"
								disabled={removeList.isPending || removeList.isSuccess}
								onClick={() => {
									if (window.confirm("Delete this list and its saved data?")) removeList.mutate();
								}}
							>
								Delete list
							</Button>
						</div>
					</>
				) : null}
			</section>
		</Page>
	);
}
