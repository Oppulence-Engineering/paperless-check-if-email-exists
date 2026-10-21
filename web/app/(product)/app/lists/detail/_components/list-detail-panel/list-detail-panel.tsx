"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { CommentsApi, Configuration, ListsApi, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useRouter, useSearchParams } from "next/navigation";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@oppulence/ui/components/tabs";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";
import { JobProblems } from "@/components/features/lists/job-problems/job-problems";
import { JobProgress } from "@/components/features/lists/job-progress/job-progress";
import { JobReview, Stat } from "@/components/features/lists/job-review/job-review";
import { JobRows } from "@/components/features/lists/job-rows/job-rows";
import { ListCompare } from "@/components/features/lists/list-compare/list-compare";
import { ListExport } from "@/components/features/lists/list-export/list-export";
import { useV1ListLists } from "@/hooks/queries/use-v1-list-lists";
import { v1ListListsKeys } from "@/hooks/queries/utils/v1-list-lists-keys";
import { V1GetList200Response } from "@/lib/api/generated/zod/v1/v1";
import { ACTIVE_LIST_STATES, ListQualitySchema } from "@/lib/api/schemas/lists";
import { category, listStatus } from "@/lib/verification/labels";

import { type ListDetailPanelPropsFields } from "./list-detail-panel.schema";

/** @oppulence-gen kind=component Owned by `list-detail-panel.lit.ts`. */
export type ListDetailPanelProps = ListDetailPanelPropsFields & ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const listsApi = new ListsApi(config);
const commentsApi = new CommentsApi(config);
const v1Api = new V1Api(config);
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

/** Risk counters in the quality report (backend/src/http/v1/lists/quality.rs). */
const RISKS = {
	disposable: "Disposable",
	catch_all: "Catch-all",
	role_account: "Role address",
	spam_trap: "Spam trap",
	full_inbox: "Full inbox",
} as const;

/** The segmented control style of the product pages, applied to the tabs. */
const TAB_LIST_CLASS =
	"h-auto w-full flex-wrap justify-start rounded-lg bg-white/[0.03] p-0.5 ring-1 ring-white/[0.08] sm:w-fit";
const TAB_CLASS =
	"flex-none rounded-md border-0 px-3 py-1 text-[#858585] after:hidden hover:text-neutral-200 data-[state=active]:bg-white/[0.08] data-[state=active]:text-white";

export function ListDetailPanel({ listId, accepted, className, ...props }: ListDetailPanelProps) {
	const client = useQueryClient();
	const router = useRouter();
	const searchParams = useSearchParams();
	const tab = searchParams.get("tab") ?? "review";
	const [comment, setComment] = useState("");
	const key = ["list-detail", listId];
	function selectedListId(): number {
		if (!listId) throw new Error("Select a list first.");
		return listId;
	}
	const detail = useQuery({
		queryKey: workflowQueryKey(...key, "detail"),
		enabled: !!listId,
		queryFn: async () =>
			V1GetList200Response.parse((await v1Api.v1GetList({ listId: selectedListId() })).data),
		refetchInterval: (query) =>
			ACTIVE_LIST_STATES.includes(query.state.data?.status ?? "") ? 5_000 : false,
	});
	// ponytail: the list detail has no file name, so the page finds it in the first 100 lists.
	const file = useV1ListLists({ limit: 100 }).data?.lists.find(
		(list) => list.id === listId,
	)?.original_filename;
	const quality = useQuery({
		queryKey: workflowQueryKey(...key, "quality"),
		enabled: !!listId,
		queryFn: async () =>
			ListQualitySchema.parse((await listsApi.v1ListQuality({ listId: selectedListId() })).data),
		refetchInterval: (query) => {
			const data = query.state.data;
			return data && ACTIVE_LIST_STATES.includes(detail.data?.status ?? "") ? 5_000 : false;
		},
	});
	const comments = useQuery({
		queryKey: workflowQueryKey(...key, "comments"),
		enabled: !!listId,
		queryFn: async () =>
			commentsSchema.parse(
				(
					await commentsApi.v1ListComments({
						listId: selectedListId(),
						limit: 50,
					})
				).data,
			),
	});
	const addComment = useMutation({
		mutationFn: async () => {
			await commentsApi.v1CreateComment({
				createCommentRequest: {
					list_id: selectedListId(),
					body: comment.trim(),
				},
			});
		},
		onSuccess: async () => {
			setComment("");
			await client.invalidateQueries({
				queryKey: workflowQueryKey(...key, "comments"),
			});
		},
	});
	const removeComment = useMutation({
		mutationFn: async (commentId: number) => {
			await commentsApi.v1DeleteComment({ commentId });
		},
		onSuccess: async () => {
			await client.invalidateQueries({
				queryKey: workflowQueryKey(...key, "comments"),
			});
		},
	});
	const removeList = useMutation({
		mutationFn: async () => {
			await v1Api.v1DeleteList({ listId: selectedListId() });
		},
		onSuccess: async () => {
			await client.invalidateQueries({ queryKey: v1ListListsKeys.all });
			router.push("/app/lists");
		},
	});
	const error = [detail, quality, comments, addComment, removeComment, removeList].find(
		(request) => request.isError,
	)?.error;
	const list = detail.data;

	return (
		<Page
			actions={
				<Link href="/app/lists" className="text-sm text-[#858585] hover:text-neutral-200">
					← All lists
				</Link>
			}
			description={
				list
					? [
							file,
							`Email column: ${list.email_column}`,
							list.source_key && `Source: ${list.source_key}`,
						]
							.filter(Boolean)
							.join(" · ")
					: undefined
			}
			title={list?.name ?? "List workspace"}
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
						{errorMessage(error)}
					</p>
				) : null}
				{accepted && ACTIVE_LIST_STATES.includes(detail.data?.status ?? "") ? (
					<p
						role="status"
						className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
					>
						List accepted. The check of each address has started.
					</p>
				) : null}
				{list ? (
					<>
						<Group>
							<Row>
								<div className="flex flex-wrap items-center gap-3">
									<Pill tone={listStatus.tone(list.status)}>{listStatus.label(list.status)}</Pill>
									<Link
										className="inline-flex items-center text-xs text-[#858585] underline-offset-4 hover:text-neutral-200 hover:underline"
										href={`/app/jobs?job=${String(list.job_id)}`}
									>
										Bulk job {list.job_id}
									</Link>
								</div>
								<div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
									<Stat label="Rows" value={list.total_rows} />
									<Stat label="Unique addresses" value={list.unique_emails ?? "—"} />
									<Stat label="Duplicates removed" value={list.deduplicated_count ?? 0} />
									<Stat
										label="Rejected rows"
										value={
											list.rejected_rows ??
											(list.unique_emails == null
												? "—"
												: Math.max(
														0,
														list.total_rows - list.unique_emails - (list.deduplicated_count ?? 0),
													))
										}
									/>
									<Stat label="Rows processed" value={list.summary.total_processed} />
								</div>
								<JobProgress jobId={list.job_id} />
							</Row>
						</Group>
						<Tabs
							value={tab}
							onValueChange={(value) => {
								const params = new URLSearchParams(searchParams);
								params.set("tab", value);
								router.replace(`?${params.toString()}`, { scroll: false });
							}}
						>
							<TabsList aria-label="List views" className={TAB_LIST_CLASS}>
								<TabsTrigger className={TAB_CLASS} value="review">
									Review
								</TabsTrigger>
								<TabsTrigger className={TAB_CLASS} value="rows">
									Rows
								</TabsTrigger>
								<TabsTrigger className={TAB_CLASS} value="export">
									Clean &amp; export
								</TabsTrigger>
								<TabsTrigger className={TAB_CLASS} value="problems">
									Problems
								</TabsTrigger>
								<TabsTrigger className={TAB_CLASS} value="compare">
									Compare
								</TabsTrigger>
							</TabsList>
							<TabsContent className="mt-6 flex flex-col gap-10" value="review">
								<div className="flex flex-col gap-4">
									<h2 className="text-lg font-medium text-white">List quality</h2>
									<Group>
										{quality.data ? (
											<Row
												description={`Average score ${String(quality.data.avg_score)} · ${String(quality.data.safe_to_send_count)} safe to send (${String(quality.data.safe_to_send_pct)}%) · ${String(quality.data.processed)} unique ${quality.data.processed === 1 ? "address" : "addresses"} checked`}
												label={
													quality.data.quality_grade === "pending"
														? "Checks pending"
														: `Grade ${quality.data.quality_grade}`
												}
											/>
										) : null}
										<Row>
											<h3 className="text-sm font-medium text-white">Categories</h3>
											<div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
												{(["valid", "risky", "unknown", "invalid"] as const).map((value) => (
													<Stat
														key={value}
														label={category.label(value)}
														value={quality.data?.categories[value] ?? "—"}
													/>
												))}
											</div>
										</Row>
										{quality.data ? (
											<Row>
												<h3 className="text-sm font-medium text-white">Risk breakdown</h3>
												<div className="grid grid-cols-2 gap-3 sm:grid-cols-5">
													{Object.entries(RISKS).map(([risk, label]) => (
														<Stat
															key={risk}
															label={label}
															value={quality.data.risk_breakdown[risk] ?? 0}
														/>
													))}
												</div>
											</Row>
										) : null}
									</Group>
								</div>
								<JobReview jobId={list.job_id} />
							</TabsContent>
							<TabsContent className="mt-6" value="rows">
								<JobRows jobId={list.job_id} />
							</TabsContent>
							<TabsContent className="mt-6" value="export">
								<ListExport listId={list.id} />
							</TabsContent>
							<TabsContent className="mt-6" value="problems">
								<JobProblems jobId={list.job_id} />
							</TabsContent>
							<TabsContent className="mt-6" value="compare">
								{list.status === "completed" ? (
									<ListCompare listId={list.id} />
								) : (
									<p className="text-sm text-[#858585]">
										You can compare this list when all its rows are checked.
									</p>
								)}
							</TabsContent>
						</Tabs>
						<Section
							title="Ongoing list health"
							description="Compare refreshed lists to find addresses that became invalid or risky. Scheduled checks cover stale addresses across your workspace and appear as separate jobs."
						>
							<div className="flex flex-wrap gap-4 text-sm text-neutral-300">
								<Link
									className="underline underline-offset-4"
									href="/app/settings?settings=verification#reverification"
								>
									Manage scheduled checks
								</Link>
								<Link className="underline underline-offset-4" href="/app/outcomes">
									Record bounces and complaints
								</Link>
								<Link className="underline underline-offset-4" href="/app/analytics">
									View lead source reports
								</Link>
							</div>
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
