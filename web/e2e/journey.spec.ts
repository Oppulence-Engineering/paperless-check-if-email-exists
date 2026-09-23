import { expect, test } from "@playwright/test";
import { randomUUID } from "node:crypto";

import { V1ListLists200Response } from "@/lib/api/generated/zod/v1/v1";

// Uses the real Better Auth, SDK, BFF, Rust API, PostgreSQL, and RabbitMQ stack.
test("workspace verification journey", async ({ browser, page, request }) => {
	test.setTimeout(60_000);
	const email = `e2e-${randomUUID()}@example.com`;
	await page.goto("/");
	await expect(
		page.getByRole("heading", { name: "Check email addresses before you send." }),
	).toBeVisible();

	const anonymous = await page.request.get("/api/backend/v1/me");
	expect(anonymous.status()).toBe(401);
	await page.goto("/app/check");
	await expect(page).toHaveURL(/\/sign-in/);

	await page.getByLabel("Email", { exact: true }).fill(email);
	await page.getByRole("button", { name: "Email me a code" }).click();
	await page.getByLabel("Sign-in code").fill(process.env.AUTH_E2E_OTP || "123456");
	await page.getByRole("button", { name: "Verify code" }).click();
	await expect(page).toHaveURL(/\/app\/check/);
	await expect(page.getByRole("navigation", { name: "Main navigation" })).toBeVisible();

	await page.reload();
	await expect(page.getByRole("heading", { name: "Check an email address" })).toBeVisible();
	await page.getByLabel("Email address").fill("test@valid.example.com");
	await page.getByLabel("Use sample result").check();
	await page.getByRole("button", { name: "Check email" }).click();
	await expect(page.getByText("test@valid.example.com")).toBeVisible();
	await expect(page.getByText("Quality score")).toBeVisible();

	await page.goto("/app/lists");
	await expect(page.getByText("No lists yet.")).toBeVisible();
	await page.getByLabel("CSV file").setInputFiles({
		name: "test.csv",
		mimeType: "text/csv",
		buffer: Buffer.from("email\ntest@valid.example.com\n"),
	});
	await page.getByLabel("Name").fill("E2E list");
	await page.getByRole("button", { name: "Upload" }).click();
	await expect(page.getByText(/List accepted/)).toBeVisible();
	await expect(page.getByText("E2E list")).toBeVisible();
	const ownListsResponse = await page.request.get("/api/backend/v1/lists");
	expect(ownListsResponse.ok()).toBe(true);
	const ownLists = V1ListLists200Response.parse(await ownListsResponse.json());
	const listId = ownLists.lists[0]?.id;
	expect(listId).toBeDefined();

	const apiKeyResponse = await page.request.post("/api/backend/v1/me/api-keys", {
		headers: { origin: new URL(page.url()).origin },
		data: { name: "E2E verification", scopes: ["verify"] },
	});
	expect(apiKeyResponse.status()).toBe(201);
	const apiKey = (await apiKeyResponse.json()).key as string;
	expect(apiKey).toMatch(/^rch_live_/);
	const apiResponse = await request.post("/v1/check_email", {
		headers: { authorization: `Bearer ${apiKey}` },
		data: { to_email: "test@valid.example.com", sandbox: true },
	});
	expect(apiResponse.status()).toBe(200);
	expect(apiResponse.headers()["content-type"]).toContain("application/json");
	expect(await apiResponse.text()).toContain("test@valid.example.com");
	expect(
		(await request.get("/v1/me", { headers: { authorization: `Bearer ${apiKey}` } })).status(),
	).toBe(200);
	expect(
		(await request.get("/v1/lists", { headers: { authorization: `Bearer ${apiKey}` } })).status(),
	).toBe(403);
	expect(
		(
			await request.post("/v1/check_email", {
				headers: { "x-reacher-secret": "invalid" },
				data: { to_email: "test@valid.example.com", sandbox: true },
			})
		).status(),
	).toBe(401);

	const otherContext = await browser.newContext({ baseURL: new URL(page.url()).origin });
	try {
		const otherPage = await otherContext.newPage();
		await otherPage.goto("/sign-in");
		await otherPage.getByLabel("Email", { exact: true }).fill(`other-${randomUUID()}@example.com`);
		await otherPage.getByRole("button", { name: "Email me a code" }).click();
		await otherPage.getByLabel("Sign-in code").fill(process.env.AUTH_E2E_OTP || "123456");
		await otherPage.getByRole("button", { name: "Verify code" }).click();
		await expect(otherPage).toHaveURL(/\/app/);
		const otherLists = await otherPage.request.get("/api/backend/v1/lists");
		expect(otherLists.ok()).toBe(true);
		expect(V1ListLists200Response.parse(await otherLists.json()).lists).toEqual([]);
		const foreignList = await otherPage.request.get(`/api/backend/v1/lists/${listId}`);
		expect(foreignList.status()).toBe(404);
		const sessionCookie = (await otherContext.cookies()).find((cookie) =>
			cookie.name.endsWith("session_token"),
		);
		if (!sessionCookie) throw new Error("Better Auth session cookie is missing");
		await otherContext.addCookies([{ ...sessionCookie, expires: Date.now() / 1000 - 60 }]);
		await otherPage.goto("/app/check");
		await expect(otherPage).toHaveURL(/\/sign-in/);
	} finally {
		await otherContext.close();
	}

	await page.goto("/app/history");
	await page.getByLabel("Email address").fill("test@valid.example.com");
	await page.getByRole("button", { name: "Search" }).click();
	await expect(page.getByText("No earlier results for this address.")).toBeVisible();

	await page.getByRole("button", { name: /Open account and workspace menu/ }).click();
	await page.getByRole("menuitem", { name: "Sign out" }).click();
	await expect(page).toHaveURL(/\/sign-in/);
	await page.goto("/app/lists");
	await expect(page).toHaveURL(/\/sign-in/);
});
