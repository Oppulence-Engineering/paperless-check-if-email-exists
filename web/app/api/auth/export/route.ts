import { NextResponse } from "next/server";

import { exportUserData } from "@/lib/auth/data-export";
import { DataExportSchema } from "@/lib/auth/data-export.schema";
import { identityAudit } from "@/lib/auth/identity-audit";
import { getOptionalSession } from "@/lib/auth/session";
import { hasValidStepUp } from "@/lib/auth/step-up";
import { describeError, log } from "@/lib/observability/logger";

/**
 * Hands a person everything this application holds about them, as one JSON
 * file. Gated behind the same recent verification as account deletion: an
 * export is a copy of someone's identity, and a stolen session should not be
 * able to take one.
 */
export async function GET(): Promise<NextResponse> {
	const session = await getOptionalSession();
	if (!session) {
		return NextResponse.json({ code: "unauthorized" }, { status: 401 });
	}

	const verified = hasValidStepUp({
		verifiedAt: session.session.stepUpVerifiedAt,
		method: session.session.stepUpMethod,
		purpose: session.session.stepUpPurpose,
		requiredPurpose: "account-delete",
		allowedMethods: ["email-otp"],
	});

	await identityAudit({
		actorId: session.user.id,
		organizationId: session.membership.organizationId,
		action: "account.export",
		targetId: session.user.id,
		result: verified ? "success" : "failure",
		requestId: null,
	});

	if (!verified) {
		return NextResponse.json(
			{ code: "step_up_required", detail: "Verify your identity again to export your data." },
			{ status: 403 },
		);
	}

	try {
		const payload = DataExportSchema.parse(await exportUserData(session.user.id));
		return new NextResponse(JSON.stringify(payload, null, 2), {
			headers: {
				"Content-Type": "application/json",
				"Content-Disposition": `attachment; filename="account-export.json"`,
				"Cache-Control": "no-store",
			},
		});
	} catch (error) {
		log.error("account export failed", describeError(error));
		return NextResponse.json(
			{ code: "export_failed", detail: "The export could not be produced." },
			{ status: 500 },
		);
	}
}
