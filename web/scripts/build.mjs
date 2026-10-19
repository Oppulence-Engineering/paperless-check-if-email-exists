import { spawnSync } from "node:child_process";

const buildDefaults = {
	BACKEND_API_URL: "http://127.0.0.1:8081",
	BACKEND_JWT_AUDIENCE: "check-if-email-exists-api",
	BETTER_AUTH_URL: "https://build.invalid",
	BETTER_AUTH_SECRET: "build-only-better-auth-secret-00000001",
	SCIM_CREDENTIAL_HASH_SECRET: "build-only-scim-credential-hash-secret-0001",
	DATABASE_URL: "postgres:///reacher-build",
	RESEND_API_KEY: "build-only",
	RESEND_FROM: "auth@example.com",
	APP_BUILD_PHASE: "1",
};

const result = spawnSync(process.execPath, ["node_modules/next/dist/bin/next", "build"], {
	env: { ...process.env, ...buildDefaults },
	stdio: "inherit",
});

process.exit(result.status ?? 1);
