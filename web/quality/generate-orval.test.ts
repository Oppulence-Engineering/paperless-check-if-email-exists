import { describe, expect, it } from "vitest";

import {
	catalogFromOpenAPI,
	isJsonMediaType,
	isReadMethod,
	toBffPath,
} from "@/scripts/generate/orval-operation";
import { parseGenerateArguments } from "@/scripts/generate";

describe("Orval operation catalog", () => {
	const catalog = catalogFromOpenAPI(
		{
			paths: {
				"/v1/widgets/{widgetId}": {
					get: {
						operationId: "getWidget",
						tags: ["Widgets"],
						responses: { "200": { content: { "application/json": {} } } },
					},
				},
				"/v1/widgets": {
					get: {
						operationId: "listWidgets",
						tags: ["Widgets"],
						parameters: [
							{ name: "limit", in: "query", schema: { type: "integer" } },
							{ name: "status", in: "query", schema: { type: "string" } },
						],
						responses: { "200": {} },
					},
					post: {
						operationId: "createWidget",
						tags: ["Widgets"],
						requestBody: { content: { "application/json": {} } },
						responses: { "201": { content: { "application/vnd.widgets+json": {} } } },
					},
				},
			},
		},
		new Map([
			["GetWidget200Response", "@/lib/api/generated/zod/widgets/widgets"],
			["ListWidgets200Response", "@/lib/api/generated/zod/widgets/widgets"],
			["ListWidgetsQueryParams", "@/lib/api/generated/zod/widgets/widgets"],
			["CreateWidget201Response", "@/lib/api/generated/zod/widgets/widgets"],
			["CreateWidgetBody", "@/lib/api/generated/zod/widgets/widgets"],
		]),
	);

	it("maps OpenAPI paths onto BFF paths and Orval schemas", () => {
		expect(toBffPath("/v1/widgets/{widgetId}")).toBe("/v1/widgets/:widgetId");
		expect(isReadMethod("GET")).toBe(true);
		expect(isReadMethod("POST")).toBe(false);
		expect(isJsonMediaType("application/problem+json; charset=utf-8")).toBe(true);
		expect(isJsonMediaType("text/event-stream")).toBe(false);

		const read = catalog.get("getWidget");
		expect(read).toMatchObject({
			name: "get-widget",
			method: "GET",
			fetchPath: "/v1/widgets/:widgetId",
			orvalSchema: "GetWidget200Response",
			orvalImport: "@/lib/api/generated/zod/widgets/widgets",
			responseMediaTypes: ["application/json"],
		});

		const list = catalog.get("listWidgets");
		expect(list).toMatchObject({
			orvalQuery: "ListWidgetsQueryParams",
			queryParams: [
				{ name: "limit", type: "number" },
				{ name: "status", type: "string" },
			],
		});

		const write = catalog.get("CreateWidget");
		expect(write).toMatchObject({
			name: "create-widget",
			method: "POST",
			orvalSchema: "CreateWidget201Response",
			orvalBody: "CreateWidgetBody",
			requestMediaTypes: ["application/json"],
			responseMediaTypes: ["application/vnd.widgets+json"],
		});
	});

	it("accepts --operation without a kebab name", () => {
		const request = parseGenerateArguments(["hook", "--operation", "listConnectors"]);
		expect(request).toMatchObject({
			kind: "hook",
			operation: "listConnectors",
		});
	});

	it("accepts upload operations and requires explicit realtime schemas", () => {
		expect(parseGenerateArguments(["upload", "--operation", "createWidget"])).toMatchObject({
			kind: "upload",
			operation: "createWidget",
		});
		expect(() => parseGenerateArguments(["realtime", "--name", "widget-event"])).toThrow();
		expect(
			parseGenerateArguments([
				"realtime",
				"--name",
				"widget-event",
				"--orval-schema",
				"WidgetEvent",
				"--orval-import",
				"@/lib/api/generated/zod/widgets/widgets",
			]),
		).toMatchObject({ kind: "realtime", orvalSchema: "WidgetEvent" });
	});
});
