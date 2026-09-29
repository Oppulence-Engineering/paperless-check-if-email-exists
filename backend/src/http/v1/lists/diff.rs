use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::list_intelligence::{classify_change, diff_group_for_change, VerificationSnapshot};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct DiffQuery {
	limit: Option<usize>,
	offset: Option<usize>,
}

#[derive(Debug, Clone, Serialize, utoipa::ToSchema)]
pub struct DiffRow {
	pub canonical_email: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub base_task_id: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compare_task_id: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub base_row_index: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compare_row_index: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub base_score: Option<i16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compare_score: Option<i16>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub base_category: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub compare_category: Option<String>,
	pub change_type: String,
}

#[derive(Debug, Default, Serialize, utoipa::ToSchema)]
pub struct DiffGroup {
	pub count: i64,
	pub rows: Vec<DiffRow>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ListDiffResponse {
	pub base_list_id: i32,
	pub compare_list_id: i32,
	pub added: DiffGroup,
	pub removed: DiffGroup,
	pub unchanged: DiffGroup,
	pub improved: DiffGroup,
	pub degraded: DiffGroup,
	pub newly_invalid: DiffGroup,
	pub newly_risky: DiffGroup,
	pub newly_safe: DiffGroup,
}

#[derive(Debug, Clone)]
struct ListSnapshotRow {
	snapshot: VerificationSnapshot,
	row_index: Option<i32>,
}

async fn http_handler(
	base_list_id: i32,
	compare_list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: DiffQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	ensure_owned_completed_list(&pg_pool, tenant_id, base_list_id).await?;
	ensure_owned_completed_list(&pg_pool, tenant_id, compare_list_id).await?;

	let base = load_list_snapshot(&pg_pool, base_list_id).await?;
	let compare = load_list_snapshot(&pg_pool, compare_list_id).await?;
	let limit = query.limit.unwrap_or(50).min(200);
	let offset = query.offset.unwrap_or(0);

	let mut response = ListDiffResponse {
		base_list_id,
		compare_list_id,
		added: DiffGroup::default(),
		removed: DiffGroup::default(),
		unchanged: DiffGroup::default(),
		improved: DiffGroup::default(),
		degraded: DiffGroup::default(),
		newly_invalid: DiffGroup::default(),
		newly_risky: DiffGroup::default(),
		newly_safe: DiffGroup::default(),
	};

	let mut keys = BTreeSet::new();
	keys.extend(base.keys().cloned());
	keys.extend(compare.keys().cloned());

	for key in keys {
		let base_row = base.get(&key);
		let compare_row = compare.get(&key);
		let (group, row) = match (base_row, compare_row) {
			(None, Some(compare_row)) => (
				"added",
				DiffRow {
					canonical_email: key.clone(),
					base_task_id: None,
					compare_task_id: Some(compare_row.snapshot.task_id),
					base_row_index: None,
					compare_row_index: compare_row.row_index,
					base_score: None,
					compare_score: compare_row.snapshot.score,
					base_category: None,
					compare_category: compare_row.snapshot.category.clone(),
					change_type: "new".to_string(),
				},
			),
			(Some(base_row), None) => (
				"removed",
				DiffRow {
					canonical_email: key.clone(),
					base_task_id: Some(base_row.snapshot.task_id),
					compare_task_id: None,
					base_row_index: base_row.row_index,
					compare_row_index: None,
					base_score: base_row.snapshot.score,
					compare_score: None,
					base_category: base_row.snapshot.category.clone(),
					compare_category: None,
					change_type: "removed".to_string(),
				},
			),
			(Some(base_row), Some(compare_row)) => {
				let change_type = classify_change(Some(&base_row.snapshot), &compare_row.snapshot);
				(
					diff_group_for_change(change_type),
					DiffRow {
						canonical_email: key.clone(),
						base_task_id: Some(base_row.snapshot.task_id),
						compare_task_id: Some(compare_row.snapshot.task_id),
						base_row_index: base_row.row_index,
						compare_row_index: compare_row.row_index,
						base_score: base_row.snapshot.score,
						compare_score: compare_row.snapshot.score,
						base_category: base_row.snapshot.category.clone(),
						compare_category: compare_row.snapshot.category.clone(),
						change_type: change_type.to_string(),
					},
				)
			}
			(None, None) => continue,
		};
		push_group(&mut response, group, row, limit, offset);
	}

	Ok(warp::reply::json(&response))
}

fn push_group(
	response: &mut ListDiffResponse,
	group: &str,
	row: DiffRow,
	limit: usize,
	offset: usize,
) {
	let target = match group {
		"added" => &mut response.added,
		"removed" => &mut response.removed,
		"improved" => &mut response.improved,
		"degraded" => &mut response.degraded,
		"newly_invalid" => &mut response.newly_invalid,
		"newly_risky" => &mut response.newly_risky,
		"newly_safe" => &mut response.newly_safe,
		_ => &mut response.unchanged,
	};
	let current_index = target.count as usize;
	target.count += 1;
	if current_index >= offset && target.rows.len() < limit {
		target.rows.push(row);
	}
}

async fn ensure_owned_completed_list(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
) -> Result<(), warp::Rejection> {
	let row = sqlx::query(
		r#"
		SELECT total_rows, status::TEXT AS status
		FROM v1_lists
		WHERE id = $1 AND tenant_id = $2
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let row = row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"List not found",
		))
	})?;
	if row.get::<String, _>("status") == "deleted" {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "List not found").into());
	}
	let processed: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_task_result WHERE (extra->>'list_id')::INTEGER = $1 AND (result IS NOT NULL OR error IS NOT NULL OR task_state = 'cancelled')",
	)
	.bind(list_id)
	.fetch_one(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	if processed < i64::from(row.get::<i32, _>("total_rows")) {
		return Err(
			ReacherResponseError::new(StatusCode::BAD_REQUEST, "List is still processing").into(),
		);
	}
	Ok(())
}

async fn load_list_snapshot(
	pg_pool: &PgPool,
	list_id: i32,
) -> Result<BTreeMap<String, ListSnapshotRow>, warp::Rejection> {
	let rows = sqlx::query(
		r#"
		SELECT DISTINCT ON (comparison_key)
			id,
			tenant_id,
			comparison_key,
			(extra->>'row_index')::INTEGER AS row_index,
			(extra->>'list_id')::INTEGER AS list_id,
			(extra->>'pipeline_run_id')::BIGINT AS pipeline_run_id,
			score,
			score_category,
			safe_to_send,
			reason_codes,
			completed_at
		FROM (
			SELECT *,
			       COALESCE(canonical_email, LOWER(COALESCE(result->>'input', payload->'input'->>'to_email'))) AS comparison_key
			FROM v1_task_result
			WHERE (extra->>'list_id')::INTEGER = $1
			  AND task_state = 'completed'::task_state
			  AND result IS NOT NULL
			  AND completed_at IS NOT NULL
		) rows
		WHERE comparison_key IS NOT NULL AND comparison_key <> ''
		ORDER BY comparison_key, is_duplicate ASC, id ASC
		"#,
	)
	.bind(list_id)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let mut snapshot = BTreeMap::new();
	for row in rows {
		let key: String = row.get("comparison_key");
		snapshot.insert(
			key.clone(),
			ListSnapshotRow {
				row_index: row.get("row_index"),
				snapshot: VerificationSnapshot {
					task_id: row.get("id"),
					tenant_id: row.get("tenant_id"),
					canonical_email: key,
					list_id: row.get("list_id"),
					pipeline_run_id: row.get("pipeline_run_id"),
					score: row.get("score"),
					category: row.get("score_category"),
					safe_to_send: row.get("safe_to_send"),
					reason_codes: row.get("reason_codes"),
					completed_at: row
						.get::<Option<DateTime<Utc>>, _>("completed_at")
						.unwrap_or_else(Utc::now),
				},
			},
		);
	}
	Ok(snapshot)
}

/// GET /v1/lists/{base_list_id}/diff/{compare_list_id}
#[utoipa::path(
	get,
	path = "/v1/lists/{base_list_id}/diff/{compare_list_id}",
	tag = "Lists",
	params(
		("base_list_id" = i32, Path, description = "Base list identifier"),
		("compare_list_id" = i32, Path, description = "Compare list identifier"),
		DiffQuery
	),
	responses((status = 200, description = "List diff", body = ListDiffResponse))
)]
pub fn v1_diff_lists(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "diff" / i32)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<DiffQuery>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
