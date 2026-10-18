"use client";

import "client-only";

import { useId, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { Textarea } from "@oppulence/ui/components/textarea";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@oppulence/ui/components/select";
import { cn } from "@oppulence/ui/lib/utils";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1AddSuppressions200Response, V1AddSuppressionsBody } from "@/lib/api/generated/zod/v1/v1";
import { suppressionReason } from "@/lib/verification/labels";

import { type SuppressionFormPropsFields } from "./suppression-form.schema";

/**
 * @oppulence-gen kind=component
 * SuppressionForm is a client data component.
 * Adds addresses or imports a CSV file into the suppression list.
 *
 * The browser reads the CSV file and sends its addresses as JSON, because
 * POST /v1/suppressions/import takes the same JSON body as POST /v1/suppressions
 * (backend/src/http/v1/suppressions/add.rs). Owned by `suppression-form.lit.ts`.
 */
export type SuppressionFormProps = SuppressionFormPropsFields & ComponentPropsWithoutRef<"form">;

const api = new V1Api(new Configuration({ basePath: "/api/backend" }));
/** The API takes a maximum of 10,000 addresses in one request (suppressions/add.rs). */
const BATCH_SIZE = 10_000;
const LABEL_CLASS = "text-xs font-medium text-neutral-400";
const SELECT_CLASS = "h-10 w-full rounded-lg";

/** Splits one CSV line into cells. Quoted cells can contain commas and "" quotes. */
function cells(line: string): string[] {
	const out = [""];
	let quoted = false;
	for (let index = 0; index < line.length; index += 1) {
		const char = line[index];
		if (char === '"' && quoted && line[index + 1] === '"') {
			out[out.length - 1] += '"';
			index += 1;
		} else if (char === '"') quoted = !quoted;
		else if (char === "," && !quoted) out.push("");
		else out[out.length - 1] += char;
	}
	return out;
}

/**
 * Reads the email column of a CSV file. The column is the header `email`, else
 * the first header that contains "mail". A file with no such header uses its
 * first column for all rows. Rows with no "@" in that column are skipped.
 */
// ponytail: one record on each line; quoted cells with line breaks need a CSV parser library.
function emailsFromCsv(text: string): { emails: string[]; skipped: number } {
	const rows = text
		.split(/\r?\n/)
		.filter((line) => line.trim())
		.map(cells);
	const header = rows[0]?.map((cell) => cell.trim().toLowerCase()) ?? [];
	const named = header.includes("email")
		? header.indexOf("email")
		: header.findIndex((cell) => cell.includes("mail"));
	const values = (named === -1 ? rows : rows.slice(1)).map((row) =>
		(row[Math.max(named, 0)] ?? "").trim(),
	);
	const emails = values.filter((value) => value.includes("@"));
	return { emails, skipped: values.length - emails.length };
}

function optional(form: FormData, key: string): string | undefined {
	const value = form.get(key);
	return typeof value === "string" && value.trim() ? value.trim() : undefined;
}

export function SuppressionForm({ className, mode, ...props }: SuppressionFormProps) {
	const id = useId();
	const queryClient = useQueryClient();
	const save = useMutation({
		mutationFn: async ({ form, file }: { form: FormData; file?: File }) => {
			const read = file
				? emailsFromCsv(await file.text())
				: {
						emails: (optional(form, "addresses") ?? "").split(/[\s,;]+/).filter(Boolean),
						skipped: 0,
					};
			if (!read.emails.length) {
				throw new Error(
					file ? "The file has no email addresses." : "Enter at least one email address.",
				);
			}
			const expires = optional(form, "expires");
			const fields = {
				reason: form.get("reason"),
				reason_detail: optional(form, "reason_detail"),
				notes: optional(form, "notes"),
				expires_at: expires ? new Date(`${expires}T00:00`).toISOString() : undefined,
				...(file ? { source_type: "import", source_ref: file.name } : {}),
			};
			const totals = { added: 0, updated: 0, duplicates: 0 };
			for (let start = 0; start < read.emails.length; start += BATCH_SIZE) {
				const body = V1AddSuppressionsBody.parse({
					emails: read.emails.slice(start, start + BATCH_SIZE),
					...fields,
				});
				const response = file
					? await api.v1ImportSuppressions({ addSuppressionsRequest: body })
					: await api.v1AddSuppressions({ addSuppressionsRequest: body });
				// The import endpoint runs the add handler, so both return the same counts.
				const counts = V1AddSuppressions200Response.parse(response.data);
				totals.added += counts.added;
				totals.updated += counts.updated ?? 0;
				totals.duplicates += counts.duplicates;
			}
			return { found: read.emails.length, skipped: read.skipped, ...totals };
		},
		onSuccess: () => queryClient.invalidateQueries({ queryKey: workflowQueryKey("suppressions") }),
	});
	const counts: [string, number][] = save.data
		? [
				["Addresses sent", save.data.found],
				["Added", save.data.added],
				["Updated", save.data.updated],
				["Duplicates", save.data.duplicates],
				...(mode === "import" ? [["Rows skipped", save.data.skipped] as [string, number]] : []),
			]
		: [];

	return (
		<form
			data-slot="suppression-form"
			className={cn("flex flex-col gap-5", className)}
			{...props}
			onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
				event.preventDefault();
				const element = event.currentTarget;
				const input = element.elements.namedItem("file");
				save.mutate(
					{
						form: new FormData(element),
						file: input instanceof HTMLInputElement ? input.files?.[0] : undefined,
					},
					{
						onSuccess: () => {
							element.reset();
						},
					},
				);
			}}
		>
			{mode === "import" ? (
				<div className="flex flex-col gap-1.5">
					<Label className={LABEL_CLASS} htmlFor={`${id}-file`}>
						CSV file
					</Label>
					<Input
						accept=".csv,text/csv"
						aria-describedby={`${id}-file-hint`}
						className="h-10 rounded-lg"
						id={`${id}-file`}
						name="file"
						required
						type="file"
					/>
					<p className="text-xs text-[#858585]" id={`${id}-file-hint`}>
						Use a column named email. A file with no email header uses the first column.
					</p>
				</div>
			) : (
				<div className="flex flex-col gap-1.5">
					<Label className={LABEL_CLASS} htmlFor={`${id}-addresses`}>
						Addresses
					</Label>
					<Textarea
						aria-describedby={`${id}-addresses-hint`}
						className="min-h-32 rounded-lg"
						id={`${id}-addresses`}
						name="addresses"
						required
					/>
					<p className="text-xs text-[#858585]" id={`${id}-addresses-hint`}>
						Put one address on each line, or split them with commas.
					</p>
				</div>
			)}
			<div className="flex flex-col gap-1.5">
				<Label className={LABEL_CLASS} htmlFor={`${id}-reason`}>
					Reason
				</Label>
				<Select defaultValue="manual" name="reason">
					<SelectTrigger className={SELECT_CLASS} id={`${id}-reason`}>
						<SelectValue />
					</SelectTrigger>
					<SelectContent>
						{suppressionReason.values.map((value) => (
							<SelectItem key={value} value={value}>
								{suppressionReason.label(value)}
							</SelectItem>
						))}
					</SelectContent>
				</Select>
			</div>
			<div className="flex flex-col gap-1.5">
				<Label className={LABEL_CLASS} htmlFor={`${id}-detail`}>
					Reason detail
				</Label>
				<Input
					className="h-10 rounded-lg"
					id={`${id}-detail`}
					name="reason_detail"
					placeholder="550 mailbox not found"
				/>
			</div>
			<div className="flex flex-col gap-1.5">
				<Label className={LABEL_CLASS} htmlFor={`${id}-notes`}>
					Notes
				</Label>
				<Textarea className="rounded-lg" id={`${id}-notes`} name="notes" />
			</div>
			<div className="flex flex-col gap-1.5">
				<Label className={LABEL_CLASS} htmlFor={`${id}-expires`}>
					Expires on
				</Label>
				<Input
					aria-describedby={`${id}-expires-hint`}
					className="h-10 rounded-lg"
					id={`${id}-expires`}
					name="expires"
					type="date"
				/>
				<p className="text-xs text-[#858585]" id={`${id}-expires-hint`}>
					Leave empty to keep the addresses suppressed with no end date.
				</p>
			</div>
			{save.error ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{save.error.message}
				</p>
			) : null}
			<div role="status">
				{counts.length ? (
					<dl className="grid grid-cols-2 gap-3 rounded-lg bg-white/[0.03] p-4 text-sm ring-1 ring-white/[0.08]">
						{counts.map(([label, value]) => (
							<div key={label}>
								<dt className="text-xs text-[#858585]">{label}</dt>
								<dd className="text-white">{value}</dd>
							</div>
						))}
					</dl>
				) : null}
			</div>
			<div className="flex justify-end">
				<Button className="rounded-lg" disabled={save.isPending} type="submit">
					{mode === "import" ? "Import file" : "Add"}
				</Button>
			</div>
		</form>
	);
}
