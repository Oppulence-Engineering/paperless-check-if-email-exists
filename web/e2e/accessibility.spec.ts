import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("@a11y public landing page", async ({ page }) => {
	await page.goto("/");
	const result = await new AxeBuilder({ page })
		.exclude("#react-scan-root")
		.withTags(["wcag2a", "wcag2aa"])
		.analyze();
	expect(result.violations).toEqual([]);
});

test("Fumadocs article renders its source content", async ({ page }) => {
	await page.goto("/blog/how-email-verification-works");
	await expect(page.getByRole("heading", { name: "How an email check works" })).toBeVisible();
	await expect(
		page.getByText(/An email check looks at more than the address format/),
	).toBeVisible();
});

test("@a11y developer API reference", async ({ page }) => {
	const blocked: string[] = [];
	page.on("console", (message) => {
		// Dev builds load react-scan and react-grab from unpkg; the CSP blocks them by design.
		if (
			message.text().includes("Content Security Policy") &&
			!message.text().includes("unpkg.com")
		) {
			blocked.push(message.text());
		}
	});
	await page.goto("/developers/reference/v1_check_email");
	await expect(page).toHaveURL(
		/\/developers\/reference#tag\/verification-and-finder\/v1_check_email$/,
	);
	await page.locator(".scalar-app").first().waitFor({ timeout: 60_000 });
	await expect(page.getByRole("heading", { name: "POST /v1/check_email" })).toBeVisible();
	const result = await new AxeBuilder({ page })
		.exclude("#react-scan-root")
		.withTags(["wcag2a", "wcag2aa"])
		.analyze();
	expect(result.violations.map((violation) => violation.id)).toEqual([]);
	expect(blocked).toEqual([]);
});
