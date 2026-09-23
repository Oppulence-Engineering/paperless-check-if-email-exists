"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

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
		<section className={cn("space-y-8", className)} data-slot="lists-panel" {...props}>
			<div>
				<h1 className="text-3xl font-semibold">Email lists</h1>
				<p className="mt-2 text-muted-foreground">
					Upload a CSV and track its verification status.
				</p>
			</div>
			<Card>
				<CardHeader>
					<CardTitle>Upload a list</CardTitle>
				</CardHeader>
				<CardContent>
					<form className="grid gap-4 sm:grid-cols-[1fr_1fr_auto] sm:items-end" onSubmit={submit}>
						<div className="space-y-2">
							<Label htmlFor="list-file">CSV file</Label>
							<Input
								accept=".csv,text/csv"
								id="list-file"
								onChange={(event) => {
									setFile(event.target.files?.[0]);
								}}
								required
								type="file"
							/>
						</div>
						<div className="space-y-2">
							<Label htmlFor="list-name">Name</Label>
							<Input
								id="list-name"
								onChange={(event) => {
									setName(event.target.value);
								}}
								placeholder="Campaign list"
								value={name}
							/>
						</div>
						<Button disabled={upload.isPending || !file} type="submit">
							{upload.isPending ? "Uploading…" : "Upload"}
						</Button>
					</form>
					{upload.isError ? (
						<p className="mt-4 text-sm text-destructive" role="alert">
							{upload.error.message}
						</p>
					) : null}
					{upload.isSuccess ? (
						<p className="mt-4 text-sm text-emerald-700" role="status">
							List accepted. Processing job {upload.data.job_id}.
						</p>
					) : null}
				</CardContent>
			</Card>
			<Card>
				<CardHeader>
					<CardTitle>Lists</CardTitle>
				</CardHeader>
				<CardContent>
					{lists.isPending ? <p role="status">Loading lists…</p> : null}
					{lists.isError ? (
						<p className="text-destructive" role="alert">
							{lists.error.message}
						</p>
					) : null}
					{lists.data ? (
						<>
							{lists.data.lists.length === 0 ? (
								<p className="text-muted-foreground">No lists yet.</p>
							) : (
								<ul className="divide-y divide-border">
									{lists.data.lists.map((list) => (
										<li
											className="flex flex-wrap items-center justify-between gap-3 py-4"
											key={list.id}
										>
											<div>
												<p className="font-medium">{list.name}</p>
												<p className="text-sm text-muted-foreground">
													{list.original_filename} · {list.total_rows} rows
												</p>
											</div>
											<span className="rounded-full bg-muted px-3 py-1 text-sm">{list.status}</span>
										</li>
									))}
								</ul>
							)}
							<div className="mt-5 flex items-center justify-between text-sm">
								<span>{lists.data.total} total</span>
								<div className="flex gap-2">
									<Button
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
							</div>
						</>
					) : null}
				</CardContent>
			</Card>
		</section>
	);
}
