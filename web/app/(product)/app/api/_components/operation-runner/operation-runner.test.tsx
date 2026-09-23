// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { OperationRunner, operationRequestPath } from "./operation-runner";

describe("OperationRunner", () => {
	afterEach(() => {
		cleanup();
		vi.unstubAllGlobals();
	});

	it("encodes path values and preserves query parameters", () => {
		expect(operationRequestPath("/v1/lists/{list_id}", { list_id: "a/b" }, "page=2")).toBe(
			"/api/backend/v1/lists/a%2Fb?page=2",
		);
		expect(() => operationRequestPath("/v1/lists/{list_id}", {}, "")).toThrow(
			"list_id is required",
		);
	});

	it("only presents the tenant operations passed to it", () => {
		render(
			<QueryClientProvider client={new QueryClient()}>
				<OperationRunner
					aria-label="API runner"
					operations={[
						{
							id: "v1_check_email",
							method: "POST",
							path: "/v1/check_email",
							family: "Verification and finder",
							scope: "verify",
							requestMedia: ["application/json"],
						},
					]}
				/>
			</QueryClientProvider>,
		);

		const component = screen.getByRole("region", { name: "API runner" });
		expect(component).toHaveAttribute("data-slot", "operation-runner");
		expect(component).toHaveTextContent("/v1/check_email");
		expect(screen.getByRole("button", { name: "Run POST" })).toBeEnabled();
	});

	it("sends a check through the same-origin session BFF", async () => {
		const fetch = vi.fn().mockResolvedValue(Response.json({ is_reachable: "safe" }));
		vi.stubGlobal("fetch", fetch);
		render(
			<QueryClientProvider client={new QueryClient()}>
				<OperationRunner
					operations={[
						{
							id: "v1_check_email",
							method: "POST",
							path: "/v1/check_email",
							family: "Verification and finder",
							scope: "verify",
							requestMedia: ["application/json"],
						},
					]}
				/>
			</QueryClientProvider>,
		);
		fireEvent.click(screen.getByRole("button", { name: "Run POST" }));
		await waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
		expect(fetch.mock.calls[0][0]).toBe("/api/backend/v1/check_email");
		expect(fetch.mock.calls[0][1]).toMatchObject({
			method: "POST",
			credentials: "include",
			body: '{"to_email":"person@example.com"}',
		});
		await waitFor(() => expect(screen.getByText(/is_reachable/)).toBeInTheDocument());
	});
});
