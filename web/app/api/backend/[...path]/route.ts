import { NextRequest } from "next/server";

import { proxyBackendAPI } from "@/lib/auth/proxy";

type RouteContext = { params: Promise<{ path: string[] }> };

async function handle(request: NextRequest, context: RouteContext) {
	const { path } = await context.params;
	return proxyBackendAPI(request, path);
}

export const GET = handle;
export const POST = handle;
export const PUT = handle;
export const PATCH = handle;
export const DELETE = handle;
