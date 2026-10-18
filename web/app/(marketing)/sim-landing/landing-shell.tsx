import Link from "next/link";
import type { ReactNode } from "react";

import { FinalCta } from "@/components/platform/landing/blocks";

import { EditorialHeader } from "../editorial/header";
import { SimFooter } from "./sim-footer";

/** Lead and trail of the closing call to action. */
export type Finale = { lead: string; trail: string };

/**
 * Page chrome for every public route, copied from the screenshot-studio
 * `MarketingShell`: nav, main, closing call to action, and footer. The
 * document scrolls; there is no inner scroll port.
 */
export function SimLandingShell({
	children,
	finale,
	showClosingCta = true,
}: {
	children: ReactNode;
	/** Page-specific closing headline. The default is the site-wide one. */
	finale?: Finale;
	/** Legal pages skip the closing call to action. */
	showClosingCta?: boolean;
}) {
	return (
		<div className="ed-root min-h-screen bg-[#090909] font-[family-name:var(--font-geist-sans)] text-white antialiased">
			<Link className="linear-skip-link" href="#main-content">
				Skip to content →
			</Link>
			<EditorialHeader />
			<main id="main-content">
				{children}
				{showClosingCta ? (
					<FinalCta
						lead={finale?.lead}
						note="No email is sent during a check. The free plan includes 100 checks a month."
						trail={finale?.trail}
					/>
				) : null}
			</main>
			<SimFooter />
		</div>
	);
}
