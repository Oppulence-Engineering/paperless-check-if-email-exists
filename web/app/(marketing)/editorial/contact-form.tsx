"use client";

import type { FormEvent } from "react";

import { ctaClassName } from "@/components/platform/landing/cta-link";
import { MUTED } from "@/components/platform/landing/primitives";
import { cn } from "@/lib/utils";

export const CONTACT_EMAIL = "hello@oppulence.io";

const FIELD =
	"h-11 w-full rounded-xl border border-white/10 bg-[#090909] px-4 text-[15px] text-white placeholder:text-[#858585] focus:border-white/30 focus:outline-none";

const LABEL = "mb-2 block text-[13px] text-white/85";

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
		<form className={cn("grid w-full gap-4", className)} onSubmit={onSubmit}>
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
					className={cn(FIELD, "h-28 resize-none py-3")}
					id="contact-details"
					name="details"
					placeholder="Your question"
				/>
			</div>
			<button className={cn(ctaClassName("primary"), "w-full justify-center")} type="submit">
				Send
			</button>
			<p className={cn("text-[13px]", MUTED)}>
				Sending opens your email app with this message to {CONTACT_EMAIL}.
			</p>
		</form>
	);
}
