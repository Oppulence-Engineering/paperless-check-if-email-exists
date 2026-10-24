import { z } from "zod";
const responseError = z.object({
	response: z.object({ status: z.number().optional(), data: z.unknown().optional() }),
});
const envelope = z.object({ error: z.string() });
export function errorMessage(error: unknown): string {
	const response = responseError.safeParse(error);
	if (response.success) {
		const { status, data } = response.data.response;
		if (status && status >= 500) return "Service unavailable. Please retry.";
		const body = envelope.safeParse(data);
		if (body.success) return body.data.error;
	}
	const message = z.object({ message: z.string() }).safeParse(error);
	return message.success ? message.data.message : "The request failed. Please retry.";
}
