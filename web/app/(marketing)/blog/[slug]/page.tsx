import { cacheLife } from "next/cache";
import { notFound } from "next/navigation";
import { Suspense } from "react";

import { blogPostSlug, publishedBlogPost, publishedBlogPosts } from "@/lib/content/editorial";

import { marketingMetadata } from "../../metadata";
import { EditorialArticle } from "../../article-page";
import { MarkdownBody } from "../../render-markdown";

export function generateStaticParams() {
	return publishedBlogPosts().map((post) => ({ slug: blogPostSlug(post) }));
}

export async function generateMetadata({ params }: { params: Promise<{ slug: string }> }) {
	const { slug } = await params;
	const post = publishedBlogPost(slug);
	if (post) {
		return marketingMetadata({
			title: post.title,
			description: post.description ?? post.title,
			path: `/blog/${slug}`,
		});
	}
	return { title: "Page not found — Verifier" };
}

async function renderBlogSlug(slug: string) {
	"use cache";
	cacheLife("days");
	const post = publishedBlogPost(slug);
	if (post) {
		const raw = await post.getText("raw");
		return (
			<EditorialArticle
				category={post.category}
				categoryHref={`/blog/category/${post.category}`}
				date={post.date}
				description={post.description ?? ""}
				eyebrow={post.category}
				path={`/blog/${slug}`}
				related={[
					{ label: "All notes", href: "/blog", description: "The editorial index." },
					{ label: "Guides", href: "/guides", description: "Evergreen product decisions." },
				]}
				title={post.title}
			>
				<MarkdownBody raw={raw} />
			</EditorialArticle>
		);
	}

	notFound();
}

export default function BlogSlugPage({ params }: { params: Promise<{ slug: string }> }) {
	return (
		<Suspense fallback={null}>
			<BlogSlugFromParams params={params} />
		</Suspense>
	);
}

async function BlogSlugFromParams({ params }: { params: Promise<{ slug: string }> }) {
	const { slug } = await params;
	return renderBlogSlug(slug);
}
