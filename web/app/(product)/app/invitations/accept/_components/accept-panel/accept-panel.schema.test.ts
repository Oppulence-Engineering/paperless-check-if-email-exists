import { describe, expect, it } from "vitest";

import { AcceptPanelPropsSchema } from "./accept-panel.schema";

describe("AcceptPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = AcceptPanelPropsSchema.safeParse({
			invitationId: "invite-1",
			returnTo: "/app/settings?settings=identity",
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(AcceptPanelPropsSchema.safeParse({}).success).toBe(false);
	});
});
