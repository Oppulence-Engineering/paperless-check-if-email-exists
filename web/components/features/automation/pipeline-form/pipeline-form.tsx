"use client";

import "client-only";

import {
	useState,
	type ChangeEvent,
	type ComponentPropsWithoutRef,
	type ReactNode,
	type SyntheticEvent,
} from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Configuration, PipelinesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { Textarea } from "@oppulence/ui/components/textarea";
import { NativeSelect, NativeSelectOption } from "@oppulence/ui/components/native-select";
import { cn } from "@oppulence/ui/lib/utils";

import {
	apiMessage,
	FILTER_CLASS,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	V1CreatePipeline201Response,
	V1UpdatePipeline200Response,
} from "@/lib/api/generated/zod/pipelines/pipelines";

import {
	WebhookHeadersSchema,
	type Pipeline,
	type PipelineFormPropsFields,
} from "./pipeline-form.schema";

/**
 * @oppulence-gen kind=component
 * PipelineForm is a client data component.
 * Creates a paused pipeline, or edits the settings of one pipeline. The form
 * offers only the list and push sources, because the backend rejects the
 * integration and bucket sources. Owned by `pipeline-form.lit.ts`.
 */
export type PipelineFormProps = PipelineFormPropsFields & {
	onSaved?: (pipeline: Pipeline) => void;
} & ComponentPropsWithoutRef<"section">;

const api = new PipelinesApi(new Configuration({ basePath: "/api/backend" }));
const FIELD_LABEL = "text-xs font-medium text-neutral-400";

function initialValues(pipeline?: Pipeline) {
	const source = pipeline?.source;
	return {
		name: pipeline?.name ?? "",
		source: source?.type === "push" ? "push" : "list_snapshot",
		list: source?.type === "list_snapshot" ? String(source.list_id) : "",
		token: source?.type === "push" ? source.token_id : "",
		cron: pipeline?.schedule.cron ?? "0 9 * * 1",
		timezone: pipeline?.schedule.timezone ?? "UTC",
		window: String(pipeline?.policy.missed_run_window_hours ?? 24),
		delta: pipeline?.verification.delta_mode ?? false,
		freshness: pipeline?.verification.freshness_days?.toString() ?? "",
		webhook: pipeline?.delivery.webhook?.url ?? "",
		headers: Object.entries(pipeline?.delivery.webhook?.headers ?? {})
			.map(([name, value]) => `${name}: ${value}`)
			.join("\n"),
		attempts: String(pipeline?.delivery.max_attempts ?? 5),
		backoff: String(pipeline?.delivery.retry_backoff_seconds ?? 300),
	};
}

type Values = ReturnType<typeof initialValues>;

/** A whole number of `min` or more, from a text field. */
function whole(value: string, min: number, label: string): number {
	const number = Number(value);
	if (!value.trim() || !Number.isSafeInteger(number) || number < min)
		throw new Error(`${label} must be a whole number of ${String(min)} or more.`);
	return number;
}

/** The request body for a create or an update. It throws a readable error for bad input. */
function toInput(values: Values, pipeline?: Pipeline) {
	if (!values.name.trim()) throw new Error("Enter a name.");
	const source =
		values.source === "push"
			? { type: "push" as const, token_id: values.token.trim(), accepted_format: "json" }
			: { type: "list_snapshot" as const, list_id: whole(values.list, 1, "The list ID") };
	if (source.type === "push" && !source.token_id) throw new Error("Enter a push token ID.");
	const headers = WebhookHeadersSchema.safeParse(values.headers);
	if (!headers.success) throw new Error(headers.error.issues[0]?.message);
	return {
		name: values.name.trim(),
		source,
		schedule: { cron: values.cron.trim(), timezone: values.timezone.trim() },
		verification: {
			delta_mode: values.delta,
			freshness_days:
				values.delta && values.freshness.trim()
					? whole(values.freshness, 1, "The days before a new check")
					: null,
		},
		policy: {
			missed_run_window_hours: whole(values.window, 1, "The missed-run window"),
			policy_id: pipeline?.policy.policy_id,
		},
		delivery: {
			dashboard: pipeline?.delivery.dashboard ?? true,
			webhook: values.webhook.trim() ? { url: values.webhook.trim(), headers: headers.data } : null,
			max_attempts: whole(values.attempts, 1, "Delivery attempts"),
			retry_backoff_seconds: whole(values.backoff, 1, "The wait between attempts"),
		},
	};
}

function Field({
	id,
	label,
	hint,
	children,
}: {
	id: string;
	label: string;
	hint?: string;
	children: ReactNode;
}) {
	return (
		<div className="flex flex-col gap-1.5">
			<Label className={FIELD_LABEL} htmlFor={id}>
				{label}
			</Label>
			{children}
			{hint ? <p className="text-xs text-[#858585]">{hint}</p> : null}
		</div>
	);
}

function FieldGroup({ title, children }: { title: string; children: ReactNode }) {
	return (
		<fieldset className="flex flex-col gap-4">
			<legend className="mb-4 text-sm font-medium text-white">{title}</legend>
			{children}
		</fieldset>
	);
}

export function PipelineForm({ className, pipeline, onSaved, ...props }: PipelineFormProps) {
	const client = useQueryClient();
	const [values, setValues] = useState(() => initialValues(pipeline));
	const save = useMutation({
		mutationFn: async () => {
			const input = toInput(values, pipeline);
			if (pipeline)
				return V1UpdatePipeline200Response.parse(
					(await api.v1UpdatePipeline({ pipelineId: pipeline.id, updatePipelineInput: input }))
						.data,
				);
			return V1CreatePipeline201Response.parse(
				(await api.v1CreatePipeline({ createPipelineInput: { ...input, status: "paused" } })).data,
			);
		},
		onSuccess: async (data) => {
			await client.invalidateQueries({ queryKey: workflowQueryKey("pipelines") });
			onSaved?.(data);
		},
	});
	/** The props that bind one text field to `values[key]`. */
	function text(key: Exclude<keyof Values, "delta">) {
		return {
			className: "h-10 rounded-lg",
			id: `pipeline-${key}`,
			value: values[key],
			onChange: (event: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
				setValues({ ...values, [key]: event.target.value });
			},
		};
	}
	const scheduled = values.source === "list_snapshot";

	return (
		<section data-slot="pipeline-form" className={cn("flex flex-col gap-4", className)} {...props}>
			{save.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(save.error)}
				</p>
			) : null}
			<form
				className="flex flex-col gap-8"
				onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
					event.preventDefault();
					save.mutate();
				}}
			>
				<FieldGroup title="Source">
					<Field id="pipeline-name" label="Name">
						<Input {...text("name")} required />
					</Field>
					<Field id="pipeline-source" label="Source">
						<NativeSelect
							className={FILTER_CLASS}
							id="pipeline-source"
							onChange={(event) => {
								setValues({
									...values,
									source: event.target.value === "push" ? "push" : "list_snapshot",
								});
							}}
							value={values.source}
						>
							<NativeSelectOption value="list_snapshot">Saved list</NativeSelectOption>
							<NativeSelectOption value="push">Push API</NativeSelectOption>
						</NativeSelect>
					</Field>
					{scheduled ? (
						<Field
							id="pipeline-list"
							label="List ID"
							hint="Each run checks the current rows of this list."
						>
							<Input {...text("list")} inputMode="numeric" />
						</Field>
					) : (
						<Field
							id="pipeline-token"
							label="Push token ID"
							hint="Your systems push rows to this pipeline through the API."
						>
							<Input {...text("token")} />
						</Field>
					)}
				</FieldGroup>
				{scheduled ? (
					<FieldGroup title="Schedule">
						<Field
							id="pipeline-cron"
							label="Cron schedule"
							hint="For example, 0 9 * * 1 runs at 09:00 each Monday."
						>
							<Input {...text("cron")} />
						</Field>
						<Field
							id="pipeline-timezone"
							label="Timezone"
							hint="An IANA name, for example UTC or Europe/Berlin."
						>
							<Input {...text("timezone")} />
						</Field>
						<Field
							id="pipeline-window"
							label="Missed-run window (hours)"
							hint="If the scheduler is late by more than this time, it skips the run."
						>
							<Input {...text("window")} inputMode="numeric" />
						</Field>
					</FieldGroup>
				) : null}
				<FieldGroup title="Verification">
					<Label
						className="flex items-center gap-2 text-sm text-neutral-200"
						htmlFor="pipeline-delta"
					>
						<Checkbox
							checked={values.delta}
							id="pipeline-delta"
							onCheckedChange={(checked) => {
								setValues({ ...values, delta: checked === true });
							}}
						/>
						Check only new or changed addresses
					</Label>
					{values.delta ? (
						<Field
							id="pipeline-freshness"
							label="Check again after (days)"
							hint="Optional. A run also checks an unchanged address when its last check is older than this."
						>
							<Input {...text("freshness")} inputMode="numeric" />
						</Field>
					) : null}
				</FieldGroup>
				<FieldGroup title="Webhook delivery">
					<Field
						id="pipeline-webhook"
						label="Webhook URL"
						hint="Optional. Use HTTPS. Each finished run sends its results to this URL."
					>
						<Input {...text("webhook")} inputMode="url" placeholder="https://" />
					</Field>
					<Field
						id="pipeline-headers"
						label="Webhook headers"
						hint='Optional. Write one header on each line, as "Name: value".'
					>
						<Textarea {...text("headers")} className="min-h-20 rounded-lg font-mono text-xs" />
					</Field>
					<Field id="pipeline-attempts" label="Delivery attempts">
						<Input {...text("attempts")} inputMode="numeric" />
					</Field>
					<Field id="pipeline-backoff" label="Wait between attempts (seconds)">
						<Input {...text("backoff")} inputMode="numeric" />
					</Field>
				</FieldGroup>
				<div className="flex justify-end">
					<Button className="rounded-lg" disabled={save.isPending} type="submit">
						{pipeline ? "Save pipeline" : "Create paused pipeline"}
					</Button>
				</div>
			</form>
		</section>
	);
}
