import { describe, expect, it } from "vitest";
import { z } from "zod";

import { createScopedStorage, removeBrowserStorageKeys } from "@/lib/storage/scoped-storage";

function memoryStorage(): Storage {
	const values = new Map<string, string>();
	return {
		get length() {
			return values.size;
		},
		clear: () => {
			values.clear();
		},
		getItem: (key) => values.get(key) ?? null,
		key: (index) => [...values.keys()][index] ?? null,
		removeItem: (key) => {
			values.delete(key);
		},
		setItem: (key, value) => values.set(key, value),
	};
}

describe("scoped browser storage", () => {
	it("removes legacy unscoped records", () => {
		const storage = memoryStorage();
		storage.setItem("legacy", "customer data");

		removeBrowserStorageKeys(["legacy"], storage);

		expect(storage.getItem("legacy")).toBeNull();
	});

	it("isolates accounts and rejects expired or invalid records", () => {
		const storage = memoryStorage();
		let now = 1_000;
		const make = (organizationId: string, userId: string) =>
			createScopedStorage({
				organizationId,
				userId,
				namespace: "preferences",
				version: 1,
				ttlMs: 100,
				schema: z.object({ density: z.enum(["compact", "comfortable"]) }),
				storage,
				now: () => now,
			});

		const alice = make("acme", "alice");
		alice.write({ density: "compact" });
		expect(alice.read()).toEqual({ density: "compact" });
		expect(make("acme", "bob").read()).toBeNull();
		expect(make("globex", "alice").read()).toBeNull();

		now = 1_101;
		expect(alice.read()).toBeNull();
	});

	it("fails closed when browser storage is unavailable", () => {
		const storage = {
			getItem: () => {
				throw new Error("disabled");
			},
			setItem: () => {
				throw new Error("quota exceeded");
			},
			removeItem: () => {
				throw new Error("disabled");
			},
		};
		const scoped = createScopedStorage({
			organizationId: "acme",
			userId: "alice",
			namespace: "preferences",
			version: 1,
			ttlMs: 100,
			schema: z.string(),
			storage,
		});

		expect(scoped.read()).toBeNull();
		expect(() => scoped.write("compact")).not.toThrow();
		expect(() => scoped.remove()).not.toThrow();
	});
});
