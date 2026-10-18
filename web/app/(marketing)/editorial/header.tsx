"use client";

import Image from "next/image";
import Link from "next/link";
import { usePathname } from "next/navigation";
import type { MouseEvent } from "react";

import { CtaLink } from "@/components/platform/landing/cta-link";
import { CONTAINER, MONO, MUTED } from "@/components/platform/landing/primitives";
import { ArrowRight, CaretDown, List } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { headerNav, headerUtilityLinks } from "../site";

/** Moves focus out of a dropdown after a click, so the menu closes on the next page. */
function blurLink(event: MouseEvent<HTMLAnchorElement>) {
	event.currentTarget.blur();
}

/**
 * Announcement bar and sticky navigation, copied from screenshot-studio
 * `LandingNav`. The menus come from `headerNav`.
 */
export function EditorialHeader() {
	const pathname = usePathname();
	return (
		<>
			<Link
				className="group flex h-10 items-center justify-center gap-2 bg-[#141414] px-6 text-center text-[13px] text-white/85 transition-colors hover:text-white sm:text-sm"
				href="/pricing"
			>
				<span className="truncate">The free plan includes 100 checks a month. Compare plans</span>
				<ArrowRight
					aria-hidden="true"
					className="size-3.5 shrink-0 transition-transform group-hover:translate-x-0.5"
				/>
			</Link>
			<header className="sticky top-0 z-50 border-b border-white/[0.06] bg-[#0a0a0a]/80 backdrop-blur-xl">
				<nav aria-label="Main" className={`${CONTAINER} flex h-16 items-center gap-4 sm:gap-8`}>
					<Link className="flex shrink-0 items-center gap-2.5" href="/">
						<Image
							alt=""
							className="size-7"
							height={28}
							priority
							src="/check-email-logo.svg"
							width={28}
						/>
						<span className="text-[15px] font-medium tracking-[-0.02em] text-white">Verifier</span>
					</Link>
					<ul className="hidden items-center gap-1 text-[15px] lg:flex">
						{headerNav.map((menu) => (
							<li className="group relative" key={menu.label}>
								<button
									aria-haspopup="true"
									className="flex items-center gap-1 rounded-full px-3 py-2 text-white/55 transition-colors duration-150 group-focus-within:text-white hover:text-white"
									type="button"
								>
									{menu.label}
									<CaretDown
										aria-hidden="true"
										className="size-3.5 transition-transform duration-150 group-focus-within:rotate-180 group-hover:rotate-180"
									/>
								</button>
								<div className="invisible absolute top-full left-0 pt-3 opacity-0 transition-[opacity,visibility] duration-150 group-focus-within:visible group-focus-within:opacity-100 group-hover:visible group-hover:opacity-100">
									<ul className="grid w-[520px] grid-cols-2 gap-1 rounded-2xl border border-white/[0.08] bg-[#111] p-2 shadow-[0_24px_60px_-12px_rgba(0,0,0,0.8)]">
										{menu.items.map((item) => (
											<li key={item.href}>
												<Link
													className="block rounded-xl px-3.5 py-3 transition-colors hover:bg-white/[0.05] focus-visible:bg-white/[0.05] focus-visible:outline-none"
													href={item.href}
													onClick={blurLink}
												>
													<span className="block text-[15px] text-white">{item.label}</span>
													<span className={cn("mt-0.5 block text-[13px]", MUTED)}>
														{item.description}
													</span>
												</Link>
											</li>
										))}
									</ul>
								</div>
							</li>
						))}
						{headerUtilityLinks.map((link) => (
							<li key={link.href}>
								<Link
									className="rounded-full px-3 py-2 text-white/55 transition-colors duration-150 hover:text-white"
									href={link.href}
								>
									{link.label}
								</Link>
							</li>
						))}
					</ul>
					<div className="ml-auto flex items-center gap-2 sm:gap-4">
						<Link
							className="hidden text-[15px] text-white transition-colors hover:text-white/70 sm:block"
							href="/sign-in"
						>
							Sign in
						</Link>
						<CtaLink
							className="h-10 pr-3 pl-4 text-sm sm:pr-4 sm:pl-5 sm:text-[15px]"
							href="/sign-up"
							label="Get started free"
						/>
						{/* The key closes the menu after a client-side navigation. */}
						<details className="group relative lg:hidden" key={pathname}>
							<summary
								aria-label="Open menu"
								className="flex size-10 cursor-pointer list-none items-center justify-center rounded-full border border-white/10 text-white [&::-webkit-details-marker]:hidden"
							>
								<List aria-hidden="true" className="size-5" />
							</summary>
							<div className="absolute top-12 right-0 max-h-[75vh] w-72 overflow-y-auto rounded-xl border border-white/10 bg-[#111] p-2 shadow-2xl">
								{headerNav.map((menu) => (
									<div
										className="border-b border-white/[0.06] py-2 last:border-b-0"
										key={menu.label}
									>
										<p
											className={cn(
												MONO,
												"px-3 py-1.5 text-[11px] tracking-[0.08em] uppercase",
												MUTED,
											)}
										>
											{menu.label}
										</p>
										{menu.items.map((item) => (
											<Link
												className="block rounded-lg px-3 py-2 text-[15px] text-white/80 hover:bg-white/[0.05] hover:text-white"
												href={item.href}
												key={item.href}
											>
												{item.label}
											</Link>
										))}
									</div>
								))}
								<div className="border-t border-white/[0.06] pt-2">
									{[...headerUtilityLinks, { href: "/sign-in", label: "Sign in" }].map((link) => (
										<Link
											className="block rounded-lg px-3 py-2 text-[15px] text-white/80 hover:bg-white/[0.05] hover:text-white"
											href={link.href}
											key={link.href}
										>
											{link.label}
										</Link>
									))}
								</div>
							</div>
						</details>
					</div>
				</nav>
			</header>
		</>
	);
}
