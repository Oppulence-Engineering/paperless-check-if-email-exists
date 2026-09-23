import { cacheLife } from "next/cache";

import { SimProductsPage } from "../sim-landing/subpages/sim-products-page";
import { marketingMetadata } from "../metadata";

export const metadata = marketingMetadata({
	title: "Products",
	description: "Check one email address, clean CSV lists, or use the self-hosted API.",
	path: "/products",
});

export default async function ProductsRoute() {
	"use cache";
	cacheLife("days");
	return <SimProductsPage />;
}
