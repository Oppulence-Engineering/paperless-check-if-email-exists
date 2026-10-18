import { ApiReference } from "@/components/features/developer-portal/api-reference/api-reference";
import { referenceDocument } from "@/lib/developer-portal/operations";

export const metadata = {
	title: "API reference",
	description:
		"Every public Verifier API operation, with models, code samples, and a client that sends requests.",
};

/** The public API reference. Scalar shows it under the sticky site header (65px). */
export default function ReferenceIndex() {
	return (
		<div className="w-full flex-1 [--scalar-custom-header-height:65px]">
			<ApiReference
				aria-label="API reference"
				document={referenceDocument("public")}
				surface="public"
			/>
		</div>
	);
}
