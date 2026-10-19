import { z } from "zod";

/** Public shape shared by the dev health route and its browser consumer. */
export const DevHealthResponseSchema = z.object({
	www: z.object({
		ok: z.boolean(),
		warnings: z.array(z.string()),
		errors: z.array(z.string()),
	}),
	api: z.object({
		ok: z.boolean(),
		status: z.number().int().optional(),
		latencyMs: z.number().int().nonnegative().optional(),
		error: z.string().optional(),
	}),
	env: z.object({
		backendApiUrl: z.url(),
		betterAuthUrl: z.url(),
	}),
});
