"use client";

import "client-only";

import {
	useState,
	type ComponentPropsWithoutRef,
	type ReactNode,
	type SyntheticEvent,
} from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section, type PillTone } from "@/components/platform/page-kit";
import { useV1CheckEmail } from "@/hooks/queries/use-v1-check-email";

import { type CheckPanelPropsFields } from "./check-panel.schema";

/**
 * @oppulence-gen kind=component
 * CheckPanel owns the form and the generated verification mutation.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `check-panel.lit.ts`.
 */
export type CheckPanelProps = CheckPanelPropsFields & ComponentPropsWithoutRef<"section">;

const CATEGORY_TONES: Record<string, PillTone> = {
	invalid: "red",
	risky: "yellow",
	unknown: "gray",
	valid: "green",
};

export function CheckPanel({ className, ...props }: CheckPanelProps) {
	const [email, setEmail] = useState("");
	const [inputError, setInputError] = useState<string | null>(null);
	const [sample, setSample] = useState(false);
	const check = useV1CheckEmail();

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const address = email.trim();
		if (!z.email().safeParse(address).success) {
			check.reset();
			setInputError("Enter a complete email address, such as name@example.com.");
			return;
		}
		setInputError(null);
		check.mutate({ to_email: address, sandbox: sample });
	}

	const result = check.data;
	return (
		<Page
			description="Review reachability, delivery risk, and the signals behind the result."
			title="Check an email address"
			width="md"
		>
			<section data-slot="check-panel" className={cn("flex flex-col gap-10", className)} {...props}>
				<Group>
					<Row>
						<h3 className="text-sm font-medium text-white">Single address check</h3>
						<form
							className="flex flex-col gap-3 sm:flex-row sm:items-end"
							noValidate
							onSubmit={submit}
						>
							<div className="flex flex-1 flex-col gap-1.5">
								<Label className="text-xs font-medium text-neutral-400" htmlFor="email-to-check">
									Email address
								</Label>
								<Input
									autoComplete="off"
									className="h-10 rounded-lg"
									id="email-to-check"
									aria-describedby={inputError ? "email-to-check-error" : undefined}
									aria-invalid={Boolean(inputError)}
									onChange={(event) => {
										setEmail(event.target.value);
										setInputError(null);
										check.reset();
									}}
									placeholder="name@example.com"
									required
									type="email"
									value={email}
								/>
							</div>
							<Label
								className="flex h-10 items-center gap-2 text-sm text-neutral-400"
								htmlFor="use-sample-result"
							>
								<Checkbox
									checked={sample}
									id="use-sample-result"
									onCheckedChange={(checked) => {
										setSample(checked === true);
									}}
								/>
								Use sample result
							</Label>
							<Button className="h-10 rounded-lg" disabled={check.isPending} type="submit">
								{check.isPending ? "Checking…" : "Check email"}
							</Button>
						</form>
						{inputError ? (
							<p className="text-sm text-red-300" id="email-to-check-error" role="alert">
								{inputError}
							</p>
						) : null}
					</Row>
				</Group>
				{check.isError ? (
					<p
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
						role="alert"
					>
						{check.error.message || "We could not check this address. Try again."}
					</p>
				) : null}

				{result ? (
					<>
						<div aria-live="polite">
							<Section
								actions={
									<Pill tone={CATEGORY_TONES[result.score.category]}>
										<span className="capitalize">{result.score.category}</span>
									</Pill>
								}
								title={result.input}
							>
								<Group>
									<Row>
										<dl className="grid gap-5 sm:grid-cols-4">
											<div>
												<dt className="text-xs text-[#858585]">Reachability</dt>
												<dd className="mt-1 text-sm font-medium text-white capitalize">
													{result.is_reachable}
												</dd>
											</div>
											<div>
												<dt className="text-xs text-[#858585]">Safe to send</dt>
												<dd className="mt-1 text-sm font-medium text-white">
													{result.score.safe_to_send ? "Yes" : "No"}
												</dd>
											</div>
											<div>
												<dt className="text-xs text-[#858585]">Quality score</dt>
												<dd className="mt-1 text-sm font-medium text-white">
													{result.score.score}/100
												</dd>
											</div>
											<div>
												<dt className="text-xs text-[#858585]">Syntax</dt>
												<dd className="mt-1 text-sm font-medium text-white">
													{result.syntax.is_valid_syntax ? "Valid" : "Invalid"}
												</dd>
											</div>
										</dl>
									</Row>
									{result.score.reason_codes.length > 0 ? (
										<Row description={result.score.reason_codes.join(", ")} label="Reasons" />
									) : null}
								</Group>
							</Section>
						</div>
						<Section title="All check details">
							<Group>
								<Row>
									<div className="text-sm">
										<ResultValue value={result} />
									</div>
								</Row>
							</Group>
						</Section>
					</>
				) : null}
			</section>
		</Page>
	);
}

export function ResultValue({ value }: { value: unknown }): ReactNode {
	if (value === null || value === undefined) return "Not available";
	if (Array.isArray(value)) {
		const items = value as unknown[];
		if (items.length === 0) return "None";
		return (
			<ol className="list-decimal space-y-2 pl-5">
				{items.map((item, index) => (
					<li key={index}>
						<ResultValue value={item} />
					</li>
				))}
			</ol>
		);
	}
	if (typeof value === "object") {
		const entries = Object.entries(value as Record<string, unknown>);
		if (entries.length === 0) return "None";
		return (
			<dl className="divide-y divide-white/[0.07]">
				{entries.map(([key, field]) => (
					<div className="grid gap-2 py-3 sm:grid-cols-[minmax(0,12rem)_minmax(0,1fr)]" key={key}>
						<dt className="font-medium break-words text-white">{key.replaceAll("_", " ")}</dt>
						<dd className="min-w-0 break-words text-[#858585]">
							<ResultValue value={field} />
						</dd>
					</div>
				))}
			</dl>
		);
	}
	if (typeof value === "boolean") return value ? "Yes" : "No";
	if (typeof value === "string" || typeof value === "number") return String(value) || "Empty";
	return "Not available";
}
