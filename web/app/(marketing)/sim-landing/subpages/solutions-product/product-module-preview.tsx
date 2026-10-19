"use client";

import Image from "next/image";
import type { ProductPreviewKey } from "./product-previews";

interface ProductModulePreviewProps {
	product: ProductPreviewKey | "register-interactive";
	layout?: "menu" | "hero" | "stage";
}

/** Showcase and feature visuals — live UI chrome instead of static PNGs. */
export function ProductModulePreview({ product, layout = "menu" }: ProductModulePreviewProps) {
	return (
		<div
			className="relative size-full min-h-[240px]"
			data-preview-layout={layout}
			data-preview-product={product}
		>
			<Image
				alt="Illustrative email verification workspace"
				className="object-cover object-top"
				fill
				sizes="(min-width: 1024px) 800px, 100vw"
				src="/marketing/email-check-preview.svg"
			/>
		</div>
	);
}
