/**
 * Permanent redirects for marketing URLs that moved or were removed in the
 * 2026-10 marketing rework. Old slugs came from the site template; they now
 * point at the closest page that describes this product.
 */
export type MarketingRedirect = { source: string; destination: string; permanent: true };

const moved: Record<string, string> = {
	// Platform pages.
	"/desktop": "/product",
	"/download": "/product",
	"/voice-app": "/features/list-cleaning",
	"/products": "/product",

	// Features.
	"/features/commitment-register": "/features/single-checks",
	"/features/account-mission-control": "/features/workspace",
	"/features/attention-queue": "/features/list-cleaning",
	"/features/governed-actions": "/features/api",
	"/features/meeting-capture": "/features/single-checks",
	"/features/live-notes": "/features/risk-scoring",
	"/features/local-vault": "/security",
	"/features/agents": "/features/reverification",

	// Use cases.
	"/use-cases/founder-operator": "/use-cases/email-marketing",
	"/use-cases/account-management": "/use-cases/marketing-operations",
	"/use-cases/meetings": "/features/list-cleaning",
	"/use-cases/research": "/features/risk-scoring",

	// Integrations.
	"/integrations/gmail": "/integrations/csv",
	"/integrations/calendar": "/integrations/csv",
	"/integrations/slack": "/integrations",
	"/integrations/hubspot": "/integrations/csv",
	"/integrations/mcp": "/integrations/api",

	// Guides.
	"/guides/desktop-vs-web": "/guides/reading-results",
	"/guides/what-stays-on-device": "/security",
	"/guides/approval-before-send": "/guides/reading-results",
	"/guides/install-oppulence": "/product",

	// Removed sections.
	"/answers": "/guides",
	"/compare": "/product",
	"/compare/:slug*": "/product",
	"/customers": "/",
	"/customers/:slug*": "/",
};

/** Template landing pages for an unrelated product. */
const removedLanders = [
	"ai-documentation-agent",
	"ai-faq-generator",
	"ai-help-center",
	"api-documentation-software",
	"automated-screenshots-for-docs",
	"chrome-extension-for-documentation",
	"code-to-docs",
	"generative-ai-customer-service",
	"help-center-software",
	"internal-knowledge-base",
	"lp/ai-help-center",
	"multilingual-knowledge-base",
	"self-service-help-widget",
	"self-updating-help-center",
	"tools/docs-debt-quiz",
	"tools/openapi-validator",
];

/** Archived template blog URLs. The live blog keeps its own slugs. */
export const removedBlogSlugs = [
	"14-stunning-zendesk-help-center-examples",
	"ai-documentation-generators-the-ultimate-guide",
	"automated-knowledge-base-the-complete-guide",
	"automatic-documentation-made-easy-for-your-business",
	"best-ai-powered-help-center-software-4-of-the-best",
	"best-api-documentation-tools",
	"best-archbee-alternatives",
	"best-document360-alternatives",
	"best-docusaurus-alternatives",
	"best-free-knowledge-base-software",
	"best-gitbook-alternatives",
	"best-help-center-software",
	"best-help-center-software-for-saas",
	"best-help-scout-alternatives",
	"best-helpdocs-alternatives",
	"best-helpjuice-alternatives",
	"best-intercom-help-center-alternatives",
	"best-it-documentation-software",
	"best-knowledge-base-for-small-teams",
	"best-knowledgeowl-alternatives",
	"best-mintlify-alternatives",
	"best-proprofs-knowledge-base-alternatives",
	"best-software-documentation-tools",
	"best-stonly-alternatives",
	"best-technical-writing-tools",
	"best-zendesk-help-center-alternatives",
	"best-zoho-desk-alternatives",
	"building-embeddable-widgets-with-svelte",
	"documentation-drift",
	"docusaurus-pricing",
	"docusaurus-review",
	"freshdesk-pricing",
	"freshdesk-vs-zendesk-review",
	"giorgas-help-center-examples-ecommerce",
	"gitbook-pricing",
	"gitbook-review",
	"gitbook-vs-mintlify",
	"help-desk-providers",
	"help-documentation-software",
	"help-scout-pricing",
	"helpdocs-pricing",
	"helpdocs-review",
	"helpjuice-pricing",
	"helpjuice-review",
	"how-to-hire-a-technical-writer",
	"how-to-keep-knowledge-base-up-to-date-automatically",
	"how-to-price-your-ai-saas-2-mistakes-we-made",
	"how-to-refresh-out-of-date-help-center",
	"how-to-scale-customer-support",
	"how-to-talk-to-users-without-getting-overwhelmed-my-system",
	"knowledge-base-for-saas-startups",
	"knowledge-base-maintenance-checklist",
	"knowledgeowl-pricing",
	"knowledgeowl-review",
	"mintlify-pricing",
	"mintlify-review",
	"open-source-knowledge-base-software",
	"proprofs-kb-pricing",
	"proprofs-kb-review",
	"saas-self-service-strategies",
	"screenshot-size-for-documentation",
	"technical-writer-interview-questions",
	"technical-writer-job-description-template",
	"user-manual-software",
];

export const marketingRedirects: MarketingRedirect[] = [
	...Object.entries(moved).map(([source, destination]) => ({
		source,
		destination,
		permanent: true as const,
	})),
	...removedLanders.map((slug) => ({
		source: `/${slug}`,
		destination: "/",
		permanent: true as const,
	})),
	...removedBlogSlugs.map((slug) => ({
		source: `/blog/${slug}`,
		destination: "/blog",
		permanent: true as const,
	})),
];
