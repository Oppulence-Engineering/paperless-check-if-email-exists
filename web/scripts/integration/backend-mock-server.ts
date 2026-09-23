import { randomUUID } from "node:crypto";
import fs from "node:fs";
import http, { type IncomingMessage, type Server, type ServerResponse } from "node:http";
import path from "node:path";

import { CapabilitiesSchema, type Capabilities } from "../../lib/backend/capabilities.schema";
import {
	BackendErrorEnvelopeSchema,
	IdempotencyKeySchema,
	RequestIdSchema,
} from "../../lib/backend/integration-contract.schema";

export const BACKEND_MOCK_ROUTES = {
	error: "/mock/errors/:status",
	items: "/mock/items",
	mutation: "/mock/mutations",
} as const;

export interface BackendMockServerOptions {
	capabilitiesPath?: string;
	host?: string;
	port?: number;
}

export interface RunningBackendMockServer {
	baseUrl: string;
	capabilities: Capabilities;
	close: () => Promise<void>;
	server: Server;
}

interface StoredMutation {
	body: unknown;
	fingerprint: string;
	status: number;
}

const DEFAULT_CAPABILITIES_PATH = "config/contracts/backend.capabilities.json";
const JSON_HEADERS = { "content-type": "application/json; charset=utf-8" };

export function loadMockBackendCapabilities(override?: string): Capabilities {
	const configured = override || process.env.BACKEND_CAPABILITIES_PATH || DEFAULT_CAPABILITIES_PATH;
	const manifestPath = path.resolve(process.cwd(), configured);
	return CapabilitiesSchema.parse(JSON.parse(fs.readFileSync(manifestPath, "utf8")));
}

export function createBackendMockServer(options: BackendMockServerOptions = {}): {
	capabilities: Capabilities;
	server: Server;
} {
	const capabilities = loadMockBackendCapabilities(options.capabilitiesPath);
	const mutations = new Map<string, StoredMutation>();
	let mutationSequence = 0;

	const server = http.createServer((request, response) => {
		void handleRequest(
			request,
			response,
			capabilities,
			mutations,
			() => {
				mutationSequence += 1;
				return mutationSequence;
			},
			() => mutationSequence,
			() => {
				mutations.clear();
				mutationSequence = 0;
			},
		).catch((error: unknown) => {
			writeError(
				response,
				500,
				requestIdFor(request),
				"mock_server_error",
				"Mock server request failed",
				false,
				{ cause: error instanceof Error ? error.message : "Unknown error" },
			);
		});
	});

	return { capabilities, server };
}

export async function startBackendMockServer(
	options: BackendMockServerOptions = {},
): Promise<RunningBackendMockServer> {
	const { capabilities, server } = createBackendMockServer(options);
	const host = options.host ?? process.env.BACKEND_MOCK_HOST ?? "127.0.0.1";
	const port = options.port ?? parsePort(process.env.BACKEND_MOCK_PORT);

	await new Promise<void>((resolve, reject) => {
		server.once("error", reject);
		server.listen(port, host, () => {
			server.off("error", reject);
			resolve();
		});
	});

	const address = server.address();
	if (!address || typeof address === "string")
		throw new Error("Mock server did not bind a TCP port");

	return {
		baseUrl: `http://${host}:${String(address.port)}`,
		capabilities,
		server,
		close: () =>
			new Promise<void>((resolve, reject) => {
				server.close((error) => {
					if (error) reject(error);
					else resolve();
				});
			}),
	};
}

async function handleRequest(
	request: IncomingMessage,
	response: ServerResponse,
	capabilities: Capabilities,
	mutations: Map<string, StoredMutation>,
	nextMutationSequence: () => number,
	currentMutationSequence: () => number,
	reset: () => void,
): Promise<void> {
	const method = request.method?.toUpperCase() ?? "GET";
	const url = new URL(request.url ?? "/", "http://backend.mock");
	const requestId = requestIdFor(request);

	if (method === "GET" && url.pathname === "/__test/state") {
		writeJson(
			response,
			200,
			{
				mutationCount: mutations.size,
				mutationSequence: currentMutationSequence(),
			},
			requestId,
		);
		return;
	}
	if (method === "POST" && url.pathname === "/__test/reset") {
		reset();
		writeJson(response, 200, { reset: true }, requestId);
		return;
	}

	if (method === "GET" && url.pathname === capabilities.health.livenessPath) {
		writeJson(response, 200, { status: "ok" }, requestId);
		return;
	}
	if (method === "GET" && url.pathname === capabilities.health.readinessPath) {
		writeJson(
			response,
			200,
			{
				status: "ready",
				service: capabilities.service.name,
				apiVersion: capabilities.service.apiVersion,
			},
			requestId,
		);
		return;
	}

	const basePath = capabilities.http.basePath === "/" ? "" : capabilities.http.basePath;
	const contractPath = url.pathname.startsWith(`${basePath}/`)
		? url.pathname.slice(basePath.length)
		: null;

	const errorMatch = contractPath?.match(/^\/mock\/errors\/(\d{3})$/);
	if (method === "GET" && errorMatch) {
		const status = Number(errorMatch[1]);
		if (status < 400 || status > 599) {
			writeError(
				response,
				400,
				requestId,
				"invalid_status",
				"Error status must be between 400 and 599",
			);
			return;
		}
		writeError(
			response,
			status,
			requestId,
			`mock_http_${String(status)}`,
			`Mock backend error (${String(status)})`,
			[408, 425, 429, 502, 503, 504].includes(status),
		);
		return;
	}

	if (method === "GET" && contractPath === BACKEND_MOCK_ROUTES.items) {
		handleItems(response, url, requestId, capabilities);
		return;
	}

	if (method === "POST" && contractPath === BACKEND_MOCK_ROUTES.mutation) {
		await handleMutation(
			request,
			response,
			requestId,
			capabilities,
			mutations,
			nextMutationSequence,
		);
		return;
	}

	writeError(
		response,
		501,
		requestId,
		"not_implemented",
		"Mock backend route is not implemented",
		false,
		{
			method,
			path: url.pathname,
		},
	);
}

function handleItems(
	response: ServerResponse,
	url: URL,
	requestId: string,
	capabilities: Capabilities,
): void {
	const requestedLimit = url.searchParams.get("limit");
	const limit = requestedLimit ? Number(requestedLimit) : capabilities.pagination.defaultPageSize;
	if (!Number.isInteger(limit) || limit < 1 || limit > capabilities.pagination.maxPageSize) {
		writeError(
			response,
			400,
			requestId,
			"invalid_pagination",
			"limit is outside the supported range",
		);
		return;
	}

	const cursor = url.searchParams.get("cursor");
	const offset = cursor ? decodeCursor(cursor) : 0;
	if (offset === null) {
		writeError(response, 400, requestId, "invalid_cursor", "cursor is not valid");
		return;
	}

	const totalItems = Math.max(capabilities.pagination.defaultPageSize * 2 + 1, 5);
	const end = Math.min(offset + limit, totalItems);
	const items = Array.from({ length: Math.max(0, end - offset) }, (_, index) => {
		const number = offset + index + 1;
		return { id: `item-${String(number)}`, name: `Mock item ${String(number)}` };
	});
	const hasMore = end < totalItems;
	writeJson(
		response,
		200,
		{
			items,
			page: { nextCursor: hasMore ? encodeCursor(end) : null, hasMore },
		},
		requestId,
	);
}

async function handleMutation(
	request: IncomingMessage,
	response: ServerResponse,
	requestId: string,
	capabilities: Capabilities,
	mutations: Map<string, StoredMutation>,
	nextMutationSequence: () => number,
): Promise<void> {
	const idempotencyKey = headerValue(request, "x-idempotency-key");
	const parsedKey = IdempotencyKeySchema.safeParse(idempotencyKey);
	if (!parsedKey.success) {
		writeError(
			response,
			400,
			requestId,
			"invalid_idempotency_key",
			"A valid x-idempotency-key header is required",
		);
		return;
	}

	const body = await readJsonBody(request, capabilities.http.maxRequestBodyBytes);
	const fingerprint = JSON.stringify(body);
	const stored = mutations.get(parsedKey.data);
	if (stored) {
		if (stored.fingerprint !== fingerprint) {
			writeError(
				response,
				409,
				requestId,
				"idempotency_conflict",
				"Idempotency key was reused with a different request body",
			);
			return;
		}
		writeJson(response, stored.status, stored.body, requestId, {
			"x-idempotency-replayed": "true",
		});
		return;
	}

	const result = { id: `mutation-${String(nextMutationSequence())}`, accepted: true, input: body };
	mutations.set(parsedKey.data, { body: result, fingerprint, status: 201 });
	writeJson(response, 201, result, requestId, { "x-idempotency-replayed": "false" });
}

async function readJsonBody(request: IncomingMessage, maxBytes: number): Promise<unknown> {
	const chunks: Uint8Array[] = [];
	let bytes = 0;
	for await (const chunk of request) {
		const buffer = Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk);
		bytes += buffer.byteLength;
		if (bytes > maxBytes) throw new Error("Request body exceeds manifest maxRequestBodyBytes");
		chunks.push(new Uint8Array(buffer));
	}
	if (chunks.length === 0) return null;
	return JSON.parse(Buffer.concat(chunks).toString("utf8"));
}

function writeError(
	response: ServerResponse,
	status: number,
	requestId: string,
	code: string,
	error: string,
	retryable = false,
	details?: Record<string, unknown>,
): void {
	const body = BackendErrorEnvelopeSchema.parse({
		error,
		code,
		requestId,
		retryable,
		...(details ? { details } : {}),
	});
	writeJson(response, status, body, requestId);
}

function writeJson(
	response: ServerResponse,
	status: number,
	body: unknown,
	requestId: string,
	headers: Record<string, string> = {},
): void {
	response.writeHead(status, { ...JSON_HEADERS, "x-request-id": requestId, ...headers });
	response.end(JSON.stringify(body));
}

function requestIdFor(request: IncomingMessage): string {
	const supplied = headerValue(request, "x-request-id");
	const parsed = RequestIdSchema.safeParse(supplied);
	return parsed.success ? parsed.data : `mock-${randomUUID()}`;
}

function headerValue(request: IncomingMessage, name: string): string | undefined {
	const value = request.headers[name];
	return Array.isArray(value) ? value[0] : value;
}

function encodeCursor(offset: number): string {
	return Buffer.from(String(offset), "utf8").toString("base64url");
}

function decodeCursor(cursor: string): number | null {
	try {
		const value = Buffer.from(cursor, "base64url").toString("utf8");
		const offset = Number(value);
		return Number.isInteger(offset) && offset >= 0 && encodeCursor(offset) === cursor
			? offset
			: null;
	} catch {
		return null;
	}
}

function parsePort(value: string | undefined): number {
	if (!value) return 4010;
	const port = Number(value);
	if (!Number.isInteger(port) || port < 0 || port > 65_535)
		throw new Error("BACKEND_MOCK_PORT must be a valid TCP port");
	return port;
}
