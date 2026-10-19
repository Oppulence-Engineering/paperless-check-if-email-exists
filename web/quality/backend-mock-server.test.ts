/* eslint-disable oppulence-web/no-direct-api-fetch -- This integration test exercises the mock HTTP server boundary directly. */
import { afterEach, beforeEach, describe, expect, it } from "vitest";
import { z } from "zod";

import {
	startBackendMockServer,
	type RunningBackendMockServer,
} from "../scripts/integration/backend-mock-server";

const PageSchema = z.strictObject({
	items: z.array(z.strictObject({ id: z.string(), name: z.string() })),
	page: z.strictObject({ nextCursor: z.string().nullable(), hasMore: z.boolean() }),
});
const MutationSchema = z.strictObject({
	id: z.string(),
	accepted: z.boolean(),
	input: z.unknown(),
});
const requestSignal = () => AbortSignal.timeout(2_000);

describe("backend mock server", () => {
	let mock: RunningBackendMockServer;

	beforeEach(async () => {
		mock = await startBackendMockServer({ port: 0 });
	});

	afterEach(async () => {
		await mock.close();
	});

	it("loads manifest health paths and service readiness metadata", async () => {
		const liveness = await fetch(`${mock.baseUrl}${mock.capabilities.health.livenessPath}`, {
			signal: requestSignal(),
		});
		const readiness = await fetch(`${mock.baseUrl}${mock.capabilities.health.readinessPath}`, {
			signal: requestSignal(),
		});

		expect(liveness.status).toBe(200);
		await expect(liveness.json()).resolves.toEqual({ status: "ok" });
		expect(readiness.status).toBe(200);
		await expect(readiness.json()).resolves.toEqual({
			status: "ready",
			service: mock.capabilities.service.name,
			apiVersion: mock.capabilities.service.apiVersion,
		});
	});

	it("serves canonical errors and a canonical 501 fallback", async () => {
		const requestId = "test-request-501";
		const error = await fetch(`${mock.baseUrl}${mock.capabilities.http.basePath}/mock/errors/503`, {
			headers: { "x-request-id": requestId },
			signal: requestSignal(),
		});
		const unhandled = await fetch(`${mock.baseUrl}${mock.capabilities.http.basePath}/unknown`, {
			headers: { "x-request-id": requestId },
			signal: requestSignal(),
		});

		expect(error.status).toBe(503);
		await expect(error.json()).resolves.toMatchObject({
			code: "mock_http_503",
			requestId,
			retryable: true,
		});
		expect(unhandled.status).toBe(501);
		await expect(unhandled.json()).resolves.toEqual({
			error: "Mock backend route is not implemented",
			code: "not_implemented",
			requestId,
			retryable: false,
			details: { method: "GET", path: `${mock.capabilities.http.basePath}/unknown` },
		});
	});

	it("paginates with opaque cursors and manifest limits", async () => {
		const first = await fetch(
			`${mock.baseUrl}${mock.capabilities.http.basePath}/mock/items?limit=2`,
			{
				signal: requestSignal(),
			},
		);
		const firstBody = PageSchema.parse(await first.json());
		const second = await fetch(
			`${mock.baseUrl}${mock.capabilities.http.basePath}/mock/items?limit=2&cursor=${encodeURIComponent(firstBody.page.nextCursor ?? "")}`,
			{ signal: requestSignal() },
		);

		expect(firstBody.page.nextCursor).toEqual(expect.any(String));
		expect(firstBody).toEqual({
			items: [
				{ id: "item-1", name: "Mock item 1" },
				{ id: "item-2", name: "Mock item 2" },
			],
			page: { nextCursor: firstBody.page.nextCursor, hasMore: true },
		});
		expect(PageSchema.parse(await second.json())).toMatchObject({
			items: [
				{ id: "item-3", name: "Mock item 3" },
				{ id: "item-4", name: "Mock item 4" },
			],
			page: { hasMore: true },
		});
	});

	it("replays idempotent mutations and rejects changed payloads", async () => {
		const endpoint = `${mock.baseUrl}${mock.capabilities.http.basePath}/mock/mutations`;
		const headers = { "content-type": "application/json", "x-idempotency-key": "test-key-123" };
		const first = await fetch(endpoint, {
			method: "POST",
			headers,
			body: JSON.stringify({ value: 1 }),
			signal: requestSignal(),
		});
		const replay = await fetch(endpoint, {
			method: "POST",
			headers,
			body: JSON.stringify({ value: 1 }),
			signal: requestSignal(),
		});
		const conflict = await fetch(endpoint, {
			method: "POST",
			headers,
			body: JSON.stringify({ value: 2 }),
			signal: requestSignal(),
		});

		expect(first.status).toBe(201);
		expect(first.headers.get("x-idempotency-replayed")).toBe("false");
		const firstBody = MutationSchema.parse(await first.json());
		expect(replay.status).toBe(201);
		expect(replay.headers.get("x-idempotency-replayed")).toBe("true");
		expect(MutationSchema.parse(await replay.json())).toEqual(firstBody);
		expect(conflict.status).toBe(409);
		await expect(conflict.json()).resolves.toMatchObject({ code: "idempotency_conflict" });

		const populatedState = await fetch(`${mock.baseUrl}/__test/state`, {
			signal: requestSignal(),
		});
		await expect(populatedState.json()).resolves.toEqual({
			mutationCount: 1,
			mutationSequence: 1,
		});

		const reset = await fetch(`${mock.baseUrl}/__test/reset`, {
			method: "POST",
			signal: requestSignal(),
		});
		expect(reset.status).toBe(200);
		const emptyState = await fetch(`${mock.baseUrl}/__test/state`, {
			signal: requestSignal(),
		});
		await expect(emptyState.json()).resolves.toEqual({
			mutationCount: 0,
			mutationSequence: 0,
		});
	});
});
