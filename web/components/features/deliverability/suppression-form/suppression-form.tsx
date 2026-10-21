"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import { useId, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { DatePicker } from "@oppulence/ui/components/date-picker";
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

/** Preserve every data row, including blank and invalid values, for server validation. */
function emailsFromCsv(text: string): { emails: string[]; skipped: number } {
	const rows: string[][] = [];
	let row = [""];
	let quoted = false;
	for (let index = 0; index < text.length; index += 1) {
		const char = text[index];
		if (char === '"' && quoted && text[index + 1] === '"') {
			row[row.length - 1] += '"';
			index += 1;
		} else if (char === '"') quoted = !quoted;
		else if (char === "," && !quoted) row.push("");
		else if ((char === "\n" || char === "\r") && !quoted) {
			rows.push(row);
			row = [""];
			if (char === "\r" && text[index + 1] === "\n") index += 1;
		} else row[row.length - 1] += char;
	}
	if (quoted) throw new Error("The CSV contains an unclosed quoted field.");
	if (row.length > 1 || row[0] || !/[\r\n]$/.test(text)) rows.push(row);
	const header = rows[0]?.map((cell) => cell.trim().toLowerCase()) ?? [];
	const named = header.includes("email")
		? header.indexOf("email")
		: header.findIndex((cell) => cell.includes("mail"));
	return {
		emails: (named === -1 ? rows : rows.slice(1)).map((record) => record[Math.max(named, 0)] ?? ""),
		skipped: 0,
	};
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
						emails: String(form.get("addresses") ?? "").split(/[\n,;]/),
						skipped: 0,
					};
			if (!read.emails.length) {
				throw new Error(
					file ? "The file has no email addresses." : "Enter at least one email address.",
				);
			}
			const expires = optional(form, "expires");
			if (expires && new Date(`${expires}T00:00`).getTime() <= Date.now())
				throw new Error("Expiry must be in the future.");
			const fields = {
				reason: form.get("reason"),
				reason_detail: optional(form, "reason_detail"),
				notes: optional(form, "notes"),
				expires_at: expires ? new Date(`${expires}T00:00`).toISOString() : undefined,
				...(file ? { source_type: "import", source_ref: file.name } : {}),
			};
			const totals = {
				added: 0,
				updated: 0,
				duplicates: 0,
				rejected: [] as { email: string; reason: string }[],
			};
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
				totals.rejected.push(...(counts.rejected ?? []));
			}
			return { found: read.emails.length, skipped: read.skipped, ...totals };
		},
		onSuccess: async () => {
			await queryClient.invalidateQueries({ queryKey: workflowQueryKey("suppressions") });
			await queryClient.invalidateQueries({ queryKey: workflowQueryKey("suppression-check") });
		},
	});
	const counts: [string, number][] = save.data
		? [
				["Addresses sent", save.data.found],
				["Added", save.data.added],
				["Updated", save.data.updated],
				["Duplicates", save.data.duplicates],
				["Rejected", save.data.rejected.length],
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
				<DatePicker
					aria-describedby={`${id}-expires-hint`}
					className="h-10 rounded-lg"
					id={`${id}-expires`}
					name="expires"
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
					{errorMessage(save.error)}
				</p>
			) : null}
			<div role="status">
				{save.data?.rejected.map((item, index) => (
					<p key={index}>
						{item.email || "Blank entry"}: {item.reason}
					</p>
				))}
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
