import { DocsBody, DocsDescription, DocsPage, DocsTitle } from "fumadocs-ui/page";
import Link from "next/link";

export const metadata = {
	title: "Developer quickstart",
	description: "Call the email verification API on the same host as the app.",
};

export default function DeveloperQuickstart() {
	return (
		<DocsPage>
			<DocsTitle>Build with the email verification API</DocsTitle>
			<DocsDescription>
				Use a workspace API key for server-to-server requests. The API is available on the same
				HTTPS host as the app at <code>/v1/*</code>.
			</DocsDescription>
			<DocsBody>
				<h2>1. Create a workspace key</h2>
				<p>
					Sign in and create a key in{" "}
					<Link href="/app/settings?settings=developer">Developer settings</Link>. Copy it when it
					is shown; the full key is not available again. Keep it on your server.
				</p>
				<h2>2. Call the API</h2>
				<p>
					Set <code>API_BASE_URL</code> to the origin in this page&apos;s address bar. Use the
					existing staging or production HTTPS host for that environment.
				</p>
				<pre>
					<code>{`export API_BASE_URL="https://YOUR_APP_HOST"
export API_KEY="rch_live_..."
curl --fail-with-body -X POST "$API_BASE_URL/v1/check_email" \\
  -H "Authorization: Bearer $API_KEY" \\
  -H "Content-Type: application/json" \\
  --data '{"to_email":"person@example.com"}'`}</code>
				</pre>
				<p>
					The key identifies its workspace. Tenant operations use the same key and cannot access
					another workspace. Signed-in browser requests use <code>/api/backend/*</code> through the
					app session instead.
				</p>
				<h2>3. Explore the contract</h2>
				<p>
					The <Link href="/developers/reference">API reference</Link> lists every operation, request
					body, response, and access level. Download the{" "}
					<Link href="/openapi.json">OpenAPI JSON</Link>
					for client generation. The generated <code>@oppulence/reacher-sdk</code> TypeScript and Go
					clients are maintained in this repository; use their source or generate a client from the
					same contract.
				</p>
				<h2>Access and compatibility</h2>
				<ul>
					<li>
						Workspace API keys call tenant <code>/v1/*</code> operations on this host.
					</li>
					<li>
						Provider callbacks are server-to-server deliveries and use their own tokens and
						signatures.
					</li>
					<li>
						Platform admin endpoints require an isolated operator credential and are not available
						through a workspace key.
					</li>
					<li>
						Legacy <code>/v0/*</code> uses a global secret in self-hosted installations; it is not
						routed on the shared app host.
					</li>
				</ul>
			</DocsBody>
		</DocsPage>
	);
}
