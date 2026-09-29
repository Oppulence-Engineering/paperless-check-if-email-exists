import { LogFieldsSchema, type LogFields, type LogLevel, type LogRecord } from "./logger.schema";

/**
 * @oppulence-gen kind=lib
 * logger is a server-safe observability helper.
 *
 * One line of JSON per event on stdout, which is what a container platform
 * can actually index. `x-request-id` already crosses browser, BFF and backend;
 * passing it here is what makes a single request greppable end to end.
 *
 * This module does not import React or fetch. Inputs are Zod-validated.
 * Owned by `logger.lit.ts`.
 */

const LEVEL_WEIGHT: Record<LogLevel, number> = { debug: 10, info: 20, warn: 30, error: 40 };

function threshold(): number {
	const configured = process.env.LOG_LEVEL?.trim().toLowerCase();
	if (configured && configured in LEVEL_WEIGHT) return LEVEL_WEIGHT[configured as LogLevel];
	return process.env.NODE_ENV === "production" ? LEVEL_WEIGHT.info : LEVEL_WEIGHT.debug;
}

/** Builds the record without writing it, so tests can assert the shape. */
export function buildLogRecord(
	level: LogLevel,
	message: string,
	fields?: LogFields,
	requestId?: string,
): LogRecord {
	return {
		level,
		message,
		time: new Date().toISOString(),
		...(requestId ? { requestId } : {}),
		...(fields && Object.keys(fields).length > 0 ? { fields: LogFieldsSchema.parse(fields) } : {}),
	};
}

function write(record: LogRecord): void {
	if (LEVEL_WEIGHT[record.level] < threshold()) return;
	const line = JSON.stringify(record);
	if (record.level === "error") console.error(line);
	else if (record.level === "warn") console.warn(line);
	else console.log(line);
}

function emit(level: LogLevel, message: string, fields?: LogFields, requestId?: string): void {
	write(buildLogRecord(level, message, fields, requestId));
}

export const log = {
	debug: (message: string, fields?: LogFields) => emit("debug", message, fields),
	info: (message: string, fields?: LogFields) => emit("info", message, fields),
	warn: (message: string, fields?: LogFields) => emit("warn", message, fields),
	error: (message: string, fields?: LogFields) => emit("error", message, fields),
};

/** A logger bound to one request, so every line carries the same id. */
export function requestLogger(requestId: string | null | undefined) {
	const id = requestId?.trim() || undefined;
	return {
		debug: (message: string, fields?: LogFields) => emit("debug", message, fields, id),
		info: (message: string, fields?: LogFields) => emit("info", message, fields, id),
		warn: (message: string, fields?: LogFields) => emit("warn", message, fields, id),
		error: (message: string, fields?: LogFields) => emit("error", message, fields, id),
	};
}

/** Error shape worth logging: message and name, never the caller's payload. */
export function describeError(error: unknown): LogFields {
	if (error instanceof Error) {
		return { name: error.name, message: error.message };
	}
	return { name: "unknown", message: String(error) };
}
