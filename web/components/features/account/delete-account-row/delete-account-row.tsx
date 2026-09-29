"use client";

import "client-only";
import * as React from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetFooter,
	SheetHeader,
	SheetTitle,
} from "@oppulence/ui/components/sheet";

import { dashboardFetch } from "@/lib/auth/client";
import { dashboardRequest } from "@/lib/auth/dashboard-fetch";
import { BackendAPIErrorSchema } from "@/lib/auth/schemas";

const ACCOUNT_DELETION_ERRORS: Record<string, string> = {
	workspace_successor_required:
		"Your workspace has other members and none of them can take ownership. Remove the other members first, then delete your account.",
	billing_cancellation_failed:
		"We could not cancel your subscription, so your account was not deleted. Try again, or contact support.",
	step_up_required: "Verify your identity again before deleting your account.",
	identity_deletion_failed:
		"Your product data was deleted, but we could not finish removing your sign-in identity. Contact support to complete the deletion.",
};
const ACCOUNT_DELETION_FALLBACK =
	"We could not delete your account. Try again, or contact support.";
const VERIFICATION_FALLBACK =
	"We could not verify your identity. Request a new code and try again.";

const ReceiptSchema = z.object({
	receiptId: z.string().min(1),
	completedAt: z.string().min(1),
});
type Receipt = z.infer<typeof ReceiptSchema>;

function signOut() {
	window.location.assign("/api/auth/logout");
}

/**
 * Self-serve account deletion (DELETE /v1/me). The API cancels every Stripe
 * subscription, revokes connectors, transfers shared workspaces, and deletes
 * the account. The user must type DELETE before the request is sent. After
 * the deletion, the sheet shows the receipt so the user can keep it; closing
 * the sheet signs the user out.
 */
export function DeleteAccountRow({ email }: { email: string }) {
	const [open, setOpen] = React.useState(false);
	const [confirmation, setConfirmation] = React.useState("");
	const [code, setCode] = React.useState("");
	const [codeSent, setCodeSent] = React.useState(false);
	const [verified, setVerified] = React.useState(false);
	const [verificationPending, setVerificationPending] = React.useState(false);
	const [pending, setPending] = React.useState(false);
	const [error, setError] = React.useState<string | null>(null);
	const [receipt, setReceipt] = React.useState<Receipt | null>(null);

	async function requestCode() {
		setVerificationPending(true);
		setError(null);
		try {
			const response = await dashboardRequest("/api/auth/email-otp/send-verification-otp", {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({ email, type: "email-verification" }),
			});
			if (!response.ok) throw new Error("verification request failed");
			setCodeSent(true);
			setCode("");
		} catch {
			setError(VERIFICATION_FALLBACK);
		} finally {
			setVerificationPending(false);
		}
	}

	async function verifyCode() {
		setVerificationPending(true);
		setError(null);
		try {
			const response = await dashboardRequest("/api/auth/email-otp/verify-email", {
				method: "POST",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({ email, otp: code }),
			});
			if (!response.ok) throw new Error("verification failed");
			setVerified(true);
		} catch {
			setError(VERIFICATION_FALLBACK);
		} finally {
			setVerificationPending(false);
		}
	}

	async function deleteAccount() {
		setPending(true);
		setError(null);
		try {
			const response = await dashboardFetch("/api/backend/me", {
				method: "DELETE",
				headers: { "Content-Type": "application/json" },
				body: JSON.stringify({ confirm: "DELETE" }),
			});
			if (response.ok) {
				const parsed = ReceiptSchema.safeParse(await response.json().catch(() => null));
				if (parsed.success) {
					setReceipt(parsed.data);
				} else {
					signOut();
				}
				return;
			}
			const problem = BackendAPIErrorSchema.safeParse(await response.json().catch(() => null));
			const code = problem.success ? problem.data.code : undefined;
			if (code === "step_up_required") setVerified(false);
			setError(ACCOUNT_DELETION_ERRORS[code ?? ""] ?? ACCOUNT_DELETION_FALLBACK);
		} catch {
			setError(ACCOUNT_DELETION_FALLBACK);
		} finally {
			setPending(false);
		}
	}

	return (
		<div className="settings-row" data-slot="delete-account-row">
			<div className="settings-row-copy">
				<p className="settings-row-label">Delete account</p>
				<p className="settings-row-description">
					Permanently delete your account, your data, and your subscription.
				</p>
			</div>
			<Button onClick={() => setOpen(true)} size="sm" variant="destructive">
				Delete account
			</Button>
			<Sheet
				onOpenChange={(next) => {
					if (!next && receipt) {
						signOut();
						return;
					}
					setOpen(next);
					if (!next) {
						setConfirmation("");
						setCode("");
						setCodeSent(false);
						setVerified(false);
						setError(null);
					}
				}}
				open={open}
			>
				<SheetContent className="flex w-full flex-col gap-4 sm:max-w-md">
					{receipt ? (
						<>
							<SheetHeader>
								<SheetTitle>Your account is deleted</SheetTitle>
								<SheetDescription>
									Keep this receipt. Support can use it to confirm the deletion.
								</SheetDescription>
							</SheetHeader>
							<dl className="flex flex-col gap-3 px-4 text-sm">
								<div className="flex flex-col gap-1">
									<dt className="text-muted-foreground">Receipt ID</dt>
									<dd className="break-all font-mono">{receipt.receiptId}</dd>
								</div>
								<div className="flex flex-col gap-1">
									<dt className="text-muted-foreground">Deleted at</dt>
									<dd className="font-mono">{receipt.completedAt}</dd>
								</div>
							</dl>
							<SheetFooter>
								<Button onClick={signOut}>Sign out</Button>
							</SheetFooter>
						</>
					) : (
						<>
							<SheetHeader>
								<SheetTitle>Delete your account</SheetTitle>
								<SheetDescription>You cannot undo this.</SheetDescription>
							</SheetHeader>
							<ul className="list-disc space-y-1.5 px-8 text-sm text-muted-foreground">
								<li>We cancel your subscription immediately. You are not charged again.</li>
								<li>We disconnect your connected accounts and delete your synced data.</li>
								<li>A shared workspace goes to another member. Their data stays.</li>
								<li>You are signed out, and you cannot sign in to this account again.</li>
							</ul>
							<div className="flex flex-col gap-2 px-4">
								<div className="rounded-none border p-3">
									<p className="text-sm font-medium">Verify your identity</p>
									<p className="mt-1 text-sm text-muted-foreground">
										We will send a one-time code to {email}. Verification is valid for 15 minutes.
									</p>
									{verified ? (
										<p className="mt-3 text-sm font-medium text-emerald-700" role="status">
											Identity verified
										</p>
									) : codeSent ? (
										<div className="mt-3 flex flex-col gap-2">
											<label className="text-sm font-medium" htmlFor="delete-account-code">
												Verification code
											</label>
											<div className="flex gap-2">
												<Input
													autoComplete="one-time-code"
													id="delete-account-code"
													inputMode="numeric"
													onChange={(event) => setCode(event.target.value)}
													value={code}
												/>
												<Button
													disabled={!code.trim() || verificationPending}
													onClick={() => void verifyCode()}
													type="button"
													variant="outline"
												>
													{verificationPending ? "Verifying…" : "Verify code"}
												</Button>
											</div>
											<Button
												className="w-fit px-0"
												disabled={verificationPending}
												onClick={() => void requestCode()}
												type="button"
												variant="link"
											>
												Send a new code
											</Button>
										</div>
									) : (
										<Button
											className="mt-3"
											disabled={verificationPending}
											onClick={() => void requestCode()}
											size="sm"
											type="button"
											variant="outline"
										>
											{verificationPending ? "Sending…" : "Send verification code"}
										</Button>
									)}
								</div>
								<label
									className="flex flex-col gap-2 text-sm font-medium"
									htmlFor="delete-account-confirmation"
								>
									Type DELETE to confirm
									<Input
										autoComplete="off"
										id="delete-account-confirmation"
										onChange={(event) => setConfirmation(event.target.value)}
										value={confirmation}
									/>
								</label>
								{error ? (
									<p className="text-sm text-destructive" role="alert">
										{error}
									</p>
								) : null}
							</div>
							<SheetFooter>
								<Button
									disabled={!verified || confirmation !== "DELETE" || pending}
									onClick={() => void deleteAccount()}
									variant="destructive"
								>
									{pending ? "Deleting…" : "Permanently delete account"}
								</Button>
							</SheetFooter>
						</>
					)}
				</SheetContent>
			</Sheet>
		</div>
	);
}
