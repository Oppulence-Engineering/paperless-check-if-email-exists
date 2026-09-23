import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Logger.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `logger.lit.ts`.
 */
export const LogLevelSchema = z.enum(["debug", "info", "warn", "error"]);

export type LogLevel = z.infer<typeof LogLevelSchema>;

/**
 * Structured fields only. Prompts, model events, evidence, tokens and customer
 * records never belong in a log line, so the value side stays scalar.
 */
export const LogFieldsSchema = z.record(
	z.string(),
	z.union([z.string(), z.number(), z.boolean(), z.null()]),
);

export type LogFields = z.infer<typeof LogFieldsSchema>;

export const LogRecordSchema = z.object({
	level: LogLevelSchema,
	message: z.string().min(1),
	time: z.string().min(1),
	requestId: z.string().optional(),
	fields: LogFieldsSchema.optional(),
});

export type LogRecord = z.infer<typeof LogRecordSchema>;
