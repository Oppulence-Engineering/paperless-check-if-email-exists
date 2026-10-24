import { describe, expect, it } from "vitest";
import { errorMessage } from "./error-message";
describe("API error messages", () => {
	it("preserves actionable validation errors", () => {
		expect(
			errorMessage({
				response: { status: 400, data: { error: "Unable to auto-detect an email column" } },
			}),
		).toBe("Unable to auto-detect an email column");
	});
	it("handles HTML outages without exposing internals", () => {
		expect(errorMessage({ response: { status: 503, data: "<html>nginx</html>" } })).toBe(
			"Service unavailable. Please retry.",
		);
	});
});
