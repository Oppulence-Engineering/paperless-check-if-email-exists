// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("../platform-operation-runner/platform-operation-runner", () => ({
	PlatformOperationRunner: ({ operations }: { operations: { id: string }[] }) => (
		<div data-testid="admin-operations">
			{operations.map((operation) => operation.id).join(",")}
		</div>
	),
}));

import { ApiPanel } from "./api-panel";

describe("ApiPanel", () => {
	afterEach(() => vi.unstubAllEnvs());

	it("only exposes platform operations when configured", () => {
		vi.stubEnv("RCH__HEADER_SECRET", "secret");
		render(<ApiPanel aria-label="Example api-panel">Content</ApiPanel>);

		const component = screen.getByRole("region", { name: "Example api-panel" });
		expect(component).toHaveAttribute("data-slot", "api-panel");
		expect(screen.getByTestId("admin-operations")).toHaveTextContent("list_tenants");
		expect(screen.getByTestId("admin-operations")).not.toHaveTextContent("v1_check_email");
	});
});
