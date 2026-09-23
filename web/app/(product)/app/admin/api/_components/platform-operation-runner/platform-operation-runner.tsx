"use client";

import "client-only";

import { useMutation } from "@tanstack/react-query";
import Link from "next/link";
import { useId, useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { dashboardRequest, redirectBrowserIfUnauthorized } from "@/lib/auth/dashboard-fetch";
import { cn } from "@oppulence/ui/lib/utils";

import { type PlatformOperationRunnerPropsFields } from "./platform-operation-runner.schema";

/** @oppulence-gen kind=component Owned by `platform-operation-runner.lit.ts`. */
export type PlatformOperationRunnerProps = PlatformOperationRunnerPropsFields &
	ComponentPropsWithoutRef<"section">;
type Operation = PlatformOperationRunnerPropsFields["operations"][number];

export function PlatformOperationRunner({
	operations,
	className,
	...props
}: PlatformOperationRunnerProps) {
	const formId = useId();
	const [id, setId] = useState(operations[0]?.id ?? "");
	const [values, setValues] = useState<Partial<Record<string, string>>>({});
	const [query, setQuery] = useState("");
	const [body, setBody] = useState("{}");
	const [reason, setReason] = useState("");
	const selected = operations.find((operation) => operation.id === id);
	const pathParams = selected
		? [...selected.path.matchAll(/\{([^}]+)\}/g)].map((match) => match[1])
		: [];
	const mutation = useMutation({
		mutationFn: async (operation: Operation) => {
			const path = operation.path.replace(/\{([^}]+)\}/g, (_segment, name: string) => {
				const value = values[name]?.trim();
				if (!value) throw new Error(`${name} is required`);
				return encodeURIComponent(value);
			});
			const search = query.trim().replace(/^\?/, "");
			const headers: Record<string, string> = {};
			if (operation.method !== "GET") headers["x-admin-reason"] = reason.trim();
			let requestBody: string | undefined;
			if (operation.requestBody) {
				try {
					JSON.parse(body);
				} catch {
					throw new Error("Request body must be valid JSON");
				}
				headers["content-type"] = "application/json";
				requestBody = body;
			}
			const response = await dashboardRequest(
				`/api/platform/backend${path}${search ? `?${search}` : ""}`,
				{
					method: operation.method,
					headers,
					body: requestBody,
				},
			);
			redirectBrowserIfUnauthorized(response.status);
			const text = await response.text();
			let formatted = text;
			if (response.headers.get("content-type")?.includes("json")) {
				try {
					formatted = JSON.stringify(JSON.parse(text), null, 2);
				} catch {
					/* Show the backend text. */
				}
			}
			if (!response.ok)
				throw new Error(`${String(response.status)} ${response.statusText}\n${formatted}`);
			return formatted || `${String(response.status)} ${response.statusText}`;
		},
	});
	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		if (!selected) return;
		if (
			selected.method !== "GET" &&
			!window.confirm(`Run ${selected.method} ${selected.path}? Reason: ${reason.trim()}`)
		)
			return;
		mutation.mutate(selected);
	}
	return (
		<section
			data-slot="platform-operation-runner"
			className={cn("grid gap-6 lg:grid-cols-2", className)}
			{...props}
		>
			<form
				onSubmit={submit}
				className="flex flex-col gap-4 rounded-lg border border-border bg-background p-5"
			>
				<label htmlFor={`${formId}-operation`} className="flex flex-col gap-1 text-sm">
					Operation
					<select
						id={`${formId}-operation`}
						className="rounded border border-border bg-background p-2"
						value={id}
						onChange={(event) => {
							setId(event.target.value);
							setValues({});
							setQuery("");
							setBody("{}");
							setReason("");
							mutation.reset();
						}}
					>
						{operations.map((operation) => (
							<option key={operation.id} value={operation.id}>
								{operation.method} {operation.path}
							</option>
						))}
					</select>
				</label>
				{selected ? (
					<>
						<p className="text-sm text-muted-foreground">{selected.description ?? selected.id}</p>
						<Link className="text-sm underline" href={`/developers/reference/${selected.id}`}>
							Operation reference
						</Link>
						{pathParams.map((name) => (
							<label
								key={name}
								htmlFor={`${formId}-path-${name}`}
								className="flex flex-col gap-1 text-sm"
							>
								{name}
								<input
									id={`${formId}-path-${name}`}
									aria-label={name}
									required
									className="rounded border border-border bg-background p-2"
									value={values[name] ?? ""}
									onChange={(event) => {
										setValues({ ...values, [name]: event.target.value });
									}}
								/>
							</label>
						))}
						<label htmlFor={`${formId}-query`} className="flex flex-col gap-1 text-sm">
							Query string
							<input
								id={`${formId}-query`}
								aria-label="Query string"
								className="rounded border border-border bg-background p-2"
								placeholder="limit=50&offset=0"
								value={query}
								onChange={(event) => {
									setQuery(event.target.value);
								}}
							/>
						</label>
						{selected.requestBody ? (
							<label htmlFor={`${formId}-body`} className="flex flex-col gap-1 text-sm">
								JSON body
								<textarea
									id={`${formId}-body`}
									aria-label="JSON body"
									className="min-h-40 rounded border border-border bg-background p-2 font-mono text-xs"
									value={body}
									onChange={(event) => {
										setBody(event.target.value);
									}}
								/>
							</label>
						) : null}
						{selected.method !== "GET" ? (
							<label htmlFor={`${formId}-reason`} className="flex flex-col gap-1 text-sm">
								Reason for change
								<input
									id={`${formId}-reason`}
									aria-label="Reason for change"
									required
									minLength={10}
									maxLength={255}
									className="rounded border border-border bg-background p-2"
									value={reason}
									onChange={(event) => {
										setReason(event.target.value);
									}}
								/>
							</label>
						) : null}
						<button
							type="submit"
							disabled={mutation.isPending}
							className="rounded bg-primary px-4 py-2 text-primary-foreground disabled:opacity-50"
						>
							{mutation.isPending ? "Running…" : `Run ${selected.method}`}
						</button>
					</>
				) : null}
			</form>
			<div
				aria-live="polite"
				className="min-h-64 rounded-lg border border-border bg-background p-5"
			>
				<h2 className="mb-3 text-base font-semibold">Response</h2>
				{mutation.isError ? (
					<>
						<pre className="overflow-auto whitespace-pre-wrap text-sm text-destructive">
							{mutation.error instanceof Error ? mutation.error.message : "Request failed"}
						</pre>
						<p className="mt-3 text-sm">
							If recent verification is required, complete a TOTP or passkey check in{" "}
							<Link className="underline" href="/app/settings?settings=security">
								Security settings
							</Link>
							.
						</p>
					</>
				) : null}
				{mutation.isSuccess ? (
					<pre className="overflow-auto whitespace-pre-wrap font-mono text-xs">{mutation.data}</pre>
				) : null}
				{mutation.isIdle ? (
					<p className="text-sm text-muted-foreground">
						Select an operation to inspect platform state.
					</p>
				) : null}
			</div>
		</section>
	);
}
