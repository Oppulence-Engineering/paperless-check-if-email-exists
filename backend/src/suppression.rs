// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

//! Shared writer for active suppression rows.
//!
//! Campaign outcome ingest and worker auto-actions both create suppressions
//! outside the HTTP suppression API. They must use the same identity as that
//! API: `(tenant_id, canonical_email)` where `status = 'active'`.
//! `20260630000003_suppression_intelligence_indexes` dropped the older
//! `(tenant_id, email)` unique constraint, and `canonical_email` is NOT NULL.

use crate::http::v1::lists::canonicalize::canonicalize_email;
use sqlx::{Acquire, PgConnection, PgPool};
use uuid::Uuid;

/// The `canonical_email` of a suppression: the same key that list
/// deduplication, outcomes and scoring use. Gmail ignores dots and `+tags`,
/// so every spelling of one Gmail mailbox shares one suppression. Every
/// writer and reader of `v1_suppression_entries.canonical_email` must use it.
pub fn suppression_key(email: &str) -> String {
	canonicalize_email(email).unwrap_or_else(|| email.trim().to_lowercase())
}

/// Whether `email` has an active, unexpired suppression for the tenant.
///
/// Entries are keyed by [`suppression_key`]. The lowercased spelling is
/// checked too, for rows written before every writer used that key.
pub async fn has_active_suppression(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	email: &str,
) -> Result<bool, sqlx::Error> {
	let lowered = email.trim().to_lowercase();
	let canonical = suppression_key(email);
	sqlx::query_scalar::<_, bool>(
		r#"
		SELECT EXISTS (
			SELECT 1
			FROM v1_suppression_entries
			WHERE tenant_id = $1
			  AND canonical_email IN ($2, $3)
			  AND status = 'active'
			  AND (expires_at IS NULL OR expires_at > NOW())
		)
		"#,
	)
	.bind(tenant_id)
	.bind(&lowered)
	.bind(&canonical)
	.fetch_one(pg_pool)
	.await
}

/// Insert one active suppression and its `created` audit event.
///
/// Returns `Ok(false)` when an active row for this address already exists.
/// The existing row is left unchanged so a second reason (for example
/// unsubscribe after complaint) does not overwrite the first. An active row
/// whose `expires_at` has passed no longer suppresses anything but still
/// holds the unique index, so it is made permanent again instead.
///
/// Runs on `conn`, so a caller inside a transaction can commit the
/// suppression together with whatever caused it.
pub async fn insert_active_suppression(
	conn: &mut PgConnection,
	tenant_id: Uuid,
	email: &str,
	reason: &str,
	source: &str,
	source_type: &str,
	actor: &str,
) -> Result<bool, sqlx::Error> {
	let email = email.trim().to_lowercase();
	let canonical_email = suppression_key(&email);
	let canonical_email = canonical_email.as_str();
	// The entry and its audit event commit together. Otherwise a failed event
	// insert leaves an entry that a retry skips (`DO NOTHING`), and the audit
	// event is never written.
	let mut tx = conn.begin().await?;

	// `reason` is bound twice: once for the enum column and once for the text
	// reason code. A single placeholder cannot satisfy both types.
	let entry_id = sqlx::query_scalar::<_, i32>(
		r#"
		INSERT INTO v1_suppression_entries (
			tenant_id, email, canonical_email, status, reason, source,
			reason_code, source_type, created_by, last_seen_at
		)
		VALUES (
			$1, $8, $2, 'active', $3::suppression_reason, $4,
			$5, $6, $7, NOW()
		)
		ON CONFLICT (tenant_id, canonical_email) WHERE status = 'active'
		DO UPDATE SET
			reason = EXCLUDED.reason,
			source = EXCLUDED.source,
			reason_code = EXCLUDED.reason_code,
			source_type = EXCLUDED.source_type,
			created_by = EXCLUDED.created_by,
			expires_at = NULL,
			last_seen_at = NOW(),
			updated_at = NOW()
		WHERE v1_suppression_entries.expires_at IS NOT NULL
			AND v1_suppression_entries.expires_at <= NOW()
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(canonical_email)
	.bind(reason)
	.bind(source)
	.bind(reason)
	.bind(source_type)
	.bind(actor)
	.bind(&email)
	.fetch_optional(&mut *tx)
	.await?;

	let Some(entry_id) = entry_id else {
		return Ok(false);
	};

	sqlx::query(
		r#"
		INSERT INTO v1_suppression_events (
			tenant_id, entry_id, canonical_email, event_type, to_status,
			reason_code, source_type, source_ref, actor_type, actor_id
		)
		VALUES ($1, $2, $3, 'created', 'active', $4, $5, $6, $7, $7)
		"#,
	)
	.bind(tenant_id)
	.bind(entry_id)
	.bind(canonical_email)
	.bind(reason)
	.bind(source_type)
	.bind(source)
	.bind(actor)
	.execute(&mut *tx)
	.await?;

	tx.commit().await?;
	Ok(true)
}
