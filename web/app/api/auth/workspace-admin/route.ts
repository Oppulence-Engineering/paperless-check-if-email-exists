import { NextRequest, NextResponse } from "next/server";

import { parseJsonBody } from "@/lib/api/routes/parse";
import { identityAudit } from "@/lib/auth/identity-audit";
import { publicOrigin } from "@/lib/auth/origin";
import { WorkspaceAdminActionSchema } from "@/lib/auth/schemas";
import { getAuthorizedSession } from "@/lib/auth/session";
import { hasValidStepUp } from "@/lib/auth/step-up";
import {
	applyWorkspaceAdminAction,
	getWorkspaceAdminPolicy,
	getWorkspaceAdminState,
	isWorkspaceAdmin,
	isWorkspaceOwner,
	WorkspaceAdminError,
} from "@/lib/auth/workspace-admin";
import { isSameOriginBrowserRequest } from "@/lib/bff/same-origin-request";

const MAX_BODY_BYTES = 384 * 1024;
const ownerActions = new Set(["transfer_ownership", "archive_workspace", "delete_workspace"]);

function response(code: string, status: number) {
	return NextResponse.json(
		{ error: code, code },
		{ status, headers: { "cache-control": "no-store" } },
	);
}

async function audit(
	request: NextRequest,
	session: NonNullable<Awaited<ReturnType<typeof getAuthorizedSession>>>,
	action: string,
	targetId: string | null,
	result: "success" | "failure",
) {
	await identityAudit({
		actorId: session.user.id,
		organizationId: session.membership.organizationId,
		action: `workspace-admin.${action}`,
		targetId,
		result,
		requestId: request.headers.get("x-request-id"),
	}).catch(() => undefined);
}

export async function GET(request: NextRequest) {
	const session = await getAuthorizedSession(request.headers);
	if (!session) return response("unauthorized", 401);
	if (!isWorkspaceAdmin(session.membership.role)) return response("forbidden", 403);
	try {
		const state = await getWorkspaceAdminState(session.membership.organizationId);
		return NextResponse.json(state, { headers: { "cache-control": "no-store" } });
	} catch (error) {
		if (error instanceof WorkspaceAdminError) return response(error.code, error.status);
		return response("workspace_state_unavailable", 503);
	}
}

export async function POST(request: NextRequest) {
	if (!isSameOriginBrowserRequest(request, publicOrigin(request)))
		return response("forbidden", 403);
	const session = await getAuthorizedSession(request.headers);
	if (!session) return response("unauthorized", 401);
	if (!isWorkspaceAdmin(session.membership.role)) return response("forbidden", 403);

	const body = await parseJsonBody(request, WorkspaceAdminActionSchema, MAX_BODY_BYTES);
	if (!body.success) return response("bad_request", 400);
	const action = body.data;
	const targetId =
		"memberId" in action ? action.memberId : "invitationId" in action ? action.invitationId : null;

	if (ownerActions.has(action.action) && !isWorkspaceOwner(session.membership.role)) {
		await audit(request, session, action.action, targetId, "failure");
		return response("owner_required", 403);
	}

	try {
		const policy = await getWorkspaceAdminPolicy(session.membership.organizationId);
		if (
			policy.requireAdminStepUp &&
			!hasValidStepUp({
				verifiedAt: session.session.stepUpVerifiedAt,
				method: session.session.stepUpMethod ?? null,
				purpose: session.session.stepUpPurpose ?? null,
				requiredPurpose: "admin",
				allowedMethods: ["passkey", "totp"],
			})
		) {
			await audit(request, session, `${action.action}.step-up-required`, targetId, "failure");
			return response("step_up_required", 403);
		}

		const result = await applyWorkspaceAdminAction(session, action, request.headers);
		await audit(request, session, action.action, targetId, "success");
		return NextResponse.json(result, { headers: { "cache-control": "no-store" } });
	} catch (error) {
		await audit(request, session, action.action, targetId, "failure");
		if (error instanceof WorkspaceAdminError) return response(error.code, error.status);
		return response("workspace_mutation_failed", 409);
	}
}
