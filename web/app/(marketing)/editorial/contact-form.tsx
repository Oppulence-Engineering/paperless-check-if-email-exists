"use client";

import type { FormEvent } from "react";

import { cn } from "@/lib/utils";

const CONTACT_EMAIL = "hello@oppulence.io";

const FIELD =
	"ed-sans h-10 w-full rounded-xl border border-[var(--ed-line)] bg-[var(--ed-bg)] px-3 text-[14px] text-[var(--ed-ink)] placeholder:text-[var(--ed-faint)] focus:border-[var(--ed-ink)] focus:outline-none";

const LABEL = "ed-sans mb-1.5 block text-[12.5px] text-[var(--ed-ink)]";

/**
 * Contact form without a backend: submitting opens the visitor's email app
 * with the details filled in, addressed to the team inbox.
 */
export function ContactForm({ className }: { className?: string }) {
	function onSubmit(event: FormEvent<HTMLFormElement>) {
		event.preventDefault();
		const data = new FormData(event.currentTarget);
		const value = (key: string) => String(data.get(key) ?? "").trim();
		const name = [value("firstName"), value("lastName")].filter(Boolean).join(" ");
		const subject = `Question${name ? ` from ${name}` : ""}`;
		const body = [`Name: ${name}`, `Email: ${value("email")}`, "", value("details")].join("\n");
		window.location.href = `mailto:${CONTACT_EMAIL}?subject=${encodeURIComponent(subject)}&body=${encodeURIComponent(body)}`;
	}

	return (
		<form className={cn("mx-auto grid w-full max-w-[460px] gap-4", className)} onSubmit={onSubmit}>
			<div className="grid grid-cols-2 gap-3">
				<div>
					<label className={LABEL} htmlFor="contact-first">
						First name
					</label>
					<input
						autoComplete="given-name"
						className={FIELD}
						id="contact-first"
						name="firstName"
						placeholder="First"
						required
					/>
				</div>
				<div>
					<label className={LABEL} htmlFor="contact-last">
						Last name
					</label>
					<input
						autoComplete="family-name"
						className={FIELD}
						id="contact-last"
						name="lastName"
						placeholder="Last"
					/>
				</div>
			</div>
			<div>
				<label className={LABEL} htmlFor="contact-email">
					Work email
				</label>
				<input
					autoComplete="email"
					className={FIELD}
					id="contact-email"
					name="email"
					placeholder="you@company.com"
					required
					type="email"
				/>
			</div>
			<div>
				<label className={LABEL} htmlFor="contact-details">
					Question
				</label>
				<textarea
					className={cn(FIELD, "h-28 resize-none py-2")}
					id="contact-details"
					name="details"
					placeholder="Your question"
				/>
			</div>
			<button
				className="ed-sans h-10 w-full rounded-full bg-[var(--ed-ink-strong)] text-[14px] font-[550] text-[var(--ed-bg)] transition-opacity hover:opacity-90"
				type="submit"
			>
				Send
			</button>
			<p className="ed-sans text-[12px] text-[var(--ed-muted)]">
				Sending opens your email app with this message to {CONTACT_EMAIL}.
			</p>
		</form>
	);
}
