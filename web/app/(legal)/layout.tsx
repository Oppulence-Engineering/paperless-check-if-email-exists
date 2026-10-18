import { LegalLayoutClient } from "./legal-layout-client";

import "@/app/(marketing)/editorial/editorial.css";

export const viewport = {
	width: "device-width",
	initialScale: 1,
	themeColor: "#090909",
};

export default function LegalLayout({ children }: { children: React.ReactNode }) {
	return <LegalLayoutClient>{children}</LegalLayoutClient>;
}
