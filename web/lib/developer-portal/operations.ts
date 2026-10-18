import "server-only";

import contract from "@/config/contracts/backend.openapi.json";

const methods = ["get", "post", "put", "patch", "delete"] as const;
type Method = (typeof methods)[number];

type OperationDocument = {
	operationId?: string;
	summary?: string;
	description?: string;
	parameters?: unknown[];
	requestBody?: unknown;
	responses?: Record<string, unknown>;
	security?: Record<string, unknown[]>[];
};

type Contract = {
	info: { version: string };
	paths: Record<string, Partial<Record<Method, OperationDocument>>>;
	security: Record<string, unknown[]>[];
	components: Record<string, Record<string, unknown>> & {
		securitySchemes: Record<string, unknown>;
	};
};

const spec = contract as unknown as Contract;

export const families = [
	"Verification and finder",
	"Bulk and observability",
	"Lists and collaboration",
	"Suppressions",
	"Feedback and providers",
	"Pipelines",
	"Workspace account",
	"Provider callback",
	"Onboarding",
	"Platform admin",
	"Legacy v0",
	"System",
] as const;

type Family = (typeof families)[number];

function familyFor(path: string): Family | undefined {
	if (["/healthz", "/readyz", "/version", "/openapi.json"].includes(path)) return "System";
	if (path.startsWith("/v0/")) return "Legacy v0";
	if (path.startsWith("/v1/admin/")) return "Platform admin";
	if (path === "/v1/check-email-with-onboard") return "Onboarding";
	if (path.startsWith("/v1/inbound/providers/")) return "Provider callback";
	if (["/v1/me", "/v1/score-policies"].some((prefix) => path.startsWith(prefix)))
		return "Workspace account";
	if (
		[
			"/v1/check_email",
			"/v1/emails/",
			"/v1/find_email",
			"/v1/reputation/",
			"/v1/reverification",
			"/v1/alerts",
		].some((prefix) => path.startsWith(prefix))
	)
		return "Verification and finder";
	if (
		["/v1/bulk", "/v1/jobs/", "/v1/query", "/v1/events", "/v1/sources/"].some((prefix) =>
			path.startsWith(prefix),
		)
	)
		return "Bulk and observability";
	if (["/v1/lists", "/v1/comments", "/v1/segments"].some((prefix) => path.startsWith(prefix)))
		return "Lists and collaboration";
	if (path.startsWith("/v1/suppressions")) return "Suppressions";
	if (
		[
			"/v1/outcomes",
			"/v1/provider-endpoints",
			"/v1/campaign-outcomes",
			"/v1/outcome-policies",
		].some((prefix) => path.startsWith(prefix))
	)
		return "Feedback and providers";
	if (path.startsWith("/v1/pipelines")) return "Pipelines";
}

export type PortalOperation = {
	id: string;
	method: Uppercase<Method>;
	path: string;
	family: Family;
	summary: string;
	description?: string;
	security: Record<string, unknown[]>[];
	parameters: unknown[];
	requestBody?: unknown;
	responses: Record<string, unknown>;
	audience: "tenant" | "admin" | "legacy" | "callback" | "onboarding" | "system";
	scope?: string;
};

export function workflowDestination(operation: PortalOperation): { href: string; label: string } {
	switch (operation.family) {
		case "System":
		case "Legacy v0":
		case "Onboarding":
		case "Provider callback":
			return { href: "/app/integrations", label: "Open setup and status journey" };
		case "Verification and finder":
			if (operation.path.startsWith("/v1/find_email"))
				return { href: "/app/finder", label: "Open finder" };
			if (operation.path.startsWith("/v1/emails/"))
				return { href: "/app/history", label: "Open history" };
			if (operation.path.startsWith("/v1/reverification"))
				return { href: "/app/settings?settings=verification", label: "Open re-check settings" };
			if (operation.path.startsWith("/v1/reputation/"))
				return { href: "/app/domains", label: "Open sender domains" };
			if (operation.path.startsWith("/v1/alerts")) return { href: "/app", label: "Open alerts" };
			return { href: "/app/check", label: "Open verification" };
		case "Bulk and observability":
			return ["/v1/events", "/v1/query", "/v1/sources/"].some((prefix) =>
				operation.path.startsWith(prefix),
			)
				? { href: "/app/analytics", label: "Open analytics" }
				: { href: "/app/jobs", label: "Open bulk jobs" };
		case "Lists and collaboration":
			if (operation.path.startsWith("/v1/segments"))
				return { href: "/app/analytics", label: "Open saved segments" };
			return { href: "/app/lists", label: "Open lists" };
		case "Suppressions":
			return { href: "/app/suppressions", label: "Open suppressions" };
		case "Feedback and providers":
			return { href: "/app/outcomes", label: "Open outcomes and provider setup" };
		case "Pipelines":
			return { href: "/app/pipelines", label: "Open pipelines" };
		case "Workspace account":
			if (operation.path.startsWith("/v1/me/api-keys"))
				return { href: "/app/settings?settings=developer", label: "Manage workspace API keys" };
			if (operation.path.startsWith("/v1/me/domains"))
				return { href: "/app/domains", label: "Manage workspace domains" };
			if (operation.path.startsWith("/v1/me/webhook"))
				return { href: "/app/settings?settings=webhooks", label: "Open webhook settings" };
			if (operation.path.startsWith("/v1/me/usage"))
				return { href: "/app/settings?settings=usage", label: "Open usage" };
			return { href: "/app/settings?settings=verification", label: "Open workspace settings" };
		case "Platform admin":
			return { href: "/app/admin/api", label: "Open platform operations" };
	}
}

function audienceFor(family: Family): PortalOperation["audience"] {
	switch (family) {
		case "Platform admin":
			return "admin";
		case "Legacy v0":
			return "legacy";
		case "Provider callback":
			return "callback";
		case "Onboarding":
			return "onboarding";
		case "System":
			return "system";
		default:
			return "tenant";
	}
}

function scopeFor(path: string, method: Uppercase<Method>): string | undefined {
	if (path.startsWith("/v1/comments")) return "lists or bulk, based on the linked resource";
	if (path.startsWith("/v1/pipelines")) {
		if (path.endsWith("/trigger") || path.endsWith("/push")) return "pipelines.trigger";
		return method === "GET" ? "pipelines.read" : "pipelines.write";
	}
	const route = path.split("/")[2];
	if (["check_email", "emails", "alerts"].includes(route)) return "verify";
	if (["bulk", "jobs", "events", "query", "sources"].includes(route)) return "bulk";
	if (route === "find_email") return "find";
	if (["lists", "segments"].includes(route)) return "lists";
	if (route === "campaign-outcomes") return method === "GET" ? "outcomes.read" : "outcomes.write";
	if (["suppressions", "outcomes"].includes(route)) return "suppressions";
	if (route === "reputation") return "reputation";
	if (
		["provider-endpoints", "reverification", "score-policies", "outcome-policies"].includes(route)
	)
		return "settings";
	if (route === "me") {
		const resource = path.split("/")[3];
		if (resource === "api-keys") return "admin";
		if (["settings", "domains", "webhook"].includes(resource)) return "settings";
	}
}

export const operations: PortalOperation[] = Object.entries(spec.paths)
	.flatMap(([path, pathItem]) =>
		methods.flatMap((method) => {
			const document = pathItem[method];
			if (!document) return [];
			const family = familyFor(path);
			if (!family) throw new Error(`Unclassified API operation: ${method.toUpperCase()} ${path}`);
			if (!document.operationId)
				throw new Error(`Missing operationId: ${method.toUpperCase()} ${path}`);
			return [
				{
					id: document.operationId,
					method: method.toUpperCase() as Uppercase<Method>,
					path,
					family,
					audience: audienceFor(family),
					scope:
						audienceFor(family) === "tenant"
							? scopeFor(path, method.toUpperCase() as Uppercase<Method>)
							: undefined,
					summary: document.summary || `${method.toUpperCase()} ${path}`,
					description: document.description,
					security: document.security ?? spec.security,
					parameters: document.parameters ?? [],
					requestBody: document.requestBody,
					responses: document.responses ?? {},
				},
			];
		}),
	)
	.sort(
		(a, b) =>
			families.indexOf(a.family) - families.indexOf(b.family) ||
			a.path.localeCompare(b.path) ||
			a.method.localeCompare(b.method),
	);

if (new Set(operations.map((operation) => operation.id)).size !== operations.length) {
	throw new Error("Duplicate API operationId in developer portal contract");
}

export function adminOperationFor(method: string, path: string[]): PortalOperation | undefined {
	return operations.find((operation) => {
		if (operation.audience !== "admin" || operation.method !== method) return false;
		const parts = operation.path.slice(1).split("/");
		return (
			parts.length === path.length &&
			parts.every(
				(part, index) => (part.startsWith("{") && part.endsWith("}")) || part === path[index],
			)
		);
	});
}

/** What each family covers. Scalar shows it under the family heading. */
const FAMILY_NOTES: Record<Family, string> = {
	"Verification and finder":
		"Check one address, find an address from a name and a domain, read the history of an address, and check the reputation of a domain.",
	"Bulk and observability":
		"Run bulk jobs, follow their progress, read the results, and query the audit log.",
	"Lists and collaboration":
		"Upload lists, review and clean them, compare two lists, and save segments.",
	Suppressions:
		"Keep the addresses that must not get mail, with a reason, an expiry, and an audit trail.",
	"Feedback and providers":
		"Send delivery outcomes and campaign results, and connect your email providers.",
	Pipelines: "Check lists again on a schedule and deliver the results.",
	"Workspace account":
		"Manage API keys, sender domains, the webhook, usage, settings, and score policies.",
	"Provider callback":
		"Email providers call these endpoints with delivery events. Each call uses the delivery token of its endpoint, not an API key.",
	Onboarding: "Check an address during sign-up, before a workspace exists.",
	"Legacy v0":
		"The first version of the API. It uses the x-reacher-secret header. New integrations use v1.",
	System: "Health and readiness checks. They need no authentication.",
	"Platform admin": "Platform operator endpoints.",
};

const WORKSPACE_GROUP = "Workspace API";

const BEARER = {
	type: "http",
	scheme: "bearer",
	description:
		"Send `Authorization: Bearer <key>`. Create keys in [Developer settings](/app/settings?settings=developer). Each key has scopes.",
};

const PUBLIC_DESCRIPTION = `Check email addresses, clean lists, and protect your sender reputation over HTTP.

1. Create an API key in [Developer settings](/app/settings?settings=developer).
2. Send it as \`Authorization: Bearer <key>\`.
3. Each key has scopes. Each operation shows the scope that it needs.

The full contract is at [/openapi.json](/openapi.json).`;

const WORKSPACE_DESCRIPTION = `Run the operations of your workspace from the browser.

- **This workspace** sends each request with your signed-in session. You do not need a key.
- **Public API** needs an API key from [Developer settings](/app/settings?settings=developer). Use it to test what your integration sends.

The [public API reference](/developers/reference) has the same operations, with code samples for each language.`;

function familySlug(family: Family): string {
	return family.toLowerCase().replaceAll(" ", "-");
}

/**
 * Every `#/components/<section>/<name>` reference in a value: `$ref` values
 * and discriminator `mapping` values.
 */
function referencesIn(value: unknown, found: Set<string>): Set<string> {
	if (typeof value === "string") {
		if (value.startsWith("#/components/")) found.add(value);
	} else if (Array.isArray(value)) {
		for (const item of value) referencesIn(item, found);
	} else if (value && typeof value === "object") {
		for (const item of Object.values(value)) referencesIn(item, found);
	}
	return found;
}

function componentExists(ref: string): boolean {
	const [section, name] = ref.replace("#/components/", "").split("/");
	return spec.components[section]?.[name] !== undefined;
}

/**
 * Drops discriminator mappings to schemas that the contract does not define.
 * The backend contract maps PipelineSource to `PipelineSource_oneOf_*` names,
 * but its variants are inline. Returns a copy; the contract does not change.
 */
function withoutDanglingMappings(component: unknown): unknown {
	const discriminator = (component as { discriminator?: { mapping?: Record<string, string> } })
		.discriminator;
	if (!discriminator?.mapping) return component;
	const mapping = Object.fromEntries(
		Object.entries(discriminator.mapping).filter(([, ref]) => componentExists(ref)),
	);
	return { ...(component as object), discriminator: { ...discriminator, mapping } };
}

/** Keeps only the components that the paths reach, so admin models stay out. */
function reachableComponents(paths: unknown): Record<string, Record<string, unknown>> {
	const kept: Record<string, Record<string, unknown>> = {};
	const pending = [...referencesIn(paths, new Set())];
	const seen = new Set<string>();
	while (pending.length > 0) {
		const ref = pending.pop() as string;
		if (seen.has(ref)) continue;
		seen.add(ref);
		if (!componentExists(ref)) continue;
		const [section, name] = ref.replace("#/components/", "").split("/");
		const component = withoutDanglingMappings(spec.components[section][name]);
		kept[section] = { ...kept[section], [name]: component };
		pending.push(...referencesIn(component, new Set()));
	}
	return kept;
}

/**
 * The OpenAPI document that Scalar shows. "workspace" has the tenant
 * operations for the signed-in explorer, with the session server first.
 * "public" has every operation except the platform admin ones.
 */
export function referenceDocument(surface: "workspace" | "public"): Record<string, unknown> {
	const included = operations.filter((operation) =>
		surface === "workspace"
			? operation.audience === "tenant"
			: operation.audience !== "admin" && operation.path !== "/version",
	);
	const paths: Record<string, Record<string, unknown>> = {};
	const schemeNames = new Set(["Authorization"]);
	for (const operation of included) {
		const method = operation.method.toLowerCase() as Method;
		const source = spec.paths[operation.path][method] as OperationDocument &
			Record<string, unknown>;
		const destination = workflowDestination(operation);
		const links = [
			operation.scope ? `**API key scope:** \`${operation.scope}\`` : undefined,
			`[${destination.label}](${destination.href})`,
		]
			.filter(Boolean)
			.join(" · ");
		// The workspace document makes the key optional on every tenant operation.
		const security = surface === "workspace" ? undefined : source.security;
		for (const requirement of security ?? []) {
			for (const name of Object.keys(requirement)) schemeNames.add(name);
		}
		paths[operation.path] = {
			...paths[operation.path],
			[method]: {
				...source,
				tags: [operation.family],
				description: [source.description, links].filter(Boolean).join("\n\n"),
				security,
			},
		};
	}

	const used = families.filter((family) =>
		included.some((operation) => operation.family === family),
	);
	const workspaceTags = used.filter((family) => audienceFor(family) === "tenant");
	const otherTags = used.filter((family) => audienceFor(family) !== "tenant");
	const securitySchemes = Object.fromEntries(
		[...schemeNames].map((name) => [
			name,
			name === "Authorization" ? BEARER : spec.components.securitySchemes[name],
		]),
	);

	return {
		openapi: (contract as { openapi: string }).openapi,
		info: {
			title: surface === "workspace" ? "Workspace API" : "Verifier API",
			version: spec.info.version,
			description: surface === "workspace" ? WORKSPACE_DESCRIPTION : PUBLIC_DESCRIPTION,
		},
		servers:
			surface === "workspace"
				? [
						{ url: "/api/backend", description: "This workspace (signed-in session)" },
						{ url: "/", description: "Public API (API key)" },
					]
				: [{ url: "/", description: "Verifier API" }],
		security: surface === "workspace" ? [{}, { Authorization: [] }] : [{ Authorization: [] }],
		tags: used.map((family) => ({ name: family, description: FAMILY_NOTES[family] })),
		"x-tagGroups": [
			{ name: WORKSPACE_GROUP, tags: workspaceTags },
			...(otherTags.length > 0
				? [{ name: "Integrations, onboarding, and status", tags: otherTags }]
				: []),
		],
		paths,
		components: { ...reachableComponents(paths), securitySchemes },
	};
}

/**
 * The public reference link for an operation. Scalar names each operation
 * `#tag/<family>/<operationId>`. Admin and unknown operations go to the index.
 */
export function referenceHref(operationId: string): string {
	const operation = operations.find((item) => item.id === operationId);
	if (!operation || operation.audience === "admin" || operation.path === "/version") {
		return "/developers/reference";
	}
	return `/developers/reference#tag/${familySlug(operation.family)}/${operation.id}`;
}
