import Link from "next/link";
import type { ReactNode } from "react";

import { ArrowLeft, ArrowRight } from "@/lib/icons";
import { cn } from "@/lib/utils";

import { JsonLd } from "../marketing-primitives";
import { articleJsonLd, breadcrumbJsonLd } from "../metadata";
import { LinkCards } from "./page-kit";
import { Container, Display, Muted, Section } from "./ui";

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
		<Section className="pt-16 sm:pt-20 lg:pt-24">
			<Container>
				<Display as="h1" size="lg">
					{title} <Muted>{subtitle}</Muted>
				</Display>
				{categories.length > 0 ? (
					<nav aria-label="Categories" className="mt-8 flex flex-wrap gap-2">
						<Link
							className={cn(
								"ed-sans rounded-full border px-3.5 py-1.5 text-[13px] capitalize transition-colors",
								!activeCategory
									? "border-[var(--ed-ink)] bg-[var(--ed-ink)] text-[var(--ed-bg)]"
									: "border-[var(--ed-line)] text-[var(--ed-body)] hover:bg-[var(--ed-subtle)]",
							)}
							href="/blog"
						>
							All
						</Link>
						{categories.map((category) => (
							<Link
								className={cn(
									"ed-sans rounded-full border px-3.5 py-1.5 text-[13px] capitalize transition-colors",
									activeCategory === category.label
										? "border-[var(--ed-ink)] bg-[var(--ed-ink)] text-[var(--ed-bg)]"
										: "border-[var(--ed-line)] text-[var(--ed-body)] hover:bg-[var(--ed-subtle)]",
								)}
								href={category.href}
								key={category.href}
							>
								{category.label}
							</Link>
						))}
					</nav>
				) : null}
				<ul className="mt-12 border-t border-[var(--ed-line)]">
					{posts.length === 0 ? (
						<li className="ed-sans py-10 text-[15px] text-[var(--ed-muted)]">
							No posts in this category yet.
						</li>
					) : (
						posts.map((post) => (
							<li className="border-b border-[var(--ed-line)]" key={post.slug}>
								<Link
									className="group grid gap-3 py-8 md:grid-cols-[1fr_200px] md:items-center"
									href={post.href}
								>
									<span>
										<span className="ed-display block text-[24px] leading-[1.3] text-[var(--ed-ink)]">
											{post.title}
										</span>
										{post.description ? (
											<span className="ed-sans mt-1.5 block max-w-[60ch] text-[15px] leading-[1.55] font-[450] text-[var(--ed-muted)]">
												{post.description}
											</span>
										) : null}
									</span>
									<span className="ed-sans flex items-center justify-between gap-3 text-[13px] text-[var(--ed-muted)] md:justify-end">
										<span className="capitalize">{post.category}</span>
										<span aria-hidden="true">·</span>
										<span>{formatDate(post.date)}</span>
										<ArrowRight
											aria-hidden="true"
											className="size-4 transition-transform group-hover:translate-x-0.5"
										/>
									</span>
								</Link>
							</li>
						))
					)}
				</ul>
			</Container>
		</Section>
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
			<article className="bg-[var(--ed-bg)] pt-14 pb-10 sm:pt-20">
				<Container className="max-w-[760px]">
					<Link
						className="ed-sans inline-flex items-center gap-1.5 text-[13px] text-[var(--ed-muted)] hover:text-[var(--ed-ink)]"
						href="/blog"
					>
						<ArrowLeft aria-hidden="true" className="size-3.5" /> Blog
					</Link>
					<p className="ed-sans mt-8 flex items-center gap-2 text-[13px] text-[var(--ed-muted)]">
						{categoryHref && category ? (
							<Link className="capitalize hover:text-[var(--ed-ink)]" href={categoryHref}>
								{category}
							</Link>
						) : (
							<span className="capitalize">{eyebrow}</span>
						)}
						{date ? (
							<>
								<span aria-hidden="true">·</span>
								<time dateTime={date}>{formatDate(date)}</time>
							</>
						) : null}
					</p>
					<Display as="h1" className="mt-3" size="lg">
						{title}
					</Display>
					{description ? (
						<p className="ed-sans mt-4 text-[18px] leading-[1.55] font-[450] text-[var(--ed-muted)]">
							{description}
						</p>
					) : null}
					<div className="ed-prose mt-12 border-t border-[var(--ed-line)] pt-10">{children}</div>
				</Container>
			</article>
			{related && related.length > 0 ? <LinkCards items={related} /> : null}
		</>
	);
}
