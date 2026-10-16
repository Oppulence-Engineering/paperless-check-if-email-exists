import { cacheLife } from "next/cache";

import {
	blogCategories,
	blogPostSlug,
	blogPostsInCategory,
	publishedBlogPosts,
} from "@/lib/content/editorial";

import { EditorialBlogIndex } from "../editorial/blog";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Blog",
	description:
		"Guides and product notes about email verification, list hygiene, and sender reputation.",
	path: "/blog",
});

export default async function BlogIndexPage() {
	"use cache";
	cacheLife("days");
	const posts = publishedBlogPosts();

	return (
		<EditorialBlogIndex
			categories={blogCategories
				.filter((category) => blogPostsInCategory(category).length > 0)
				.map((category) => ({ label: category, href: `/blog/category/${category}` }))}
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
			subtitle="Notes on cleaner sending."
			title="Blog."
		/>
	);
}
