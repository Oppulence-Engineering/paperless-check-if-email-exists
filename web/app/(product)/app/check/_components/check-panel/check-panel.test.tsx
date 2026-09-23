// @vitest-environment jsdom
import "@testing-library/jest-dom/vitest";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { fireEvent, render, screen, waitFor } from "@testing-library/react";
import { V1Api } from "@oppulence/reacher-sdk";
import { afterEach, expect, it, vi } from "vitest";
import { CheckPanel } from "./check-panel";

afterEach(() => vi.restoreAllMocks());

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
