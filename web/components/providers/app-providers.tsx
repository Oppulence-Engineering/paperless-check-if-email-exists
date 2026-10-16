"use client";

import { Toaster } from "@oppulence/ui/components/sonner";
import { ThemeProvider } from "next-themes";
import { usePathname } from "next/navigation";
import { NuqsAdapter } from "nuqs/adapters/next/app";
import { Suspense, type ReactNode } from "react";

import { DevProviders } from "@/components/providers/dev-providers";

/**
 * Root client boundary for the whole app. Mirrors the Better Auth docs shell:
 * theme class on `<html>` and global shadcn toasts.
 */
export function AppProviders({
	children,
	devToolsEnabled,
}: {
	children: ReactNode;
	devToolsEnabled: boolean;
}) {
	// The signed-in app uses the polar.sh dark theme, like the public site and sign-in.
	const forcedTheme = usePathname()?.startsWith("/app") ? "dark" : undefined;
	return (
		<ThemeProvider
			attribute="class"
			defaultTheme="system"
			disableTransitionOnChange
			enableSystem
			forcedTheme={forcedTheme}
		>
			{/*
        Nuqs reads searchParams. Next.js Cache Components requires that
        request-time API behind a Suspense boundary so the root layout can
        still emit a static shell (see Request-time APIs in the production
        checklist).
      */}
			<Suspense fallback={null}>
				<NuqsAdapter>
					{children}
					<DevProviders enabled={devToolsEnabled} />
					<Toaster richColors closeButton position="bottom-right" />
				</NuqsAdapter>
			</Suspense>
		</ThemeProvider>
	);
}
