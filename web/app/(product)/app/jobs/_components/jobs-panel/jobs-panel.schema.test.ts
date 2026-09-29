import { expect, it } from "vitest";
import { JobsPanelPropsSchema } from "./jobs-panel.schema";

it("accepts only positive job IDs from a deep link", () => {
	expect(JobsPanelPropsSchema.safeParse({ initialJobId: 9 }).success).toBe(true);
	expect(JobsPanelPropsSchema.safeParse({ initialJobId: -1 }).success).toBe(false);
});
