// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { cleanup, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

import { PlatformOperationRunner } from "./platform-operation-runner";

describe("PlatformOperationRunner", () => {
	afterEach(() => {
		cleanup();
		vi.unstubAllGlobals();
	});

	it("sends an audited mutation request through the private BFF", async () => {
		const fetch = vi.fn().mockResolvedValue(Response.json({ id: "tenant-1" }));
		vi.stubGlobal("fetch", fetch);
		vi.stubGlobal("confirm", () => true);
		render(
			<QueryClientProvider client={new QueryClient()}>
				<PlatformOperationRunner
					operations={[
						{
							id: "create_tenant",
							method: "POST",
							path: "/v1/admin/tenants",
							requestBody: true,
						},
					]}
				/>
			</QueryClientProvider>,
		);
		fireEvent.change(screen.getByLabelText("Reason for change"), {
			target: { value: "Support request 123" },
		});
		fireEvent.click(screen.getByRole("button", { name: "Run POST" }));
		await waitFor(() => expect(fetch).toHaveBeenCalledTimes(1));
		expect(fetch.mock.calls[0][0]).toBe("/api/platform/backend/v1/admin/tenants");
		expect(fetch.mock.calls[0][1].headers["x-admin-reason"]).toBe("Support request 123");

		expect(screen.getByText(/tenant-1/)).toBeInTheDocument();
	});
});
