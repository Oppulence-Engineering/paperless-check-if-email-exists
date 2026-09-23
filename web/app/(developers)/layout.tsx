import type * as PageTree from "fumadocs-core/page-tree";
import { DocsLayout } from "fumadocs-ui/layouts/docs";
import { RootProvider } from "fumadocs-ui/provider/next";
import type { ReactNode } from "react";

import { families, operations } from "@/lib/developer-portal/operations";

const tree: PageTree.Root = {
	name: "Developer portal",
	children: [
		{ type: "page", name: "Quickstart", url: "/developers" },
		{ type: "page", name: "API reference", url: "/developers/reference" },
		...families.map((family) => ({
			type: "folder" as const,
			name: family,
			children: operations
				.filter((operation) => operation.family === family)
				.map((operation) => ({
					type: "page" as const,
					name: `${operation.method} ${operation.path}`,
					url: `/developers/reference/${operation.id}`,
				})),
		})),
	],
};

export default function DeveloperLayout({ children }: { children: ReactNode }) {
	return (
		<RootProvider>
			<DocsLayout
				tree={tree}
				nav={{ title: "Check If Email Exists · Developers", url: "/developers" }}
				links={[
					{ text: "Workspace", url: "/app" },
					{ text: "OpenAPI JSON", url: "/openapi.json" },
				]}
				searchToggle={{ enabled: false }}
			>
				{children}
			</DocsLayout>
		</RootProvider>
	);
}
