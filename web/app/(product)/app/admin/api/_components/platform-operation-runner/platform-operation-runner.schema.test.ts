import { describe, expect, it } from "vitest";

import { PlatformOperationRunnerPropsSchema } from "./platform-operation-runner.schema";

describe("PlatformOperationRunnerPropsSchema", () => {
	it("requires an explicit operation inventory", () => {
		expect(PlatformOperationRunnerPropsSchema.safeParse({}).success).toBe(false);
		expect(PlatformOperationRunnerPropsSchema.safeParse({ operations: [] }).success).toBe(true);
	});
});
