import { Suspense } from "react";

import { AuthShell } from "@/components/auth/auth-shell";
import { branding } from "@/lib/auth/branding";
import { getAuthRuntimeConfig } from "@/lib/auth/config";
import { safeReturnTo } from "@/lib/auth/origin";
import { createMetadata } from "@/lib/metadata";

export const metadata = createMetadata({
	title: "Sign in",
	description: "Sign in securely to verify email addresses and manage list quality.",
	robots: { index: false, follow: false },
});

type SignInSearchParams = Promise<{ error?: string; invitation?: string; return_to?: string }>;

async function SignInContent({ searchParams }: { searchParams: SignInSearchParams }) {
	const params = await searchParams;
	const brand = await branding();
	const authConfig = getAuthRuntimeConfig();
	const returnTo = safeReturnTo(params.return_to);
	const callback = params.invitation
		? `/app/invitations/accept?${new URLSearchParams({
				invitation: params.invitation,
				return_to: returnTo,
			})}`
		: returnTo;
	return (
		<AuthShell
			brandName={brand.name}
			error={params.error}
			googleEnabled={Boolean(authConfig.google)}
			logoUrl={brand.wordmarkUrl || brand.logoUrl}
			microsoftEnabled={Boolean(authConfig.microsoft)}
			mode="sign-in"
			privacyUrl={brand.privacyUrl}
			returnTo={callback}
			termsUrl={brand.termsUrl}
		/>
	);
}

export default function SignInPage({ searchParams }: { searchParams: SignInSearchParams }) {
	// searchParams is runtime data, so it must resolve under Suspense.
	// One AuthShell fallback — do not also add loading.tsx with the same tree.
	return (
		<Suspense fallback={<AuthShell mode="sign-in" returnTo="/app" />}>
			<SignInContent searchParams={searchParams} />
		</Suspense>
	);
}
