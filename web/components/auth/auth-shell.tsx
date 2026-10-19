"use client";

import "client-only";

import Link from "next/link";
import { useState, type FormEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { authClient } from "@/lib/auth/auth-client";
import { authCallbackPath } from "@/lib/auth/return-to";
import { ArrowRightLine, CircleNotch, Key } from "@/lib/icons";

/* Shared styles for the auth fields, labels, and secondary buttons. */
const INPUT_CLASS =
	"h-10 rounded-lg border-white/[0.08] bg-white/[0.03] px-3.5 md:text-sm dark:bg-white/[0.03]";
const LABEL_CLASS = "text-sm font-medium text-neutral-300";
const SECONDARY_BUTTON_CLASS =
	"h-10 rounded-lg border-white/[0.08] bg-white/[0.04] shadow-none hover:bg-white/[0.07] dark:border-white/[0.08] dark:bg-white/[0.04]";

const AUTH_ERROR_MESSAGES: Record<string, string> = {
	sign_in_unavailable: "Sign-in is temporarily unavailable. Please try again.",
};

/**
 * The page's loading fallback renders with the default "/app" before the
 * search params arrive. A form used then reads the address bar with the same
 * rule as the page, so a chosen plan or an invitation is kept. A path that the
 * server resolved is used as it is.
 */
function destination(fallback: string): string {
	if (fallback !== "/app") return fallback;
	const params = new URLSearchParams(window.location.search);
	return authCallbackPath({
		invitation: params.get("invitation"),
		return_to: params.get("return_to"),
	});
}

export function AuthShell({
	mode,
	error,
	returnTo,
	brandName = "Verifier",
	googleEnabled = false,
	logoUrl = "/check-email-logo.svg",
	microsoftEnabled = false,
	privacyUrl = "/privacy",
	termsUrl = "/terms",
}: {
	mode: "sign-in" | "sign-up";
	error?: string;
	returnTo: string;
	brandName?: string;
	googleEnabled?: boolean;
	logoUrl?: string;
	microsoftEnabled?: boolean;
	privacyUrl?: string;
	termsUrl?: string;
}) {
	const isSignUp = mode === "sign-up";
	const hasSocialProvider = googleEnabled || microsoftEnabled;
	const [email, setEmail] = useState("");
	const [otp, setOtp] = useState("");
	const [otpSent, setOtpSent] = useState(false);
	const [pending, setPending] = useState(false);
	const [clientError, setClientError] = useState<string>();
	const errorMessage = error
		? AUTH_ERROR_MESSAGES[error] || "We couldn't complete sign-in. Please try again."
		: undefined;
	const crossHref = isSignUp
		? `/sign-in?${new URLSearchParams({ return_to: returnTo })}`
		: `/sign-up?${new URLSearchParams({ return_to: returnTo })}`;

	async function social(provider: "google" | "microsoft") {
		setPending(true);
		setClientError(undefined);
		const result = await authClient.signIn.social({
			provider,
			callbackURL: destination(returnTo),
		});
		if (result.error) {
			setClientError(result.error.message || "Sign-in is temporarily unavailable.");
			setPending(false);
		}
	}

	async function submitEmail(event: FormEvent<HTMLFormElement>) {
		event.preventDefault();
		setPending(true);
		setClientError(undefined);
		const result = otpSent
			? await authClient.signIn.emailOtp({ email, otp })
			: await authClient.emailOtp.sendVerificationOtp({
					email,
					type: "sign-in",
				});
		if (result.error) {
			setClientError(result.error.message || "We couldn't complete sign-in.");
			setPending(false);
			return;
		}
		if (otpSent) window.location.assign(destination(returnTo));
		else {
			setOtpSent(true);
			setPending(false);
		}
	}

	async function signInWithPasskey() {
		setPending(true);
		setClientError(undefined);
		const result = await authClient.signIn.passkey({});
		if (result.error) {
			setClientError(result.error.message || "Passkey sign-in failed.");
			setPending(false);
			return;
		}
		window.location.assign(destination(returnTo));
	}

	async function signInWithSso() {
		if (!email) {
			setClientError("Enter your work email first.");
			return;
		}
		setPending(true);
		setClientError(undefined);
		const result = await authClient.signIn.sso({
			email,
			callbackURL: destination(returnTo),
		});
		if (result.error) {
			setClientError(result.error.message || "SSO sign-in failed.");
			setPending(false);
		}
	}

	return (
		<main
			className="flex min-h-dvh flex-col items-center justify-center bg-[#090909] px-4 py-12 text-white"
			key={mode}
		>
			<div className="flex w-full max-w-md flex-col gap-8 rounded-3xl bg-[#111111] p-8 sm:p-12">
				<header className="flex flex-col gap-4">
					<Link aria-label={`${brandName} home`} className="w-fit" href="/">
						<img
							alt=""
							className="size-12 rounded-xl object-contain"
							height={48}
							src={logoUrl}
							width={48}
						/>
					</Link>
					<div className="flex flex-col gap-1.5">
						<p className="text-xs font-medium text-[#858585]">{isSignUp ? "Start" : "Sign in"}</p>
						{/* Same headline as the public CTA so this does not read as a second product. */}
						<h1 className="text-2xl text-white">Know which addresses can receive mail</h1>
						<p className="text-lg text-balance text-neutral-400">
							{isSignUp
								? `Create a workspace with ${googleEnabled ? "Google or " : ""}your work email. Check addresses, review results, and clean lists together.`
								: `Continue with ${googleEnabled ? "Google or " : ""}your work email to open your verification workspace.`}
						</p>
					</div>
				</header>

				<div className="space-y-6">
					<form className="space-y-5" onSubmit={(event) => void submitEmail(event)}>
						<div className="space-y-2">
							<Label className={LABEL_CLASS} htmlFor="auth-email">
								Email
							</Label>
							<Input
								autoComplete="email webauthn"
								className={INPUT_CLASS}
								id="auth-email"
								onChange={(event) => setEmail(event.target.value)}
								placeholder="you@company.com"
								required
								type="email"
								value={email}
							/>
						</div>
						{otpSent ? (
							<div className="space-y-2">
								<Label className={LABEL_CLASS} htmlFor="auth-otp">
									Sign-in code
								</Label>
								<Input
									autoComplete="one-time-code"
									className={`${INPUT_CLASS} font-mono tracking-[0.3em]`}
									id="auth-otp"
									inputMode="numeric"
									maxLength={8}
									onChange={(event) => setOtp(event.target.value)}
									required
									value={otp}
								/>
							</div>
						) : null}
						{errorMessage || clientError ? (
							<p
								className="rounded-lg border border-destructive/30 bg-destructive/10 px-3.5 py-3 text-sm text-destructive"
								role="alert"
							>
								{errorMessage || clientError}
							</p>
						) : null}
						<Button className="h-10 w-full rounded-lg px-4" disabled={pending} type="submit">
							{pending ? <CircleNotch aria-hidden className="animate-spin" /> : null}
							{otpSent ? "Verify code" : "Email me a code"}
							{pending ? null : <ArrowRightLine />}
						</Button>
					</form>

					<div className="space-y-3">
						<div aria-hidden className="flex items-center gap-6 text-sm text-[#858585]">
							<span className="h-px flex-1 bg-white/[0.08]" />
							or
							<span className="h-px flex-1 bg-white/[0.08]" />
						</div>
						{hasSocialProvider ? (
							<div className="grid grid-cols-1 gap-2">
								{googleEnabled ? (
									<Button
										className={SECONDARY_BUTTON_CLASS}
										disabled={pending}
										onClick={() => void social("google")}
										type="button"
										variant="outline"
									>
										Continue with Google
									</Button>
								) : null}
								{microsoftEnabled ? (
									<Button
										className={SECONDARY_BUTTON_CLASS}
										disabled={pending}
										onClick={() => void social("microsoft")}
										type="button"
										variant="outline"
									>
										Continue with Microsoft
									</Button>
								) : null}
							</div>
						) : null}
						<Button
							className={`${SECONDARY_BUTTON_CLASS} w-full`}
							disabled={pending}
							onClick={() => void signInWithPasskey()}
							type="button"
							variant="outline"
						>
							Sign in with a passkey
						</Button>
						<Button
							className={`${SECONDARY_BUTTON_CLASS} w-full`}
							disabled={pending}
							onClick={() => void signInWithSso()}
							type="button"
							variant="outline"
						>
							<Key />
							Continue with company SSO
						</Button>
					</div>

					<p className="text-center text-sm text-muted-foreground">
						{isSignUp ? "Have an account?" : "New here?"}{" "}
						<Link
							className="font-medium text-foreground underline-offset-4 hover:underline"
							href={crossHref}
						>
							{isSignUp ? "Sign in" : "Create a workspace"}
						</Link>
					</p>
				</div>
			</div>

			<p className="mt-6 max-w-sm text-center text-xs text-balance text-[#858585]">
				Review our{" "}
				<Link className="text-neutral-300 hover:text-white" href={termsUrl}>
					Terms
				</Link>{" "}
				and{" "}
				<Link className="text-neutral-300 hover:text-white" href={privacyUrl}>
					Privacy Policy
				</Link>
				.
			</p>
		</main>
	);
}
