import { afterEach, describe, expect, it, vi } from "vitest";

import { buildLogRecord, describeError, log, requestLogger } from "./logger";
import { LogRecordSchema } from "./logger.schema";

afterEach(() => {
	vi.restoreAllMocks();
	delete process.env.LOG_LEVEL;
});

describe("logger", () => {
	it("builds a record the schema accepts", () => {
		const record = buildLogRecord("info", "server started", { env: "test" }, "req-1");
		expect(LogRecordSchema.safeParse(record).success).toBe(true);
		expect(record).toMatchObject({ level: "info", message: "server started", requestId: "req-1" });
	});

	it("writes one line of JSON so a log platform can index it", () => {
		const sink = vi.spyOn(console, "log").mockImplementation(() => undefined);
		log.info("scan finished", { count: 3 });

		expect(sink).toHaveBeenCalledOnce();
		const parsed: unknown = JSON.parse(sink.mock.calls[0][0] as string);
		expect(parsed).toMatchObject({ level: "info", message: "scan finished", fields: { count: 3 } });
	});

	it("carries one request id across every line of a request", () => {
		const sink = vi.spyOn(console, "warn").mockImplementation(() => undefined);
		requestLogger("req-42").warn("slow upstream", { ms: 900 });

		const parsed = JSON.parse(sink.mock.calls[0][0] as string) as { requestId: string };
		expect(parsed.requestId).toBe("req-42");
	});

	it("drops a level the deployment did not ask for", () => {
		process.env.LOG_LEVEL = "warn";
		const sink = vi.spyOn(console, "log").mockImplementation(() => undefined);
		log.info("noise");
		expect(sink).not.toHaveBeenCalled();
	});

	it("sends errors to stderr and keeps the payload out of the line", () => {
		const sink = vi.spyOn(console, "error").mockImplementation(() => undefined);
		log.error("upstream failed", describeError(new TypeError("boom")));

		const parsed = JSON.parse(sink.mock.calls[0][0] as string) as {
			fields: Record<string, string>;
		};
		expect(parsed.fields).toEqual({ name: "TypeError", message: "boom" });
	});

	it("describes a thrown value that is not an Error", () => {
		expect(describeError("nope")).toEqual({ name: "unknown", message: "nope" });
	});
});
