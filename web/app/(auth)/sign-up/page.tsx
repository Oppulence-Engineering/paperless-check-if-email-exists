import { Suspense } from "react";

import { AuthShell } from "@/components/auth/auth-shell";
import { branding } from "@/lib/auth/branding";
import { getAuthRuntimeConfig } from "@/lib/auth/config";
import { safeReturnTo } from "@/lib/auth/origin";
import { createMetadata } from "@/lib/metadata";

export const metadata = createMetadata({
	title: "Create a workspace",
	description: "Start a Check If Email Exists workspace with passwordless authentication.",
	robots: { index: false, follow: false },
});

type SignUpSearchParams = Promise<{ error?: string; return_to?: string }>;

async function SignUpContent({ searchParams }: { searchParams: SignUpSearchParams }) {
	const params = await searchParams;
	const brand = await branding();
	const authConfig = getAuthRuntimeConfig();
	return (
		<AuthShell
			brandName={brand.name}
			error={params.error}
			googleEnabled={Boolean(authConfig.google)}
			logoUrl={brand.wordmarkUrl || brand.logoUrl}
			microsoftEnabled={Boolean(authConfig.microsoft)}
			mode="sign-up"
			privacyUrl={brand.privacyUrl}
			returnTo={safeReturnTo(params.return_to)}
			termsUrl={brand.termsUrl}
		/>
	);
}

export default function SignUpPage({ searchParams }: { searchParams: SignUpSearchParams }) {
	// searchParams is runtime data, so it must resolve under Suspense.
	// One AuthShell fallback — do not also add loading.tsx with the same tree.
	return (
		<Suspense fallback={<AuthShell mode="sign-up" returnTo="/app" />}>
			<SignUpContent searchParams={searchParams} />
		</Suspense>
	);
}
