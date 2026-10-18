import { redirect } from "next/navigation";

import { getOptionalSession } from "@/lib/auth/session";

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

	return children;
}
