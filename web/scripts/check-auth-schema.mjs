import { readFile } from "node:fs/promises";

import pg from "pg";

const sql = await readFile(new URL("../config/auth/schema.sql", import.meta.url), "utf8");
const expected = new Set([...sql.matchAll(/create table "([^"]+)"/g)].map((match) => match[1]));
const expectedSessionColumns = new Set([
	"stepUpMethod",
	"stepUpPurpose",
	"ssoProviderId",
	"ssoOrganizationId",
]);
const expectedJwksColumns = new Set(["alg", "crv"]);
const expectedSsoColumns = new Set(["domainVerified"]);
const pool = new pg.Pool({ connectionString: process.env.DATABASE_URL });

try {
	const result = await pool.query(
		`select table_name from information_schema.tables
     where table_schema = 'auth' and table_type = 'BASE TABLE'`,
	);
	const actual = new Set(result.rows.map((row) => row.table_name));
	actual.delete("auth_migration");
	const missing = [...expected].filter((table) => !actual.has(table));
	const unexpected = [...actual].filter((table) => !expected.has(table));
	const sessionSecurity = await pool.query(
		`select column_name from information_schema.columns
     where table_schema = 'auth' and table_name = 'session' and column_name = any($1)`,
		[[...expectedSessionColumns]],
	);
	const actualSessionColumns = new Set(sessionSecurity.rows.map((row) => row.column_name));
	const missingSessionColumns = [...expectedSessionColumns].filter(
		(column) => !actualSessionColumns.has(column),
	);
	const jwks = await pool.query(
		`select column_name from information_schema.columns
     where table_schema = 'auth' and table_name = 'jwks' and column_name = any($1)`,
		[[...expectedJwksColumns]],
	);
	const actualJwksColumns = new Set(jwks.rows.map((row) => row.column_name));
	const missingJwksColumns = [...expectedJwksColumns].filter(
		(column) => !actualJwksColumns.has(column),
	);
	const sso = await pool.query(
		`select column_name from information_schema.columns
     where table_schema = 'auth' and table_name = 'ssoProvider' and column_name = any($1)`,
		[[...expectedSsoColumns]],
	);
	const actualSsoColumns = new Set(sso.rows.map((row) => row.column_name));
	const missingSsoColumns = [...expectedSsoColumns].filter(
		(column) => !actualSsoColumns.has(column),
	);
	if (
		missing.length ||
		unexpected.length ||
		missingSessionColumns.length ||
		missingJwksColumns.length ||
		missingSsoColumns.length
	) {
		throw new Error(
			`auth schema drift: missing=[${missing.join(", ")}] unexpected=[${unexpected.join(", ")}] sessionSecurity=[${missingSessionColumns.join(", ")}] jwks=[${missingJwksColumns.join(", ")}] sso=[${missingSsoColumns.join(", ")}]`,
		);
	}
	console.log(`auth schema matches ${expected.size} committed tables`);
} finally {
	await pool.end();
}
