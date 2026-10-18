// @vitest-environment jsdom

import "@testing-library/jest-dom/vitest";

import { render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

vi.mock("next/dynamic", () => ({
	default: () =>
		function Scalar({ configuration }: { configuration: { customCss?: string } }) {
			return <div data-testid="scalar">{configuration.customCss}</div>;
		},
}));

import { ApiReference, scalarConfiguration, withAbsoluteServers } from "./api-reference";

const document = { openapi: "3.0.3", info: { title: "Test", version: "1" }, paths: {} };

describe("ApiReference", () => {
	it("renders Scalar in a labelled section", () => {
		render(<ApiReference aria-label="API reference" document={document} surface="public" />);

		const component = screen.getByRole("region", { name: "API reference" });
		expect(component).toHaveAttribute("data-slot", "api-reference");
		expect(screen.getByTestId("scalar")).toHaveTextContent("#090909");
	});

	it("keeps Scalar on this host and off its own services", () => {
		const configuration = scalarConfiguration(document, "workspace") as Record<string, unknown>;
		expect(configuration).toMatchObject({
			content: document,
			withDefaultFonts: false,
			hideClientButton: true,
			showDeveloperTools: "never",
			agent: { disabled: true },
			mcp: { disabled: true },
			telemetry: false,
			persistAuth: false,
		});
		expect(configuration).not.toHaveProperty("proxyUrl");
		expect(configuration.customCss).toContain("#111111");
	});

	it("uses the operationId as the last part of each anchor", () => {
		const { generateOperationSlug } = scalarConfiguration(document, "public") as {
			generateOperationSlug: (operation: {
				operationId?: string;
				method: string;
				path: string;
			}) => string;
		};
		expect(
			generateOperationSlug({ operationId: "v1_check_email", method: "POST", path: "/x" }),
		).toBe("v1_check_email");
		expect(generateOperationSlug({ method: "GET", path: "/healthz" })).toBe("get/healthz");
	});

	it("makes relative server URLs absolute on this host", () => {
		const shaped = withAbsoluteServers(
			{ servers: [{ url: "/api/backend" }, { url: "/" }, { url: "https://api.example.test" }] },
			"https://app.example.test",
		);
		expect(shaped.servers).toEqual([
			{ url: "https://app.example.test/api/backend" },
			{ url: "https://app.example.test" },
			{ url: "https://api.example.test" },
		]);
	});
});
