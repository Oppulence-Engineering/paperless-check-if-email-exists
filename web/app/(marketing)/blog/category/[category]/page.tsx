import { cacheLife } from "next/cache";
import { notFound } from "next/navigation";
import { Suspense } from "react";

import {
	blogCategories,
	blogPostSlug,
	blogPostsInCategory,
	isBlogCategory,
} from "@/lib/content/editorial";

import { EditorialBlogIndex } from "../../../editorial/blog";
import { marketingMetadata } from "../../../metadata";

export function generateStaticParams() {
	return blogCategories.map((category) => ({ category }));
}

export async function generateMetadata({ params }: { params: Promise<{ category: string }> }) {
	const { category } = await params;
	if (!isBlogCategory(category)) return { title: "Page not found — Verifier" };
	return marketingMetadata({
		title: `${category} notes`,
		description: `Verifier notes in the ${category} category.`,
		path: `/blog/category/${category}`,
	});
}

async function renderBlogCategory(category: string) {
	"use cache";
	cacheLife("days");
	if (!isBlogCategory(category)) notFound();
	const posts = blogPostsInCategory(category);

	return (
		<EditorialBlogIndex
			activeCategory={category}
			categories={blogCategories
				.filter((item) => blogPostsInCategory(item).length > 0)
				.map((item) => ({ label: item, href: `/blog/category/${item}` }))}
			posts={posts.map((post) => {
				const slug = blogPostSlug(post);
				return {
					slug,
					href: `/blog/${slug}`,
					title: post.title,
					description: post.description,
					category: post.category,
					date: post.date,
				};
			})}
			subtitle={`Notes about ${category}.`}
			title="Blog."
		/>
	);
}

export default function BlogCategoryPage({ params }: { params: Promise<{ category: string }> }) {
	return (
		<Suspense fallback={null}>
			<BlogCategoryFromParams params={params} />
		</Suspense>
	);
}

async function BlogCategoryFromParams({ params }: { params: Promise<{ category: string }> }) {
	const { category } = await params;
	return renderBlogCategory(category);
}
