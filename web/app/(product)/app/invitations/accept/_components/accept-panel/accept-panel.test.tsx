// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { render, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";

import { AcceptPanel } from "./accept-panel";

describe("AcceptPanel", () => {
	it("forwards accessible section props and renders its content", () => {
		render(
			<AcceptPanel
				aria-label="Invitation confirmation"
				invitationId="invite-1"
				returnTo="/app/settings?settings=identity"
			/>,
		);

		const component = screen.getByRole("region", { name: "Invitation confirmation" });
		expect(component).toHaveAttribute("data-slot", "accept-panel");
		expect(screen.getByRole("button", { name: "Accept invitation" })).toBeVisible();
		expect(screen.getByRole("link", { name: "Cancel" })).toHaveAttribute(
			"href",
			"/app/settings?settings=identity",
		);
		expect(component.querySelector('input[name="invitation"]')).toHaveValue("invite-1");
	});
});
