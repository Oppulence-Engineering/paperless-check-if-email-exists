import type { MetadataRoute } from "next";
import { baseUrl } from "@/lib/metadata";
import {
	blogCategories,
	blogPostSlug,
	blogPostsInCategory,
	publishedBlogPosts,
} from "@/lib/content/editorial";
import { allCapabilityPages } from "@/app/(marketing)/catalog";

export default function sitemap(): MetadataRoute.Sitemap {
	const paths = new Set([
		"/",
		"/product",
		"/web",
		"/features",
		"/use-cases",
		"/integrations",
		"/guides",
		"/resources",
		"/pricing",
		"/contact",
		"/security",
		"/changelog",
		"/blog",
		"/privacy",
		"/terms",
		"/responsible-disclosure",
		...allCapabilityPages.map((page) => page.path),
		...blogCategories
			.filter((category) => blogPostsInCategory(category).length > 0)
			.map((category) => `/blog/category/${category}`),
		...publishedBlogPosts().map((post) => `/blog/${blogPostSlug(post)}`),
	]);
	return [...paths].map((path) => ({
		url: new URL(path, baseUrl).href,
		changeFrequency: "monthly" as const,
		priority: path === "/" ? 1 : 0.5,
	}));
}
