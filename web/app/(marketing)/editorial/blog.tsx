import Link from "next/link";
import type { ReactNode } from "react";

import { CONTAINER, MONO, MUTED } from "@/components/platform/landing/primitives";
import { ArrowRight } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { JsonLd } from "../marketing-primitives";
import { articleJsonLd, breadcrumbJsonLd } from "../metadata";
import { LinkCards } from "./page-kit";

export type BlogPostItem = {
	slug: string;
	href: string;
	title: string;
	description?: string;
	category: string;
	date: string;
};

function formatDate(date?: string) {
	if (!date) return "";
	return new Date(date).toLocaleDateString("en-US", {
		month: "short",
		day: "numeric",
		year: "numeric",
		timeZone: "UTC",
	});
}

const PILL = "rounded-full px-3 py-1.5 text-[13px] capitalize transition-colors duration-150";

/** Blog index and category pages. */
export function EditorialBlogIndex({
	title,
	subtitle,
	categories,
	activeCategory,
	posts,
}: {
	title: string;
	subtitle: string;
	categories: { label: string; href: string }[];
	activeCategory?: string;
	posts: BlogPostItem[];
}) {
	return (
		<>
			<section className={cn(CONTAINER, "pt-16 pb-24 sm:pt-24 sm:pb-32")}>
				<p
					className={cn(
						MONO,
						"flex items-center gap-2 text-[12px] tracking-[0.08em] uppercase",
						MUTED,
					)}
				>
					{activeCategory ? (
						<>
							<Link
								className="inline-flex items-center transition-colors hover:text-white"
								href="/blog"
							>
								Blog
							</Link>
							<span aria-hidden="true">/</span>
						</>
					) : null}
					<span className="text-white">{activeCategory ?? "Blog"}</span>
				</p>
				<h1 className="mt-8 text-[40px] leading-[1] tracking-[-0.035em] text-balance sm:text-[56px] lg:text-[64px]">
					<span className="block text-white">{title}</span>{" "}
					<span className={cn("block", MUTED)}>{subtitle}</span>
				</h1>
				{categories.length > 0 ? (
					<nav aria-label="Categories" className="mt-10 flex flex-wrap gap-1">
						<Link
							className={cn(
								PILL,
								!activeCategory ? "bg-white/[0.09] text-white" : "text-[#858585] hover:text-white",
							)}
							href="/blog"
						>
							All
						</Link>
						{categories.map((category) => (
							<Link
								className={cn(
									PILL,
									activeCategory === category.label
										? "bg-white/[0.09] text-white"
										: "text-[#858585] hover:text-white",
								)}
								href={category.href}
								key={category.href}
							>
								{category.label}
							</Link>
						))}
					</nav>
				) : null}
			</section>
			<div className={cn(CONTAINER, "pb-24 sm:pb-32")}>
				<ul className="border-t border-[#212121]">
					{posts.length === 0 ? (
						<li className={cn("py-10 text-lg", MUTED)}>No posts in this category yet.</li>
					) : (
						posts.map((post) => (
							<li className="border-b border-[#212121]" key={post.slug}>
								<Link
									className="group grid gap-3 py-8 md:grid-cols-[1fr_240px] md:items-baseline md:gap-8"
									href={post.href}
								>
									<span>
										<span className="block text-2xl tracking-[-0.02em] text-white">
											{post.title}
										</span>
										{post.description ? (
											<span
												className={cn("mt-2 block max-w-[60ch] text-base leading-[1.55]", MUTED)}
											>
												{post.description}
											</span>
										) : null}
									</span>
									<span
										className={cn(
											MONO,
											"flex items-center justify-between gap-3 text-[12px] tracking-[0.06em] uppercase md:justify-end",
											MUTED,
										)}
									>
										<span>{post.category}</span>
										<span aria-hidden="true">·</span>
										<span>{formatDate(post.date)}</span>
										<ArrowRight
											aria-hidden="true"
											className="size-4 text-white transition-transform group-hover:translate-x-0.5"
										/>
									</span>
								</Link>
							</li>
						))
					)}
				</ul>
			</div>
		</>
	);
}

/** Long-form article with readable prose. */
export function EditorialArticleView({
	title,
	description,
	path,
	eyebrow,
	date,
	category,
	categoryHref,
	children,
	related,
	crumbs,
}: {
	title: string;
	description: string;
	path: string;
	eyebrow: string;
	date?: string;
	category?: string;
	categoryHref?: string;
	children: ReactNode;
	related?: { label: string; href: string; description?: string }[];
	crumbs?: { name: string; path: string }[];
}) {
	return (
		<>
			<JsonLd data={articleJsonLd({ title, description, path })} />
			<JsonLd
				data={breadcrumbJsonLd(
					crumbs ?? [
						{ name: "Home", path: "/" },
						{ name: "Blog", path: "/blog" },
						{ name: title, path },
					],
				)}
			/>
			<article className={cn(CONTAINER, "pt-16 pb-24 sm:pt-24 sm:pb-32")}>
				<div className="mx-auto max-w-[760px]">
					<p
						className={cn(
							MONO,
							"flex items-center gap-2 text-[12px] tracking-[0.08em] uppercase",
							MUTED,
						)}
					>
						<Link
							className="inline-flex items-center transition-colors hover:text-white"
							href="/blog"
						>
							Blog
						</Link>
						<span aria-hidden="true">/</span>
						{categoryHref && category ? (
							<Link className="transition-colors hover:text-white" href={categoryHref}>
								{category}
							</Link>
						) : (
							<span>{eyebrow}</span>
						)}
						{date ? (
							<>
								<span aria-hidden="true">·</span>
								<time className="text-white" dateTime={date}>
									{formatDate(date)}
								</time>
							</>
						) : null}
					</p>
					<h1 className="mt-8 text-[40px] leading-[1.05] tracking-[-0.035em] text-balance text-white sm:text-[56px]">
						{title}
					</h1>
					{description ? (
						<p className="mt-6 text-lg leading-[1.5] text-white/65">{description}</p>
					) : null}
					<div className="ed-prose mt-12 border-t border-[#212121] pt-10">{children}</div>
				</div>
			</article>
			{related && related.length > 0 ? <LinkCards items={related} /> : null}
		</>
	);
}
