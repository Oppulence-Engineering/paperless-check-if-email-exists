"use client";

import Image from "next/image";

import { cn } from "@/lib/sim/cn";

import type { ProductPreviewKey } from "./product-previews";
import { PRODUCT_STAGE_ENTER } from "./product-stage-preview";

interface ProductHeroPreviewProps {
	product: ProductPreviewKey;
	compact?: boolean;
}

/** Feature-page heroes render the same UI as the corresponding navigation preview. */
export function ProductHeroPreview({ product, compact = false }: ProductHeroPreviewProps) {
	return (
		<div
			className={cn(
				"sim-product-preview relative isolate overflow-hidden",
				compact ? "h-full min-h-[240px]" : "h-[420px] max-lg:h-[400px]",
				!compact && product !== "govern" && "max-sm:h-[320px]",
			)}
			data-product-hero={product}
			data-product-preview=""
		>
			<div className={cn("absolute inset-0", !compact && PRODUCT_STAGE_ENTER)}>
				<Image
					alt="Illustrative email verification workspace"
					className="object-cover object-top"
					fill
					sizes="(min-width: 1024px) 800px, 100vw"
					src="/marketing/email-check-preview.svg"
				/>
			</div>
		</div>
	);
}
