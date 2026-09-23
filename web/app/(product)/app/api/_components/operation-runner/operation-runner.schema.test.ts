import { describe, expect, it } from "vitest";

import { OperationRunnerPropsSchema } from "./operation-runner.schema";

describe("OperationRunnerPropsSchema", () => {
	it("requires an operation inventory", () => {
		expect(OperationRunnerPropsSchema.safeParse({}).success).toBe(false);
		expect(OperationRunnerPropsSchema.safeParse({ operations: [] }).success).toBe(true);
	});
});
