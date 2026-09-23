import { NextRequest, NextResponse } from "next/server";

import { parseJsonBody } from "@/lib/api/routes/parse";
import { auth } from "@/lib/auth/auth";
import { identityAudit } from "@/lib/auth/identity-audit";
import { publicOrigin } from "@/lib/auth/origin";
import { getAuthorizedSession } from "@/lib/auth/session";
import {
	WorkspaceCreateRequestSchema,
	WorkspaceCreateResponseSchema,
	WorkspaceSwitchRequestSchema,
	WorkspaceSwitchResponseSchema,
} from "@/lib/auth/schemas";
import { isSameOriginBrowserRequest } from "@/lib/bff/same-origin-request";

const MAX_WORKSPACE_MUTATION_BODY_BYTES = 8 * 1024;

export async function PUT(request: NextRequest) {
	if (!isSameOriginBrowserRequest(request, publicOrigin(request))) {
		return NextResponse.json({ error: "forbidden", code: "forbidden" }, { status: 403 });
	}

	const current = await getAuthorizedSession(request.headers);
	if (!current) {
		return NextResponse.json({ error: "unauthenticated", code: "unauthorized" }, { status: 401 });
	}

	const body = await parseJsonBody(
		request,
		WorkspaceCreateRequestSchema,
		MAX_WORKSPACE_MUTATION_BODY_BYTES,
	);
	if (!body.success) {
		return NextResponse.json({ error: "invalid workspace", code: "bad_request" }, { status: 400 });
	}

	let workspace: Awaited<ReturnType<typeof auth.api.createOrganization>>;
	try {
		workspace = await auth.api.createOrganization({
			headers: request.headers,
			body: {
				name: body.data.name,
				slug: body.data.slug,
			},
		});
	} catch {
		await identityAudit({
			actorId: current.user.id,
			organizationId: current.membership.organizationId,
			action: "organization.create",
			targetId: body.data.slug,
			result: "failure",
			requestId: request.headers.get("x-request-id"),
		}).catch(() => undefined);
		return NextResponse.json(
			{ error: "workspace creation rejected", code: "workspace_create_failed" },
			{ status: 409 },
		);
	}

	await identityAudit({
		actorId: current.user.id,
		organizationId: workspace.id,
		action: "organization.create",
		targetId: workspace.id,
		result: "success",
		requestId: request.headers.get("x-request-id"),
	}).catch(() => undefined);

	return NextResponse.json(
		WorkspaceCreateResponseSchema.parse({
			activeOrganizationId: workspace.id,
			workspace: {
				id: workspace.id,
				name: workspace.name,
				slug: workspace.slug,
				role: "owner",
				logoUrl: workspace.logo ?? null,
			},
		}),
		{ headers: { "cache-control": "no-store" } },
	);
}

export async function POST(request: NextRequest) {
	if (!isSameOriginBrowserRequest(request, publicOrigin(request))) {
		return NextResponse.json({ error: "forbidden", code: "forbidden" }, { status: 403 });
	}

	const current = await getAuthorizedSession(request.headers);
	if (!current) {
		return NextResponse.json({ error: "unauthenticated", code: "unauthorized" }, { status: 401 });
	}

	const body = await parseJsonBody(
		request,
		WorkspaceSwitchRequestSchema,
		MAX_WORKSPACE_MUTATION_BODY_BYTES,
	);
	if (!body.success) {
		return NextResponse.json({ error: "invalid workspace", code: "bad_request" }, { status: 400 });
	}

	if (body.data.organizationId === current.membership.organizationId) {
		return NextResponse.json(
			WorkspaceSwitchResponseSchema.parse({
				activeOrganizationId: current.membership.organizationId,
			}),
			{ headers: { "cache-control": "no-store" } },
		);
	}

	try {
		await auth.api.setActiveOrganization({
			headers: request.headers,
			body: { organizationId: body.data.organizationId },
		});
	} catch {
		await identityAudit({
			actorId: current.user.id,
			organizationId: current.membership.organizationId,
			action: "organization.switch",
			targetId: body.data.organizationId,
			result: "failure",
			requestId: request.headers.get("x-request-id"),
		}).catch(() => undefined);
		return NextResponse.json(
			{ error: "workspace switch rejected", code: "forbidden" },
			{ status: 403 },
		);
	}

	await identityAudit({
		actorId: current.user.id,
		organizationId: body.data.organizationId,
		action: "organization.switch",
		targetId: current.membership.organizationId,
		result: "success",
		requestId: request.headers.get("x-request-id"),
	}).catch(() => undefined);

	return NextResponse.json(
		WorkspaceSwitchResponseSchema.parse({ activeOrganizationId: body.data.organizationId }),
		{ headers: { "cache-control": "no-store" } },
	);
}
