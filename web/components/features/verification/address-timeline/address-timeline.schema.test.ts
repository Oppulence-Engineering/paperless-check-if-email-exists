import { describe, expect, it } from "vitest";

import { AddressTimelinePropsSchema } from "./address-timeline.schema";

describe("AddressTimelinePropsSchema", () => {
	it("parses an address", () => {
		expect(AddressTimelinePropsSchema.safeParse({ email: "maya@brightlane.co" }).success).toBe(
			true,
		);
	});

	it("rejects an empty address", () => {
		expect(AddressTimelinePropsSchema.safeParse({ email: "" }).success).toBe(false);
	});
});
