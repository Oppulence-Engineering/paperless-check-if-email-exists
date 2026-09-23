import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { parse } from "yaml";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
const read = (name: string) => readFileSync(resolve(root, name), "utf8");

describe("full stack distribution", () => {
	it("builds the Rust API and Next.js into one image", () => {
		const dockerfile = read("Dockerfile");
		expect(dockerfile).toContain("AS backend-build");
		expect(dockerfile).toContain("AS web-build");
		expect(dockerfile).toContain("/srv/reacher_backend");
		expect(dockerfile).toContain("/src/web/.next/standalone");
		expect(dockerfile).toContain("config/auth/migrations");
		expect(dockerfile).toContain("EXPOSE 3000");
		expect(dockerfile).toContain("/readyz");
	});

	it("starts auth migration, the private backend, and the web runtime", () => {
		const entrypoint = read("scripts/full-stack-entrypoint.sh");
		expect(entrypoint).toContain("migrate-auth.mjs");
		expect(entrypoint).toContain("RCH__HEADER_SECRET");
		expect(entrypoint).toContain("/srv/reacher_backend");
		expect(entrypoint).toContain("server.js");
		expect(entrypoint).toContain("trap stop EXIT INT TERM");
	});

	it("extends the repository's Compose development suite", () => {
		const compose = parse(read("docker-compose.yml")) as {
			services: Record<string, { healthcheck?: unknown }>;
		};
		const makefile = read("Makefile");
		const dev = read("scripts/dev.sh");
		expect(compose.services.postgres.healthcheck).toBeDefined();
		expect(compose.services.rabbitmq.healthcheck).toBeDefined();
		expect(makefile).toContain("bash scripts/dev.sh");
		expect(dev).toContain("docker compose up -d --wait postgres rabbitmq");
		expect(dev).toContain("scripts/watch-backend.sh");
		expect(dev).toContain("next dev --port 3000");
	});
});
