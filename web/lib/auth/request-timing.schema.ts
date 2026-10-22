import { z } from "zod";
/** @oppulence-gen kind=schema Owned by `request-timing.lit.ts`. */
export const RequestTimingInputSchema = z.object({
	phase: z.enum(["auth", "token", "upstream", "pool_wait", "total"]),
});
export type RequestTimingInput = z.infer<typeof RequestTimingInputSchema>;
