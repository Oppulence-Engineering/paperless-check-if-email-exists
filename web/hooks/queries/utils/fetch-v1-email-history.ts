import { Configuration, VerificationApi } from "@oppulence/reacher-sdk";
import { z } from "zod";

import {
	V1EmailHistory200Response,
	V1EmailHistoryParams,
	V1EmailHistoryQueryParams,
} from "@/lib/api/generated/zod/verification/verification";

export type V1EmailHistoryQuery = z.input<typeof V1EmailHistoryQueryParams>;
const sdk = new VerificationApi(new Configuration({ basePath: "/api/backend" }));

/** @oppulence-gen kind=hook; owned by `use-v1-email-history.lit.ts`. */
export async function fetchV1EmailHistory(
	email: string,
	query?: V1EmailHistoryQuery,
	signal?: AbortSignal,
) {
	const params = V1EmailHistoryParams.parse({ email });
	const filters = V1EmailHistoryQueryParams.parse(query ?? {});
	const response = await sdk.v1EmailHistory(
		{ email: params.email, limit: filters.limit },
		{ signal },
	);
	return V1EmailHistory200Response.parse(response.data);
}
