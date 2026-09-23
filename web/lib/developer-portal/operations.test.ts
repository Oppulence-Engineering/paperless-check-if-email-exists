import { expect, it } from "vitest";
import { existsSync } from "node:fs";
import { resolve } from "node:path";

import contract from "@/config/contracts/backend.openapi.json";
import { adminOperationFor, families, operations, workflowDestination } from "./operations";

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
		expect(destination.href, operation.id).toMatch(/^\/app\//);
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
