"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { keepPreviousData, useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, V1Api, type SuppressionReason } from "@oppulence/reacher-sdk";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from "@oppulence/ui/components/sheet";
import { cn } from "@oppulence/ui/lib/utils";
import { SuppressionAudit } from "@/components/features/deliverability/suppression-audit/suppression-audit";
import { SuppressionForm } from "@/components/features/deliverability/suppression-form/suppression-form";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";
import {
	V1CheckSuppression200Response,
	V1ListSuppressions200Response,
} from "@/lib/api/generated/zod/v1/v1";
import { saveFile } from "@/lib/downloads";
import { suppressionReason, suppressionSource, suppressionStatus } from "@/lib/verification/labels";

import { type SuppressionsPanelPropsFields } from "./suppressions-panel.schema";

/** @oppulence-gen kind=component Owned by `suppressions-panel.lit.ts`. */
export type SuppressionsPanelProps = SuppressionsPanelPropsFields &
	ComponentPropsWithoutRef<"section">;

const api = new V1Api(new Configuration({ basePath: "/api/backend" }));
const PAGE_SIZE = 20;
const LABEL_CLASS = "text-xs font-medium text-neutral-400";
const SELECT_CLASS =
	"h-10 w-full rounded-lg bg-white/[0.03] px-2 text-sm text-neutral-200 ring-1 ring-white/[0.08] outline-none sm:w-44";

function date(value: string | null | undefined, empty: string): string {
	return value ? new Date(value).toLocaleString() : empty;
}

export function SuppressionsPanel({ className, ...props }: SuppressionsPanelProps) {
	const client = useQueryClient();
	const [offset, setOffset] = useState(0);
	const [filters, setFilters] = useState({
		reason: "",
		status: "active",
		sourceType: "",
		includeExpired: false,
	});
	const [sheet, setSheet] = useState<"add" | "import">();
	const [audit, setAudit] = useState<{ id: number; email: string }>();
	const [checkInput, setCheckInput] = useState("");
	const [checkEmail, setCheckEmail] = useState("");
	const [notice, setNotice] = useState("");
	const filter = (next: Partial<typeof filters>) => {
		setFilters((current) => ({ ...current, ...next }));
		setOffset(0);
	};
	const query = {
		status: filters.status,
		reason: filters.reason || undefined,
		sourceType: filters.sourceType || undefined,
		includeExpired: filters.includeExpired,
	};
	const entries = useQuery({
		queryKey: workflowQueryKey(
			"suppressions",
			offset,
			filters.status,
			filters.reason,
			filters.sourceType,
			filters.includeExpired,
		),
		placeholderData: keepPreviousData,
		queryFn: async () =>
			V1ListSuppressions200Response.parse(
				(await api.v1ListSuppressions({ limit: PAGE_SIZE, offset, ...query })).data,
			),
	});
	const check = useQuery({
		queryKey: workflowQueryKey("suppression-check", checkEmail),
		enabled: !!checkEmail,
		queryFn: async () =>
			V1CheckSuppression200Response.parse(
				(await api.v1CheckSuppression({ email: checkEmail })).data,
			),
	});
	const remove = useMutation({
		mutationFn: async (id: number) => {
			await api.v1DeleteSuppression({ id });
		},
		onSuccess: async () => {
			setNotice("Suppression removed.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("suppressions") });
			await client.invalidateQueries({ queryKey: workflowQueryKey("suppression-events") });
		},
	});
	const exportCsv = useMutation({
		mutationFn: async () => {
			const response = await api.v1ExportSuppressions(
				{
					status: query.status,
					// The reason filter offers only SuppressionReason values.
					reason: query.reason as SuppressionReason | undefined,
					sourceType: query.sourceType,
					includeExpired: query.includeExpired,
				},
				{ responseType: "blob" },
			);
			saveFile(response.data, "suppressions.csv");
		},
	});
	const error = [entries, check, remove, exportCsv].find((request) => request.isError)?.error;
	const rows = entries.data?.entries ?? [];
	const total = entries.data?.total ?? 0;
	return (
		<Page
			actions={
				<>
					<Button
						className="rounded-lg"
						size="sm"
						onClick={() => {
							setSheet("add");
						}}
					>
						Add addresses
					</Button>
					<Button
						className="rounded-lg"
						size="sm"
						variant="outline"
						onClick={() => {
							setSheet("import");
						}}
					>
						Import CSV
					</Button>
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
				</>
			}
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
				<Section title="Suppressed addresses">
					<div className="flex flex-col gap-3 sm:flex-row sm:flex-wrap sm:items-end">
						<div className="flex flex-col gap-1.5">
							<Label className={LABEL_CLASS} htmlFor="suppression-filter-status">
								Status
							</Label>
							<select
								className={SELECT_CLASS}
								id="suppression-filter-status"
								value={filters.status}
								onChange={(event) => {
									filter({ status: event.target.value });
								}}
							>
								{suppressionStatus.values.map((value) => (
									<option key={value} value={value}>
										{suppressionStatus.label(value)}
									</option>
								))}
								<option value="all">All statuses</option>
							</select>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className={LABEL_CLASS} htmlFor="suppression-filter-reason">
								Reason
							</Label>
							<select
								className={SELECT_CLASS}
								id="suppression-filter-reason"
								value={filters.reason}
								onChange={(event) => {
									filter({ reason: event.target.value });
								}}
							>
								<option value="">All reasons</option>
								{suppressionReason.values.map((value) => (
									<option key={value} value={value}>
										{suppressionReason.label(value)}
									</option>
								))}
							</select>
						</div>
						<div className="flex flex-col gap-1.5">
							<Label className={LABEL_CLASS} htmlFor="suppression-filter-source">
								Source
							</Label>
							<select
								className={SELECT_CLASS}
								id="suppression-filter-source"
								value={filters.sourceType}
								onChange={(event) => {
									filter({ sourceType: event.target.value });
								}}
							>
								<option value="">All sources</option>
								{suppressionSource.values.map((value) => (
									<option key={value} value={value}>
										{suppressionSource.label(value)}
									</option>
								))}
							</select>
						</div>
						<Label
							className="flex h-10 items-center gap-2 text-sm text-neutral-300"
							htmlFor="suppression-filter-expired"
						>
							<Checkbox
								checked={filters.includeExpired}
								id="suppression-filter-expired"
								onCheckedChange={(checked) => {
									filter({ includeExpired: checked === true });
								}}
							/>
							Include expired entries
						</Label>
					</div>
					<Group>
						{rows.length ? (
							<div
								aria-label="Suppressed addresses"
								// `relative` keeps the sr-only header inside the scroll area on narrow screens.
								className="relative overflow-x-auto p-5"
								role="region"
								// eslint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- axe scrollable-region-focusable requires focus on scrolling regions.
								tabIndex={0}
							>
								<table className="w-full min-w-[1100px] text-left text-sm">
									<thead>
										<tr className="border-b border-border">
											<th className="py-2 pr-4 font-medium text-foreground">Address</th>
											<th className="py-2 pr-4 font-medium text-foreground">Reason</th>
											<th className="py-2 pr-4 font-medium text-foreground">Source</th>
											<th className="py-2 pr-4 font-medium text-foreground">Notes</th>
											<th className="py-2 pr-4 font-medium text-foreground">Expires</th>
											<th className="py-2 pr-4 font-medium text-foreground">Added</th>
											<th className="py-2 pr-4 font-medium text-foreground">Last seen</th>
											<th className="py-2 pr-4 font-medium text-foreground">Status</th>
											<th className="py-2 font-medium text-foreground">
												<span className="sr-only">Actions</span>
											</th>
										</tr>
									</thead>
									<tbody>
										{rows.map((entry) => (
											<tr className="border-b border-border/60 align-top" key={entry.id}>
												<td className="py-3 pr-4 break-all text-white">{entry.email}</td>
												<td className="py-3 pr-4">
													<Pill tone={suppressionReason.tone(entry.reason)}>
														{suppressionReason.label(entry.reason)}
													</Pill>
													{entry.reason_detail ? (
														<p className="mt-1 text-xs text-[#858585]">{entry.reason_detail}</p>
													) : null}
												</td>
												<td className="py-3 pr-4 text-neutral-300">
													{suppressionSource.label(entry.source_type ?? entry.source)}
													{entry.source_ref ? (
														<p className="mt-1 text-xs break-all text-[#858585]">
															{entry.source_ref}
														</p>
													) : null}
												</td>
												<td className="max-w-56 py-3 pr-4 text-[#858585]">{entry.notes || "—"}</td>
												<td className="py-3 pr-4 text-[#858585]">
													{date(entry.expires_at, "Never")}
												</td>
												<td className="py-3 pr-4 text-[#858585]">{date(entry.created_at, "—")}</td>
												<td className="py-3 pr-4 text-[#858585]">
													{date(entry.last_seen_at, "—")}
												</td>
												<td className="py-3 pr-4">
													<div className="flex flex-wrap gap-1">
														<Pill tone={suppressionStatus.tone(entry.status)}>
															{suppressionStatus.label(entry.status)}
														</Pill>
														{entry.expires_at && new Date(entry.expires_at) <= new Date() ? (
															<Pill>Expired</Pill>
														) : null}
													</div>
												</td>
												<td className="py-2">
													<div className="flex justify-end gap-2">
														<Button
															size="sm"
															variant="secondary"
															onClick={() => {
																setAudit({ id: entry.id, email: entry.email });
															}}
														>
															Audit
														</Button>
														{entry.status === "active" ? (
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
														) : null}
													</div>
												</td>
											</tr>
										))}
									</tbody>
								</table>
							</div>
						) : (
							<Row>
								<p className="text-sm text-[#858585]" role="status">
									{entries.isPending ? "Loading suppressions…" : "No suppressions found."}
								</p>
							</Row>
						)}
					</Group>
					<div className="flex flex-wrap items-center justify-between gap-3">
						<p className="text-xs text-[#858585]">
							{total && !entries.isPlaceholderData
								? `Showing ${String(offset + 1)}–${String(offset + rows.length)} of ${String(total)}`
								: ""}
						</p>
						<div className="flex gap-2">
							<Button
								className="rounded-lg"
								size="sm"
								variant="outline"
								disabled={offset === 0 || entries.isFetching}
								onClick={() => {
									setOffset(Math.max(0, offset - PAGE_SIZE));
								}}
							>
								Previous
							</Button>
							<Button
								className="rounded-lg"
								size="sm"
								variant="outline"
								disabled={offset + PAGE_SIZE >= total || entries.isFetching}
								onClick={() => {
									setOffset(offset + PAGE_SIZE);
								}}
							>
								Next
							</Button>
						</div>
					</div>
				</Section>
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
										{check.data.suppressed
											? [
													"Suppressed",
													suppressionReason.label(check.data.reason),
													check.data.reason_detail,
													`since ${date(check.data.created_at, "—")}`,
													check.data.expires_at
														? `expires ${date(check.data.expires_at, "")}`
														: null,
												]
													.filter(Boolean)
													.join(" · ")
											: "Not suppressed"}
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
			</section>
			<Sheet
				open={!!sheet}
				onOpenChange={(open) => {
					if (!open) setSheet(undefined);
				}}
			>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-lg">
					<SheetHeader>
						<SheetTitle>{sheet === "import" ? "Import a CSV file" : "Add addresses"}</SheetTitle>
						<SheetDescription>
							{sheet === "import"
								? "Each address in the file gets the reason, detail, notes, and expiry that you set."
								: "Suppressed addresses stay out of sends until you remove them or they expire."}
						</SheetDescription>
					</SheetHeader>
					{sheet ? <SuppressionForm className="px-4" key={sheet} mode={sheet} /> : null}
				</SheetContent>
			</Sheet>
			<Sheet
				open={!!audit}
				onOpenChange={(open) => {
					if (!open) setAudit(undefined);
				}}
			>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-lg">
					<SheetHeader>
						<SheetTitle className="break-all">{audit?.email}</SheetTitle>
						<SheetDescription>Audit timeline. The newest change is first.</SheetDescription>
					</SheetHeader>
					{audit ? <SuppressionAudit className="px-4" entryId={audit.id} /> : null}
				</SheetContent>
			</Sheet>
		</Page>
	);
}
