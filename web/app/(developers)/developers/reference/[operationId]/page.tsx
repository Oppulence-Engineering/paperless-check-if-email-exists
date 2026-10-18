import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { Suspense } from "react";

import {
	operationById,
	operations,
	schemaByRef,
	workflowDestination,
	type PortalOperation,
} from "@/lib/developer-portal/operations";

import { CodeBlock, DocsPage, DocsSection, bodyClassName, linkClassName } from "../../docs-kit";

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
			return "Legacy v0 API. It is not available on the hosted service. Use the equivalent v1 operation.";
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

// params is runtime data for unknown IDs, so it resolves under Suspense.
export default function OperationPage({ params }: Props) {
	return (
		<Suspense fallback={null}>
			<OperationContent params={params} />
		</Suspense>
	);
}

async function OperationContent({ params }: Props) {
	const operation = operationById((await params).operationId);
	if (!operation) notFound();
	const body = bodyContent(operation);
	const example = curlExample(operation);
	const workflow = workflowDestination(operation);

	return (
		<DocsPage
			lead={operation.description ?? operation.summary}
			title={
				<span className="font-mono text-2xl sm:text-3xl">
					{operation.method} {operation.path}
				</span>
			}
		>
			<p className={bodyClassName}>
				<Link className={linkClassName} href="/developers/reference">
					API reference
				</Link>{" "}
				· {operation.family} · <code>{operation.id}</code>
			</p>
			<DocsSection title="Access">
				<div className="space-y-3">
					<p className={bodyClassName}>{accessText(operation)}</p>
					<p className={bodyClassName}>
						<Link className={linkClassName} href={workflow.href}>
							{workflow.label}
						</Link>
					</p>
					{operation.audience === "tenant" ? (
						<p className={bodyClassName}>
							<strong className="font-medium text-foreground">Required scope:</strong>{" "}
							{operation.scope ?? "any authenticated workspace key"}
						</p>
					) : null}
					{operation.audience === "tenant" ? (
						<p className={bodyClassName}>
							Create a key in{" "}
							<Link className={linkClassName} href="/app/settings?settings=developer">
								Developer settings
							</Link>
							. Set <code>API_BASE_URL</code> to the current app origin.
						</p>
					) : null}
				</div>
			</DocsSection>
			{example ? (
				<DocsSection title="Request example">
					<CodeBlock>{example}</CodeBlock>
					{operation.path.includes("{") ? (
						<p className={`mt-3 ${bodyClassName}`}>
							Replace each <code>{"{parameter}"}</code> segment in the path with its URL-encoded
							value.
						</p>
					) : null}
				</DocsSection>
			) : null}
			{operation.parameters.length > 0 ? (
				<DocsSection title="Parameters">
					<CodeBlock>{JSON.stringify(operation.parameters, null, 2)}</CodeBlock>
				</DocsSection>
			) : null}
			{Object.entries(body).map(([media, content]) => (
				<DocsSection key={media} title={`Request body · ${media}`}>
					<CodeBlock>{JSON.stringify(documentedSchema(content.schema), null, 2)}</CodeBlock>
				</DocsSection>
			))}
			<DocsSection title="Responses">
				<div className="space-y-6">
					{Object.entries(operation.responses).map(([status, response]) => (
						<div key={status}>
							<h3 className="mb-2 text-base font-medium text-foreground">{status}</h3>
							<CodeBlock>{JSON.stringify(response, null, 2)}</CodeBlock>
						</div>
					))}
				</div>
				<p className={`mt-6 ${bodyClassName}`}>
					<Link className={linkClassName} href="/openapi.json">
						Download the full OpenAPI contract
					</Link>
				</p>
			</DocsSection>
		</DocsPage>
	);
}
