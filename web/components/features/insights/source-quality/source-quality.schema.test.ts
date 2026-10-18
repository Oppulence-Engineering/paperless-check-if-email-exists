import { describe, expect, it } from "vitest";

import { SourceQualityPropsSchema } from "./source-quality.schema";

describe("SourceQualityPropsSchema", () => {
	it("parses empty props, because the part reads its own data", () => {
		expect(SourceQualityPropsSchema.safeParse({}).success).toBe(true);
	});
});
