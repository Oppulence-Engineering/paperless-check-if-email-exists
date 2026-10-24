import { MarketingLayoutClient } from "./marketing-layout-client";

import "./marketing.css";
import "./editorial/editorial.css";

export const viewport = {
	width: "device-width",
	initialScale: 1,
	themeColor: "#090909",
};

export default function Layout({ children }: { children: React.ReactNode }) {
	return <MarketingLayoutClient>{children}</MarketingLayoutClient>;
}
