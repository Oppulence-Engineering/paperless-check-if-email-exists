import { marketingFontVariables } from "@/lib/fonts";

import { LegalLayoutClient } from "./legal-layout-client";

import "@/app/(marketing)/marketing-sim-theme.css";
import "@/app/(marketing)/marketing-subpages.css";
import "@/app/(marketing)/editorial/editorial.css";

export const viewport = {
	width: "device-width",
	initialScale: 1,
	themeColor: [
		{ media: "(prefers-color-scheme: light)", color: "#ffffff" },
		{ media: "(prefers-color-scheme: dark)", color: "#111111" },
	],
};

export default function LegalLayout({ children }: { children: React.ReactNode }) {
	return (
		<div className={marketingFontVariables}>
			<LegalLayoutClient>{children}</LegalLayoutClient>
		</div>
	);
}
