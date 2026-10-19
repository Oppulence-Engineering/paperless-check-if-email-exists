import type { ReactNode } from "react";

import { SimLandingShell } from "@/app/(marketing)/sim-landing/landing-shell";

import "@/app/(marketing)/editorial/editorial.css";

export const viewport = {
	width: "device-width",
	initialScale: 1,
	themeColor: "#090909",
};

/** The developer portal uses the public site chrome, as in screenshot-studio. */
export default function DeveloperLayout({ children }: { children: ReactNode }) {
	return <SimLandingShell showClosingCta={false}>{children}</SimLandingShell>;
}
