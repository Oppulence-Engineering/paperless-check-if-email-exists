import { describe, expect, it } from "vitest";

import { authCallbackPath, safeReturnTo } from "./return-to";

describe("auth return paths", () => {
	it("keeps only same-origin relative paths", () => {
		expect(safeReturnTo("/app/settings?settings=usage&plan=starter")).toBe(
			"/app/settings?settings=usage&plan=starter",
		);
		expect(safeReturnTo("//evil.example/app")).toBe("/app");
		expect(safeReturnTo("https://evil.example")).toBe("/app");
		expect(safeReturnTo(null)).toBe("/app");
	});

	it("sends an invitee to the acceptance page, and keeps the return path", () => {
		expect(authCallbackPath({ invitation: "inv_1", return_to: "/app/settings" })).toBe(
			"/app/invitations/accept?invitation=inv_1&return_to=%2Fapp%2Fsettings",
		);
		expect(authCallbackPath({ return_to: "/app/settings?plan=starter" })).toBe(
			"/app/settings?plan=starter",
		);
	});
});
