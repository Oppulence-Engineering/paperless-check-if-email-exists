export type ProductDemoBeatId = "read" | "confirm" | "watch";

export const PRODUCT_DEMO_BEATS: Readonly<Record<ProductDemoBeatId, string>> = {
	read: "Check one address.",
	confirm: "See why it is risky.",
	watch: "Clean the whole list.",
};

export const PRODUCT_DEMO_FRAMES: Readonly<
	Record<ProductDemoBeatId, { src: string; alt: string }>
> = {
	read: {
		src: "/marketing/email-check-preview.svg",
		alt: "Sample email check result",
	},
	confirm: {
		src: "/marketing/email-check-preview.svg",
		alt: "Sample result with reason codes",
	},
	watch: {
		src: "/marketing/email-check-preview.svg",
		alt: "Sample list results",
	},
};

export const PRODUCT_DEMO_BEAT_ORDER: ProductDemoBeatId[] = ["read", "confirm", "watch"];

export const PRODUCT_DEMO_BEAT_MS = 4200;
