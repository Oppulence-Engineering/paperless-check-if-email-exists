import { describe, expect, it } from "vitest";

import { ExplorerFiltersSchema, ResultExplorerPropsSchema } from "./result-explorer.schema";

describe("ResultExplorerPropsSchema", () => {
	it("parses empty and full filters", () => {
		expect(ResultExplorerPropsSchema.safeParse({ filters: {} }).success).toBe(true);
		expect(
			ExplorerFiltersSchema.safeParse({
				category: "valid",
				safeToSend: true,
				minScore: 0,
				maxScore: 100,
				jobId: 18,
				since: "2026-10-01",
				until: "2026-10-18",
				recommendationAction: "send",
				policyMode: "growth",
				policyDecision: "review",
			}).success,
		).toBe(true);
	});

	it("rejects missing filters and values out of range", () => {
		expect(ResultExplorerPropsSchema.safeParse({}).success).toBe(false);
		expect(ExplorerFiltersSchema.safeParse({ minScore: 101 }).success).toBe(false);
		expect(ExplorerFiltersSchema.safeParse({ jobId: 0 }).success).toBe(false);
		expect(ExplorerFiltersSchema.safeParse({ since: "18/10/2026" }).success).toBe(false);
	});
});
