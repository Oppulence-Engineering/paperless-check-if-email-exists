// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, expect, it, vi } from "vitest";

import { authClient } from "@/lib/auth/auth-client";
import { AuthShell } from "./auth-shell";

vi.mock("@/components/auth/auth-ledger-preview", () => ({
	AuthLedgerPreview: () => null,
}));
vi.mock("@/lib/auth/auth-client", () => ({
	authClient: {
		emailOtp: {
			sendVerificationOtp: vi.fn().mockResolvedValue({ error: null }),
		},
		signIn: {
			social: vi.fn(),
			emailOtp: vi.fn(),
			passkey: vi.fn(),
			sso: vi.fn(),
		},
	},
}));

afterEach(() => {
	cleanup();
	vi.clearAllMocks();
});

it("sends an email code and reveals the shadcn code field", async () => {
	render(<AuthShell mode="sign-in" returnTo="/app/check" />);

	const email = screen.getByRole("textbox", { name: "Email" });
	expect(email).toHaveAttribute("data-slot", "input");
	fireEvent.change(email, { target: { value: "test@example.com" } });
	fireEvent.click(screen.getByRole("button", { name: "Email me a code" }));

	await waitFor(() =>
		expect(authClient.emailOtp.sendVerificationOtp).toHaveBeenCalledWith({
			email: "test@example.com",
			type: "sign-in",
		}),
	);
	expect(await screen.findByRole("textbox", { name: "Sign-in code" })).toHaveAttribute(
		"data-slot",
		"input",
	);
});
