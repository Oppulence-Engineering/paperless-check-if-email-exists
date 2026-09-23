import { DocsBody, DocsDescription, DocsPage, DocsTitle } from "fumadocs-ui/page";
import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";

import {
	operationById,
	operations,
	schemaByRef,
	type PortalOperation,
} from "@/lib/developer-portal/operations";

type Props = { params: Promise<{ operationId: string }> };

export function generateStaticParams() {
	return operations.map(({ id }) => ({ operationId: id }));
}

export async function generateMetadata({ params }: Props): Promise<Metadata> {
	const operation = operationById((await params).operationId);
	return { title: operation ? `${operation.method} ${operation.path}` : "Unknown API operation" };
}

function accessText(operation: PortalOperation): string {
	switch (operation.audience) {
		case "admin":
			return "Private platform operator API. Requires x-reacher-secret on the internal backend and explicit operator authorization. Workspace API keys cannot call this route.";
		case "legacy":
			return "Legacy self-hosted v0 API. Requires x-reacher-secret. The shared app host does not route v0; migrate to the equivalent v1 operation.";
		case "callback":
			return "Provider-to-server delivery. The provider sends its delivery token in the URL and its configured signature. Do not call this from a browser or expose the token in client code.";
		case "onboarding":
			return "Backend-only onboarding entry point. The shared app host returns 403; use the app sign-up flow.";
		case "system":
			return operation.path === "/version"
				? "Backend-only version endpoint. The shared app host does not route it."
				: "Public system endpoint on the app host. No workspace API key is required.";
		default:
			return "Workspace API key: Authorization: Bearer <key>. The key is scoped to its workspace; the backend checks operation permissions and quota.";
	}
}

function bodyContent(operation: PortalOperation): Record<string, { schema?: unknown }> {
	const body = operation.requestBody as
		{ content?: Record<string, { schema?: unknown }> } | undefined;
	return body?.content ?? {};
}

function documentedSchema(schema: unknown): unknown {
	if (!schema || typeof schema !== "object") return schema;
	const ref = (schema as { $ref?: string }).$ref;
	return ref ? { $ref: ref, definition: schemaByRef(ref) } : schema;
}

function curlExample(operation: PortalOperation): string | undefined {
	if (operation.audience !== "tenant") return;
	const body = bodyContent(operation);
	const media = Object.keys(body)[0];
	const lines = [`curl --fail-with-body -X ${operation.method} "$API_BASE_URL${operation.path}"`];
	lines.push('  -H "Authorization: Bearer $API_KEY"');
	if (media === "application/json") {
		lines.push('  -H "Content-Type: application/json"');
		lines.push("  --data-binary @request.json");
	} else if (media === "multipart/form-data") {
		lines.push('  -F "file=@contacts.csv"');
	}
	return lines.join(" \\\n");
}

export default async function OperationPage({ params }: Props) {
	const operation = operationById((await params).operationId);
	if (!operation) notFound();
	const body = bodyContent(operation);
	const example = curlExample(operation);

	return (
		<DocsPage>
			<DocsTitle>
				{operation.method} {operation.path}
			</DocsTitle>
			<DocsDescription>{operation.description ?? operation.summary}</DocsDescription>
			<DocsBody>
				<p>
					<strong>Family:</strong> {operation.family} · <strong>Operation ID:</strong>{" "}
					<code>{operation.id}</code>
				</p>
				<h2>Access</h2>
				<p>{accessText(operation)}</p>
				{operation.audience === "tenant" ? (
					<p>
						<strong>Required scope:</strong> {operation.scope ?? "any authenticated workspace key"}
					</p>
				) : null}
				{operation.audience === "tenant" ? (
					<p>
						Create a key in <Link href="/app/settings?settings=developer">Developer settings</Link>.
						Set <code>API_BASE_URL</code> to the current app origin.
					</p>
				) : null}
				{example ? (
					<>
						<h2>Request example</h2>
						<pre>
							<code>{example}</code>
						</pre>
					</>
				) : null}
				{operation.path.includes("{") ? (
					<p>
						Replace each <code>{"{parameter}"}</code> segment in the path with its URL-encoded
						value.
					</p>
				) : null}
				{operation.parameters.length > 0 ? (
					<>
						<h2>Parameters</h2>
						<pre>
							<code>{JSON.stringify(operation.parameters, null, 2)}</code>
						</pre>
					</>
				) : null}
				{Object.entries(body).map(([media, content]) => (
					<section key={media}>
						<h2>Request body · {media}</h2>
						<pre>
							<code>{JSON.stringify(documentedSchema(content.schema), null, 2)}</code>
						</pre>
					</section>
				))}
				<h2>Responses</h2>
				{Object.entries(operation.responses).map(([status, response]) => (
					<section key={status}>
						<h3>{status}</h3>
						<pre>
							<code>{JSON.stringify(response, null, 2)}</code>
						</pre>
					</section>
				))}
				<p>
					<Link href="/openapi.json">Download the full OpenAPI contract</Link>
				</p>
			</DocsBody>
		</DocsPage>
	);
}
