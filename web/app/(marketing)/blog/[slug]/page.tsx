import { cacheLife } from "next/cache";
import { notFound } from "next/navigation";
import { Suspense } from "react";

import { blogPostSlug, publishedBlogPost, publishedBlogPosts } from "@/lib/content/editorial";

import { blogPages, getMarketingPage } from "../../marketing-data";
import { marketingMetadata } from "../../metadata";
import { EditorialArticle } from "../../article-page";
import { MarkdownBody } from "../../render-markdown";
import { alternativeFromSlug } from "../../seo-theme";
import { SimBlogArchivePage } from "../../sim-landing/subpages/sim-marketing-bullet-page";
import { SimSeoAlternativePage } from "../../sim-landing/subpages/sim-seo-alternative-page";

export function generateStaticParams() {
	const editorial = publishedBlogPosts().map((post) => blogPostSlug(post));
	const archive = blogPages.map((page) => page.path.replace(/^blog\//, ""));
	return [...new Set([...editorial, ...archive])].map((slug) => ({ slug }));
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
	const archive = getMarketingPage(`blog/${slug}`);
	if (archive) {
		return marketingMetadata({
			title: archive.title,
			description: archive.description,
			path: `/${archive.path}`,
		});
	}
	return { title: "Page not found — Check If Email Exists" };
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

	const archive = getMarketingPage(`blog/${slug}`);
	if (archive) {
		const oldAlternative = alternativeFromSlug(slug);
		if (oldAlternative) {
			const alternative = {
				...oldAlternative,
				title: archive.title,
				description: archive.description,
				whatTheyAre:
					"This legacy comparison route is kept for existing links. Consult the named product's current documentation for its features and pricing.",
				whatWeAre:
					"Check If Email Exists checks address syntax, DNS, MX, and mailbox signals without sending a message.",
				keepThemWhen: ["You need the named product's original use case."],
				chooseUsWhen: ["You need to check an email address or clean a CSV list."],
				faqs: [
					{
						question: "What does Check If Email Exists do?",
						answer:
							"It verifies email addresses and reports the available signals and uncertainty.",
					},
				],
			};
			return <SimSeoAlternativePage alternative={alternative} page={archive} />;
		}
		return <SimBlogArchivePage page={archive} />;
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
