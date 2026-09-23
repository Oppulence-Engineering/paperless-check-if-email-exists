import { DocsBody, DocsDescription, DocsPage, DocsTitle } from "fumadocs-ui/page";
import Link from "next/link";

import { families, operations } from "@/lib/developer-portal/operations";

import { OperationIndex } from "./operation-index";

export const metadata = { title: "API reference", description: "All backend API operations." };

export default function ReferenceIndex() {
	return (
		<DocsPage>
			<DocsTitle>API reference</DocsTitle>
			<DocsDescription>
				{operations.length} operations across {families.length} families. The source contract is
				available as <Link href="/openapi.json">OpenAPI JSON</Link>.
			</DocsDescription>
			<DocsBody>
				<OperationIndex families={families} operations={operations} />
			</DocsBody>
		</DocsPage>
	);
}
