"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { CommentsApi, Configuration, ListsApi, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { type ListDetailPanelPropsFields } from "./list-detail-panel.schema";

/** @oppulence-gen kind=component Owned by `list-detail-panel.lit.ts`. */
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
	const [exportId, setExportId] = useState<number>();
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
		mutationFn: async () =>
			z.object({ id: z.number() }).parse(
				(
					await listsApi.v1CreateRemediationExport({
						listId: selectedListId(),
						requestBody: { plan_id: plan.data?.id, partitions: ["safe_to_send"], format: "csv" },
					})
				).data,
			),
		onSuccess: (data) => {
			setExportId(data.id);
			setNotice("Safe-to-send export is ready.");
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
		mutationFn: async (kind: "cleaned" | "safe") => {
			const id = selectedListId();
			const response =
				kind === "cleaned"
					? await v1Api.v1DownloadList({ listId: id, format: "csv" }, { responseType: "blob" })
					: await listsApi.v1DownloadRemediationExport(
							{ listId: id, exportId: exportId ?? 0 },
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
		<section data-slot="list-detail-panel" className={cn("space-y-6", className)} {...props}>
			<Link href="/app/lists" className="text-sm underline">
				← All lists
			</Link>
			{!listId ? <p role="alert">Choose a list from the list index.</p> : null}
			{error ? (
				<p role="alert" className="text-destructive">
					{error.message}
				</p>
			) : null}
			{notice ? <p role="status">{notice}</p> : null}
			{detail.data ? (
				<>
					<header>
						<h1 className="text-3xl font-semibold">{detail.data.name}</h1>
						<p className="text-muted-foreground">
							{detail.data.total_rows} rows · {detail.data.status} · Email column:{" "}
							{detail.data.email_column}
						</p>
						<Link className="underline" href={`/app/jobs?job=${String(detail.data.job_id)}`}>
							Track verification job
						</Link>
					</header>
					<Card>
						<CardHeader>
							<CardTitle>Quality and downloads</CardTitle>
						</CardHeader>
						<CardContent className="space-y-4">
							{quality.data ? (
								<>
									<p className="text-xl font-semibold">Grade {quality.data.quality_grade}</p>
									<p>
										{quality.data.safe_to_send_count} safe to send ({quality.data.safe_to_send_pct}
										%) · {quality.data.processed} processed
									</p>
									<div className="flex flex-wrap gap-3">
										{Object.entries(quality.data.categories)
											.filter(([label]) => !label.endsWith("_pct"))
											.map(([label, count]) => (
												<span className="rounded border px-3 py-1 text-sm" key={label}>
													{label}: {count}
												</span>
											))}
									</div>
								</>
							) : null}
							<Button
								type="button"
								variant="outline"
								disabled={downloadFile.isPending}
								onClick={() => {
									downloadFile.mutate("cleaned");
								}}
							>
								Download cleaned CSV
							</Button>
						</CardContent>
					</Card>
					<Card>
						<CardHeader>
							<CardTitle>Remediation</CardTitle>
						</CardHeader>
						<CardContent className="space-y-4">
							<p className="text-sm text-muted-foreground">
								Review normalization, duplicate removal, suppression, and risky rows before export.
							</p>
							{plan.data ? (
								<>
									<p>
										Plan {plan.data.status} ·{" "}
										{Object.entries(plan.data.summary_counts)
											.map(([label, count]) => `${label}: ${String(count)}`)
											.join(" · ")}
									</p>
									{plan.data.preview_rows.length ? (
										<div className="max-h-72 overflow-auto">
											<table className="w-full text-sm">
												<thead>
													<tr>
														<th className="p-2 text-left">Original</th>
														<th className="p-2 text-left">Effective</th>
														<th className="p-2 text-left">Decision</th>
													</tr>
												</thead>
												<tbody>
													{plan.data.preview_rows.map((row) => (
														<tr className="border-t" key={row.id}>
															<td className="p-2">{row.original_email}</td>
															<td className="p-2">{row.effective_email}</td>
															<td className="p-2">{row.classification}</td>
														</tr>
													))}
												</tbody>
											</table>
										</div>
									) : null}
									<Button
										type="button"
										disabled={createExport.isPending}
										onClick={() => {
											createExport.mutate();
										}}
									>
										Prepare safe-to-send CSV
									</Button>
									{exportId ? (
										<Button
											type="button"
											variant="outline"
											disabled={downloadFile.isPending}
											onClick={() => {
												downloadFile.mutate("safe");
											}}
										>
											Download safe-to-send CSV
										</Button>
									) : null}
								</>
							) : (
								<Button
									type="button"
									disabled={createPlan.isPending}
									onClick={() => {
										createPlan.mutate();
									}}
								>
									Create remediation plan
								</Button>
							)}
						</CardContent>
					</Card>
					<Card>
						<CardHeader>
							<CardTitle>Team comments</CardTitle>
						</CardHeader>
						<CardContent className="space-y-4">
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
									id="list-comment"
									value={comment}
									onChange={(event) => {
										setComment(event.target.value);
									}}
									placeholder="Add a note about this list"
								/>
								<Button type="submit" disabled={addComment.isPending || !comment.trim()}>
									Post
								</Button>
							</form>
							{comments.data?.comments.length ? (
								<ul className="space-y-2">
									{comments.data.comments.map((item) => (
										<li
											className="flex items-start justify-between gap-3 rounded border p-3"
											key={item.id}
										>
											<div>
												<p>{item.body}</p>
												<p className="text-xs text-muted-foreground">
													{item.author || "Team member"} ·{" "}
													{new Date(item.created_at).toLocaleString()}
												</p>
											</div>
											<Button
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
										</li>
									))}
								</ul>
							) : (
								<p className="text-sm text-muted-foreground">No comments yet.</p>
							)}
						</CardContent>
					</Card>
					<div>
						<Button
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
	);
}
