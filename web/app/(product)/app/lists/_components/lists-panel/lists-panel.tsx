"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import Link from "next/link";
import { useRouter } from "next/navigation";
import { useState, useRef, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation } from "@tanstack/react-query";
import { Configuration, ListsApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { Progress } from "@oppulence/ui/components/progress";
import { cn } from "@oppulence/ui/lib/utils";

import { EmptyState, Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";
import { ListBullets } from "@/lib/icons";
import { useV1CreateList } from "@/hooks/queries/use-v1-create-list";
import { useV1ListLists } from "@/hooks/queries/use-v1-list-lists";
import { V1PreviewList200Response } from "@/lib/api/generated/zod/lists/lists";
import type { V1ListLists200Response } from "@/lib/api/generated/zod/v1/v1";
import type { z } from "zod";
import { ACTIVE_LIST_STATES } from "@/lib/api/schemas/lists";
import { listStatus } from "@/lib/verification/labels";

import { type ListsPanelPropsFields } from "./lists-panel.schema";

export type ListsPanelProps = ListsPanelPropsFields & ComponentPropsWithoutRef<"section">;

const listsApi = new ListsApi(new Configuration({ basePath: "/api/backend" }));
const fieldLabel = "text-xs font-medium text-neutral-400";

/** The checked rows of one list that is still processing. It polls every 5 seconds. */
function ListProgress({ list }: { list: z.infer<typeof V1ListLists200Response>["lists"][number] }) {
	const processed = list.total_processed ?? 0;
	const total = Math.max(1, list.total_rows);
	return (
		<div className="flex w-full items-center gap-3">
			<Progress
				aria-label={`${list.name} progress`}
				className="bg-white/[0.06]"
				value={(processed / total) * 100}
			/>
			<span className="shrink-0 text-xs text-[#858585]">
				{processed} of {list.total_rows}
			</span>
		</div>
	);
}

/** @oppulence-gen kind=component; owned by `lists-panel.lit.ts`. */
export function ListsPanel({ className, ...props }: ListsPanelProps) {
	const router = useRouter();
	const [file, setFile] = useState<File>();
	const [name, setName] = useState("");
	const [emailColumn, setEmailColumn] = useState("");
	const [sourceKey, setSourceKey] = useState("");
	const [offset, setOffset] = useState(0);
	const lists = useV1ListLists({ limit: 20, offset }, (data) =>
		data.lists.some((list) => ACTIVE_LIST_STATES.includes(list.status)),
	);
	const upload = useV1CreateList();
	const formRef = useRef<HTMLFormElement>(null);
	const requestKey = useRef(crypto.randomUUID());
	const preview = useMutation({
		mutationFn: async () => {
			if (!file) throw new Error("Choose a CSV file.");
			return V1PreviewList200Response.parse(
				(
					await listsApi.v1PreviewList({
						file,
						name: name.trim() || undefined,
						emailColumn: emailColumn.trim() || undefined,
						sourceKey: sourceKey.trim() || undefined,
					})
				).data,
			);
		},
	});

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		if (!file) return;
		if (!preview.data) {
			preview.mutate();
			return;
		}
		upload.mutate(
			{
				idempotencyKey: requestKey.current,
				file,
				name: name.trim() || undefined,
				email_column: emailColumn.trim() || undefined,
				source_key: sourceKey.trim() || undefined,
			},
			{
				onSuccess: (data) => {
					setFile(undefined);
					setName("");
					setEmailColumn("");
					setSourceKey("");
					preview.reset();
					formRef.current?.reset();
					requestKey.current = crypto.randomUUID();
					router.push(`/app/lists/detail?list=${String(data.list_id)}&accepted=1`);
				},
			},
		);
	}

	return (
		<Page description="Upload a CSV, then review, clean, and export it." title="Email lists">
			<section className={cn("flex flex-col gap-10", className)} data-slot="lists-panel" {...props}>
				<form
					ref={formRef}
					onSubmit={submit}
					onChange={() => {
						preview.reset();
						upload.reset();
						requestKey.current = crypto.randomUUID();
					}}
				>
					<fieldset className="min-w-0" disabled={preview.isPending || upload.isPending}>
						<Group>
							<Row>
								<h3 className="text-sm font-medium text-white">Upload a list</h3>
								<div className="grid gap-3 sm:grid-cols-[1fr_1fr_auto] sm:items-end">
									<div className="flex flex-col gap-1.5">
										<Label className={fieldLabel} htmlFor="list-file">
											CSV file
										</Label>
										<Input
											accept=".csv,text/csv"
											className="h-10 rounded-lg"
											id="list-file"
											onChange={(event) => {
												setFile(event.target.files?.[0]);
											}}
											required
											type="file"
										/>
									</div>
									<div className="flex flex-col gap-1.5">
										<Label className={fieldLabel} htmlFor="list-name">
											Name
										</Label>
										<Input
											className="h-10 rounded-lg"
											id="list-name"
											onChange={(event) => {
												setName(event.target.value);
											}}
											placeholder="Campaign list"
											value={name}
										/>
									</div>
									<Button
										className="h-10 rounded-lg"
										disabled={upload.isPending || preview.isPending || !file}
										type="submit"
									>
										{upload.isPending
											? "Uploading…"
											: preview.isPending
												? "Preparing preview…"
												: preview.data
													? "Confirm and verify"
													: "Preview upload"}
									</Button>
								</div>
								<div className="flex flex-col gap-1.5">
									<Label className={fieldLabel} htmlFor="list-source-key">
										Lead source
									</Label>
									<Input
										aria-describedby="list-source-key-help"
										className="h-10 rounded-lg"
										id="list-source-key"
										onChange={(event) => {
											setSourceKey(event.target.value);
										}}
										placeholder="hubspot"
										value={sourceKey}
									/>
									<p className="text-xs text-[#858585]" id="list-source-key-help">
										Where the list came from. Insights compare quality by source.
									</p>
								</div>
								<details>
									<summary className="cursor-pointer text-sm text-[#858585] hover:text-neutral-200">
										More options
									</summary>
									<div className="mt-3 grid gap-3 sm:grid-cols-2">
										<div className="flex flex-col gap-1.5">
											<Label className={fieldLabel} htmlFor="list-email-column">
												Email column
											</Label>
											<Input
												className="h-10 rounded-lg"
												id="list-email-column"
												onChange={(event) => {
													setEmailColumn(event.target.value);
												}}
												placeholder="Found from the header"
												value={emailColumn}
											/>
										</div>
									</div>
								</details>
							</Row>
						</Group>
					</fieldset>
				</form>
				{preview.data ? (
					<div role="status">
						<p>
							{preview.data.total_rows} rows · {preview.data.unique_emails} unique addresses ·{" "}
							{preview.data.duplicates} duplicates · {preview.data.rejected.length} rejected
						</p>
						<p>Verifying this list will use {preview.data.estimated_credits} credits.</p>
						{preview.data.rejected.map((row) => (
							<p key={row.row_number}>
								Row {row.row_number}: {row.value || "Blank address"} — {row.reason}
							</p>
						))}
					</div>
				) : null}
				{preview.isError ? <p role="alert">{errorMessage(preview.error)}</p> : null}
				{upload.isError ? (
					<p
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
						role="alert"
					>
						{errorMessage(upload.error)}
					</p>
				) : null}
				<Section title="Lists">
					{lists.data?.lists.length === 0 ? (
						<EmptyState
							description="Upload a CSV above to check every address in it."
							icon={ListBullets}
							title="No lists yet."
						/>
					) : (
						<Group>
							{lists.isPending ? (
								<Row>
									<p className="text-sm text-[#858585]" role="status">
										Loading lists…
									</p>
								</Row>
							) : null}
							{lists.isError ? (
								<Row>
									<p className="text-sm text-red-300" role="alert">
										{errorMessage(lists.error)}
									</p>
								</Row>
							) : null}
							{lists.data ? (
								<>
									<div className="px-5 py-3 text-xs font-medium text-[#858585]">
										{lists.data.total} total
									</div>
									{lists.data.lists.map((list) => (
										<div
											className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
											key={list.id}
										>
											<div className="min-w-0 flex-1">
												<Link
													className="block truncate text-sm font-medium text-white hover:underline"
													href={`/app/lists/detail?list=${String(list.id)}`}
												>
													{list.name}
												</Link>
												<p className="truncate text-xs text-[#858585]">
													{list.original_filename} · {list.total_rows} rows
												</p>
												<p className="text-xs text-[#858585]">
													Uploaded {new Date(list.created_at).toLocaleString()}
													{list.completed_at
														? ` · Done ${new Date(list.completed_at).toLocaleString()}`
														: ""}
												</p>
											</div>
											<Pill tone={listStatus.tone(list.status)}>
												{listStatus.label(list.status)}
											</Pill>
											{ACTIVE_LIST_STATES.includes(list.status) ? (
												<ListProgress list={list} />
											) : null}
										</div>
									))}
								</>
							) : null}
						</Group>
					)}
					{lists.data ? (
						<div className="flex justify-end gap-2">
							<Button
								className="rounded-lg"
								disabled={offset === 0}
								onClick={() => {
									setOffset(Math.max(0, offset - 20));
								}}
								type="button"
								variant="outline"
							>
								Previous
							</Button>
							<Button
								className="rounded-lg"
								disabled={offset + 20 >= lists.data.total}
								onClick={() => {
									setOffset(offset + 20);
								}}
								type="button"
								variant="outline"
							>
								Next
							</Button>
						</div>
					) : null}
				</Section>
			</section>
		</Page>
	);
}
