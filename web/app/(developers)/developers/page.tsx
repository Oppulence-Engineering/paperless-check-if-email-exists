import Link from "next/link";

import {
	CodeBlock,
	DocsPage,
	DocsSection,
	bodyClassName,
	cardSurface,
	linkClassName,
} from "./docs-kit";

export const metadata = {
	title: "Developer quickstart",
	description: "Call the email verification API on the same host as the app.",
};

const RESOURCES = [
	{
		href: "/developers/reference",
		label: "API reference",
		detail: "Every operation, request body, response, and access level.",
	},
	{
		href: "/openapi.json",
		label: "OpenAPI JSON",
		detail: "The source contract for client generation.",
	},
];

export default function DeveloperQuickstart() {
	return (
		<DocsPage
			lead={
				<>
					Use a workspace API key for server-to-server requests. The API is available on the same
					HTTPS host as the app at <code>/v1/*</code>.
				</>
			}
			title="Build with the email verification API"
		>
			<DocsSection id="resources" title="Start here">
				<div className="grid grid-cols-1 gap-4 sm:grid-cols-2">
					{RESOURCES.map((resource) => (
						<Link className={cardSurface} href={resource.href} key={resource.href}>
							<p className="mb-1 font-medium text-foreground">{resource.label}</p>
							<p className="text-sm text-muted-foreground">{resource.detail}</p>
						</Link>
					))}
				</div>
			</DocsSection>

			<DocsSection id="quickstart" title="Quickstart">
				<div className="space-y-8">
					<div id="create-key">
						<h3 className="mb-2 text-base font-medium text-foreground">
							1. Create a workspace key
						</h3>
						<p className={bodyClassName}>
							Sign in and create a key in{" "}
							<Link className={linkClassName} href="/app/settings?settings=developer">
								Developer settings
							</Link>
							. Copy it when it is shown; the full key is not available again. Keep it on your
							server.
						</p>
					</div>
					<div id="call-api">
						<h3 className="mb-2 text-base font-medium text-foreground">2. Call the API</h3>
						<p className={`mb-3 ${bodyClassName}`}>
							Set <code>API_BASE_URL</code> to the origin in this page&apos;s address bar. Use the
							existing staging or production HTTPS host for that environment.
						</p>
						<CodeBlock>{`export API_BASE_URL="https://YOUR_APP_HOST"
export API_KEY="rch_live_..."
curl --fail-with-body -X POST "$API_BASE_URL/v1/check_email" \\
  -H "Authorization: Bearer $API_KEY" \\
  -H "Content-Type: application/json" \\
  --data '{"to_email":"person@example.com"}'`}</CodeBlock>
						<p className={`mt-3 ${bodyClassName}`}>
							The key identifies its workspace. Tenant operations use the same key and cannot access
							another workspace. Signed-in browser requests use <code>/api/backend/*</code> through
							the app session instead.
						</p>
					</div>
					<div id="contract">
						<h3 className="mb-2 text-base font-medium text-foreground">3. Explore the contract</h3>
						<p className={bodyClassName}>
							The{" "}
							<Link className={linkClassName} href="/developers/reference">
								API reference
							</Link>{" "}
							lists every operation, request body, response, and access level. Download the{" "}
							<Link className={linkClassName} href="/openapi.json">
								OpenAPI JSON
							</Link>{" "}
							for client generation. The generated <code>@oppulence/reacher-sdk</code> TypeScript
							and Go clients are maintained in this repository; use their source or generate a
							client from the same contract.
						</p>
					</div>
				</div>
			</DocsSection>

			<DocsSection id="access" title="Access and compatibility">
				<ul className={`list-disc space-y-2 pl-5 ${bodyClassName}`}>
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
						Legacy <code>/v0/*</code> routes are not available on the hosted service. Use the v1
						equivalents.
					</li>
				</ul>
			</DocsSection>
		</DocsPage>
	);
}
