import { defineConfig, devices } from "@playwright/test";

const baseURL = process.env.E2E_BASE_URL || "http://localhost:3000";

export default defineConfig({
	testDir: "./e2e",
	fullyParallel: false,
	workers: 1,
	forbidOnly: Boolean(process.env.CI),
	retries: process.env.CI ? 1 : 0,
	reporter: process.env.CI ? "github" : "list",
	use: { ...devices["Desktop Chrome"], baseURL, trace: "on-first-retry" },
	webServer: process.env.E2E_BASE_URL
		? undefined
		: {
				command: "bash ../scripts/dev.sh",
				url: `${baseURL}/readyz`,
				reuseExistingServer: !process.env.CI,
				timeout: 240_000,
			},
});
