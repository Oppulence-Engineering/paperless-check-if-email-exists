import Image from "next/image";
import Link from "next/link";

import { CONTAINER, MUTED } from "@/components/platform/landing/primitives";
import { ArrowUpRight } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { footerGroups } from "../site";

/** Link columns over an oversized wordmark, copied from screenshot-studio `LandingFooter`. */
export function SimFooter() {
	return (
		<footer className="overflow-hidden">
			<div className={CONTAINER}>
				<div className="grid gap-16 border-t border-[#212121] py-24 lg:grid-cols-[1fr_2fr]">
					<div className="flex flex-col justify-between gap-10">
						<Link className="flex items-center gap-3" href="/">
							<Image alt="" className="size-9" height={36} src="/check-email-logo.svg" width={36} />
							<span className="text-2xl tracking-[-0.03em] text-white">Verifier</span>
						</Link>
						<div className="space-y-6">
							<Link
								className="inline-flex items-center gap-1.5 border-b border-white pb-1 text-base text-white"
								href="/sign-up"
							>
								Get started
								<ArrowUpRight aria-hidden="true" className="size-4" />
							</Link>
							<p className={cn("text-[15px]", MUTED)}>© 2026 Oppulence Engineering · Verifier</p>
						</div>
					</div>
					<nav
						aria-label="Footer"
						className="grid grid-cols-2 gap-x-8 gap-y-12 sm:grid-cols-3 lg:grid-cols-5"
					>
						{footerGroups.map((column) => (
							<div key={column.title}>
								<p className={cn("text-[15px]", MUTED)}>{column.title}</p>
								<ul className="mt-5 space-y-3">
									{column.items.map((link) => (
										<li key={link.href}>
											<Link
												className="text-[15px] text-white transition-colors hover:text-white/65"
												href={link.href}
											>
												{link.label}
											</Link>
										</li>
									))}
								</ul>
							</div>
						))}
					</nav>
				</div>
			</div>
			<p
				aria-hidden="true"
				className="bg-gradient-to-b from-white/[0.14] to-transparent bg-clip-text pb-2 text-center text-[13.5vw] leading-[0.8] tracking-[-0.06em] whitespace-nowrap text-transparent select-none"
			>
				Verifier
			</p>
		</footer>
	);
}
