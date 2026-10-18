"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, ListsApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { describeFilters } from "@/components/features/insights/result-explorer/result-explorer";
import { type ExplorerFilters } from "@/components/features/insights/result-explorer/result-explorer.schema";
import { Group } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	V1CreateSavedSegmentBody,
	V1GetSavedSegment200Response,
	V1ListSavedSegments200Response,
	V1UpdateSavedSegmentBody,
} from "@/lib/api/generated/zod/lists/lists";

import { type SavedSegmentsPropsFields } from "./saved-segments.schema";

/**
 * @oppulence-gen kind=component
 * SavedSegments is a client data component.
 * Saves, opens, renames, and deletes saved segments of explorer filters.
 *
 * It uses /v1/segments through the generated SDK and TanStack Query. The
 * segments have the `tenant` scope, because the explorer reads all results of
 * the workspace. Owned by `saved-segments.lit.ts`.
 */
export type SavedSegmentsProps = SavedSegmentsPropsFields & {
	onOpen: (filters: ExplorerFilters) => void;
} & ComponentPropsWithoutRef<"section">;

const api = new ListsApi(new Configuration({ basePath: "/api/backend" }));
const PAGE = 20;
const SCOPE = "tenant";
/** A segment keeps only these policy decisions (`validate_segment_filter` in list_intelligence.rs). */
const SEGMENT_DECISIONS = ["send", "review", "suppress"];

/** The segment filter for the explorer filters, and the filters that a segment cannot keep. */
export function toSegmentFilter(filters: ExplorerFilters) {
	const decision =
		filters.policyDecision && SEGMENT_DECISIONS.includes(filters.policyDecision)
			? filters.policyDecision
			: undefined;
	const filter = {
		category: filters.category,
		safe_to_send: filters.safeToSend,
		score_min: filters.minScore,
		score_max: filters.maxScore,
		policy_decision: decision,
	};
	const skipped = describeFilters({
		...filters,
		category: undefined,
		safeToSend: undefined,
		minScore: undefined,
		maxScore: undefined,
		policyDecision: decision ? undefined : filters.policyDecision,
	});
	return { filter, skipped };
}

/** The explorer filters in a segment filter. The explorer has no filter for the other keys. */
export function fromSegmentFilter(value: unknown): ExplorerFilters {
	const parsed = V1GetSavedSegment200Response.shape.filter.safeParse(value);
	if (!parsed.success) return {};
	const filter = parsed.data;
	return {
		// ponytail: GET /v1/query takes one category, so a segment with more opens with the first.
		category: Array.isArray(filter.category) ? filter.category[0] : filter.category,
		safeToSend: filter.safe_to_send,
		minScore: filter.score_min,
		maxScore: filter.score_max,
		policyDecision: filter.policy_decision,
	};
}

export function SavedSegments({ className, filters, onOpen, ...props }: SavedSegmentsProps) {
	const queryClient = useQueryClient();
	const [offset, setOffset] = useState(0);
	const [renaming, setRenaming] = useState<number>();
	const [message, setMessage] = useState<{ error?: string; text?: string }>({});
	const segments = useQuery({
		queryKey: workflowQueryKey("segments", SCOPE, offset),
		placeholderData: keepPreviousData,
		queryFn: async () =>
			V1ListSavedSegments200Response.parse(
				(await api.v1ListSavedSegments({ scope: SCOPE, limit: PAGE, offset })).data,
			),
	});
	const handlers = (text: string) => ({
		onSuccess: async () => {
			setMessage({ text });
			await queryClient.invalidateQueries({ queryKey: workflowQueryKey("segments") });
		},
		onError: (error: Error) => {
			setMessage({ error: apiMessage(error) });
		},
	});
	const create = useMutation({
		mutationFn: async (name: string) => {
			await api.v1CreateSavedSegment({
				createSavedSegmentRequest: V1CreateSavedSegmentBody.parse({
					name,
					scope: SCOPE,
					filter,
				}),
			});
		},
		...handlers("Segment saved."),
	});
	const rename = useMutation({
		mutationFn: async ({ id, name }: { id: number; name: string }) => {
			await api.v1UpdateSavedSegment({
				segmentId: id,
				updateSavedSegmentRequest: V1UpdateSavedSegmentBody.parse({ name }),
			});
		},
		...handlers("Segment renamed."),
	});
	const remove = useMutation({
		mutationFn: async (id: number) => {
			await api.v1DeleteSavedSegment({ segmentId: id });
		},
		...handlers("Segment deleted."),
	});
	// Read the segment again, so the explorer gets its latest filters.
	const open = useMutation({
		mutationFn: async (id: number) =>
			V1GetSavedSegment200Response.parse((await api.v1GetSavedSegment({ segmentId: id })).data),
		onSuccess: (segment) => {
			onOpen(fromSegmentFilter(segment.filter));
		},
		onError: (error: Error) => {
			setMessage({ error: apiMessage(error) });
		},
	});
	const { filter, skipped } = toSegmentFilter(filters);
	const kept = describeFilters(fromSegmentFilter(filter));
	const rows = segments.data?.segments ?? [];
	const total = segments.data?.total ?? 0;
	const busy = create.isPending || rename.isPending || remove.isPending || open.isPending;

	return (
		<section data-slot="saved-segments" className={cn("flex flex-col gap-6", className)} {...props}>
			<form
				className="flex flex-col gap-3"
				onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
					event.preventDefault();
					const element = event.currentTarget;
					const name = String(new FormData(element).get("name") ?? "").trim();
					if (!name) return;
					create.mutate(name, {
						onSuccess: () => {
							element.reset();
						},
					});
				}}
			>
				<div className="flex flex-col gap-3 sm:flex-row sm:items-end">
					<div className="flex flex-1 flex-col gap-1.5">
						<Label className="text-xs font-medium text-neutral-400" htmlFor="segment-name">
							Segment name
						</Label>
						<Input className="h-10 rounded-lg" id="segment-name" name="name" required />
					</div>
					<Button className="h-10 rounded-lg" disabled={busy} type="submit">
						Save current filters
					</Button>
				</div>
				<p className="text-xs text-[#858585]">
					The segment keeps: {kept.length ? kept.join(" · ") : "all results"}.
				</p>
				{skipped.length ? (
					<p className="text-xs text-amber-300">
						A saved segment cannot keep these filters: {skipped.join(" · ")}.
					</p>
				) : null}
			</form>
			{message.error ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{message.error}
				</p>
			) : null}
			{message.text ? (
				<p className="text-sm text-emerald-300" role="status">
					{message.text}
				</p>
			) : null}
			{segments.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(segments.error)}
				</p>
			) : null}
			<Group>
				{rows.length ? (
					rows.map((segment) => {
						const description = describeFilters(fromSegmentFilter(segment.filter));
						return renaming === segment.id ? (
							<form
								className="flex flex-col gap-3 p-5 sm:flex-row sm:items-end"
								key={segment.id}
								onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
									event.preventDefault();
									const name = String(new FormData(event.currentTarget).get("name") ?? "").trim();
									if (!name) return;
									rename.mutate(
										{ id: segment.id, name },
										{
											onSuccess: () => {
												setRenaming(undefined);
											},
										},
									);
								}}
							>
								<div className="flex flex-1 flex-col gap-1.5">
									<Label
										className="text-xs font-medium text-neutral-400"
										htmlFor={`segment-rename-${String(segment.id)}`}
									>
										New name for {segment.name}
									</Label>
									<Input
										className="h-10 rounded-lg"
										defaultValue={segment.name}
										id={`segment-rename-${String(segment.id)}`}
										name="name"
										required
									/>
								</div>
								<div className="flex gap-2">
									<Button className="h-10 rounded-lg" disabled={busy} type="submit">
										Save name
									</Button>
									<Button
										className="h-10 rounded-lg"
										onClick={() => {
											setRenaming(undefined);
										}}
										type="button"
										variant="outline"
									>
										Cancel
									</Button>
								</div>
							</form>
						) : (
							<article className="flex flex-col gap-3 p-5" key={segment.id}>
								<div className="min-w-0">
									<h3 className="text-sm font-medium break-all text-white">{segment.name}</h3>
									<p className="mt-0.5 text-xs text-[#858585]">
										{description.length ? description.join(" · ") : "All results"}
									</p>
									<p className="mt-0.5 text-xs text-[#858585]">
										Changed {new Date(segment.updated_at).toLocaleString()}
									</p>
								</div>
								<div className="flex flex-wrap gap-2">
									<Button
										aria-label={`Open ${segment.name}`}
										className="rounded-lg"
										disabled={busy}
										onClick={() => {
											open.mutate(segment.id);
										}}
										size="sm"
										type="button"
									>
										Open
									</Button>
									<Button
										aria-label={`Rename ${segment.name}`}
										className="rounded-lg"
										onClick={() => {
											setRenaming(segment.id);
										}}
										size="sm"
										type="button"
										variant="outline"
									>
										Rename
									</Button>
									<Button
										aria-label={`Delete ${segment.name}`}
										className="rounded-lg"
										disabled={busy}
										onClick={() => {
											remove.mutate(segment.id);
										}}
										size="sm"
										type="button"
										variant="outline"
									>
										Delete
									</Button>
								</div>
							</article>
						);
					})
				) : (
					<p className="p-5 text-sm text-[#858585]">
						{segments.isPending ? "Loading segments…" : "No saved segments yet."}
					</p>
				)}
			</Group>
			{total > PAGE ? (
				<div className="flex justify-end gap-2">
					<Button
						className="rounded-lg"
						disabled={offset === 0}
						onClick={() => {
							setOffset(Math.max(0, offset - PAGE));
						}}
						size="sm"
						type="button"
						variant="outline"
					>
						Previous
					</Button>
					<Button
						className="rounded-lg"
						disabled={offset + PAGE >= total}
						onClick={() => {
							setOffset(offset + PAGE);
						}}
						size="sm"
						type="button"
						variant="outline"
					>
						Next
					</Button>
				</div>
			) : null}
		</section>
	);
}
