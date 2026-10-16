import { redirect } from "next/navigation";

import { getOptionalSession } from "@/lib/auth/session";
import { marketingFontVariables } from "@/lib/fonts";
import { cn } from "@/lib/utils";

import "@/app/(marketing)/sim-landing/sim-landing.css";
import "./auth-theme.css";

// The authenticated redirect must resolve before any login UI is streamed.
export const instant = false;

export const viewport = {
	width: "device-width",
	initialScale: 1,
	themeColor: "#090909",
};

export default async function AuthLayout({ children }: { children: React.ReactNode }) {
	if (await getOptionalSession()) {
		redirect("/app");
	}

	return (
		<div className="dark">
			<div className={cn("sim-landing-root min-h-svh", marketingFontVariables)} data-auth-route>
				{children}
			</div>
		</div>
	);
}
