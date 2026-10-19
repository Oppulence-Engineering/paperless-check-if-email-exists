import { createHash } from "node:crypto";
import { readdir, readFile } from "node:fs/promises";

import pg from "pg";

const migrationDirectory = new URL("../config/auth/migrations/", import.meta.url);
const migrationFiles = (await readdir(migrationDirectory))
	.filter((filename) => /^\d+_[a-z0-9_]+\.sql$/.test(filename))
	.sort();
const migrations = await Promise.all(
	migrationFiles.map(async (filename) => {
		const sql = await readFile(new URL(filename, migrationDirectory), "utf8");
		return {
			id: filename.replace(/\.sql$/, ""),
			sql,
			checksum: createHash("sha256").update(sql).digest("hex"),
		};
	}),
);
const pool = new pg.Pool({
	connectionString: process.env.AUTH_MIGRATION_DATABASE_URL || process.env.DATABASE_URL,
});
const client = await pool.connect();

try {
	await client.query("begin");
	await client.query(
		"select pg_advisory_xact_lock(hashtext('check-if-email-exists-auth-migrations'))",
	);
	await client.query("create schema if not exists auth");
	await client.query("set local search_path=auth,pg_catalog");
	await client.query(`create table if not exists auth_migration (
    id text primary key,
    checksum text not null,
    "appliedAt" timestamptz default CURRENT_TIMESTAMP not null
  )`);
	let appliedCount = 0;
	for (const migration of migrations) {
		const applied = await client.query("select checksum from auth_migration where id = $1", [
			migration.id,
		]);
		if (
			applied.rows[0]?.checksum !== undefined &&
			applied.rows[0].checksum !== migration.checksum
		) {
			throw new Error(`${migration.id} changed after it was applied`);
		}
		if (applied.rowCount === 0) {
			await client.query(migration.sql);
			await client.query("insert into auth_migration (id, checksum) values ($1, $2)", [
				migration.id,
				migration.checksum,
			]);
			appliedCount += 1;
		}
	}
	await client.query("commit");
	console.log(
		appliedCount === 0
			? `${migrations.length} auth migrations already applied`
			: `applied ${appliedCount} auth migration(s)`,
	);
} catch (error) {
	await client.query("rollback");
	throw error;
} finally {
	client.release();
	await pool.end();
}
