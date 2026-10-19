"use client";

import { usePathname } from "next/navigation";
import type { ReactNode } from "react";

import { type Finale, SimLandingShell } from "./sim-landing/landing-shell";

/** Closing headlines for pages that have their own. Other pages use the default. */
const FINALES: Record<string, Finale> = {
	"/product": {
		lead: "Send only to addresses that exist.",
		trail: "Start with 100 free checks a month.",
	},
	"/web": {
		lead: "Bring your lists into one place.",
		trail: "Invite your team and clean your first list today.",
	},
	"/security": {
		lead: "Questions about security?",
		trail: "Tell us what your review needs, and we will answer it.",
	},
	"/resources": { lead: "Ready to clean a list?", trail: "Start with 100 free checks a month." },
	"/pricing": {
		lead: "Clean your next list for free.",
		trail: "Start with 100 free checks a month.",
	},
};

/** Shared chrome for every public route in `(marketing)`. */
export function MarketingLayoutClient({ children }: { children: ReactNode }) {
	const pathname = usePathname();
	return <SimLandingShell finale={FINALES[pathname]}>{children}</SimLandingShell>;
}
