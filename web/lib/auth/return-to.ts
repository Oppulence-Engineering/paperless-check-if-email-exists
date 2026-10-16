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
