import Link from "next/link";
import type { ReactNode } from "react";

import { cn } from "@/lib/sim/cn";

import { EditorialClosing } from "../editorial/closing";
import { EditorialHeader } from "../editorial/header";
import { SimFooter } from "./sim-footer";

type SimLandingShellVariant = "editorial" | "subpage";

type SimLandingShellProps = {
	children: ReactNode;
	/** Editorial pages render their own sections; legacy subpages keep the linear wrapper. */
	variant?: SimLandingShellVariant;
	/** Legal and other long-form pages skip the closing section. */
	showClosingCta?: boolean;
};

/**
 * Sim `LandingShell` scroll port for all Oppulence marketing routes.
 *
 * The internal scroll container keeps sticky header math and footer artwork
 * contained the way sim.ai does, instead of relying on document scroll.
 */
export function SimLandingShell({
	children,
	variant = "subpage",
	showClosingCta = true,
}: SimLandingShellProps) {
	const isEditorial = variant === "editorial";

	return (
		<div className="sim-landing-scroll-port sim-landing-root relative h-full min-h-0 flex-1 overflow-y-auto overscroll-y-none bg-[var(--bg)] text-[var(--text-primary)] [--text-muted:var(--text-secondary)]">
			<div aria-hidden="true" className="sim-landing-scroll-sentinel -mb-px h-px" />
			<Link className="linear-skip-link" href="#main-content">
				Skip to content →
			</Link>
			<EditorialHeader />
			<main
				className={cn("flex flex-col", !isEditorial && "sim-marketing-page-main")}
				id="main-content"
			>
				{isEditorial ? children : <div className="linear-shell linear-guides">{children}</div>}
			</main>
			<div className={cn(!isEditorial && "pt-24 max-sm:pt-16")}>
				{showClosingCta ? <EditorialClosing /> : null}
				<SimFooter />
			</div>
		</div>
	);
}
