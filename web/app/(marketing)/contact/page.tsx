import { cacheLife } from "next/cache";

import { EditorialContact } from "../editorial/pages";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Contact",
	description: "How to reach the Verifier team for product and security questions.",
	path: "/contact",
});

export default async function ContactPage() {
	"use cache";
	cacheLife("days");
	return <EditorialContact />;
}
