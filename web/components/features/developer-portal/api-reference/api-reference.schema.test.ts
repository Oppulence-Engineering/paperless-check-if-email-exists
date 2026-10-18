import { describe, expect, it } from "vitest";

import { ApiReferencePropsSchema } from "./api-reference.schema";

describe("ApiReferencePropsSchema", () => {
	it("parses a document and a surface", () => {
		const parsed = ApiReferencePropsSchema.safeParse({
			document: { openapi: "3.0.3", paths: {} },
			surface: "public",
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects a missing document or an unknown surface", () => {
		expect(ApiReferencePropsSchema.safeParse({ surface: "public" }).success).toBe(false);
		expect(ApiReferencePropsSchema.safeParse({ document: {}, surface: "admin" }).success).toBe(
			false,
		);
	});
});
