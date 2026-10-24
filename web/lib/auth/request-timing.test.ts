import { expect, it, vi } from "vitest";
vi.mock("server-only", () => ({}));
import { measurePhase, requestTimings } from "./request-timing";
it("keeps concurrent request timings isolated and records failures", async () => {
	const a = {},
		b = {};
	await Promise.all([
		requestTimings.run(a, () => measurePhase("auth", async () => 1)),
		requestTimings.run(b, () =>
			measurePhase("token", async () => {
				throw new Error("test");
			}).catch(() => undefined),
		),
	]);
	expect(a).toHaveProperty("auth");
	expect(a).not.toHaveProperty("token");
	expect(b).toHaveProperty("token");
	expect(requestTimings.getStore()).toBeUndefined();
});
