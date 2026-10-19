import { z } from "zod";

const DevEnvSchema = z.object({
	betterAuthSecret: z.string().min(32),
	backendApiUrl: z.url(),
	backendJwtAudience: z.string().min(1),
	betterAuthUrl: z.url(),
	databaseUrl: z.url(),
});

export type DevEnvReport = {
	ok: boolean;
	vars: z.infer<typeof DevEnvSchema>;
	warnings: string[];
	errors: string[];
};

/** Read local environment defaults for the dev toolkit. */
export function readDevEnvFromProcess(env: NodeJS.ProcessEnv = process.env): DevEnvReport {
	const warnings: string[] = [];
	const errors: string[] = [];

	const betterAuthSecret = env.BETTER_AUTH_SECRET || "dev-only-better-auth-secret-change-me";

	if (betterAuthSecret.startsWith("dev-only")) {
		warnings.push("Using the development Better Auth secret — do not use it in production.");
	}

	const backendApiUrl = env.BACKEND_API_URL || "http://127.0.0.1:8081";
	const backendJwtAudience = env.BACKEND_JWT_AUDIENCE || "check-if-email-exists-api";
	const betterAuthUrl = env.BETTER_AUTH_URL || "http://localhost:3000";
	const databaseUrl = env.DATABASE_URL || "postgres://postgres:postgres@127.0.0.1:25432/reacher";

	const parsed = DevEnvSchema.safeParse({
		betterAuthSecret,
		backendApiUrl,
		backendJwtAudience,
		betterAuthUrl,
		databaseUrl,
	});

	if (!parsed.success) {
		errors.push(...parsed.error.issues.map((issue) => issue.message));
		return {
			ok: false,
			vars: {
				betterAuthSecret,
				backendApiUrl,
				backendJwtAudience,
				betterAuthUrl,
				databaseUrl,
			},
			warnings,
			errors,
		};
	}

	return { ok: errors.length === 0, vars: parsed.data, warnings, errors };
}
