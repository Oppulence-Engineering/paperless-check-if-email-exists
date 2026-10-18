import { describe, expect, it } from "vitest";

import { ActivityFeedPropsSchema } from "./activity-feed.schema";

describe("ActivityFeedPropsSchema", () => {
	it("parses empty props, because the part keeps its own filters", () => {
		expect(ActivityFeedPropsSchema.safeParse({}).success).toBe(true);
	});
});
