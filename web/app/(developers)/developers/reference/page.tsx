import Link from "next/link";

import { families, operations } from "@/lib/developer-portal/operations";

import { DocsPage, linkClassName } from "../docs-kit";
import { OperationIndex } from "./operation-index";

export const metadata = { title: "API reference", description: "All backend API operations." };

export default function ReferenceIndex() {
	return (
		<DocsPage
			lead={
				<>
					{operations.length} operations across {families.length} families. The source contract is
					available as{" "}
					<Link className={linkClassName} href="/openapi.json">
						OpenAPI JSON
					</Link>
					.
				</>
			}
			title="API reference"
		>
			<OperationIndex families={families} operations={operations} />
		</DocsPage>
	);
}
