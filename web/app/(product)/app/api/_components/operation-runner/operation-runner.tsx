"use client";

import "client-only";

import { useMutation } from "@tanstack/react-query";
import Link from "next/link";
import { useId, useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { dashboardRequest, redirectBrowserIfUnauthorized } from "@/lib/auth/dashboard-fetch";
import { cn } from "@oppulence/ui/lib/utils";

import { type OperationRunnerPropsFields } from "./operation-runner.schema";

/** @oppulence-gen kind=component Owned by `operation-runner.lit.ts`. */
export type OperationRunnerProps = OperationRunnerPropsFields & ComponentPropsWithoutRef<"section">;
type Operation = OperationRunnerPropsFields["operations"][number];

export function operationRequestPath(
	path: string,
	values: Partial<Record<string, string>>,
	query: string,
) {
	const resolved = path.replace(/\{([^}]+)\}/g, (_segment, name: string) => {
		const value = values[name]?.trim();
		if (!value) throw new Error(`${name} is required`);
		return encodeURIComponent(value);
	});
	const search = query.trim().replace(/^\?/, "");
	return `/api/backend${resolved}${search ? `?${search}` : ""}`;
}

export function OperationRunner({ operations, className, ...props }: OperationRunnerProps) {
	const formId = useId();
	const [id, setId] = useState(operations[0]?.id ?? "");
	const [values, setValues] = useState<Partial<Record<string, string>>>({});
	const [query, setQuery] = useState("");
	const [body, setBody] = useState('{"to_email":"person@example.com"}');
	const [file, setFile] = useState<File | null>(null);
	const [headers, setHeaders] = useState<Partial<Record<string, string>>>({});
	const selected = operations.find((operation) => operation.id === id);
	const pathParams = selected
		? [...selected.path.matchAll(/\{([^}]+)\}/g)].map((match) => match[1])
		: [];
	const media = selected?.requestMedia[0];

	const mutation = useMutation({
		mutationFn: async (operation: Operation) => {
			const requestHeaders: Record<string, string> = {};
			for (const [name, value] of Object.entries(headers))
				if (value?.trim()) requestHeaders[name] = value.trim();
			let requestBody: BodyInit | undefined;
			if (media === "application/json") {
				try {
					JSON.parse(body);
				} catch {
					throw new Error("Request body must be valid JSON");
				}
				requestHeaders["content-type"] = "application/json";
				requestBody = body;
			} else if (media === "multipart/form-data") {
				if (!file) throw new Error("Choose a CSV file");
				const form = new FormData();
				form.append("file", file);
				for (const name of ["name", "email_column", "source_key"]) {
					const value = values[name]?.trim();
					if (value) form.append(name, value);
				}
				requestBody = form;
			}
			const response = await dashboardRequest(operationRequestPath(operation.path, values, query), {
				method: operation.method,
				headers: requestHeaders,
				body: requestBody,
				timeoutMs: media === "multipart/form-data" ? 120_000 : 30_000,
			});
			redirectBrowserIfUnauthorized(response.status);
			const contentType = response.headers.get("content-type") ?? "";
			if (
				response.ok &&
				(response.headers.get("content-disposition")?.includes("attachment") ||
					(!contentType.includes("json") && !contentType.startsWith("text/")))
			) {
				const blob = await response.blob();
				const url = URL.createObjectURL(blob);
				const anchor = document.createElement("a");
				anchor.href = url;
				anchor.download = (
					response.headers.get("content-disposition")?.match(/filename="?([^";]+)"?/)?.[1] ??
					"download"
				).replace(/[^\w.-]/g, "_");
				anchor.click();
				setTimeout(() => {
					URL.revokeObjectURL(url);
				}, 1_000);
				return `Downloaded ${anchor.download} (${String(blob.size)} bytes)`;
			}
			const text = await response.text();
			let formatted = text;
			if (contentType.includes("json")) {
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
			(selected.method === "DELETE" ||
				/\/(cancel|retry|pause|resume|trigger|push)$/.test(selected.path)) &&
			!window.confirm(`Run ${selected.method} ${selected.path} in the active workspace?`)
		)
			return;
		mutation.mutate(selected);
	}

	return (
		<section
			data-slot="operation-runner"
			className={cn("grid gap-6 lg:grid-cols-2", className)}
			{...props}
		>
			<form
				onSubmit={submit}
				className="flex flex-col gap-4 rounded-lg border border-border bg-background p-5"
			>
				<label htmlFor={`${formId}-operation`} className="flex flex-col gap-2 text-sm">
					Operation
					<select
						id={`${formId}-operation`}
						className="rounded border border-border bg-background p-2"
						value={id}
						onChange={(event) => {
							setId(event.target.value);
							setValues({});
							setQuery("");
							setBody(
								event.target.value === "v1_check_email"
									? '{"to_email":"person@example.com"}'
									: "{}",
							);
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
						<p className="text-sm text-muted-foreground">
							{selected.description ?? selected.id} · Scope:{" "}
							{selected.scope ?? "authenticated workspace"}
						</p>
						<Link className="text-sm underline" href={`/developers/reference/${selected.id}`}>
							Request and response schema
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
								placeholder="page=1&limit=50"
								value={query}
								onChange={(event) => {
									setQuery(event.target.value);
								}}
							/>
						</label>
						{media === "application/json" ? (
							<label htmlFor={`${formId}-body`} className="flex flex-col gap-1 text-sm">
								JSON body
								<textarea
									id={`${formId}-body`}
									aria-label="JSON body"
									className="min-h-40 rounded border border-border bg-background p-2 font-mono text-xs"
									spellCheck={false}
									value={body}
									onChange={(event) => {
										setBody(event.target.value);
									}}
								/>
							</label>
						) : null}
						{media === "multipart/form-data" ? (
							<>
								<label htmlFor={`${formId}-file`} className="flex flex-col gap-1 text-sm">
									CSV file
									<input
										id={`${formId}-file`}
										aria-label="CSV file"
										required
										type="file"
										accept=".csv,text/csv"
										onChange={(event) => {
											setFile(event.target.files?.[0] ?? null);
										}}
									/>
								</label>
								{["name", "email_column", "source_key"].map((name) => (
									<label
										key={name}
										htmlFor={`${formId}-field-${name}`}
										className="flex flex-col gap-1 text-sm"
									>
										{name}
										<input
											id={`${formId}-field-${name}`}
											aria-label={name}
											className="rounded border border-border bg-background p-2"
											value={values[name] ?? ""}
											onChange={(event) => {
												setValues({ ...values, [name]: event.target.value });
											}}
										/>
									</label>
								))}
							</>
						) : null}
						<details>
							<summary className="cursor-pointer text-sm">Request headers</summary>
							<div className="mt-3 grid gap-3">
								{[
									"x-idempotency-key",
									"if-match",
									"x-approval-token",
									"x-continuation-token",
									"range",
								].map((name) => (
									<label
										key={name}
										htmlFor={`${formId}-header-${name}`}
										className="flex flex-col gap-1 text-sm"
									>
										{name}
										<input
											id={`${formId}-header-${name}`}
											aria-label={name}
											className="rounded border border-border bg-background p-2"
											value={headers[name] ?? ""}
											onChange={(event) => {
												setHeaders({ ...headers, [name]: event.target.value });
											}}
										/>
									</label>
								))}
							</div>
						</details>
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
					<pre className="overflow-auto whitespace-pre-wrap text-sm text-destructive">
						{mutation.error instanceof Error ? mutation.error.message : "Request failed"}
					</pre>
				) : null}
				{mutation.isSuccess ? (
					<pre className="overflow-auto whitespace-pre-wrap font-mono text-xs">{mutation.data}</pre>
				) : null}
				{mutation.isIdle ? (
					<p className="text-sm text-muted-foreground">
						Select an operation and run it to see the result.
					</p>
				) : null}
			</div>
		</section>
	);
}
