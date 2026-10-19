import { Configuration, V1Api } from "@oppulence/reacher-sdk";
import { z } from "zod";

import { V1ListLists200Response, V1ListListsQueryParams } from "@/lib/api/generated/zod/v1/v1";

export type V1ListListsQuery = z.input<typeof V1ListListsQueryParams>;

const sdk = new V1Api(new Configuration({ basePath: "/api/backend" }));

/** @oppulence-gen kind=hook; owned by `use-v1-list-lists.lit.ts`. */
export async function fetchV1ListLists(query?: V1ListListsQuery, signal?: AbortSignal) {
	const params = V1ListListsQueryParams.parse(query ?? {});
	const response = await sdk.v1ListLists(params, { signal });
	return V1ListLists200Response.parse(response.data);
}
