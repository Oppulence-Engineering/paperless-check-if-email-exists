import { z } from "zod";

export const LogoutQuerySchema = z.object({
	return_to: z.string().optional(),
});
