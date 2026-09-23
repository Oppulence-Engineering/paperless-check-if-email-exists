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
	paths: Record<string, Partial<Record<Method, OperationDocument>>>;
	security: Record<string, unknown[]>[];
	components: { schemas: Record<string, unknown> };
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

export type Family = (typeof families)[number];

function familyFor(path: string): Family | undefined {
	if (["/healthz", "/readyz", "/version", "/openapi.json"].includes(path)) return "System";
	if (path.startsWith("/v0/")) return "Legacy v0";
	if (path.startsWith("/v1/admin/")) return "Platform admin";
	if (path === "/v1/check-email-with-onboard") return "Onboarding";
	if (path.startsWith("/v1/inbound/providers/")) return "Provider callback";
	if (path.startsWith("/v1/me")) return "Workspace account";
	if (
		[
			"/v1/check_email",
			"/v1/emails/",
			"/v1/find_email",
			"/v1/reputation/",
			"/v1/reverification/",
		].some((prefix) => path.startsWith(prefix))
	)
		return "Verification and finder";
	if (
		["/v1/bulk", "/v1/jobs/", "/v1/query", "/v1/events", "/v1/sources/"].some((prefix) =>
			path.startsWith(prefix),
		)
	)
		return "Bulk and observability";
	if (["/v1/lists", "/v1/comments"].some((prefix) => path.startsWith(prefix)))
		return "Lists and collaboration";
	if (path.startsWith("/v1/suppressions")) return "Suppressions";
	if (["/v1/outcomes", "/v1/provider-endpoints"].some((prefix) => path.startsWith(prefix)))
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
			if (operation.path.startsWith("/v1/reverification/"))
				return { href: "/app/settings?settings=verification", label: "Open reverification status" };
			if (operation.path.startsWith("/v1/reputation/"))
				return { href: "/app/analytics", label: "Open domain reputation" };
			return { href: "/app/check", label: "Open verification" };
		case "Bulk and observability":
			return ["/v1/events", "/v1/query", "/v1/sources/"].some((prefix) =>
				operation.path.startsWith(prefix),
			)
				? { href: "/app/analytics", label: "Open analytics" }
				: { href: "/app/jobs", label: "Open bulk jobs" };
		case "Lists and collaboration":
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
	if (["check_email", "emails"].includes(route)) return "verify";
	if (["bulk", "jobs", "events", "query", "sources"].includes(route)) return "bulk";
	if (route === "find_email") return "find";
	if (route === "lists") return "lists";
	if (["suppressions", "outcomes"].includes(route)) return "suppressions";
	if (route === "reputation") return "reputation";
	if (["provider-endpoints", "reverification"].includes(route)) return "settings";
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

export function operationById(id: string): PortalOperation | undefined {
	return operations.find((operation) => operation.id === id);
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

export function schemaByRef(ref: string): unknown {
	const name = ref.replace("#/components/schemas/", "");
	return spec.components.schemas[name];
}
