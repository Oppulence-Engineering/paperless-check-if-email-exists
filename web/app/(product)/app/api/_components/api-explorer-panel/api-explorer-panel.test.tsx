// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

vi.mock("@/components/features/developer-portal/api-reference/api-reference", () => ({
	ApiReference: ({ document, surface }: { document: { paths: object }; surface: string }) => (
		<div data-surface={surface} data-testid="api-reference">
			{Object.keys(document.paths).join(",")}
		</div>
	),
}));

import { ApiExplorerPanel } from "./api-explorer-panel";

describe("ApiExplorerPanel", () => {
	it("shows the workspace operations in the API reference", () => {
		render(<ApiExplorerPanel aria-label="Example api-explorer-panel" />);

		const component = screen.getByRole("region", { name: "Example api-explorer-panel" });
		expect(component).toHaveAttribute("data-slot", "api-explorer-panel");
		expect(screen.getByRole("heading", { name: "API explorer", level: 1 })).toBeInTheDocument();
		const reference = screen.getByTestId("api-reference");
		expect(reference).toHaveAttribute("data-surface", "workspace");
		expect(reference).toHaveTextContent("/v1/check_email");
		expect(reference).not.toHaveTextContent("/v1/admin");
	});
});
