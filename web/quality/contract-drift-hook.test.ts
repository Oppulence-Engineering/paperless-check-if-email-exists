import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

describe("generated API contract drift protection", () => {
	it("uses the vendored backend contract unless a deployment selects another", () => {
		const config = readFileSync(
			new URL("../config/contracts/orval.config.ts", import.meta.url),
			"utf8",
		);

		expect(config).toContain("process.env.BACKEND_OPENAPI_PATH");
		expect(config).toContain('"config/contracts/backend.openapi.json"');
	});

	it("keeps generated contract drift in the merge gate", () => {
		const packageJson = JSON.parse(
			readFileSync(new URL("../package.json", import.meta.url), "utf8"),
		) as { scripts: Record<string, string> };
		const verify = readFileSync(new URL("../scripts/run-verify.mjs", import.meta.url), "utf8");

		expect(packageJson.scripts["contracts:check"]).toBe(
			"node scripts/check-generated-contracts.mjs",
		);
		expect(verify).toContain('"contracts:check"');
	});
});
