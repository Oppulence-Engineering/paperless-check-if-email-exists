"use client";

import "client-only";

import { useMutation } from "@tanstack/react-query";
import Link from "next/link";
import { useId, useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { dashboardRequest, redirectBrowserIfUnauthorized } from "@/lib/auth/dashboard-fetch";
import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { NativeSelect, NativeSelectOption } from "@oppulence/ui/components/native-select";
import { Textarea } from "@oppulence/ui/components/textarea";
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
				className="flex flex-col gap-4 rounded-2xl bg-white/[0.015] p-5 ring-1 ring-white/[0.08]"
			>
				<Label
					htmlFor={`${formId}-operation`}
					className="flex-col items-stretch gap-1.5 text-neutral-400"
				>
					Operation
					<NativeSelect
						id={`${formId}-operation`}
						className="h-10 rounded-lg"
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
							<NativeSelectOption key={operation.id} value={operation.id}>
								{operation.method} {operation.path}
							</NativeSelectOption>
						))}
					</NativeSelect>
				</Label>
				{selected ? (
					<>
						<p className="text-sm text-[#858585]">
							{selected.description ?? selected.id} · Scope:{" "}
							{selected.scope ?? "authenticated workspace"}
						</p>
						<Link
							className="text-sm text-neutral-300 underline hover:text-white"
							href={`/developers/reference/${selected.id}`}
						>
							Request and response schema
						</Link>
						{pathParams.map((name) => (
							<Label
								key={name}
								htmlFor={`${formId}-path-${name}`}
								className="flex-col items-stretch gap-1.5 text-neutral-400"
							>
								{name}
								<Input
									id={`${formId}-path-${name}`}
									aria-label={name}
									required
									className="h-10 rounded-lg"
									value={values[name] ?? ""}
									onChange={(event) => {
										setValues({ ...values, [name]: event.target.value });
									}}
								/>
							</Label>
						))}
						<Label
							htmlFor={`${formId}-query`}
							className="flex-col items-stretch gap-1.5 text-neutral-400"
						>
							Query string
							<Input
								id={`${formId}-query`}
								aria-label="Query string"
								className="h-10 rounded-lg"
								placeholder="page=1&limit=50"
								value={query}
								onChange={(event) => {
									setQuery(event.target.value);
								}}
							/>
						</Label>
						{media === "application/json" ? (
							<Label
								htmlFor={`${formId}-body`}
								className="flex-col items-stretch gap-1.5 text-neutral-400"
							>
								JSON body
								<Textarea
									id={`${formId}-body`}
									aria-label="JSON body"
									className="min-h-40 rounded-lg font-mono text-xs md:text-xs"
									spellCheck={false}
									value={body}
									onChange={(event) => {
										setBody(event.target.value);
									}}
								/>
							</Label>
						) : null}
						{media === "multipart/form-data" ? (
							<>
								<Label
									htmlFor={`${formId}-file`}
									className="flex-col items-stretch gap-1.5 text-neutral-400"
								>
									CSV file
									<Input
										id={`${formId}-file`}
										aria-label="CSV file"
										required
										type="file"
										accept=".csv,text/csv"
										onChange={(event) => {
											setFile(event.target.files?.[0] ?? null);
										}}
									/>
								</Label>
								{["name", "email_column", "source_key"].map((name) => (
									<Label
										key={name}
										htmlFor={`${formId}-field-${name}`}
										className="flex-col items-stretch gap-1.5 text-neutral-400"
									>
										{name}
										<Input
											id={`${formId}-field-${name}`}
											aria-label={name}
											className="h-10 rounded-lg"
											value={values[name] ?? ""}
											onChange={(event) => {
												setValues({ ...values, [name]: event.target.value });
											}}
										/>
									</Label>
								))}
							</>
						) : null}
						<details>
							<summary className="cursor-pointer text-sm text-neutral-400 hover:text-neutral-200">
								Request headers
							</summary>
							<div className="mt-3 grid gap-3">
								{[
									"x-idempotency-key",
									"if-match",
									"x-approval-token",
									"x-continuation-token",
									"range",
								].map((name) => (
									<Label
										key={name}
										htmlFor={`${formId}-header-${name}`}
										className="flex-col items-stretch gap-1.5 text-neutral-400"
									>
										{name}
										<Input
											id={`${formId}-header-${name}`}
											aria-label={name}
											className="h-10 rounded-lg"
											value={headers[name] ?? ""}
											onChange={(event) => {
												setHeaders({ ...headers, [name]: event.target.value });
											}}
										/>
									</Label>
								))}
							</div>
						</details>
						<Button className="rounded-lg" type="submit" disabled={mutation.isPending}>
							{mutation.isPending ? "Running…" : `Run ${selected.method}`}
						</Button>
					</>
				) : null}
			</form>
			<div
				aria-live="polite"
				className="min-h-64 rounded-2xl bg-white/[0.015] p-5 ring-1 ring-white/[0.08]"
			>
				<h2 className="mb-3 text-sm font-medium text-white">Response</h2>
				{mutation.isError ? (
					<pre className="overflow-auto whitespace-pre-wrap text-sm text-red-300">
						{mutation.error instanceof Error ? mutation.error.message : "Request failed"}
					</pre>
				) : null}
				{mutation.isSuccess ? (
					<pre className="overflow-auto whitespace-pre-wrap font-mono text-xs text-neutral-300">
						{mutation.data}
					</pre>
				) : null}
				{mutation.isIdle ? (
					<p className="text-sm text-[#858585]">
						Select an operation and run it to see the result.
					</p>
				) : null}
			</div>
		</section>
	);
}
