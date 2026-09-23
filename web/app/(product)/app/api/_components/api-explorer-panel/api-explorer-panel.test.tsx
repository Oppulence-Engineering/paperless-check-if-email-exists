// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

vi.mock("../operation-runner/operation-runner", () => ({
	OperationRunner: ({ operations }: { operations: { id: string }[] }) => (
		<div data-testid="tenant-operations">
			{operations.map((operation) => operation.id).join(",")}
		</div>
	),
}));

import { ApiExplorerPanel } from "./api-explorer-panel";

describe("ApiExplorerPanel", () => {
	it("forwards accessible section props and renders its content", () => {
		render(<ApiExplorerPanel aria-label="Example api-explorer-panel">Content</ApiExplorerPanel>);

		const component = screen.getByRole("region", { name: "Example api-explorer-panel" });
		expect(component).toHaveAttribute("data-slot", "api-explorer-panel");
		expect(component).toHaveTextContent("API explorer");
		expect(screen.getByTestId("tenant-operations")).toHaveTextContent("v1_check_email");
		expect(screen.getByTestId("tenant-operations")).not.toHaveTextContent("list_tenants");
	});
});
