import { errorMessage } from "@/lib/api/error-message";
import { z } from "zod";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";

import {
	V1CheckEmail200Response,
	V1CheckEmail202Response,
	V1CheckEmailBody,
} from "@/lib/api/generated/zod/v1/v1";
import { waitForRetry } from "@/lib/backend/integration-contract";

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
		let response = await sdk.v1CheckEmail(
			{ checkEmailRequest: V1CheckEmailBody.parse(body), idempotencyKey: crypto.randomUUID() },
			{ signal },
		);
		while (response.status === 202) {
			const pending = V1CheckEmail202Response.parse(response.data);
			await waitForRetry(1000, signal);
			response = await sdk.v1GetVerificationOperation(
				{ operationId: pending.operation_id },
				{ signal },
			);
		}
		return V1CheckEmail200Response.parse(response.data);
	} catch (error) {
		throw new Error(errorMessage(error));
	}
}
