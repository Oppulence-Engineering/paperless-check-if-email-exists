"use client";

import "client-only";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import {
	Configuration,
	V1Api,
	type CreateResearchRequest,
	type ResearchCandidate,
	type ResearchExportMode,
} from "@oppulence/reacher-sdk";
import { useRef, useState, type ComponentPropsWithoutRef } from "react";
import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { Textarea } from "@oppulence/ui/components/textarea";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@oppulence/ui/components/select";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	V1CreateResearch200Response,
	V1ListResearch200Response,
	V1GetResearch200Response,
	V1ExportResearch200Response,
} from "@/lib/api/generated/zod/v1/v1";
import { type ResearchPanelPropsFields } from "./research-panel.schema";

/** @oppulence-gen kind=component Owned by `research-panel.lit.ts`. */
export type ResearchPanelProps = ResearchPanelPropsFields & ComponentPropsWithoutRef<"section">;
const api = new V1Api(new Configuration({ basePath: "/api/backend" }));
const REASONS: Record<string, string> = {
	published_person_and_matching_title_with_verified_mailbox:
		"The company publishes this person, role and address. The mailbox passed verification.",
	company_domain_required: "Add a business domain to identify the company.",
	company_evidence_unavailable: "We could not obtain usable evidence from the company website.",
	verification_limit_reached: "The approved verification limit was reached.",
	verification_unavailable:
		"Verification could not complete. Review this contact before trying again.",
	verification_outcome_unresolved:
		"A previous verification has an unresolved outcome. We did not submit another paid request.",
	verification_evidence_invalid: "The verification result could not be confirmed.",
	catch_all_requires_review:
		"This domain accepts arbitrary addresses. The individual mailbox remains uncertain.",
	contact_suppressed: "The sending policy blocks this contact.",
	person_or_role_requires_review:
		"The mailbox works, but the person, employment or role still needs confirmation.",
	no_usable_verified_match: "No checked address met the sending requirements.",
	no_matching_contact_published_on_checked_pages:
		"No matching contact was found on the pages checked.",
	research_interrupted: "Research was interrupted. Review the available evidence.",
	research_did_not_complete: "Research did not complete. This row needs review.",
	incomplete_verification_evidence: "The available evidence is incomplete.",
	research_pending: "Research is queued or in progress.",
};
const STATUSES: Record<string, string> = {
	ready: "Ready",
	needs_review: "Needs review",
	no_match: "No match",
	queued: "Queued",
	running: "Researching",
};

export function researchRefetchInterval(status?: string): number | false {
	return status === "running" ? 5_000 : false;
}

function Evidence({ candidate }: { candidate: ResearchCandidate }) {
	const source =
		candidate.evidence_url && /^https?:\/\//i.test(candidate.evidence_url)
			? candidate.evidence_url
			: undefined;
	return (
		<article className="space-y-3 rounded-lg border border-white/10 p-4 text-sm">
			<div>
				<p className="font-medium text-white">
					{candidate.name || "Unidentified person"}
					{candidate.title ? ` · ${candidate.title}` : ""}
				</p>
				<p className="break-all text-neutral-300">{candidate.email}</p>
			</div>
			<dl className="grid gap-3 text-xs sm:grid-cols-2 lg:grid-cols-4">
				<div>
					<dt className="text-neutral-500">Person</dt>
					<dd>
						{candidate.person_status === "published" ? "Named on company website" : "Unconfirmed"}
					</dd>
				</div>
				<div>
					<dt className="text-neutral-500">Employment</dt>
					<dd>
						{candidate.employment_status === "published_on_company_site"
							? "Listed on company website; not independently confirmed"
							: "Unconfirmed"}
					</dd>
				</div>
				<div>
					<dt className="text-neutral-500">Role</dt>
					<dd>
						{candidate.role_status === "title_matches"
							? "Published title matches target"
							: "Needs confirmation"}
					</dd>
				</div>
				<div>
					<dt className="text-neutral-500">Mailbox</dt>
					<dd>
						{candidate.mailbox
							? `${candidate.mailbox.result} · ${candidate.mailbox.action || "review"}${candidate.mailbox.catch_all ? " · catch-all" : ""}`
							: "Not verified"}
					</dd>
				</div>
			</dl>
			<p className="text-xs text-neutral-400">
				Address:{" "}
				{candidate.email_source === "published"
					? "published by the company"
					: candidate.email_source === "pattern"
						? "inferred from a name pattern; ownership unconfirmed"
						: "provided in your import"}
				.
			</p>
			{candidate.verification_error ? (
				<p className="text-xs text-amber-300">
					{REASONS[candidate.verification_error] || "Verification needs review."}
				</p>
			) : null}
			<div className="flex flex-wrap gap-3 text-xs text-neutral-400">
				{source ? (
					<a className="text-blue-300 underline" href={source} target="_blank" rel="noreferrer">
						Company source
					</a>
				) : (
					<span>No published source</span>
				)}
				{candidate.observed_at ? <span>Observed {candidate.observed_at}</span> : null}
			</div>
		</article>
	);
}

export function ResearchPanel({ initialSessionId, className, ...props }: ResearchPanelProps) {
	const client = useQueryClient();
	const [name, setName] = useState("");
	const [role, setRole] = useState("CEO");
	const [content, setContent] = useState("");
	const [format, setFormat] = useState<"csv" | "json">("csv");
	const [candidates, setCandidates] = useState(2);
	const [limit, setLimit] = useState(10);
	const [sessionId, setSessionId] = useState(initialSessionId);
	const [selected, setSelected] = useState<string[]>([]);
	const [message, setMessage] = useState("");
	const [validation, setValidation] = useState("");
	const submitted = useRef<{ fingerprint: string; key: string } | null>(null);
	const sessions = useQuery({
		queryKey: workflowQueryKey("research", "sessions"),
		queryFn: async () => V1ListResearch200Response.parse((await api.v1ListResearch()).data),
	});
	const create = useMutation({
		mutationFn: async (request: CreateResearchRequest) =>
			V1CreateResearch200Response.parse(
				(await api.v1CreateResearch({ createResearchRequest: request })).data,
			),
		onSuccess: (data) => {
			setSessionId(data.id);
			setSelected([]);
			setMessage("");
			void client.invalidateQueries({
				queryKey: workflowQueryKey("research", "sessions"),
			});
		},
	});
	const result = useQuery({
		queryKey: workflowQueryKey("research", sessionId),
		enabled: !!sessionId,
		queryFn: async () => {
			if (!sessionId || !/^[1-9]\d*$/.test(sessionId))
				throw new Error("Select a valid research run.");
			return V1GetResearch200Response.parse((await api.v1GetResearch({ sessionId })).data);
		},
		refetchInterval: (query) => researchRefetchInterval(query.state.data?.status),
	});
	const download = useMutation({
		mutationFn: async (mode: ResearchExportMode) => {
			if (!sessionId || !/^[1-9]\d*$/.test(sessionId) || !selected.length)
				throw new Error("Select contacts to export.");
			return V1ExportResearch200Response.parse(
				(
					await api.v1ExportResearch({
						sessionId,
						researchExportRequest: { mode, row_keys: selected },
					})
				).data,
			);
		},
		onSuccess: (data, mode) => {
			if (data.exported_rows > 0) {
				const url = URL.createObjectURL(new Blob([data.csv], { type: "text/csv;charset=utf-8" }));
				const link = document.createElement("a");
				link.href = url;
				link.download = data.filename;
				link.click();
				setTimeout(() => {
					URL.revokeObjectURL(url);
				}, 1_000);
			}
			setMessage(
				mode === "review"
					? `Rows exported for review: ${String(data.exported_rows)}. This is not an approved sending list.`
					: `${String(data.exported_rows)} contact rows exported. ${String(data.excluded_row_keys.length)} selected rows excluded by the current sending requirements.`,
			);
		},
	});
	const readySelected = result.data?.rows.some(
		(row) => row.status === "ready" && selected.includes(row.row_key),
	);
	const error =
		validation ||
		(create.isError
			? "Could not start research. Check the import and limits, then retry the same request. Contact support if it continues."
			: result.isError
				? "Could not load this research run. Refresh to try again."
				: download.isError
					? "The export could not complete. No contacts were downloaded."
					: sessions.isError
						? "Previous research runs could not be loaded."
						: "");
	return (
		<Page
			title="Company research"
			description="Turn a company list into contacts with evidence you can review before export."
		>
			<section data-slot="research-panel" className={cn("space-y-8", className)} {...props}>
				<Section
					title="Start research"
					description="Pilot: up to 10 rows per run. We check company websites and verify candidate mailboxes. A title match does not prove purchasing authority."
				>
					<form
						onSubmit={(event) => {
							event.preventDefault();
							setValidation("");
							setMessage("");
							if (new TextEncoder().encode(content).length > 1_048_576) {
								setValidation("The import must be 1 MB or smaller.");
								return;
							}
							const input = {
								name: name.trim(),
								target_role: role.trim(),
								import_content: content,
								import_format: format,
								max_candidates: candidates,
								max_verifications: limit,
							};
							const fingerprint = JSON.stringify(input);
							if (submitted.current?.fingerprint !== fingerprint)
								submitted.current = {
									fingerprint,
									key: crypto.randomUUID(),
								};
							create.mutate({
								...input,
								idempotency_key: submitted.current.key,
							});
						}}
					>
						<Group>
							<Row>
								<div className="grid gap-4 md:grid-cols-2">
									<div className="space-y-2">
										<Label htmlFor="research-name">Run name</Label>
										<Input
											id="research-name"
											required
											maxLength={100}
											value={name}
											onChange={(event) => {
												setName(event.target.value);
											}}
											placeholder="Regional manufacturers"
										/>
									</div>
									<div className="space-y-2">
										<Label htmlFor="research-role">Target role</Label>
										<Input
											id="research-role"
											required
											maxLength={120}
											value={role}
											onChange={(event) => {
												setRole(event.target.value);
											}}
											placeholder="CEO"
										/>
									</div>
									<div className="space-y-2">
										<Label htmlFor="research-format">Import format</Label>
										<Select
											value={format}
											onValueChange={(value) => {
												setFormat(value === "json" ? "json" : "csv");
											}}
										>
											<SelectTrigger id="research-format">
												<SelectValue />
											</SelectTrigger>
											<SelectContent>
												<SelectItem value="csv">CSV</SelectItem>
												<SelectItem value="json">JSON</SelectItem>
											</SelectContent>
										</Select>
									</div>
									<div className="space-y-2">
										<Label htmlFor="research-file">Upload a company list</Label>
										<Input
											id="research-file"
											type="file"
											accept=".csv,.json,text/csv,application/json"
											onChange={(event) => {
												const file = event.target.files?.[0];
												if (!file) return;
												if (file.size > 1_048_576) {
													setValidation("The import must be 1 MB or smaller.");
													event.target.value = "";
													return;
												}
												void file
													.text()
													.then((text) => {
														setContent(text);
														setFormat(file.name.toLowerCase().endsWith(".json") ? "json" : "csv");
														setValidation("");
													})
													.catch(() => {
														setValidation("Could not read the file. Paste its contents below.");
													});
											}}
										/>
									</div>
								</div>
								<div className="space-y-2">
									<Label htmlFor="research-content">Company data</Label>
									<Textarea
										id="research-content"
										required
										rows={5}
										value={content}
										onChange={(event) => {
											setContent(event.target.value);
										}}
										placeholder={
											"company_name,company_domain,contact_name,contact_title,contact_email\nAcme,acme.com,,,"
										}
										aria-describedby="research-import-help"
									/>
									<p id="research-import-help" className="text-xs text-neutral-400">
										Use company_name and company_domain. Optional contact_name, contact_title and
										contact_email support list repair. JSON accepts an array of objects with the
										same fields. Oversized lists are rejected in full.
									</p>
								</div>
								<div className="grid gap-4 md:grid-cols-2">
									<div className="space-y-2">
										<Label htmlFor="research-candidates">Maximum candidates per company</Label>
										<Input
											id="research-candidates"
											type="number"
											required
											min={1}
											max={3}
											value={candidates}
											onChange={(event) => {
												setCandidates(Number(event.target.value));
											}}
										/>
									</div>
									<div className="space-y-2">
										<Label htmlFor="research-limit">Maximum verifications for this run</Label>
										<Input
											id="research-limit"
											type="number"
											required
											min={1}
											max={30}
											value={limit}
											onChange={(event) => {
												setLimit(Number(event.target.value));
											}}
										/>
									</div>
								</div>
								<p className="text-xs text-neutral-400">
									Verification uses your existing allowance. We stop at your limit; unresolved
									requests count toward it. Website research adds no third-party research charge.
								</p>
								<div className="flex flex-wrap gap-3">
									<Button type="submit" disabled={create.isPending}>
										{create.isPending
											? "Starting…"
											: `Research with up to ${String(limit)} verification${limit === 1 ? "" : "s"}`}
									</Button>
									<Button
										type="button"
										variant="outline"
										disabled={create.isPending}
										onClick={() => {
											submitted.current = null;
											create.reset();
											setSessionId(undefined);
											setSelected([]);
											setMessage("");
										}}
									>
										Start a separate run
									</Button>
								</div>
							</Row>
						</Group>
					</form>
				</Section>
				{error ? (
					<p role="alert" className="text-sm text-red-300">
						{error}
					</p>
				) : null}
				{message ? (
					<p role="status" className="text-sm text-neutral-300">
						{message}
					</p>
				) : null}
				{sessions.data?.sessions.length ? (
					<Section title="Previous runs">
						<div className="flex flex-wrap gap-2">
							{sessions.data.sessions.map((session) => (
								<Button
									key={session.id}
									variant={session.id === sessionId ? "default" : "outline"}
									onClick={() => {
										setSessionId(session.id);
										setSelected([]);
										setMessage("");
									}}
								>
									{session.name || `Research ${session.id}`}
								</Button>
							))}
						</div>
					</Section>
				) : null}
				{result.isFetching && !result.data ? <p role="status">Loading research…</p> : null}
				{result.data ? (
					<Section
						title={result.data.session.name}
						description={`${result.data.target_role} · ${String(result.data.verification_requests)} of ${String(result.data.max_verifications)} verification requests reserved or used · ${result.data.status === "running" ? "Research in progress" : "Research complete"}`}
					>
						<div className="flex flex-wrap items-center gap-3">
							<Button
								variant="outline"
								onClick={() => {
									setSelected(
										result.data.rows
											.filter((row) => row.status === "ready")
											.map((row) => row.row_key),
									);
								}}
							>
								Select ready contacts
							</Button>
							<Button
								disabled={!readySelected || download.isPending}
								onClick={() => {
									download.mutate("approved");
								}}
							>
								Approve and export ready contacts
							</Button>
							<Button
								variant="outline"
								disabled={!selected.length || download.isPending}
								onClick={() => {
									download.mutate("review");
								}}
							>
								Export selected for review
							</Button>
							<Button
								variant="ghost"
								disabled={result.isFetching}
								onClick={() => {
									void result.refetch();
								}}
							>
								Refresh
							</Button>
						</div>
						<p className="text-xs text-neutral-400">
							Approved export rechecks current verification history, freshness and suppression
							policy. Review exports retain uncertainty and are not a sending list.
						</p>
						<Group>
							{result.data.rows.map((row) => (
								<Row key={row.row_key}>
									<div className="flex items-start gap-3">
										<Checkbox
											id={`research-${row.row_key}`}
											checked={selected.includes(row.row_key)}
											onCheckedChange={(checked) => {
												setSelected((keys) =>
													checked === true
														? [...keys.filter((key) => key !== row.row_key), row.row_key]
														: keys.filter((key) => key !== row.row_key),
												);
											}}
										/>
										<div className="flex-1 space-y-2">
											<div className="flex flex-wrap items-center gap-3">
												<Label htmlFor={`research-${row.row_key}`}>
													{row.company_name || row.company_domain || "Imported company"}
												</Label>
												<Pill
													tone={
														row.status === "ready"
															? "green"
															: row.status === "needs_review"
																? "yellow"
																: "gray"
													}
												>
													{STATUSES[row.status] || "Needs review"}
												</Pill>
											</div>
											<p className="text-xs text-neutral-400">
												{REASONS[row.reason] || "Review the evidence before using this contact."}
											</p>
										</div>
									</div>
									{row.candidates.map((candidate) => (
										<Evidence key={candidate.email} candidate={candidate} />
									))}
									{row.checked_pages.length ? (
										<details className="text-xs text-neutral-400">
											<summary className="cursor-pointer">
												{row.checked_pages.length} company pages checked
											</summary>
											<ul className="mt-2 space-y-1">
												{row.checked_pages.map((url) => (
													<li className="break-all" key={url}>
														{url}
													</li>
												))}
											</ul>
										</details>
									) : null}
								</Row>
							))}
						</Group>
					</Section>
				) : null}
			</section>
		</Page>
	);
}
