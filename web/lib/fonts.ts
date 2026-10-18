import { Geist, Geist_Mono, Inter } from "next/font/google";
import localFont from "next/font/local";

import { cn } from "@/lib/utils";

/**
 * Root font variables loaded once in `app/layout.tsx`.
 *
 * Every surface uses Geist Sans and Geist Mono, as screenshot-studio does.
 * Inter stays for legacy classes that still name it.
 */
const fontSans = Geist({
	variable: "--font-geist-sans",
	subsets: ["latin"],
	display: "swap",
});

const fontMono = Geist_Mono({
	variable: "--font-geist-mono",
	subsets: ["latin"],
	display: "swap",
});

const fontInter = Inter({
	variable: "--font-inter",
	subsets: ["latin"],
	weight: ["400", "500", "600"],
	display: "swap",
	preload: false,
});

const fontDisplay = localFont({
	src: [
		{
			path: "../public/fonts/F37Stout-Regular.woff2",
			weight: "400",
			style: "normal",
		},
	],
	variable: "--font-f37-stout",
	display: "swap",
	preload: false,
});

export const fontVariables = cn(
	fontSans.variable,
	fontMono.variable,
	fontInter.variable,
	fontDisplay.variable,
);
