import { execFileSync, spawnSync } from "node:child_process";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../..");
const snapshot = fs.mkdtempSync(path.join(os.tmpdir(), "cie-web-gitleaks-"));

try {
	const files = execFileSync(
		"git",
		["ls-files", "--cached", "--others", "--exclude-standard", "-z", "--", "web"],
		{ cwd: root, encoding: "utf8", maxBuffer: 50 * 1024 * 1024 },
	).split("\0");
	for (const file of files) {
		if (!file) continue;
		const source = path.join(root, file);
		if (!fs.existsSync(source) || !fs.lstatSync(source).isFile()) continue;
		const destination = path.join(snapshot, file);
		fs.mkdirSync(path.dirname(destination), { recursive: true });
		fs.copyFileSync(source, destination);
	}
	console.log("[gitleaks] scanning current web source");
	const result = spawnSync(
		"go",
		[
			"run",
			"github.com/zricethezav/gitleaks/v8@v8.30.1",
			"dir",
			"--redact",
			"--no-banner",
			snapshot,
		],
		{ cwd: root, stdio: "inherit" },
	);
	process.exitCode = result.status ?? 1;
} finally {
	fs.rmSync(snapshot, { recursive: true, force: true });
}
