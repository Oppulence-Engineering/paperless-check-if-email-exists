"use client";

import "client-only";

import { passkeyClient } from "@better-auth/passkey/client";
import { ssoClient } from "@better-auth/sso/client";
import { createAuthClient } from "better-auth/react";
import { emailOTPClient, organizationClient, twoFactorClient } from "better-auth/client/plugins";

export const authClient = createAuthClient({
	plugins: [
		organizationClient(),
		emailOTPClient(),
		passkeyClient(),
		twoFactorClient(),
		ssoClient({ domainVerification: { enabled: true } }),
	],
});
