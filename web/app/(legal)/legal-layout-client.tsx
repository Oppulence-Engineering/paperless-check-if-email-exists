import type { ReactNode } from "react";

import { SimLandingShell } from "@/app/(marketing)/sim-landing/landing-shell";

/** Legal routes share the marketing chrome, without the closing call to action. */
export function LegalLayoutClient({ children }: { children: ReactNode }) {
	return <SimLandingShell showClosingCta={false}>{children}</SimLandingShell>;
}
