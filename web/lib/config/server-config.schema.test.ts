import { describe, expect, it } from "vitest";

import { ConfigProblemSchema, ServerConfigSchema } from "./server-config.schema";

const config = {
	isProduction: false,
	siteUrl: "",
	platformAdminEmails: [],
	brand: { name: "Acme" },
	realtime: {
		allowedOrigins: [],
		backendConnectionPath: "/v1/realtime/connection",
		handshakeTimeoutMs: 10_000,
	},
};

describe("ServerConfigSchema", () => {
	it("parses the generated domain props", () => {
		expect(ServerConfigSchema.safeParse(config).success).toBe(true);
		expect(ConfigProblemSchema.safeParse({ variable: "X", message: "is required" }).success).toBe(
			true,
		);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(ServerConfigSchema.safeParse({}).success).toBe(false);
		expect(
			ServerConfigSchema.safeParse({ ...config, brand: { name: "", logoUrl: "nope" } }).success,
		).toBe(false);
		expect(
			ServerConfigSchema.safeParse({
				...config,
				realtime: { ...config.realtime, backendConnectionPath: "v1" },
			}).success,
		).toBe(false);
	});
});
