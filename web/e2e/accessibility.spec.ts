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
