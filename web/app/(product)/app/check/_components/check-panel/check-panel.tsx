"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
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
		<section
			data-slot="check-panel"
			className={cn("mx-auto max-w-4xl space-y-8", className)}
			{...props}
		>
			<div>
				<p className="text-sm font-medium text-emerald-600 dark:text-emerald-400">Verification</p>
				<h1 className="mt-2 text-3xl font-semibold tracking-tight">Check an email address</h1>
				<p className="mt-3 text-muted-foreground">
					Review reachability, delivery risk, and the signals behind the result.
				</p>
			</div>

			<Card>
				<CardHeader>
					<CardTitle>Single address check</CardTitle>
				</CardHeader>
				<CardContent>
					<form
						className="flex flex-col gap-4 sm:flex-row sm:items-end"
						noValidate
						onSubmit={submit}
					>
						<div className="flex-1 space-y-2">
							<Label htmlFor="email-to-check">Email address</Label>
							<Input
								autoComplete="off"
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
							{inputError ? (
								<p className="text-sm text-destructive" id="email-to-check-error" role="alert">
									{inputError}
								</p>
							) : null}
						</div>
						<Label className="flex items-center gap-2 text-sm" htmlFor="use-sample-result">
							<Checkbox
								checked={sample}
								id="use-sample-result"
								onCheckedChange={(checked) => {
									setSample(checked === true);
								}}
							/>
							Use sample result
						</Label>
						<Button disabled={check.isPending} type="submit">
							{check.isPending ? "Checking…" : "Check email"}
						</Button>
					</form>
					{check.isError ? (
						<p className="mt-4 text-sm text-destructive" role="alert">
							{check.error.message || "We could not check this address. Try again."}
						</p>
					) : null}
				</CardContent>
			</Card>

			{result ? (
				<Card aria-live="polite">
					<CardHeader>
						<CardTitle className="flex flex-wrap items-center justify-between gap-3">
							<span>{result.input}</span>
							<span className="rounded-full bg-muted px-3 py-1 text-sm capitalize">
								{result.score.category}
							</span>
						</CardTitle>
					</CardHeader>
					<CardContent className="grid gap-5 sm:grid-cols-3">
						<div>
							<p className="text-sm text-muted-foreground">Reachability</p>
							<p className="mt-1 font-semibold capitalize">{result.is_reachable}</p>
						</div>
						<div>
							<p className="text-sm text-muted-foreground">Safe to send</p>
							<p className="mt-1 font-semibold">{result.score.safe_to_send ? "Yes" : "No"}</p>
						</div>
						<div>
							<p className="text-sm text-muted-foreground">Quality score</p>
							<p className="mt-1 font-semibold">{result.score.score}/100</p>
						</div>
						<div>
							<p className="text-sm text-muted-foreground">Syntax</p>
							<p className="mt-1 font-semibold">
								{result.syntax.is_valid_syntax ? "Valid" : "Invalid"}
							</p>
						</div>
						{result.score.reason_codes.length > 0 ? (
							<div className="sm:col-span-3">
								<p className="text-sm text-muted-foreground">Reasons</p>
								<p className="mt-1 text-sm">{result.score.reason_codes.join(", ")}</p>
							</div>
						) : null}
					</CardContent>
				</Card>
			) : null}
		</section>
	);
}
