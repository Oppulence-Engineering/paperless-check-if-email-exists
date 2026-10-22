import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const appRoot = path.dirname(path.dirname(fileURLToPath(import.meta.url)));
// Initial route references measured on 2026-10-21, with 5% regression headroom.
const budgets = {
	"/app": {
		js: 1016417,
		css: 325775,
		largest: 275601,
	},
	"/app/lists": {
		js: 1011921,
		css: 325775,
		largest: 275601,
	},
	"/app/lists/detail": {
		js: 1040516,
		css: 325775,
		largest: 275601,
	},
	"/app/analytics": {
		js: 1209943,
		css: 325775,
		largest: 275601,
	},
	"/app/research": {
		js: 1022427,
		css: 325775,
		largest: 275601,
	},
	"/app/settings": {
		js: 653148,
		css: 325775,
		largest: 138626,
	},
};
let failed = false;
for (const [route, budget] of Object.entries(budgets)) {
	const suffix = `/app/(product)${route}/page`;
	const manifestPath = path.join(
		appRoot,
		`.next/server/app/(product)${route}/page_client-reference-manifest.js`,
	);
	const source = fs.readFileSync(manifestPath, "utf8").trim();
	const manifest = JSON.parse(source.slice(source.lastIndexOf(" = ") + 3, source.lastIndexOf(";")));
	const entry = (kind) => {
		const key = Object.keys(manifest[kind]).find((key) => key.endsWith(suffix));
		if (!key) throw new Error(`Missing ${route} ${kind} manifest`);
		return manifest[kind][key];
	};
	const sizes = entry("entryJSFiles").map(
		(file) => fs.statSync(path.join(appRoot, ".next", file)).size,
	);
	const measured = {
		js: sizes.reduce((sum, size) => sum + size, 0),
		css: entry("entryCSSFiles").reduce(
			(sum, file) => sum + fs.statSync(path.join(appRoot, ".next", file.path)).size,
			0,
		),
		largest: Math.max(...sizes),
	};
	for (const [kind, bytes] of Object.entries(measured)) {
		console.log(
			`${route} ${kind}: ${(bytes / 1024).toFixed(1)} KiB / ${(budget[kind] / 1024).toFixed(1)} KiB`,
		);
		if (bytes > budget[kind]) failed = true;
	}
}
if (failed) {
	console.error("Bundle budget exceeded. Run pnpm analyze to inspect the route graph.");
	process.exit(1);
}
