import type { ReactNode } from "react";

/*
 * Developer portal styles copied from screenshot-studio
 * (lib/seo/docs-shared.ts and app/[locale]/developers/page.tsx).
 */

export const INTER =
	"Inter, ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif";

export const cardSurface =
	"rounded-2xl bg-card p-4 ring-1 ring-inset ring-border shadow-[var(--card-highlight-shadow)]";

export const linkClassName =
	"text-foreground underline decoration-foreground/30 underline-offset-4 transition-colors hover:decoration-foreground/60";

export const codeBlockClassName =
	"overflow-x-auto rounded-xl bg-foreground/[0.04] p-4 text-[13px] leading-relaxed text-foreground ring-1 ring-inset ring-border";

/** Body text of a docs section. */
export const bodyClassName = "leading-relaxed text-muted-foreground";

/** The docs column: a title, a lead paragraph, and sections 48px apart. */
export function DocsPage({
	children,
	lead,
	title,
}: {
	children: ReactNode;
	lead: ReactNode;
	title: ReactNode;
}) {
	return (
		<div className="mx-auto max-w-3xl flex-1 px-6 pt-28 pb-16 sm:pb-24">
			<h1
				className="mb-4 text-3xl font-semibold tracking-[-0.03em] text-foreground sm:text-4xl"
				style={{ fontFamily: INTER }}
			>
				{title}
			</h1>
			<p className="mb-12 text-lg leading-relaxed text-muted-foreground">{lead}</p>
			<div className="space-y-12">{children}</div>
		</div>
	);
}

export function DocsSection({
	children,
	id,
	title,
}: {
	children: ReactNode;
	id?: string;
	title: ReactNode;
}) {
	return (
		<section id={id}>
			<h2
				className="mb-4 text-xl font-semibold tracking-[-0.02em] text-foreground"
				style={{ fontFamily: INTER }}
			>
				{title}
			</h2>
			{children}
		</section>
	);
}

/** A scrolling code block. It takes keyboard focus so that keyboard users can scroll it. */
export function CodeBlock({ children }: { children: string }) {
	return (
		// eslint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- axe scrollable-region-focusable requires focus on scrolling regions.
		<pre className={codeBlockClassName} tabIndex={0}>
			<code>{children}</code>
		</pre>
	);
}
