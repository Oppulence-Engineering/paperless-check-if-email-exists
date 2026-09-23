export type ProductDemoBeatId = "read" | "confirm" | "watch";

export const PRODUCT_DEMO_BEATS: Readonly<Record<ProductDemoBeatId, string>> = {
	read: "Check one address.",
	confirm: "Review the signals.",
	watch: "Clean a whole list.",
};

export const PRODUCT_DEMO_FRAMES: Readonly<
	Record<ProductDemoBeatId, { src: string; alt: string }>
> = {
	read: {
		src: "/marketing/email-check-preview.svg",
		alt: "Email verification result preview",
	},
	confirm: {
		src: "/marketing/email-check-preview.svg",
		alt: "Email verification signals preview",
	},
	watch: {
		src: "/marketing/email-check-preview.svg",
		alt: "Bulk list verification preview",
	},
};

export const PRODUCT_DEMO_BEAT_ORDER: ProductDemoBeatId[] = ["read", "confirm", "watch"];

export const PRODUCT_DEMO_BEAT_MS = 4200;
