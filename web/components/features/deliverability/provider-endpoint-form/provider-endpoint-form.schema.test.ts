import { describe, expect, it } from "vitest";

import { ProviderEndpointFormPropsSchema } from "./provider-endpoint-form.schema";

const endpoint = {
	endpoint_id: "2cb867f0-70d2-4538-af2d-afeb45151c63",
	provider: "postmark",
	label: "Postmark primary",
	status: "paused",
	allowed_ips: ["192.0.2.10"],
	provider_configured: false,
	webhook_path:
		"/v1/inbound/providers/postmark/2cb867f0-70d2-4538-af2d-afeb45151c63/{delivery_token}",
	created_at: "2026-10-18T04:53:29.186130Z",
	updated_at: "2026-10-18T04:53:29.186130Z",
};

describe("ProviderEndpointFormPropsSchema", () => {
	it("parses the create form and the edit form", () => {
		expect(ProviderEndpointFormPropsSchema.safeParse({}).success).toBe(true);
		expect(ProviderEndpointFormPropsSchema.safeParse({ endpoint }).success).toBe(true);
	});

	it("rejects an endpoint without its IP allow list", () => {
		expect(
			ProviderEndpointFormPropsSchema.safeParse({
				endpoint: { ...endpoint, allowed_ips: undefined },
			}).success,
		).toBe(false);
	});
});
