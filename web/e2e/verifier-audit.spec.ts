import { expect, test } from "@playwright/test";
import { randomUUID } from "node:crypto";
import { writeFile } from "node:fs/promises";
import AxeBuilder from "@axe-core/playwright";
import { SAMPLE_REPUTATION_REPORT } from "@/components/features/deliverability/reputation-report/reputation-report.schema";

test("audit recovery, explicit checks, private navigation and mobile drawers @a11y", async ({
	page,
}) => {
	test.setTimeout(150_000);
	await page.goto("/sign-in");
	await page.getByLabel("Email", { exact: true }).fill(`audit-${randomUUID()}@example.com`);
	await page.getByRole("button", { name: "Email me a code" }).click();
	await page.getByLabel("Sign-in code").fill(process.env.AUTH_E2E_OTP || "123456");
	await page.getByRole("button", { name: "Verify code" }).click();
	await expect(page).toHaveURL(/\/app/);

	const requests: { path: string; milliseconds: number }[] = [];
	page.on("requestfinished", (request) => {
		const path = new URL(request.url()).pathname;
		if (path.startsWith("/api/backend/")) {
			const timing = request.timing();
			requests.push({ path, milliseconds: timing.responseEnd });
		}
	});
	await page.goto("/app");
	await expect(page.getByRole("heading", { name: "Overview", exact: true })).toBeVisible();
	await page.waitForLoadState("networkidle");
	await writeFile("../.context/overview-after.json", JSON.stringify(requests, null, 2));
	expect(requests.filter((r) => r.path.includes("provider-endpoints"))).toHaveLength(0);

	await page.goto("/app/check?email=Legacy%2Btag%40example.com");
	await expect(page.getByLabel("Email address")).toHaveValue("Legacy+tag@example.com");
	await expect(page).not.toHaveURL(/email=/);

	let healthChecks = 0;
	await page.route("**/api/backend/v1/reputation/check", async (route) => {
		healthChecks++;
		if (healthChecks === 1)
			await route.fulfill({
				status: 503,
				contentType: "text/html",
				body: "<html>Unavailable</html>",
			});
		else await route.fulfill({ status: 200, json: SAMPLE_REPUTATION_REPORT });
	});
	await page.goto("/app/domains");
	await page.getByLabel("Domain to check").fill("https://Gmail.com/");
	expect(healthChecks).toBe(0);
	await page.getByRole("button", { name: "Check domain", exact: true }).click();
	await expect(page.getByRole("main").getByRole("alert")).toContainText("Service unavailable");
	expect(healthChecks).toBe(1);
	await page.getByRole("button", { name: "Retry", exact: true }).click();
	await expect(page.getByText("82/100")).toBeVisible();
	expect(healthChecks).toBe(2);

	await page.goto("/app/jobs?job=2147483647");
	await expect(page.getByRole("main").getByRole("alert")).toContainText("Job not found");
	await expect(page.getByRole("main").getByRole("alert")).toHaveCount(1);
	await page.getByLabel("Job ID").fill("abc");
	await page.getByRole("button", { name: "Open", exact: true }).click();
	await expect(page.getByRole("main").getByRole("alert")).toContainText("Enter a valid job ID");
	await expect(page.getByRole("heading", { name: "Job 2147483647", exact: true })).toHaveCount(0);

	await page.setViewportSize({ width: 390, height: 844 });
	await page.goto("/app/suppressions");
	await page.getByRole("button", { name: "Add addresses", exact: true }).click();
	await page
		.getByLabel("Addresses", { exact: true })
		.fill("Test.User+qa@Example.com\ntest.user+qa@example.com\nnot-an-email");
	page.once("dialog", (dialog) => dialog.dismiss());
	await page.keyboard.press("Escape");
	await expect(page.getByRole("dialog")).toBeVisible();
	await page.getByRole("button", { name: "Add", exact: true }).click();
	await expect(page.getByText("Invalid email address", { exact: false })).toBeVisible();
	await page.keyboard.press("Escape");
	await expect(page.getByRole("dialog")).toHaveCount(0);
	const remove = page.getByRole("button", { name: "Remove", exact: true }).first();
	await expect(remove).toBeVisible();
	const box = await remove.boundingBox();
	expect(box).not.toBeNull();
	expect(box!.x + box!.width).toBeLessThanOrEqual(390);
	const results = await new AxeBuilder({ page })
		.exclude("#react-scan-root")
		.withTags(["wcag2a", "wcag2aa"])
		.analyze();
	expect(
		results.violations.filter((v) => v.impact === "critical" || v.impact === "serious"),
	).toEqual([]);
	await page.screenshot({ path: "../.context/suppressions-mobile.png", fullPage: true });
});
