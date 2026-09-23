import React, { createElement, type ImgHTMLAttributes } from "react";
import { vi } from "vitest";

// @sim/emcn icons compile with the classic JSX transform and expect React in
// scope. Next supplies that at runtime; Vitest does not unless we hoist it.
globalThis.React = React;

// Radix-based shadcn controls measure their hidden form controls in jsdom.
globalThis.ResizeObserver = class {
	observe() {}
	unobserve() {}
	disconnect() {}
};

// next/image relies on Next's runtime loader. Component tests only need the
// resulting accessible image contract, so render a native image deterministically.
// Next 16 cache helpers only run inside the App Router. Unit tests call the
// same modules, so treat cacheLife/cacheTag as no-ops instead of throwing.
vi.mock("next/cache", () => ({
	cacheLife: () => undefined,
	cacheTag: () => undefined,
}));

vi.mock("next/image", () => ({
	default: (
		props: Omit<ImgHTMLAttributes<HTMLImageElement>, "src"> & {
			src: string | { src: string };
			fill?: boolean;
			preload?: boolean;
			sizes?: string;
		},
	) => {
		const { src, ...rest } = props;
		delete (rest as Record<string, unknown>).blurDataURL;
		delete rest.fill;
		delete (rest as Record<string, unknown>).placeholder;
		delete rest.preload;
		delete (rest as Record<string, unknown>).priority;
		delete (rest as Record<string, unknown>).quality;
		delete rest.sizes;
		delete (rest as Record<string, unknown>).unoptimized;
		return createElement("img", {
			...rest,
			src: typeof src === "string" ? src : src.src,
		});
	},
}));
