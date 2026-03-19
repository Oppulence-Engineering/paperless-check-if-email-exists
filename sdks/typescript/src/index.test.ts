import {
	Configuration,
	EmailCategory,
	V1ApiAxiosParamCreator,
} from "./index";

describe("TypeScript SDK", () => {
	test("exports configuration, models, and request builders", async () => {
		const configuration = new Configuration({
			apiKey: "Bearer rch_live_example",
			basePath: "https://api.example.com",
		});

		expect(configuration.isJsonMime("application/json")).toBe(true);
		expect(EmailCategory.Invalid).toBe("invalid");

		const requestArgs = await V1ApiAxiosParamCreator(configuration).v1CheckEmail({
			to_email: "user@example.com",
		});

		expect(requestArgs.url).toBe("/v1/check_email");
		expect(requestArgs.options.method).toBe("POST");
		const headers = requestArgs.options.headers as Record<string, string>;
		expect(headers.Authorization).toBe(
			"Bearer rch_live_example"
		);
		expect(requestArgs.options.data).toBe(
			JSON.stringify({ to_email: "user@example.com" })
		);
	});
});
