"use client";

import Image from "next/image";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useEffect, useId, useRef, useState } from "react";

import { ArrowRight, CaretDown, List, X } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { headerNav } from "../site";
import { AmazonSesLogo, MailgunLogo, PostmarkLogo, SendGridLogo } from "./logos";
import { ButtonLink, Crosshairs } from "./ui";

const MENU_INTEGRATIONS = [
	{ name: "SendGrid", href: "/integrations/sendgrid", logo: SendGridLogo },
	{ name: "Mailgun", href: "/integrations/mailgun", logo: MailgunLogo },
	{ name: "Postmark", href: "/integrations/postmark", logo: PostmarkLogo },
	{ name: "Amazon SES", href: "/integrations/amazon-ses", logo: AmazonSesLogo },
];

const productGroup = headerNav.find((group) => group.label === "Features");
const useCaseGroup = headerNav.find((group) => group.label === "Use cases");

const TOP_LINKS = [
	{ label: "Integrations", href: "/integrations" },
	{ label: "Security", href: "/security" },
	{ label: "Pricing", href: "/pricing" },
];

function Wordmark() {
	return (
		<Link aria-label="Verifier home" className="flex items-center gap-2.5" href="/">
			<Image alt="" className="rounded-[5px]" height={26} src="/check-email-logo.svg" width={26} />
			<span className="ed-display text-[20px] leading-none tracking-[-0.02em] text-[var(--ed-ink)]">
				Verifier
			</span>
		</Link>
	);
}

function MenuHeading({ numeral, children }: { numeral: string; children: string }) {
	return (
		<p className="ed-fig-label mb-2 flex items-center gap-2 text-[#757575]">
			<span className="text-[var(--ed-fig)]">{numeral}</span>
			{children}
		</p>
	);
}

/** One numbered row: number, title, and a one-line description. */
function MenuRow({
	number,
	label,
	description,
	href,
	onNavigate,
}: {
	number: number;
	label: string;
	description?: string;
	href: string;
	onNavigate: () => void;
}) {
	return (
		<li className="border-b border-[var(--ed-line)] last:border-0">
			<Link
				className="group grid grid-cols-[28px_1fr_auto] items-baseline gap-3 py-2.5"
				href={href}
				onClick={onNavigate}
			>
				<span className="ed-mono text-[10.5px] text-[var(--ed-fig)]">
					{String(number).padStart(2, "0")}
				</span>
				<span className="min-w-0">
					<span className="block text-[14px] text-[var(--ed-ink)]">{label}</span>
					{description ? (
						<span className="block truncate text-[12.5px] text-[var(--ed-muted)]">
							{description}
						</span>
					) : null}
				</span>
				<ArrowRight
					aria-hidden="true"
					className="size-3.5 self-center text-[#3a3a3a] transition-colors group-hover:text-[var(--ed-ink)]"
				/>
			</Link>
		</li>
	);
}

/** Product menu laid out as contents: features, use cases, and integration logos. */
function ProductMenu({ onNavigate }: { onNavigate: () => void }) {
	return (
		<div>
			<div className="grid grid-cols-[1.15fr_1fr]">
				<div className="border-r border-[var(--ed-line)] p-5">
					<MenuHeading numeral="I">Features</MenuHeading>
					<ul>
						{productGroup?.items.map((item, index) => (
							<MenuRow
								description={item.description}
								href={item.href}
								key={item.href}
								label={item.label}
								number={index + 1}
								onNavigate={onNavigate}
							/>
						))}
					</ul>
				</div>
				<div className="flex flex-col p-5">
					<MenuHeading numeral="II">Use cases</MenuHeading>
					<ul>
						{useCaseGroup?.items.map((item, index) => (
							<MenuRow
								description={item.description}
								href={item.href}
								key={item.href}
								label={item.label}
								number={index + 1}
								onNavigate={onNavigate}
							/>
						))}
					</ul>
					<div className="mt-auto pt-6">
						<MenuHeading numeral="III">Integrations</MenuHeading>
						<ul className="grid grid-cols-4 border-t border-l border-[var(--ed-line)]">
							{MENU_INTEGRATIONS.map(({ name, href, logo: Logo }) => (
								<li className="border-r border-b border-[var(--ed-line)]" key={href}>
									<Link
										aria-label={name}
										className="flex h-14 items-center justify-center transition-colors hover:bg-[#111111]"
										href={href}
										onClick={onNavigate}
										title={name}
									>
										<Logo aria-hidden="true" className="size-6" />
									</Link>
								</li>
							))}
						</ul>
						<Link
							className="mt-3 inline-flex items-center gap-1.5 text-[12.5px] text-[var(--ed-muted)] hover:text-[var(--ed-ink)]"
							href="/integrations"
							onClick={onNavigate}
						>
							All integrations <ArrowRight aria-hidden="true" className="size-3" />
						</Link>
					</div>
				</div>
			</div>
			<div className="flex items-center justify-between border-t border-[var(--ed-line)] px-5 py-3">
				<p className="ed-fig-label text-[#757575]">No email is sent during a check</p>
				<Link
					className="ed-fig-label inline-flex items-center gap-1.5 text-[var(--ed-ink)] hover:text-[var(--ed-fig)]"
					href="/sign-up"
					onClick={onNavigate}
				>
					Get started free <ArrowRight aria-hidden="true" className="size-3" />
				</Link>
			</div>
		</div>
	);
}

/** Site header in the Polar layout: wordmark and plain links left, account actions right. */
export function EditorialHeader() {
	const pathname = usePathname();
	const [menuOpen, setMenuOpen] = useState(false);
	const [mobileOpen, setMobileOpen] = useState(false);
	const menuId = useId();
	const menuRef = useRef<HTMLDivElement>(null);

	useEffect(() => {
		if (!menuOpen) return;
		const onKey = (event: KeyboardEvent) => {
			if (event.key === "Escape") setMenuOpen(false);
		};
		const onPointer = (event: PointerEvent) => {
			if (!menuRef.current?.contains(event.target as Node)) setMenuOpen(false);
		};
		document.addEventListener("keydown", onKey);
		document.addEventListener("pointerdown", onPointer);
		return () => {
			document.removeEventListener("keydown", onKey);
			document.removeEventListener("pointerdown", onPointer);
		};
	}, [menuOpen]);

	const productActive =
		pathname.startsWith("/features") ||
		pathname.startsWith("/use-cases") ||
		pathname === "/product" ||
		pathname === "/web";

	return (
		<header className="ed-sans sticky top-0 z-50 bg-[var(--ed-header-glass)] backdrop-blur-md">
			<div className="mx-auto flex h-16 w-full max-w-[1280px] items-center justify-between gap-8 px-4 sm:px-6 lg:px-8">
				<div className="flex items-center gap-8">
					<Wordmark />

					<nav aria-label="Main" className="relative hidden lg:block" ref={menuRef}>
						<ul className="flex items-center gap-1">
							<li className="relative">
								<button
									aria-controls={menuId}
									aria-expanded={menuOpen}
									className={cn(
										"flex h-8 items-center gap-1 px-2.5 text-[15px] text-[var(--ed-muted)] transition-colors hover:text-[var(--ed-ink)]",
										(menuOpen || productActive) && "text-[var(--ed-ink)]",
									)}
									onClick={() => setMenuOpen((open) => !open)}
									type="button"
								>
									Product
									<CaretDown
										aria-hidden="true"
										className={cn("size-3 transition-transform", menuOpen && "rotate-180")}
									/>
								</button>
								{menuOpen ? (
									<div
										className="ed-sans absolute top-[calc(100%+14px)] left-0 w-[820px] border border-white/15 bg-[var(--ed-bg)] shadow-[0_32px_80px_-24px_rgb(0_0_0/0.9)]"
										id={menuId}
									>
										<Crosshairs />
										<ProductMenu onNavigate={() => setMenuOpen(false)} />
									</div>
								) : null}
							</li>
							{TOP_LINKS.map((link) => (
								<li key={link.href}>
									<Link
										aria-current={pathname === link.href ? "page" : undefined}
										className={cn(
											"flex h-8 items-center px-2.5 text-[15px] text-[var(--ed-muted)] transition-colors hover:text-[var(--ed-ink)]",
											pathname.startsWith(link.href) && "text-[var(--ed-ink)]",
										)}
										href={link.href}
									>
										{link.label}
									</Link>
								</li>
							))}
						</ul>
					</nav>
				</div>

				<div className="flex items-center gap-3">
					<Link
						className="px-2 text-[15px] text-[var(--ed-ink)] transition-opacity hover:opacity-80"
						href="/sign-in"
					>
						Log in
					</Link>
					<ButtonLink className="hidden sm:inline-flex" href="/sign-up" size="md">
						Get started
					</ButtonLink>
					<button
						aria-controls="ed-mobile-nav"
						aria-expanded={mobileOpen}
						aria-label={mobileOpen ? "Close menu" : "Open menu"}
						className="inline-flex size-9 items-center justify-center rounded-full border border-[var(--ed-line)] text-[var(--ed-ink)] lg:hidden"
						onClick={() => setMobileOpen((open) => !open)}
						type="button"
					>
						{mobileOpen ? (
							<X aria-hidden="true" className="size-4" />
						) : (
							<List aria-hidden="true" className="size-4" />
						)}
					</button>
				</div>
			</div>

			{mobileOpen ? (
				<nav
					aria-label="Mobile"
					className="border-t border-[var(--ed-line)] bg-[var(--ed-bg)] px-4 pt-3 pb-6 sm:px-6 lg:hidden"
					id="ed-mobile-nav"
				>
					{[productGroup, useCaseGroup].map((group) =>
						group ? (
							<div className="mb-4" key={group.label}>
								<p className="ed-mono mb-1 text-[10px] tracking-[0.2em] text-[var(--ed-faint)] uppercase">
									{group.label}
								</p>
								<ul>
									{group.items.map((item) => (
										<li key={item.href}>
											<Link
												className="block py-1.5 text-[15px] text-[var(--ed-ink)]"
												href={item.href}
												onClick={() => setMobileOpen(false)}
											>
												{item.label}
											</Link>
										</li>
									))}
								</ul>
							</div>
						) : null,
					)}
					<ul className="border-t border-[var(--ed-line)] pt-3">
						{TOP_LINKS.map((link) => (
							<li key={link.href}>
								<Link
									className="block py-1.5 text-[15px] text-[var(--ed-ink)]"
									href={link.href}
									onClick={() => setMobileOpen(false)}
								>
									{link.label}
								</Link>
							</li>
						))}
					</ul>
					<div className="mt-4 flex gap-2">
						<ButtonLink href="/sign-up" size="md">
							Start free
						</ButtonLink>
					</div>
				</nav>
			) : null}
		</header>
	);
}
