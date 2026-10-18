"use client";

import "client-only";

import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import { EmptyState, Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";
import { ListBullets } from "@/lib/icons";
import { useV1CreateList } from "@/hooks/queries/use-v1-create-list";
import { useV1ListLists } from "@/hooks/queries/use-v1-list-lists";

import { type ListsPanelPropsFields } from "./lists-panel.schema";

export type ListsPanelProps = ListsPanelPropsFields & ComponentPropsWithoutRef<"section">;

/** @oppulence-gen kind=component; owned by `lists-panel.lit.ts`. */
export function ListsPanel({ className, ...props }: ListsPanelProps) {
	const [file, setFile] = useState<File>();
	const [name, setName] = useState("");
	const [offset, setOffset] = useState(0);
	const lists = useV1ListLists({ limit: 20, offset });
	const upload = useV1CreateList();

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		if (!file) return;
		upload.mutate({ file, name: name.trim() || undefined });
	}

	return (
		<Page description="Upload a CSV and track its verification status." title="Email lists">
			<section className={cn("flex flex-col gap-10", className)} data-slot="lists-panel" {...props}>
				<form onSubmit={submit}>
					<Group>
						<Row>
							<h3 className="text-sm font-medium text-white">Upload a list</h3>
							<div className="grid gap-3 sm:grid-cols-[1fr_1fr_auto] sm:items-end">
								<div className="flex flex-col gap-1.5">
									<Label className="text-xs font-medium text-neutral-400" htmlFor="list-file">
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
									<Label className="text-xs font-medium text-neutral-400" htmlFor="list-name">
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
									disabled={upload.isPending || !file}
									type="submit"
								>
									{upload.isPending ? "Uploading…" : "Upload"}
								</Button>
							</div>
						</Row>
					</Group>
				</form>
				{upload.isError ? (
					<p
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
						role="alert"
					>
						{upload.error.message}
					</p>
				) : null}
				{upload.isSuccess ? (
					<p
						className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
						role="status"
					>
						List accepted.{" "}
						<Link className="underline" href={`/app/jobs?job=${String(upload.data.job_id)}`}>
							Track job {upload.data.job_id}
						</Link>
						.
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
										{lists.error.message}
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
											</div>
											<Pill>{list.status}</Pill>
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
