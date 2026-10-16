import {
	EnvelopeIcon,
	GlobeIcon,
	PlugsConnectedIcon,
	SealCheckIcon,
	TrayIcon,
	WarningCircleIcon,
} from "@/lib/icons";

import { cn } from "@/lib/sim/cn";

import { HOME_INSET, LANDING_CONTENT_WIDTH, LANDING_GUTTER } from "./tokens";

const sources = [
	{ label: "Address format", icon: EnvelopeIcon },
	{ label: "Domain and MX", icon: GlobeIcon },
	{ label: "Mailbox", icon: TrayIcon },
	{ label: "Catch-all grading", icon: WarningCircleIcon },
	{ label: "Safe to send", icon: SealCheckIcon },
	{ label: "Delivery feedback", icon: PlugsConnectedIcon },
] as const;

/** Sim `Proof` — connector row in the homepage inset. */
export function SimProof() {
	return (
		<section
			aria-label="What each check covers"
			className={cn(LANDING_CONTENT_WIDTH, LANDING_GUTTER)}
			id="proof"
		>
			<div className={cn(HOME_INSET, "flex justify-center")}>
				<div className="flex flex-wrap items-center justify-center gap-x-10 gap-y-4 text-[var(--text-secondary)] max-sm:gap-x-6">
					{sources.map((source) => {
						const Icon = source.icon;
						return (
							<div
								className="inline-flex items-center gap-2 text-[14px] font-medium tracking-[-0.01em]"
								key={source.label}
							>
								<Icon aria-hidden="true" className="size-4 opacity-70" />
								{source.label}
							</div>
						);
					})}
				</div>
			</div>
		</section>
	);
}
