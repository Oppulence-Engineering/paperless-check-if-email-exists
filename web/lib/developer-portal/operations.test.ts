import { expect, it } from "vitest";

import contract from "@/config/contracts/backend.openapi.json";
import { families, operations } from "./operations";

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
