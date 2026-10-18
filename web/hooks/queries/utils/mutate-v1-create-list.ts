import { Configuration, V1Api } from "@oppulence/reacher-sdk";
import { z } from "zod";

import { V1CreateList202Response, V1CreateListBody } from "@/lib/api/generated/zod/v1/v1";

export const V1CreateListInputSchema = V1CreateListBody;
export type V1CreateListInput = z.input<typeof V1CreateListInputSchema>;

const sdk = new V1Api(new Configuration({ basePath: "/api/backend" }));

/** @oppulence-gen kind=mutation; owned by `use-v1-create-list.lit.ts`. */
export async function fetchV1CreateList(body: V1CreateListInput, signal?: AbortSignal) {
	const input = V1CreateListBody.parse(body);
	// The SDK takes camelCase names; a snake_case key would be dropped without an error.
	const response = await sdk.v1CreateList(
		{
			file: input.file,
			name: input.name,
			emailColumn: input.email_column,
			sourceKey: input.source_key,
			policyId: input.policy_id,
		},
		{ signal },
	);
	return V1CreateList202Response.parse(response.data);
}
