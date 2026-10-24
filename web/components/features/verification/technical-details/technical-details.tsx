import { freeProviderOf } from "@/lib/verification/check-result";
import type { ComponentPropsWithoutRef, ReactNode } from "react";

import { cn } from "@oppulence/ui/lib/utils";

import { Group, Row } from "@/components/platform/page-kit";
import { miscDetails, mxDetails, smtpDetails } from "@/lib/verification/check-result";

import { type TechnicalDetailsPropsFields } from "./technical-details.schema";

/**
 * @oppulence-gen kind=component
 * TechnicalDetails is a server presentation component.
 * Shows the syntax, mail server, SMTP, and account details of one check.
 * It replaces the raw JSON view and starts closed.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `technical-details.lit.ts`.
 */
export type TechnicalDetailsProps = TechnicalDetailsPropsFields &
	ComponentPropsWithoutRef<"section">;

function yesNo(value: boolean | null | undefined): string {
	if (value === null || value === undefined) return "Not checked";
	return value ? "Yes" : "No";
}

function Facts({ items }: { items: [string, ReactNode][] }) {
	return (
		<dl className="grid gap-x-6 gap-y-2 text-sm sm:grid-cols-2">
			{items.map(([label, value]) => (
				<div className="flex justify-between gap-4" key={label}>
					<dt className="text-[#858585]">{label}</dt>
					<dd className="text-right text-neutral-200">{value}</dd>
				</div>
			))}
		</dl>
	);
}

function CheckError({ error }: { error: { type: string; message: string } }) {
	return (
		<p className="text-sm text-red-300">
			{error.type}: {error.message}
		</p>
	);
}

export function TechnicalDetails({ className, result, ...props }: TechnicalDetailsProps) {
	const mx = mxDetails(result);
	const smtp = smtpDetails(result);
	const misc = miscDetails(result);
	const debug = result.debug;
	const seconds = debug ? debug.duration.secs + debug.duration.nanos / 1e9 : null;

	return (
		<section data-slot="technical-details" className={cn(className)} {...props}>
			<details className="group">
				<summary className="flex cursor-pointer list-none items-center justify-between rounded-2xl bg-white/[0.015] px-5 py-4 text-sm font-medium text-white ring-1 ring-white/[0.08] group-open:rounded-b-none [&::-webkit-details-marker]:hidden">
					Technical details
					<span className="text-xs text-[#858585] group-open:hidden">Show</span>
					<span className="hidden text-xs text-[#858585] group-open:inline">Hide</span>
				</summary>
				<Group className="rounded-t-none">
					<Row label="Address format" layout="stacked">
						<Facts
							items={[
								["Valid format", yesNo(result.syntax.is_valid_syntax)],
								["Name", result.syntax.username || "None"],
								["Domain", result.syntax.domain || "None"],
								[
									"Normalized",
									result.syntax.normalized_email ?? result.score.normalized_email ?? "None",
								],
							]}
						/>
					</Row>
					<Row label="Mail servers (MX)" layout="stacked">
						{result.external_verification ? (
							<p className="text-sm text-neutral-300">
								Individual MX diagnostics were not returned by the verification service.
							</p>
						) : mx.error ? (
							<CheckError error={mx.error} />
						) : (
							<div className="flex flex-col gap-2">
								<Facts items={[["Accepts mail", yesNo(mx.details?.accepts_mail)]]} />
								{mx.details?.records.length ? (
									<ul className="font-mono text-xs text-neutral-300">
										{mx.details.records.map((record) => (
											<li key={record}>{record}</li>
										))}
									</ul>
								) : null}
							</div>
						)}
					</Row>
					<Row label="Mail server answers (SMTP)" layout="stacked">
						{result.external_verification ? (
							<p className="text-sm text-neutral-300">
								Checked by{" "}
								{result.external_verification.service === "millionverifier"
									? "MillionVerifier"
									: result.external_verification.service}
								. Direct mail-server diagnostics are unavailable.
							</p>
						) : smtp.error ? (
							<CheckError error={smtp.error} />
						) : (
							<Facts
								items={[
									["Server answers", yesNo(smtp.details?.can_connect_smtp)],
									["Mailbox accepts mail", yesNo(smtp.details?.is_deliverable)],
									["Accepts all addresses", yesNo(smtp.details?.is_catch_all)],
									["Mailbox is full", yesNo(smtp.details?.has_full_inbox)],
									["Mailbox is disabled", yesNo(smtp.details?.is_disabled)],
								]}
							/>
						)}
					</Row>
					<Row label="Account" layout="stacked">
						{misc.error ? (
							<CheckError error={misc.error} />
						) : (
							<Facts
								items={[
									["Disposable", yesNo(misc.details?.is_disposable)],
									["Role address", yesNo(misc.details?.is_role_account)],
									["Spam trap domain", yesNo(misc.details?.is_spam_trap_domain)],
									["Consumer address", yesNo(freeProviderOf(result))],
									["Found in a data breach", yesNo(misc.details?.haveibeenpwned)],
									["Gravatar", misc.details?.gravatar_url ? "Yes" : "No"],
								]}
							/>
						)}
					</Row>
					{debug ? (
						<Row label="Check run" layout="stacked">
							<Facts
								items={[
									[
										"Method",
										result.external_verification?.service ??
											debug.smtp.verif_method?.type ??
											"Not recorded",
									],
									["Duration", seconds === null ? "Not recorded" : `${seconds.toFixed(2)} s`],

									["Started", new Date(debug.start_time).toLocaleString()],
								]}
							/>
						</Row>
					) : null}
				</Group>
			</details>
		</section>
	);
}
