import type { Metadata } from "next";
import { cacheLife } from "next/cache";

import { productPage } from "../marketing-data";
import { EditorialProduct } from "../editorial/pages";
import { marketingMetadata } from "../metadata";

export const metadata: Metadata = marketingMetadata({
	title: productPage.title,
	description: productPage.description,
	path: "/product",
});

export default async function ProductRoute() {
	"use cache";
	cacheLife("days");
	return <EditorialProduct />;
}
