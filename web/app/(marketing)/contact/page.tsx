import { cacheLife } from "next/cache";

import { SimContactPage } from "../sim-landing/subpages/sim-contact-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Contact",
	description: "How to reach the Check If Email Exists team for product and security questions.",
	path: "/contact",
});

export default async function ContactPage() {
	"use cache";
	cacheLife("days");
	return <SimContactPage />;
}
