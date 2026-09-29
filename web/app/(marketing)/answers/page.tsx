import { cacheLife } from "next/cache";

import { SimAnswersPage } from "../sim-landing/subpages/sim-answers-page";
import { marketingMetadata } from "../metadata";
import { seoAlternativeList, seoLanderList } from "../seo-theme";
import { SITE_NAME, SITE_URL } from "../site";

const TITLE = "Answers";
const DESCRIPTION =
	"Legacy public URLs remain available. Current guidance covers email verification, list hygiene, and the API.";

export const metadata = marketingMetadata({
	title: TITLE,
	description: DESCRIPTION,
	path: "/answers",
});

const itemList = {
	"@context": "https://schema.org",
	"@type": "ItemList",
	name: `${SITE_NAME} answers`,
	itemListElement: seoLanderList.map((lander, index) => ({
		"@type": "ListItem",
		position: index + 1,
		name: lander.query,
		url: `${SITE_URL}/${lander.path}`,
	})),
};

export default async function AnswersPage() {
	"use cache";
	cacheLife("days");

	return (
		<SimAnswersPage
			alternativeItems={seoAlternativeList.map((page) => ({
				href: `/blog/${page.slug}`,
				title: `${page.competitor} alternatives`,
				body: "A retained legacy comparison route with current email verification guidance.",
			}))}
			answerItems={seoLanderList.map((lander) => ({
				href: `/${lander.path}`,
				title: lander.query,
				body: "A retained legacy route with current email verification guidance.",
			}))}
			description={DESCRIPTION}
			itemListJsonLd={itemList}
		/>
	);
}
