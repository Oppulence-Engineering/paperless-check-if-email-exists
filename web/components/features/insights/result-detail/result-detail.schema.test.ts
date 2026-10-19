import { describe, expect, it } from "vitest";

import { ResultDetailPropsSchema } from "./result-detail.schema";

describe("ResultDetailPropsSchema", () => {
	it("parses a query row and drops unknown backend fields", () => {
		const parsed = ResultDetailPropsSchema.safeParse({
			row: {
				id: 7,
				job_id: null,
				email: "a@example.com",
				task_state: "completed",
				created_at: "2026-10-18T05:28:53Z",
				new_field: 1,
			},
		});
		expect(parsed.success).toBe(true);
		expect(parsed.data?.row).not.toHaveProperty("new_field");
	});

	it("rejects a row without an id", () => {
		expect(
			ResultDetailPropsSchema.safeParse({ row: { task_state: "completed", created_at: "x" } })
				.success,
		).toBe(false);
	});
});
