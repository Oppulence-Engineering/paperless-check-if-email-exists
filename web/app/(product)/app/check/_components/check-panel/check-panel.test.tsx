// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { V1CheckEmail200Response } from "@/lib/api/generated/zod/v1/v1";
import { CheckPanel, ResultValue } from "./check-panel";

afterEach(() => {
	cleanup();
	vi.restoreAllMocks();
});

it("accepts the raw submitted address in an API result", () => {
	expect(V1CheckEmail200Response.shape.input.safeParse("yoanyomba@solomon-ai.c").success).toBe(
		true,
	);
});

it("shows every returned result field, including nested and empty values", () => {
	render(
		<ResultValue
			value={{
				syntax: { is_valid_syntax: false, error: null },
				reasons: ["invalid_syntax"],
				mx: {},
			}}
		/>,
	);
	expect(screen.getByText("syntax")).toBeVisible();
	expect(screen.getByText("is valid syntax")).toBeVisible();
	expect(screen.getByText("No")).toBeVisible();
	expect(screen.getByText("Not available")).toBeVisible();
	expect(screen.getByText("invalid_syntax")).toBeVisible();
	expect(screen.getByText("None")).toBeVisible();
});

it("shows an inline error for an incomplete email address", () => {
	const check = vi.spyOn(V1Api.prototype, "v1CheckEmail");
	const client = new QueryClient();
	render(
		<QueryClientProvider client={client}>
			<CheckPanel />
		</QueryClientProvider>,
	);
	const input = screen.getByLabelText("Email address");
	fireEvent.change(input, { target: { value: "yoanyomba@solomon-ai.c" } });
	fireEvent.click(screen.getByRole("button", { name: "Check email" }));
	expect(screen.getByRole("alert")).toHaveTextContent("Enter a complete email address");
	expect(input).toHaveAttribute("aria-invalid", "true");
	expect(check).not.toHaveBeenCalled();
	fireEvent.change(input, { target: { value: "not-an-email" } });
	fireEvent.click(screen.getByRole("button", { name: "Check email" }));
	expect(screen.getByRole("alert")).toHaveTextContent("Enter a complete email address");
	expect(check).not.toHaveBeenCalled();
});

it("shows loading and backend errors for an SDK check", async () => {
	let failCheck!: (error: Error) => void;
	const response = new Promise<never>((_resolve, reject) => {
		failCheck = reject;
	});
	const check = vi.spyOn(V1Api.prototype, "v1CheckEmail").mockReturnValue(response);
	const client = new QueryClient({ defaultOptions: { mutations: { retry: false } } });
	render(
		<QueryClientProvider client={client}>
			<CheckPanel />
		</QueryClientProvider>,
	);
	fireEvent.change(screen.getByLabelText("Email address"), {
		target: { value: "user@example.com" },
	});
	fireEvent.click(screen.getByRole("button", { name: "Check email" }));
	await waitFor(() =>
		expect(check).toHaveBeenCalledWith(
			{ checkEmailRequest: { to_email: "user@example.com", sandbox: false } },
			expect.any(Object),
		),
	);
	expect(screen.getByRole("button", { name: "Checking…" })).toBeDisabled();
	failCheck(new Error("Backend unavailable"));
	expect(await screen.findByRole("alert")).toHaveTextContent("Backend unavailable");
});
