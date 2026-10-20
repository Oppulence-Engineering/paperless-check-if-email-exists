"use client";

import "client-only";

import {
	useEffect,
	useRef,
	useState,
	type ComponentPropsWithoutRef,
	type SyntheticEvent,
} from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Row, Section } from "@/components/platform/page-kit";
import { AddressTimeline } from "@/components/features/verification/address-timeline/address-timeline";
import { ProviderDetails } from "@/components/features/verification/provider-details/provider-details";
import { RiskFactors } from "@/components/features/verification/risk-factors/risk-factors";
import { SignalChecklist } from "@/components/features/verification/signal-checklist/signal-checklist";
import { TechnicalDetails } from "@/components/features/verification/technical-details/technical-details";
import { VerdictSummary } from "@/components/features/verification/verdict-summary/verdict-summary";
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

export function CheckPanel({ className, initialEmail, ...props }: CheckPanelProps) {
	const [email, setEmail] = useState(initialEmail ?? "");
	const [inputError, setInputError] = useState<string | null>(null);
	const [sample, setSample] = useState(false);
	const check = useV1CheckEmail();
	const { mutate } = check;
	const checkedInitial = useRef(false);

	// Check the `?email=` address one time. The ref stops a second run in Strict Mode.
	useEffect(() => {
		if (checkedInitial.current || !initialEmail) return;
		checkedInitial.current = true;
		const address = initialEmail.trim();
		if (z.email().safeParse(address).success) mutate({ to_email: address, sandbox: false });
	}, [initialEmail, mutate]);

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
							<Section title={result.input}>
								<VerdictSummary result={result} />
							</Section>
						</div>
						<Section title="Risk">
							<RiskFactors result={result} />
						</Section>
						<Section title="Signals">
							<SignalChecklist result={result} />
						</Section>
						<Section title="Provider and policy">
							<ProviderDetails result={result} />
						</Section>
						<TechnicalDetails result={result} />
						<Section
							description="Earlier checks, delivery events, and suppression status for this address."
							title="Address history"
						>
							<AddressTimeline email={result.input} />
						</Section>
					</>
				) : null}
			</section>
		</Page>
	);
}
