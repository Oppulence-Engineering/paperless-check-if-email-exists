import "server-only";

import { z } from "zod";

export type ParseResult<T> = { success: true; data: T } | { success: false };

export function parseSearchParams<T extends z.ZodType>(
	searchParams: URLSearchParams,
	schema: T,
): ParseResult<z.infer<T>> {
	const result = schema.safeParse(Object.fromEntries(searchParams.entries()));
	return result.success ? { success: true, data: result.data } : { success: false };
}

export async function readBoundedBody(
	request: Pick<Request, "headers" | "body">,
	maxBytes: number,
): Promise<ParseResult<ArrayBuffer>> {
	const declaredLength = request.headers.get("content-length");
	if (declaredLength && (!/^\d+$/.test(declaredLength) || Number(declaredLength) > maxBytes)) {
		return { success: false };
	}
	if (!request.body) return { success: true, data: new ArrayBuffer(0) };

	const reader = request.body.getReader();
	const chunks: Uint8Array[] = [];
	let length = 0;
	try {
		for (let next = await reader.read(); !next.done; next = await reader.read()) {
			length += next.value.byteLength;
			if (length > maxBytes) {
				await reader.cancel();
				return { success: false };
			}
			chunks.push(next.value);
		}
	} finally {
		reader.releaseLock();
	}

	const body = new Uint8Array(length);
	let offset = 0;
	for (const chunk of chunks) {
		body.set(chunk, offset);
		offset += chunk.byteLength;
	}
	return { success: true, data: body.buffer };
}

export async function readJsonBody(
	request: Pick<Request, "headers" | "body">,
	maxBytes: number,
): Promise<ParseResult<unknown>> {
	const body = await readBoundedBody(request, maxBytes);
	if (!body.success) return body;
	if (body.data.byteLength === 0) return { success: true, data: {} };

	try {
		return { success: true, data: JSON.parse(new TextDecoder().decode(body.data)) };
	} catch {
		return { success: false };
	}
}

export function parseJsonBody<T extends z.ZodType>(
	request: Request,
	schema: T,
	maxBytes: number,
): Promise<ParseResult<z.infer<T>>> {
	return readJsonBody(request, maxBytes).then((body) => {
		if (!body.success) return { success: false };

		const result = schema.safeParse(body.data);
		return result.success ? { success: true, data: result.data } : { success: false };
	});
}

export function parseRouteParams<T extends z.ZodType>(
	params: unknown,
	schema: T,
): ParseResult<z.infer<T>> {
	const result = schema.safeParse(params);
	return result.success ? { success: true, data: result.data } : { success: false };
}
