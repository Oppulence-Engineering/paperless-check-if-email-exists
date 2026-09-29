import type { MetadataRoute } from "next";
import { baseUrl } from "@/lib/metadata";
import { blogCategories, blogPostSlug, publishedBlogPosts } from "@/lib/content/editorial";
import { allCapabilityPages } from "@/app/(marketing)/catalog";
import { comparePages } from "@/app/(marketing)/compare-catalog";
import { marketingPaths } from "@/app/(marketing)/marketing-data";

export default function sitemap(): MetadataRoute.Sitemap {
	const paths = new Set([
		"/",
		"/features",
		"/use-cases",
		"/integrations",
		"/guides",
		"/resources",
		"/products",
		"/web",
		"/desktop",
		"/voice-app",
		"/download",
		"/pricing",
		"/compare",
		"/contact",
		"/security",
		"/answers",
		"/changelog",
		"/customers",
		...marketingPaths.map((path) => `/${path}`),
		...allCapabilityPages.map((page) => page.path),
		...comparePages.map((page) => page.path),
		...blogCategories.map((category) => `/blog/category/${category}`),
		...publishedBlogPosts().map((post) => `/blog/${blogPostSlug(post)}`),
	]);
	return [...paths].map((path) => ({
		url: new URL(path, baseUrl).href,
		changeFrequency: "monthly" as const,
		priority: path === "/" ? 1 : 0.5,
	}));
}
