// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, fireEvent, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { BillingPlan } from "./billing-plan";

afterEach(cleanup);

describe("BillingPlan", () => {
	it("offers both paid plans to a Free workspace owner", () => {
		const onUpgrade = vi.fn();
		render(<BillingPlan billingEnabled canManage onUpgrade={onUpgrade} plan="free" />);

		expect(screen.getByText("Free")).toBeInTheDocument();
		expect(screen.getByText("100 checks a month · 3 seats")).toBeInTheDocument();
		const buttons = screen.getAllByRole("button", { name: "Upgrade" });
		expect(buttons).toHaveLength(2);
		fireEvent.click(buttons[1]);
		expect(onUpgrade).toHaveBeenCalledWith("professional");
	});

	it("lets a paid workspace open the billing portal", () => {
		const onManage = vi.fn();
		render(<BillingPlan billingEnabled canManage onManage={onManage} plan="starter" />);

		expect(screen.getAllByRole("button", { name: "Upgrade" })).toHaveLength(1);
		fireEvent.click(screen.getByRole("button", { name: "Manage billing" }));
		expect(onManage).toHaveBeenCalled();
	});

	it("explains why a member or an unbilled deployment cannot buy", () => {
		const { rerender } = render(<BillingPlan billingEnabled canManage={false} plan="free" />);
		expect(screen.getByText("Only owners and admins can change the plan.")).toBeInTheDocument();
		expect(screen.queryByRole("button")).not.toBeInTheDocument();

		rerender(<BillingPlan billingEnabled={false} canManage plan="free" />);
		expect(screen.getByText(/Online billing is not set up here/)).toBeInTheDocument();
	});
});
