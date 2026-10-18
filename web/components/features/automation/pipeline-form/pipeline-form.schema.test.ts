import { describe, expect, it } from "vitest";

import { PipelineFormPropsSchema, WebhookHeadersSchema } from "./pipeline-form.schema";

const pipeline = {
	id: 1,
	tenant_id: "9a8b3676-6178-5756-9e7d-1172c6bd5408",
	name: "Weekly",
	status: "paused",
	source: { type: "list_snapshot", list_id: 4 },
	schedule: { cron: "0 9 * * 1", timezone: "UTC" },
	verification: { delta_mode: false },
	policy: { missed_run_window_hours: 24 },
	delivery: { dashboard: true, max_attempts: 5, retry_backoff_seconds: 300 },
	created_at: "2026-10-18T05:28:45.213779Z",
	updated_at: "2026-10-18T05:28:45.213779Z",
};

describe("PipelineFormPropsSchema", () => {
	it("parses the create form and the edit form", () => {
		expect(PipelineFormPropsSchema.safeParse({}).success).toBe(true);
		expect(PipelineFormPropsSchema.safeParse({ pipeline }).success).toBe(true);
	});

	it("rejects a pipeline without its schedule", () => {
		expect(
			PipelineFormPropsSchema.safeParse({ pipeline: { ...pipeline, schedule: undefined } }).success,
		).toBe(false);
	});
});

describe("WebhookHeadersSchema", () => {
	it("reads one header on each line and skips empty lines", () => {
		expect(WebhookHeadersSchema.parse("X-Team: growth\n\nAuthorization: Bearer a:b")).toEqual({
			"X-Team": "growth",
			Authorization: "Bearer a:b",
		});
		expect(WebhookHeadersSchema.parse("")).toEqual({});
	});

	it("rejects a line without a name", () => {
		expect(WebhookHeadersSchema.safeParse("no colon here").success).toBe(false);
		expect(WebhookHeadersSchema.safeParse(": value").success).toBe(false);
	});
});
