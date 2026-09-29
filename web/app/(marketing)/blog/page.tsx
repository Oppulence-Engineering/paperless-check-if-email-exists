import { cacheLife } from "next/cache";

import { blogCategories, blogPostSlug, publishedBlogPosts } from "@/lib/content/editorial";

import { SimBlogIndexPage } from "../sim-landing/subpages/sim-blog-index-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Blog",
	description:
		"Guides and product notes about email verification, list hygiene, and running the open-source application.",
	path: "/blog",
});

export default async function BlogIndexPage() {
	"use cache";
	cacheLife("days");
	const posts = publishedBlogPosts();

	return (
		<SimBlogIndexPage
			categories={blogCategories.map((category) => ({
				label: category,
				href: `/blog/category/${category}`,
			}))}
			description="Guides about verification results, list hygiene, API use, and running the application."
			eyebrow="[blog]"
			listHeading="Published notes"
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
			title="Notes on email verification."
		/>
	);
}
