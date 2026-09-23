import { z } from "zod";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";

import { V1CheckEmail200Response, V1CheckEmailBody } from "@/lib/api/generated/zod/v1/v1";

export const V1CheckEmailInputSchema = V1CheckEmailBody;

export type V1CheckEmailInput = z.input<typeof V1CheckEmailInputSchema>;

const sdk = new V1Api(new Configuration({ basePath: "/api/backend" }));
/**
 * @oppulence-gen kind=mutation
 * Same-origin SDK write through the Better Auth BFF. The generated Zod schema
 * validates the backend response. Owned by `use-v1-check-email.lit.ts`.
 */
export async function fetchV1CheckEmail(body: V1CheckEmailInput, signal?: AbortSignal) {
	try {
		const response = await sdk.v1CheckEmail(
			{ checkEmailRequest: V1CheckEmailBody.parse(body) },
			{ signal },
		);
		return V1CheckEmail200Response.parse(response.data);
	} catch (error) {
		const envelope = z
			.object({ response: z.object({ data: z.object({ error: z.string() }) }) })
			.safeParse(error);
		if (envelope.success) throw new Error(envelope.data.response.data.error);
		throw error;
	}
}
