import { existsSync, mkdirSync, readdirSync, symlinkSync } from "node:fs";
import path from "node:path";
import { pathToFileURL } from "node:url";

const outputRoot = path.resolve(".next/standalone");

function findServer(directory) {
	if (!existsSync(directory)) return undefined;
	const direct = path.join(directory, "server.js");
	if (existsSync(direct)) return direct;

	for (const entry of readdirSync(directory, { withFileTypes: true })) {
		if (!entry.isDirectory() || entry.name === "node_modules") continue;
		const found = findServer(path.join(directory, entry.name));
		if (found) return found;
	}
	return undefined;
}

function linkAssets(source, target) {
	if (!existsSync(source) || existsSync(target)) return;
	mkdirSync(path.dirname(target), { recursive: true });
	symlinkSync(source, target, "dir");
}

const server = findServer(outputRoot);
if (!server) throw new Error("Run `pnpm build` before starting the standalone server.");

const args = process.argv.slice(2);
const option = (name) => {
	const index = args.indexOf(name);
	return index >= 0 ? args[index + 1] : undefined;
};

process.env.HOSTNAME = option("--hostname") ?? process.env.HOSTNAME ?? "0.0.0.0";
process.env.PORT = option("--port") ?? process.env.PORT ?? "3000";

const appRoot = path.dirname(server);
linkAssets(path.resolve("public"), path.join(appRoot, "public"));
linkAssets(path.resolve(".next/static"), path.join(appRoot, ".next/static"));

await import(pathToFileURL(server).href);
