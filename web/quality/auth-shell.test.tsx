// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";

import { AuthShell } from "@/components/auth/auth-shell";

afterEach(cleanup);

describe("AuthShell", () => {
	it("shows a useful sign-in error instead of an internal code", () => {
		render(<AuthShell error="sign_in_unavailable" mode="sign-in" returnTo="/app" />);

		expect(screen.getByText("Sign-in is temporarily unavailable. Please try again.")).toBeVisible();
		expect(screen.queryByText("sign_in_unavailable")).not.toBeInTheDocument();
	});

	it("uses Better Auth controls without a legacy broker URL", () => {
		render(<AuthShell mode="sign-in" returnTo="/app/settings" />);

		expect(screen.queryByRole("button", { name: "Continue with Google" })).not.toBeInTheDocument();
		expect(
			screen.queryByRole("button", { name: "Continue with Microsoft" }),
		).not.toBeInTheDocument();
		expect(screen.getByRole("button", { name: "Email me a code" })).toBeVisible();
		expect(document.querySelector('a[href*="workos"]')).not.toBeInTheDocument();
	});

	it("shows only configured social providers", () => {
		render(<AuthShell googleEnabled mode="sign-in" returnTo="/app/settings" />);

		expect(screen.getByRole("button", { name: "Continue with Google" })).toBeVisible();
		expect(
			screen.queryByRole("button", { name: "Continue with Microsoft" }),
		).not.toBeInTheDocument();
	});

	it("keeps the form as the only product copy", () => {
		render(<AuthShell mode="sign-up" returnTo="/app" />);

		expect(screen.getByRole("link", { name: "Sign in" })).toHaveAttribute(
			"href",
			"/sign-in?return_to=%2Fapp",
		);
		expect(
			screen.getByRole("heading", { name: "Know which addresses can receive mail" }),
		).toBeVisible();
		expect(screen.queryByText("Acme Corp")).not.toBeInTheDocument();
		expect(screen.queryByText("Security review by Friday")).not.toBeInTheDocument();
		expect(screen.queryByText("Design partner")).not.toBeInTheDocument();
	});
});
