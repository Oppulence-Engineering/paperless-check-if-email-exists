import { expect, it } from "vitest";
import { RequestTimingInputSchema } from "./request-timing.schema";
it("accepts only fixed timing names", () => {
	expect(RequestTimingInputSchema.safeParse({ phase: "auth" }).success).toBe(true);
	expect(RequestTimingInputSchema.safeParse({ phase: "person@example.com" }).success).toBe(false);
});
