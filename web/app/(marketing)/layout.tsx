import { marketingFontVariables } from "@/lib/fonts";

import { MarketingLayoutClient } from "./marketing-layout-client";

export const viewport = {
	width: "device-width",
	initialScale: 1,
	themeColor: "#000000",
};

export default function Layout({ children }: { children: React.ReactNode }) {
	return (
		<div className={marketingFontVariables}>
			<MarketingLayoutClient>{children}</MarketingLayoutClient>
		</div>
	);
}
