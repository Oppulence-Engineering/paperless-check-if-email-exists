import type { GenerateRequest } from "@/config/generate/input-schemas";

import {
	isJsonMediaType,
	isReadMethod,
	resolveOperation,
	type ResolvedOperation,
} from "./orval-operation";

/**
 * Fill path/schema/name from `--operation` and refuse local Zod stubs.
 * Catalog reads always use the real app root so tests can write into a temp
 * tree without copying OpenAPI + Orval output.
 */

type OrvalCapable = Extract<
	GenerateRequest,
	{ kind: "hook" | "mutation" | "upload" | "page" | "feature" }
>;

function hasOrvalFields(request: {
	orvalSchema?: string;
	orvalImport?: string;
	path?: string;
}): boolean {
	return Boolean(request.orvalSchema && request.orvalImport && request.path);
}

function requireOrvalContract(request: OrvalCapable, kindLabel: string): void {
	if (hasOrvalFields(request)) return;
	throw new Error(
		`${kindLabel} requires --operation <operationId> or --orval-schema + --orval-import + --path. Local Zod stubs are not generated; Orval owns API shapes.`,
	);
}

function requireName(
	request: { name?: string },
	kindLabel: string,
): asserts request is { name: string } {
	if (!request.name) {
		throw new Error(`${kindLabel} requires --name or --operation`);
	}
}

function requireJsonResponse(operation: ResolvedOperation, kindLabel: string): void {
	if (
		operation.responseMediaTypes.length > 0 &&
		!operation.responseMediaTypes.some(isJsonMediaType)
	) {
		throw new Error(
			`${operation.operationId} returns ${operation.responseMediaTypes.join(", ")}; ${kindLabel} requires a JSON response contract.`,
		);
	}
}

function applyUnitOperation<
	T extends Extract<GenerateRequest, { kind: "hook" | "mutation" | "upload" }>,
>(request: T, operation: ResolvedOperation): T {
	return {
		...request,
		name: request.name ?? operation.name,
		path: request.path ?? operation.fetchPath,
		orvalSchema: request.orvalSchema ?? operation.orvalSchema,
		orvalImport: request.orvalImport ?? operation.orvalImport,
		orvalQuery: request.orvalQuery ?? operation.orvalQuery,
		queryParams: request.queryParams.length > 0 ? request.queryParams : operation.queryParams,
		...(request.kind === "mutation" || request.kind === "upload"
			? {
					orvalBody: request.orvalBody ?? operation.orvalBody,
					method: request.method ?? operation.method,
				}
			: {}),
	};
}

function applyComposerOperation<T extends Extract<GenerateRequest, { kind: "page" | "feature" }>>(
	request: T,
	operation: ResolvedOperation,
): T {
	return {
		...request,
		path: request.path ?? operation.fetchPath,
		orvalSchema: request.orvalSchema ?? operation.orvalSchema,
		orvalImport: request.orvalImport ?? operation.orvalImport,
		orvalQuery: request.orvalQuery ?? operation.orvalQuery,
		queryParams: request.queryParams.length > 0 ? request.queryParams : operation.queryParams,
	};
}

export function resolveGenerateRequest(
	request: GenerateRequest,
	catalogRoot: string,
): GenerateRequest {
	if (
		request.kind !== "hook" &&
		request.kind !== "mutation" &&
		request.kind !== "upload" &&
		request.kind !== "page" &&
		request.kind !== "feature"
	) {
		return request;
	}

	let resolved: OrvalCapable = request;
	if (request.operation) {
		const operation = resolveOperation(request.operation, catalogRoot);
		requireJsonResponse(operation, `gen ${request.kind}`);
		if (
			request.kind === "mutation" &&
			operation.requestMediaTypes.length > 0 &&
			!operation.requestMediaTypes.some(isJsonMediaType)
		) {
			throw new Error(
				`${request.operation} requires ${operation.requestMediaTypes.join(", ")}; gen mutation only sends JSON.`,
			);
		}
		if (request.kind === "hook" && !isReadMethod(operation.method)) {
			throw new Error(
				`${request.operation} is ${operation.method}; use \`gen mutation --operation ${request.operation}\`.`,
			);
		}
		if (
			(request.kind === "mutation" || request.kind === "upload") &&
			isReadMethod(operation.method)
		) {
			throw new Error(
				`${request.operation} is ${operation.method}; use \`gen hook --operation ${request.operation}\`.`,
			);
		}
		if (
			(request.kind === "page" || request.kind === "feature") &&
			!isReadMethod(operation.method)
		) {
			throw new Error(
				`${request.operation} is ${operation.method}; pages and features only prefetch GET operations. Use \`gen mutation\`.`,
			);
		}
		resolved =
			request.kind === "page" || request.kind === "feature"
				? applyComposerOperation(request, operation)
				: applyUnitOperation(request, operation);
	}

	if (resolved.kind === "hook") {
		requireOrvalContract(resolved, "gen hook");
		requireName(resolved, "gen hook");
		return resolved;
	}
	if (resolved.kind === "mutation") {
		requireOrvalContract(resolved, "gen mutation");
		requireName(resolved, "gen mutation");
		return { ...resolved, method: resolved.method ?? "POST" };
	}
	if (resolved.kind === "upload") {
		requireOrvalContract(resolved, "gen upload");
		requireName(resolved, "gen upload");
		return { ...resolved, method: resolved.method ?? "POST" };
	}
	if (resolved.operation || resolved.orvalSchema) {
		requireOrvalContract(resolved, `gen ${resolved.kind}`);
	}
	return resolved;
}
