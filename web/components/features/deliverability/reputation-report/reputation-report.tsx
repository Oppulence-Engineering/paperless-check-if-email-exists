import type { ComponentPropsWithoutRef, ReactNode } from "react";

import { cn } from "@oppulence/ui/lib/utils";

import { Pill, Row, type PillTone } from "@/components/platform/page-kit";
import { dmarcPolicy, reputationRisk } from "@/lib/verification/labels";

import { type ReputationReportPropsFields } from "./reputation-report.schema";

/**
 * @oppulence-gen kind=component
 * ReputationReport is a server presentation component.
 * Shows the result of a domain reputation check.
 *
 * It shows rows without a card frame, so put it in a `Group`. Data arrives
 * through validated props. This module does not fetch, persist, or read
 * cookies. Owned by `reputation-report.lit.ts`.
 */
export type ReputationReportProps = ReputationReportPropsFields & {
	/** A control for the score row, for example a "Check again" button. */
	action?: ReactNode;
} & ComponentPropsWithoutRef<"section">;

type Check = { label: string; tone: PillTone; value: string };

const FOUND = { tone: "green", value: "Found" } as const;
const MISSING = { tone: "red", value: "Not found" } as const;

export function ReputationReport({ action, className, report, ...props }: ReputationReportProps) {
	const dns = report.dns_records;
	const info = report.domain_info;
	const listed = report.blacklist_results.filter((result) => result.listed === true).length;
	const unknown = report.blacklist_results.filter((result) => result.listed == null).length;
	const checks: Check[] = [
		{
			label: "SPF",
			...(dns.has_spf ? { tone: "gray", value: "Record found · syntax not checked" } : MISSING),
		},
		// Selector coverage is incomplete; no discovered key means unknown.
		{
			label: "DKIM",
			...(dns.has_dkim === true
				? { ...FOUND, value: "Key detected" }
				: { tone: "gray", value: "Unknown" }),
		},
		{
			label: "DMARC policy",
			...(dns.has_dmarc
				? { tone: dmarcPolicy.tone(dns.dmarc_policy), value: dmarcPolicy.label(dns.dmarc_policy) }
				: MISSING),
		},
		{
			label: "MX records",
			...(dns.has_mx
				? { tone: "green", value: `${String(dns.mx_records.length)} found` }
				: MISSING),
		},
		{
			label: "Blocklists",
			...(!dns.has_mx
				? { tone: "gray", value: "Not checked" }
				: listed
					? { tone: "red", value: `Listed on ${String(listed)} · ${String(unknown)} unknown` }
					: unknown
						? { tone: "yellow", value: `Incomplete · ${String(unknown)} unknown` }
						: { tone: "green", value: "Not listed" }),
		},
	];
	const registration = [
		[
			"Domain age",
			info.domain_age_days == null ? null : `${info.domain_age_days.toLocaleString()} days`,
		],
		["Registrar", info.registrar],
		["Registered on", info.created_at ? new Date(info.created_at).toLocaleDateString() : null],
	] as const;

	return (
		<section
			data-slot="reputation-report"
			className={cn("flex flex-col divide-y divide-white/[0.07]", className)}
			{...props}
		>
			<Row
				description={`${report.domain} · ${report.cached ? "Saved result. A saved result can be up to 24 hours old." : "New result."}`}
				label="Health score"
			>
				<span className="text-sm text-white tabular-nums">{report.score}/100</span>
				<Pill tone={reputationRisk.tone(report.risk_level)}>
					{reputationRisk.label(report.risk_level)}
				</Pill>
				{action}
			</Row>
			{unknown > 0 ? (
				<Row>
					<p role="status" className="text-sm text-amber-300">
						Blocklist coverage is incomplete. This score does not confirm a clean result across all
						providers.
					</p>
				</Row>
			) : null}
			<Row>
				<dl className="grid grid-cols-2 gap-x-6 gap-y-4 lg:grid-cols-4">
					{checks.map((check) => (
						<div key={check.label}>
							<dt className="text-xs text-[#858585]">{check.label}</dt>
							<dd className="mt-1">
								<Pill tone={check.tone}>{check.value}</Pill>
							</dd>
						</div>
					))}
					{registration.map(([label, value]) => (
						<div key={label}>
							<dt className="text-xs text-[#858585]">{label}</dt>
							<dd className="mt-1 text-sm break-words text-white">{value || "Not available"}</dd>
						</div>
					))}
				</dl>
				{!dns.has_dkim || !dns.has_mx ? (
					<ul className="space-y-1 text-xs text-[#858585]">
						{!dns.has_dkim ? (
							<li>
								DKIM: only common selectors were checked. Another selector may contain a key. This
								check does not verify a message signature.
							</li>
						) : null}
						{!dns.has_mx ? (
							<li>Blocklists: the check needs an MX record, so it did not run.</li>
						) : null}
					</ul>
				) : null}
			</Row>
			{dns.mx_records.length ? (
				<Row label="MX records" layout="stacked">
					<ul className="space-y-1 font-mono text-xs break-all text-neutral-300">
						{dns.mx_records.map((record) => (
							<li key={record}>
								{/* A single "." is a null MX record (RFC 7505): the domain accepts no mail. */}
								{record === "."
									? ". (null MX: the domain does not accept mail)"
									: record.replace(/\.$/, "")}
							</li>
						))}
					</ul>
				</Row>
			) : null}
			{dns.has_mx && report.blacklist_results.length ? (
				<Row
					description="The check looks up the IP address of the first MX record on each list."
					label="Blocklists"
					layout="stacked"
				>
					<ul className="grid gap-x-6 gap-y-2 sm:grid-cols-2">
						{report.blacklist_results.map((result) => (
							<li
								className="flex items-center justify-between gap-3 text-sm text-neutral-300"
								key={result.provider}
							>
								{result.provider}
								<Pill tone={result.listed == null ? "gray" : result.listed ? "red" : "green"}>
									{result.error === "not_configured"
										? "Unavailable"
										: result.status === "skipped"
											? "Not checked"
											: result.listed == null
												? "Query failed"
												: result.listed
													? "Listed"
													: "Not listed"}
								</Pill>
							</li>
						))}
					</ul>
				</Row>
			) : null}
		</section>
	);
}
