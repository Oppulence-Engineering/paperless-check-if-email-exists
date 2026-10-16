/** Allows only same-origin relative redirects after authentication. */
export function safeReturnTo(raw: string | null | undefined): string {
	if (!raw || !raw.startsWith("/") || raw.startsWith("//")) return "/app";
	try {
		const parsed = new URL(raw, "https://backend-ui.invalid");
		return parsed.pathname + parsed.search + parsed.hash;
	} catch {
		return "/app";
	}
}

/**
 * Where to go after authentication: the invitation acceptance page when the
 * link carries an invitation, else the safe return path.
 */
export function authCallbackPath(params: {
	invitation?: string | null;
	return_to?: string | null;
}): string {
	const returnTo = safeReturnTo(params.return_to);
	return params.invitation
		? `/app/invitations/accept?${new URLSearchParams({ invitation: params.invitation, return_to: returnTo }).toString()}`
		: returnTo;
}
