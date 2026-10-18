import { referenceHref } from "@/lib/developer-portal/operations";

/**
 * Old links point to one page for each operation. Scalar shows all operations
 * on one page, so send each link to the anchor of its operation. The browser
 * keeps the hash after a redirect.
 */
export async function GET(
	_request: Request,
	{ params }: { params: Promise<{ operationId: string }> },
) {
	const { operationId } = await params;
	return new Response(null, { status: 308, headers: { location: referenceHref(operationId) } });
}
