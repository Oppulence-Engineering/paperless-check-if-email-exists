import { expect, it } from "vitest";
import { existsSync, readdirSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";

import contract from "@/config/contracts/backend.openapi.json";
import {
	adminOperationFor,
	families,
	operations,
	referenceDocument,
	referenceHref,
	workflowDestination,
} from "./operations";

it("classifies every API method once for the developer portal", () => {
	const documented = Object.values(contract.paths).reduce(
		(total, item) =>
			total +
			Object.keys(item).filter((method) =>
				["get", "post", "put", "patch", "delete"].includes(method),
			).length,
		0,
	);
	expect(operations).toHaveLength(documented);
	expect(new Set(operations.map((operation) => operation.id)).size).toBe(documented);
	expect(new Set(operations.map((operation) => operation.family))).toEqual(new Set(families));
	expect(
		operations
			.filter((operation) => operation.audience === "tenant" && !operation.scope)
			.map((operation) => operation.path),
	).toEqual(["/v1/me", "/v1/me/usage"]);
	expect(operations.find((operation) => operation.id === "v1_trigger_pipeline")?.scope).toBe(
		"pipelines.trigger",
	);
});

it("matches only declared platform methods and paths", () => {
	expect(adminOperationFor("GET", ["v1", "admin", "tenants"])?.id).toBe("list_tenants");
	expect(adminOperationFor("DELETE", ["v1", "admin", "tenants", "tenant-1"])?.id).toBe(
		"delete_tenant",
	);
	expect(adminOperationFor("GET", ["v1", "admin", "unknown"])).toBeUndefined();
	expect(adminOperationFor("POST", ["v1", "admin", "tenants", "tenant-1"])).toBeUndefined();
	expect(adminOperationFor("GET", ["v1", "me"])).toBeUndefined();
});

it("gives every operation a reachable product or setup journey", () => {
	for (const operation of operations) {
		const destination = workflowDestination(operation);
		expect(destination.label, operation.id).toBeTruthy();
		expect(destination.href, operation.id).toMatch(/^\/app(\/|\?|$)/);
		expect(destination.href, operation.id).not.toBe("/app/api");
		const pathname = destination.href.split("?")[0];
		expect(
			existsSync(resolve(process.cwd(), "app/(product)", pathname.slice(1), "page.tsx")),
			`${operation.id} points to a missing product page: ${pathname}`,
		).toBe(true);
		if (["system", "legacy", "onboarding", "callback"].includes(operation.audience)) {
			expect(destination.href, operation.id).toBe("/app/integrations");
		}
	}
});

type ReferenceDocument = {
	servers: { url: string }[];
	security: Record<string, unknown[]>[];
	paths: Record<string, Record<string, { tags: string[]; security?: unknown[] }>>;
	components: Record<string, Record<string, { type?: string; scheme?: string }>>;
	"x-tagGroups": { name: string; tags: string[] }[];
};

function referencesIn(value: unknown): string[] {
	return JSON.stringify(value).match(/#\/components\/[A-Za-z]+\/[\w.-]+/g) ?? [];
}

it("builds the workspace reference from tenant operations only", () => {
	const document = referenceDocument("workspace") as ReferenceDocument;
	const tenantPaths = new Set(
		operations.filter((operation) => operation.audience === "tenant").map((op) => op.path),
	);
	expect(new Set(Object.keys(document.paths))).toEqual(tenantPaths);
	expect(document.servers.map((server) => server.url)).toEqual(["/api/backend", "/"]);
	// The session server needs no key; the public server needs one.
	expect(document.security).toEqual([{}, { Authorization: [] }]);
	for (const item of Object.values(document.paths)) {
		for (const operation of Object.values(item)) expect(operation.security).toBeUndefined();
	}
	expect(document["x-tagGroups"].map((group) => group.name)).toEqual(["Workspace API"]);
});

it("builds the public reference without admin operations", () => {
	const document = referenceDocument("public") as ReferenceDocument;
	const paths = Object.keys(document.paths);
	expect(paths.some((path) => path.startsWith("/v1/admin"))).toBe(false);
	expect(paths).not.toContain("/version");
	// It takes the operator secret, like the admin routes.
	expect(paths).not.toContain("/v1/check-email-with-onboard");
	expect(paths).toContain("/v1/check_email");
	expect(paths).toContain("/v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token}");
	expect(document.security).toEqual([{ Authorization: [] }]);
	expect(document.components.securitySchemes.Authorization).toMatchObject({
		type: "http",
		scheme: "bearer",
	});
	expect(document.components.securitySchemes.AdminSecret).toBeUndefined();
	expect(document.components.schemas.AdminApiKey).toBeUndefined();
	const tagged = new Set(
		Object.values(document.paths).flatMap((item) =>
			Object.values(item).flatMap((operation) => operation.tags),
		),
	);
	expect(new Set(document["x-tagGroups"].flatMap((group) => group.tags))).toEqual(tagged);
});

it("keeps every component reference of both references resolvable", () => {
	for (const surface of ["workspace", "public"] as const) {
		const document = referenceDocument(surface) as ReferenceDocument;
		for (const ref of referencesIn(document)) {
			const [section, name] = ref.replace("#/components/", "").split("/");
			expect(document.components[section]?.[name], `${surface}: ${ref}`).toBeDefined();
		}
	}
});

it("links each operation to its Scalar anchor", () => {
	expect(referenceHref("v1_check_email")).toBe(
		"/developers/reference#tag/verification-and-finder/v1_check_email",
	);
	expect(referenceHref("list_tenants")).toBe("/developers/reference");
	expect(referenceHref("unknown")).toBe("/developers/reference");
	// Links in the app use the anchors directly, so they must match.
	const files = readdirSync(resolve(process.cwd(), "app"), { recursive: true, withFileTypes: true })
		.filter((entry) => entry.isFile() && /\.tsx?$/.test(entry.name))
		.map((entry) => join(entry.parentPath, entry.name));
	for (const file of files) {
		for (const [href, id] of readFileSync(file, "utf8").matchAll(
			/\/developers\/reference#tag\/[\w-]+\/(\w+)/g,
		)) {
			expect(href, file).toBe(referenceHref(id));
		}
	}
});
