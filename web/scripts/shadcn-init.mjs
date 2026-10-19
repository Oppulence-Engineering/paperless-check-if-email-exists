#!/usr/bin/env node
/**
 * Validates the vendored shadcn wiring without overwriting config.
 * Shared primitives install into `vendor/oppulence/ui` via `pnpm ui:add`.
 */
import { spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = path.dirname(path.dirname(fileURLToPath(import.meta.url)));
const uiRoot = path.resolve(appRoot, "vendor/oppulence/ui");
const executable = path.join(
	appRoot,
	"node_modules/.bin",
	process.platform === "win32" ? "shadcn.cmd" : "shadcn",
);

function run(args, cwd) {
	const result = spawnSync(executable, args, { cwd, stdio: "inherit" });
	if ((result.status ?? 1) !== 0) {
		process.exit(result.status ?? 1);
	}
}

if (!fs.existsSync(executable)) {
	console.error("shadcn CLI is not installed. Run `pnpm install` in web/.");
	process.exit(2);
}

for (const [label, cwd] of [
	["check-if-email-exists-web", appRoot],
	["@oppulence/ui", uiRoot],
]) {
	console.log(`\n=== shadcn info (${label}) ===`);
	run(["info"], cwd);
}

console.log(`
shadcn is configured for this application.

Add shared primitives to vendor/oppulence/ui:
  pnpm ui:add -- <component>
  pnpm ui:add -- button --diff

Inspect upstream changes before overwriting existing primitives.
`);
