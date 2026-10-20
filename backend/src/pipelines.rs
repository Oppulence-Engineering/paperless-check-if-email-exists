use crate::config::{BackendConfig, PipelinesConfig};
use crate::http::v1::bulk::post::publish_task;
use crate::http::v1::lists::canonicalize::canonicalize_email;
use crate::http::CheckEmailRequest;
use crate::tenant::auth::resolve_tenant_context_by_id;
use crate::tenant::quota::release_reserved_usage;
use crate::tenant::webhook::{sign_payload, WEBHOOK_SIGNATURE_HEADER};
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskMetadata};
use anyhow::{bail, Context, Result};
use check_if_email_exists::network::is_private_ip;
use check_if_email_exists::{Reachable, LOG_TARGET};
use chrono::{DateTime, Duration, Utc};
use chrono_tz::Tz;
use cron::Schedule;
use lapin::BasicProperties;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use tokio::time::Duration as TokioDuration;
use tracing::{info, warn};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "pipeline_status", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum PipelineStatus {
	Active,
	Paused,
	Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "pipeline_source_type", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum PipelineSourceType {
	ListSnapshot,
	Integration,
	Push,
	Bucket,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "pipeline_run_status", rename_all = "lowercase")]
#[serde(rename_all = "snake_case")]
pub enum PipelineRunStatus {
	Queued,
	Preparing,
	FetchingSource,
	Publishing,
	Running,
	Delivering,
	Completed,
	Failed,
	Cancelled,
	Skipped,
}

impl PipelineRunStatus {
	pub fn is_terminal(&self) -> bool {
		matches!(
			self,
			Self::Completed | Self::Failed | Self::Cancelled | Self::Skipped
		)
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type, ToSchema)]
#[sqlx(type_name = "pipeline_delivery_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum PipelineDeliveryStatus {
	NotRequested,
	Pending,
	Delivered,
	RetryScheduled,
	Failed,
}

#[derive(Debug, thiserror::Error)]
pub enum PipelineRequestError {
	#[error("{0}")]
	Validation(String),
	#[error("{0}")]
	NotFound(String),
	#[error("{0}")]
	Conflict(String),
	#[error(transparent)]
	Internal(#[from] anyhow::Error),
}

impl PipelineRequestError {
	fn validation(message: impl Into<String>) -> Self {
		Self::Validation(message.into())
	}

	fn not_found(message: impl Into<String>) -> Self {
		Self::NotFound(message.into())
	}

	fn conflict(message: impl Into<String>) -> Self {
		Self::Conflict(message.into())
	}
}

impl From<sqlx::Error> for PipelineRequestError {
	fn from(err: sqlx::Error) -> Self {
		Self::Internal(err.into())
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PipelineSource {
	ListSnapshot {
		list_id: i32,
	},
	Integration {
		provider: String,
		connection_id: String,
		audience_id: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		field_mapping: Option<Value>,
	},
	Push {
		token_id: String,
		accepted_format: String,
	},
	Bucket {
		provider: String,
		bucket: String,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		prefix: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		region: Option<String>,
		#[serde(default, skip_serializing_if = "Option::is_none")]
		path_pattern: Option<String>,
	},
}

impl PipelineSource {
	pub fn source_type(&self) -> PipelineSourceType {
		match self {
			Self::ListSnapshot { .. } => PipelineSourceType::ListSnapshot,
			Self::Integration { .. } => PipelineSourceType::Integration,
			Self::Push { .. } => PipelineSourceType::Push,
			Self::Bucket { .. } => PipelineSourceType::Bucket,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineSchedule {
	pub cron: String,
	pub timezone: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct PipelineVerificationSettings {
	#[serde(default)]
	pub delta_mode: bool,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub freshness_days: Option<i32>,
}

fn default_missed_run_window_hours() -> i32 {
	24
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelinePolicyConfig {
	#[serde(default = "default_missed_run_window_hours")]
	pub missed_run_window_hours: i32,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub policy_id: Option<i64>,
}

impl Default for PipelinePolicyConfig {
	fn default() -> Self {
		Self {
			missed_run_window_hours: default_missed_run_window_hours(),
			policy_id: None,
		}
	}
}

fn default_dashboard_delivery() -> bool {
	true
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct PipelineDeliveryWebhook {
	pub url: String,
	#[serde(default)]
	pub headers: HashMap<String, String>,
}

fn default_delivery_max_attempts() -> i32 {
	5
}

fn default_delivery_retry_backoff_seconds() -> i32 {
	300
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineDeliveryConfig {
	#[serde(default = "default_dashboard_delivery")]
	pub dashboard: bool,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub webhook: Option<PipelineDeliveryWebhook>,
	#[serde(default = "default_delivery_max_attempts")]
	pub max_attempts: i32,
	#[serde(default = "default_delivery_retry_backoff_seconds")]
	pub retry_backoff_seconds: i32,
}

impl Default for PipelineDeliveryConfig {
	fn default() -> Self {
		Self {
			dashboard: default_dashboard_delivery(),
			webhook: None,
			max_attempts: default_delivery_max_attempts(),
			retry_backoff_seconds: default_delivery_retry_backoff_seconds(),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineView {
	pub id: i64,
	pub tenant_id: Uuid,
	pub name: String,
	pub status: PipelineStatus,
	pub source: PipelineSource,
	pub schedule: PipelineSchedule,
	pub verification: PipelineVerificationSettings,
	pub policy: PipelinePolicyConfig,
	pub delivery: PipelineDeliveryConfig,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub next_run_at: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_scheduled_at: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_run_id: Option<i64>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PipelineRunView {
	pub id: i64,
	pub pipeline_id: i64,
	pub tenant_id: Uuid,
	pub trigger_type: String,
	pub status: PipelineRunStatus,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scheduled_for: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub started_at: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub completed_at: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub job_id: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub list_id: Option<i32>,
	pub source_snapshot: Value,
	pub stats: Value,
	pub billed_emails: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result_location: Option<Value>,
	pub delivery_status: PipelineDeliveryStatus,
	pub delivery_attempts: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_delivery_attempt_at: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub next_delivery_attempt_at: Option<DateTime<Utc>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub delivery_error: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error_code: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error_message: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreatePipelineInput {
	pub name: String,
	pub source: PipelineSource,
	pub schedule: PipelineSchedule,
	#[serde(default)]
	pub verification: PipelineVerificationSettings,
	#[serde(default)]
	pub policy: PipelinePolicyConfig,
	#[serde(default)]
	pub delivery: PipelineDeliveryConfig,
	#[serde(default = "default_pipeline_status")]
	pub status: PipelineStatus,
}

fn default_pipeline_status() -> PipelineStatus {
	PipelineStatus::Active
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UpdatePipelineInput {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub source: Option<PipelineSource>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub schedule: Option<PipelineSchedule>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub verification: Option<PipelineVerificationSettings>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub policy: Option<PipelinePolicyConfig>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub delivery: Option<PipelineDeliveryConfig>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub status: Option<PipelineStatus>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct TriggerPipelineInput {
	#[serde(default)]
	pub force: bool,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct TriggerPipelineResponse {
	pub run_id: i64,
	pub status: PipelineRunStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PushPipelineInput {
	pub rows: Vec<Map<String, Value>>,
	#[serde(default = "default_push_email_column")]
	pub email_column: String,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub source_key: Option<String>,
}

fn default_push_email_column() -> String {
	"email".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PushPipelineResponse {
	pub batch_id: i64,
	pub run_id: i64,
	pub status: PipelineRunStatus,
	pub accepted_rows: i32,
	pub replayed: bool,
}

#[derive(Debug, Clone)]
pub struct PipelineListQuery {
	pub status: Option<PipelineStatus>,
	pub limit: i64,
	pub offset: i64,
}

#[derive(Debug)]
struct PreparedList {
	source_list_id: Option<i32>,
	headers: Vec<String>,
	email_column: String,
	rows: Vec<Map<String, Value>>,
	source_filename: String,
	source_name: String,
	unique_email_count: i32,
	deduplicated_count: i32,
	canonical_groups: HashMap<String, Vec<usize>>,
	blank_indices: Vec<usize>,
	invalid_indices: Vec<usize>,
}

#[derive(Debug, Default)]
struct DeltaSelectionSummary {
	selected_unique_emails: i32,
	skipped_unchanged: i32,
}

#[derive(Debug)]
struct PipelineContactStateRow {
	canonical_email: String,
	source_hash: String,
	last_verified_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
struct ClaimedRun {
	run_id: i64,
	pipeline_id: i64,
	skipped: bool,
}

pub fn spawn_pipeline_scheduler(config: Arc<BackendConfig>, pg_pool: PgPool) {
	let tick_seconds = config.pipelines.tick_seconds.max(1);
	tokio::spawn(async move {
		loop {
			if let Err(err) = run_pipeline_scheduler_cycle(Arc::clone(&config), &pg_pool).await {
				warn!(target: LOG_TARGET, error = ?err, "Pipeline scheduler cycle failed");
			}
			tokio::time::sleep(TokioDuration::from_secs(tick_seconds)).await;
		}
	});
}

pub async fn run_pipeline_scheduler_cycle(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
) -> Result<()> {
	if !config.pipelines.enable {
		return Ok(());
	}

	recover_stranded_pending_deliveries(pg_pool).await?;
	retry_due_pipeline_deliveries(pg_pool, config.pipelines.max_due_per_tick).await?;
	crate::smtp_egress::require_smtp_ready(&config)?;

	recover_stalled_pipeline_runs(config.as_ref(), pg_pool, config.pipelines.max_due_per_tick)
		.await?;

	let recovered =
		recover_stranded_queued_pipeline_runs(pg_pool, config.pipelines.max_due_per_tick).await?;
	if !recovered.is_empty() {
		info!(
			target: LOG_TARGET,
			count = recovered.len(),
			"Recovered stranded queued pipeline runs"
		);

		for run in recovered {
			spawn_pipeline_run_execution(Arc::clone(&config), pg_pool.clone(), run);
		}
	}

	let claimed = claim_due_pipeline_runs(&config.pipelines, pg_pool).await?;
	if !claimed.is_empty() {
		info!(target: LOG_TARGET, count = claimed.len(), "Claimed due pipeline runs");

		for run in claimed {
			if run.skipped {
				continue;
			}
			spawn_pipeline_run_execution(Arc::clone(&config), pg_pool.clone(), run);
		}
	}

	Ok(())
}

fn spawn_pipeline_run_execution(config: Arc<BackendConfig>, pg_pool: PgPool, run: ClaimedRun) {
	tokio::spawn(async move {
		if let Err(err) = execute_pipeline_run(config, &pg_pool, run.run_id).await {
			warn!(
				target: LOG_TARGET,
				pipeline_id = run.pipeline_id,
				run_id = run.run_id,
				error = ?err,
				"Failed to execute pipeline run"
			);
		}
	});
}

async fn pause_pipeline_for_invalid_schedule<'a>(
	tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
	pipeline: &PipelineRowForUpdate,
	scheduled_for: DateTime<Utc>,
	error_message: &str,
) -> Result<ClaimedRun> {
	let run_id: i64 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_pipeline_runs (
			pipeline_id, tenant_id, trigger_type, status, scheduled_for,
			completed_at, source_snapshot, stats, error_code, error_message
		)
		VALUES ($1, $2, 'schedule', 'skipped', $3, NOW(), $4, '{}'::jsonb, $5, $6)
		RETURNING id
		"#,
	)
	.bind(pipeline.id)
	.bind(pipeline.tenant_id)
	.bind(scheduled_for)
	.bind(&pipeline.source_config)
	.bind("invalid_schedule")
	.bind(error_message)
	.fetch_one(&mut **tx)
	.await?;

	sqlx::query(
		"UPDATE v1_pipelines SET status = 'paused'::pipeline_status, next_run_at = NULL, last_scheduled_at = NOW(), last_run_id = $2, updated_at = NOW() WHERE id = $1",
	)
	.bind(pipeline.id)
	.bind(run_id)
	.execute(&mut **tx)
	.await?;

	Ok(ClaimedRun {
		run_id,
		pipeline_id: pipeline.id,
		skipped: true,
	})
}

async fn claim_due_pipeline_runs(
	pipeline_config: &PipelinesConfig,
	pg_pool: &PgPool,
) -> Result<Vec<ClaimedRun>> {
	let mut tx = pg_pool.begin().await?;
	let rows = sqlx::query(
		r#"
		SELECT p.id, p.next_run_at
		FROM v1_pipelines p
		INNER JOIN tenants t ON t.id = p.tenant_id
		WHERE p.status = 'active'::pipeline_status
		  AND p.deleted_at IS NULL
		  AND p.next_run_at IS NOT NULL
		  AND p.next_run_at <= NOW()
		  AND t.status = 'active'::tenant_status
		ORDER BY p.next_run_at ASC
		LIMIT $1
		FOR UPDATE SKIP LOCKED
		"#,
	)
	.bind(pipeline_config.max_due_per_tick)
	.fetch_all(&mut *tx)
	.await?;

	let mut claimed = Vec::new();
	for row in rows {
		let pipeline_id: i64 = row.get("id");
		let next_run_at: DateTime<Utc> = row.get("next_run_at");
		let Some(pipeline) = fetch_pipeline_row_for_update(&mut tx, pipeline_id).await? else {
			continue;
		};
		let policy: PipelinePolicyConfig =
			serde_json::from_value(pipeline.policy_config.clone()).unwrap_or_default();
		let effective_missed_run_window_hours = policy
			.missed_run_window_hours
			.min(pipeline_config.max_missed_run_age_hours.max(1));
		let next_future_run = match compute_next_run_at(
			&pipeline.schedule_cron,
			&pipeline.schedule_timezone,
			Utc::now(),
		) {
			Ok(next_future_run) => next_future_run,
			Err(err) => {
				warn!(
					target: LOG_TARGET,
					pipeline_id = pipeline.id,
					error = ?err,
					"Pausing pipeline with invalid schedule during due-run claim"
				);
				claimed.push(
					pause_pipeline_for_invalid_schedule(
						&mut tx,
						&pipeline,
						next_run_at,
						&format!(
							"Pipeline paused because schedule validation failed during scheduler claim: {err}"
						),
					)
					.await?,
				);
				continue;
			}
		};
		if has_active_run(&mut tx, pipeline.id).await? {
			let run_id: i64 = sqlx::query_scalar(
				r#"
				INSERT INTO v1_pipeline_runs (
					pipeline_id, tenant_id, trigger_type, status, scheduled_for,
					completed_at, source_snapshot, stats, error_code, error_message
				)
				VALUES ($1, $2, 'schedule', 'skipped', $3, NOW(), $4, '{}'::jsonb, $5, $6)
				RETURNING id
				"#,
			)
			.bind(pipeline.id)
			.bind(pipeline.tenant_id)
			.bind(next_run_at)
			.bind(&pipeline.source_config)
			.bind("active_run_in_progress")
			.bind("Scheduled run skipped because a previous pipeline run is still active")
			.fetch_one(&mut *tx)
			.await?;

			sqlx::query(
				"UPDATE v1_pipelines SET next_run_at = $2, last_scheduled_at = NOW(), last_run_id = $3, updated_at = NOW() WHERE id = $1",
			)
			.bind(pipeline.id)
			.bind(next_future_run)
			.bind(run_id)
			.execute(&mut *tx)
			.await?;

			claimed.push(ClaimedRun {
				run_id,
				pipeline_id: pipeline.id,
				skipped: true,
			});
			continue;
		}
		if Utc::now() - next_run_at > Duration::hours(i64::from(effective_missed_run_window_hours))
		{
			let run_id: i64 = sqlx::query_scalar(
				r#"
				INSERT INTO v1_pipeline_runs (
					pipeline_id, tenant_id, trigger_type, status, scheduled_for,
					completed_at, source_snapshot, stats, error_code, error_message
				)
				VALUES ($1, $2, 'schedule', 'skipped', $3, NOW(), $4, '{}'::jsonb, $5, $6)
				RETURNING id
				"#,
			)
			.bind(pipeline.id)
			.bind(pipeline.tenant_id)
			.bind(next_run_at)
			.bind(&pipeline.source_config)
			.bind("missed_schedule_window")
			.bind("Scheduled run skipped because it exceeded the missed-run recovery window")
			.fetch_one(&mut *tx)
			.await?;

			sqlx::query(
				"UPDATE v1_pipelines SET next_run_at = $2, last_scheduled_at = NOW(), last_run_id = $3, updated_at = NOW() WHERE id = $1",
			)
			.bind(pipeline.id)
			.bind(next_future_run)
			.bind(run_id)
			.execute(&mut *tx)
			.await?;

			claimed.push(ClaimedRun {
				run_id,
				pipeline_id: pipeline.id,
				skipped: true,
			});
			continue;
		}

		let run_id: i64 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_pipeline_runs (
				pipeline_id, tenant_id, trigger_type, status, scheduled_for,
				source_snapshot, stats
			)
			VALUES ($1, $2, 'schedule', 'queued', $3, $4, '{}'::jsonb)
			RETURNING id
			"#,
		)
		.bind(pipeline.id)
		.bind(pipeline.tenant_id)
		.bind(next_run_at)
		.bind(&pipeline.source_config)
		.fetch_one(&mut *tx)
		.await?;

		sqlx::query(
			"UPDATE v1_pipelines SET next_run_at = $2, last_scheduled_at = NOW(), last_run_id = $3, updated_at = NOW() WHERE id = $1",
		)
		.bind(pipeline.id)
		.bind(next_future_run)
		.bind(run_id)
		.execute(&mut *tx)
		.await?;

		claimed.push(ClaimedRun {
			run_id,
			pipeline_id: pipeline.id,
			skipped: false,
		});
	}

	tx.commit().await?;
	Ok(claimed)
}

async fn recover_stranded_queued_pipeline_runs(
	pg_pool: &PgPool,
	limit: i32,
) -> Result<Vec<ClaimedRun>> {
	let rows = sqlx::query(
		r#"
		WITH ranked_candidates AS (
			SELECT
				pr.id,
				pr.pipeline_id,
				ROW_NUMBER() OVER (PARTITION BY pr.pipeline_id ORDER BY pr.created_at ASC, pr.id ASC) AS row_num
			FROM v1_pipeline_runs pr
			INNER JOIN v1_pipelines p ON p.id = pr.pipeline_id
			INNER JOIN tenants t ON t.id = pr.tenant_id
			WHERE pr.status = 'queued'::pipeline_run_status
			  AND pr.started_at IS NULL
			  AND pr.updated_at < NOW() - INTERVAL '5 minutes'
			  AND p.deleted_at IS NULL
			  AND t.status = 'active'::tenant_status
			  AND NOT EXISTS (
				SELECT 1
				FROM v1_pipeline_runs sibling
				WHERE sibling.pipeline_id = pr.pipeline_id
				  AND sibling.id <> pr.id
				  AND sibling.status IN (
					'preparing'::pipeline_run_status,
					'fetching_source'::pipeline_run_status,
					'publishing'::pipeline_run_status,
					'running'::pipeline_run_status,
					'delivering'::pipeline_run_status
				  )
			  )
		),
		candidates AS (
			SELECT pr.id, pr.pipeline_id
			FROM v1_pipeline_runs pr
			INNER JOIN ranked_candidates ranked ON ranked.id = pr.id
			WHERE ranked.row_num = 1
			ORDER BY pr.id ASC
			LIMIT $1
			FOR UPDATE OF pr SKIP LOCKED
		)
		UPDATE v1_pipeline_runs pr
		SET updated_at = NOW()
		FROM candidates c
		WHERE pr.id = c.id
		RETURNING pr.id, c.pipeline_id
		"#,
	)
	.bind(limit)
	.fetch_all(pg_pool)
	.await?;

	Ok(rows
		.into_iter()
		.map(|row| ClaimedRun {
			run_id: row.get("id"),
			pipeline_id: row.get("pipeline_id"),
			skipped: false,
		})
		.collect())
}

/// Finishes or fails runs that stopped making progress.
///
/// Runs in 'preparing', 'fetching_source' or 'publishing' belong to an
/// executor; if its process died, nothing else moves them on, and an active run
/// makes the scheduler skip every later run of the pipeline. Execution touches
/// the run at each phase and does a few statements per row, so 30 quiet minutes
/// means the executor is gone. Runs left in 'delivering', or in 'running' after
/// their job finished, missed finalization and get it retried; a run still not
/// finished after that, other than a running one, is marked failed.
async fn recover_stalled_pipeline_runs(
	config: &BackendConfig,
	pg_pool: &PgPool,
	limit: i32,
) -> Result<()> {
	// Bumping updated_at claims the rows, so another scheduler instance does not
	// pick the same runs for the next 30 minutes.
	let rows = sqlx::query(
		r#"
		WITH stalled AS (
			SELECT pr.id
			FROM v1_pipeline_runs pr
			LEFT JOIN v1_bulk_job j ON j.id = pr.job_id
			WHERE pr.updated_at < NOW() - INTERVAL '30 minutes'
			  AND (
				pr.status IN (
					'preparing'::pipeline_run_status,
					'fetching_source'::pipeline_run_status,
					'publishing'::pipeline_run_status,
					'delivering'::pipeline_run_status
				)
				OR (
					pr.status = 'running'::pipeline_run_status
					AND j.status IN ('completed'::job_state, 'failed'::job_state, 'cancelled'::job_state)
				)
			  )
			ORDER BY pr.updated_at ASC
			LIMIT $1
			FOR UPDATE OF pr SKIP LOCKED
		)
		UPDATE v1_pipeline_runs pr
		SET updated_at = NOW()
		FROM stalled
		WHERE pr.id = stalled.id
		RETURNING pr.id, pr.status::TEXT
		"#,
	)
	.bind(limit.max(1))
	.fetch_all(pg_pool)
	.await?;

	for row in rows {
		let run_id: i64 = row.get("id");
		let status = parse_pipeline_run_status(&row.get::<String, _>("status"))?;
		if matches!(
			status,
			PipelineRunStatus::Delivering | PipelineRunStatus::Running
		) {
			if let Err(err) = maybe_finalize_pipeline_run(config, pg_pool, run_id).await {
				warn!(target: LOG_TARGET, run_id = run_id, error = ?err, "Retrying stalled pipeline run finalization failed");
			}
			if status == PipelineRunStatus::Running {
				continue;
			}
		}

		warn!(target: LOG_TARGET, run_id = run_id, status = ?status, "Failing stalled pipeline run");
		let err = anyhow::anyhow!(
			"Pipeline run made no progress for 30 minutes while {}",
			enum_str(&status)
		);
		// Does nothing if the retried finalization above finished the run.
		if let Err(mark_err) =
			mark_pipeline_run_execution_failed(pg_pool, run_id, "stalled", &err).await
		{
			warn!(target: LOG_TARGET, run_id = run_id, error = ?mark_err, "Failed to mark stalled pipeline run as failed");
		}
	}
	Ok(())
}

/// Recovers deliveries stranded in 'pending' status (e.g., after a process crash).
/// Resets them to 'retry_scheduled' so the normal retry loop picks them up.
async fn recover_stranded_pending_deliveries(pg_pool: &PgPool) -> Result<()> {
	let result = sqlx::query(
		r#"
		UPDATE v1_pipeline_runs
		SET delivery_status = 'retry_scheduled'::pipeline_delivery_status,
		    next_delivery_attempt_at = NOW(),
		    updated_at = NOW()
		WHERE delivery_status = 'pending'::pipeline_delivery_status
		  AND updated_at < NOW() - INTERVAL '5 minutes'
		"#,
	)
	.execute(pg_pool)
	.await?;
	if result.rows_affected() > 0 {
		warn!(
			target: LOG_TARGET,
			count = result.rows_affected(),
			"Recovered stranded pending deliveries"
		);
	}
	Ok(())
}

async fn retry_due_pipeline_deliveries(pg_pool: &PgPool, limit: i32) -> Result<()> {
	let mut tx = pg_pool.begin().await?;
	let rows = sqlx::query(
		r#"
		SELECT id
		FROM v1_pipeline_runs
		WHERE (
			delivery_status = 'retry_scheduled'::pipeline_delivery_status
			AND next_delivery_attempt_at IS NOT NULL
			AND next_delivery_attempt_at <= NOW()
		) OR (
			delivery_status = 'pending'::pipeline_delivery_status
			AND next_delivery_attempt_at IS NOT NULL
			AND next_delivery_attempt_at <= NOW()
		)
		ORDER BY next_delivery_attempt_at ASC
		LIMIT $1
		FOR UPDATE SKIP LOCKED
		"#,
	)
	.bind(limit.max(1))
	.fetch_all(&mut *tx)
	.await?;

	let mut run_ids = Vec::with_capacity(rows.len());
	for row in rows {
		let run_id: i64 = row.get("id");
		sqlx::query(
			r#"
			UPDATE v1_pipeline_runs
			SET delivery_status = 'pending'::pipeline_delivery_status,
			    next_delivery_attempt_at = NULL,
			    updated_at = NOW()
			WHERE id = $1
			"#,
		)
		.bind(run_id)
		.execute(&mut *tx)
		.await?;
		run_ids.push(run_id);
	}
	tx.commit().await?;

	for run_id in run_ids {
		if let Err(err) = attempt_pipeline_run_delivery(pg_pool, run_id, false).await {
			warn!(target: LOG_TARGET, run_id = run_id, error = ?err, "Retrying pipeline delivery failed");
		}
	}

	Ok(())
}

pub async fn create_pipeline(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	input: CreatePipelineInput,
	pipeline_config: &PipelinesConfig,
) -> Result<PipelineView> {
	if input.status == PipelineStatus::Deleted {
		return Err(PipelineRequestError::validation(
			"Use the delete endpoint to delete a pipeline",
		)
		.into());
	}

	validate_pipeline_input(
		&mut *pg_pool.acquire().await?,
		tenant_id,
		&input.source,
		&input.schedule,
		&input.policy,
		&input.delivery,
		pipeline_config.min_interval_seconds,
	)
	.await?;

	let next_run_at = match (&input.source, &input.status) {
		(PipelineSource::Push { .. }, _) => None,
		(_, PipelineStatus::Active) => Some(compute_next_run_at(
			&input.schedule.cron,
			&input.schedule.timezone,
			Utc::now(),
		)?),
		(_, PipelineStatus::Paused | PipelineStatus::Deleted) => None,
	};

	let source_type = input.source.source_type();
	let source_config = serde_json::to_value(&input.source)?;
	let verification = serde_json::to_value(&input.verification)?;
	let policy = serde_json::to_value(&input.policy)?;
	let delivery = serde_json::to_value(&input.delivery)?;

	let row = sqlx::query(
		r#"
		INSERT INTO v1_pipelines (
			tenant_id, name, status, source_type, source_config,
			schedule_cron, schedule_timezone, verification_settings,
			policy_config, delivery_config, next_run_at
		)
		VALUES ($1, $2, $3::pipeline_status, $4::pipeline_source_type, $5, $6, $7, $8, $9, $10, $11)
		RETURNING id, tenant_id, name, status::TEXT, source_config, schedule_cron, schedule_timezone,
		          verification_settings, policy_config, delivery_config, next_run_at, last_scheduled_at, last_run_id,
		          created_at, updated_at
		"#,
	)
	.bind(tenant_id)
	.bind(&input.name)
	.bind(enum_str(&input.status))
	.bind(enum_str(&source_type))
	.bind(&source_config)
	.bind(&input.schedule.cron)
	.bind(&input.schedule.timezone)
	.bind(&verification)
	.bind(&policy)
	.bind(&delivery)
	.bind(next_run_at)
	.fetch_one(pg_pool)
	.await?;

	row_to_pipeline_view(&row)
}

pub async fn list_pipelines(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	query: PipelineListQuery,
) -> Result<(Vec<PipelineView>, i64)> {
	let rows = sqlx::query(
		r#"
		SELECT id, tenant_id, name, status::TEXT, source_config, schedule_cron, schedule_timezone,
		       verification_settings, policy_config, delivery_config, next_run_at, last_scheduled_at, last_run_id,
		       created_at, updated_at
		FROM v1_pipelines
		WHERE tenant_id = $1
		  AND deleted_at IS NULL
		  AND ($2::pipeline_status IS NULL OR status = $2::pipeline_status)
		ORDER BY created_at DESC
		LIMIT $3 OFFSET $4
		"#,
	)
	.bind(tenant_id)
	.bind(query.status.as_ref().map(enum_str))
	.bind(query.limit)
	.bind(query.offset)
	.fetch_all(pg_pool)
	.await?;
	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*)
		FROM v1_pipelines
		WHERE tenant_id = $1
		  AND deleted_at IS NULL
		  AND ($2::pipeline_status IS NULL OR status = $2::pipeline_status)
		"#,
	)
	.bind(tenant_id)
	.bind(query.status.as_ref().map(enum_str))
	.fetch_one(pg_pool)
	.await?;

	let mut items = Vec::with_capacity(rows.len());
	for row in rows {
		items.push(row_to_pipeline_view(&row)?);
	}
	Ok((items, total))
}

pub async fn get_pipeline(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
) -> Result<Option<PipelineView>> {
	let row = sqlx::query(
		r#"
		SELECT id, tenant_id, name, status::TEXT, source_config, schedule_cron, schedule_timezone,
		       verification_settings, policy_config, delivery_config, next_run_at, last_scheduled_at, last_run_id,
		       created_at, updated_at
		FROM v1_pipelines
		WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
		"#,
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await?;
	row.map(|row| row_to_pipeline_view(&row)).transpose()
}

/// Like `get_pipeline`, but locks the row until `tx` ends. Updates read
/// next_run_at and write it back; the scheduler claims due pipelines with
/// FOR UPDATE SKIP LOCKED, so holding the lock keeps a claim from advancing
/// next_run_at in between, which the write would undo and run the pipeline twice.
async fn get_pipeline_for_update(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	tenant_id: Uuid,
	pipeline_id: i64,
) -> Result<Option<PipelineView>> {
	let row = sqlx::query(
		r#"
		SELECT id, tenant_id, name, status::TEXT, source_config, schedule_cron, schedule_timezone,
		       verification_settings, policy_config, delivery_config, next_run_at, last_scheduled_at, last_run_id,
		       created_at, updated_at
		FROM v1_pipelines
		WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
		FOR UPDATE
		"#,
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.fetch_optional(&mut **tx)
	.await?;
	row.map(|row| row_to_pipeline_view(&row)).transpose()
}

pub async fn update_pipeline(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
	input: UpdatePipelineInput,
	pipeline_config: &PipelinesConfig,
) -> Result<Option<PipelineView>> {
	if input.status.as_ref() == Some(&PipelineStatus::Deleted) {
		return Err(PipelineRequestError::validation(
			"Use the delete endpoint to delete a pipeline",
		)
		.into());
	}

	let mut tx = pg_pool.begin().await?;
	let current = match get_pipeline_for_update(&mut tx, tenant_id, pipeline_id).await? {
		Some(p) => p,
		None => return Ok(None),
	};

	let schedule_changed = input.schedule.is_some();
	let next_source = input.source.unwrap_or_else(|| current.source.clone());
	let next_schedule = input.schedule.unwrap_or_else(|| current.schedule.clone());
	let next_verification = input
		.verification
		.unwrap_or_else(|| current.verification.clone());
	let next_policy = input.policy.unwrap_or_else(|| current.policy.clone());
	let next_delivery = input.delivery.unwrap_or_else(|| current.delivery.clone());
	let next_status = input.status.unwrap_or_else(|| current.status.clone());
	let next_name = input.name.unwrap_or_else(|| current.name.clone());

	validate_pipeline_input(
		&mut tx,
		tenant_id,
		&next_source,
		&next_schedule,
		&next_policy,
		&next_delivery,
		pipeline_config.min_interval_seconds,
	)
	.await?;

	let next_run_at = match (&next_source, &next_status) {
		(PipelineSource::Push { .. }, _) => None,
		(_, &PipelineStatus::Active)
			if current.status == PipelineStatus::Active && !schedule_changed =>
		{
			current.next_run_at.or(Some(compute_next_run_at(
				&next_schedule.cron,
				&next_schedule.timezone,
				Utc::now(),
			)?))
		}
		(_, &PipelineStatus::Active) => Some(compute_next_run_at(
			&next_schedule.cron,
			&next_schedule.timezone,
			Utc::now(),
		)?),
		(_, &PipelineStatus::Paused | &PipelineStatus::Deleted) => None,
	};

	let row = sqlx::query(
		r#"
		UPDATE v1_pipelines
		SET name = $3,
		    status = $4::pipeline_status,
		    source_type = $5::pipeline_source_type,
		    source_config = $6,
		    schedule_cron = $7,
		    schedule_timezone = $8,
		    verification_settings = $9,
		    policy_config = $10,
		    delivery_config = $11,
		    next_run_at = $12,
		    updated_at = NOW()
		WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
		RETURNING id, tenant_id, name, status::TEXT, source_config, schedule_cron, schedule_timezone,
		          verification_settings, policy_config, delivery_config, next_run_at, last_scheduled_at, last_run_id,
		          created_at, updated_at
		"#,
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.bind(next_name)
	.bind(enum_str(&next_status))
	.bind(enum_str(&next_source.source_type()))
	.bind(serde_json::to_value(&next_source)?)
	.bind(next_schedule.cron)
	.bind(next_schedule.timezone)
	.bind(serde_json::to_value(&next_verification)?)
	.bind(serde_json::to_value(&next_policy)?)
	.bind(serde_json::to_value(&next_delivery)?)
	.bind(next_run_at)
	.fetch_optional(&mut *tx)
	.await?;
	tx.commit().await?;

	row.map(|row| row_to_pipeline_view(&row)).transpose()
}

pub async fn delete_pipeline(pg_pool: &PgPool, tenant_id: Uuid, pipeline_id: i64) -> Result<bool> {
	let result = sqlx::query(
		r#"
		UPDATE v1_pipelines
		SET status = 'deleted'::pipeline_status,
		    deleted_at = NOW(),
		    next_run_at = NULL,
		    updated_at = NOW()
		WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
		"#,
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.execute(pg_pool)
	.await?;
	Ok(result.rows_affected() > 0)
}

pub async fn set_pipeline_status(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
	status: PipelineStatus,
	min_interval_seconds: i32,
) -> Result<Option<PipelineView>> {
	let mut tx = pg_pool.begin().await?;
	let current = match get_pipeline_for_update(&mut tx, tenant_id, pipeline_id).await? {
		Some(p) => p,
		None => return Ok(None),
	};
	let next_run_at = match (&current.source, &status) {
		(PipelineSource::Push { .. }, _) => None,
		(_, &PipelineStatus::Active) => {
			validate_cron_expression(
				&current.schedule.cron,
				&current.schedule.timezone,
				min_interval_seconds,
				Utc::now(),
			)
			.map_err(|err| PipelineRequestError::validation(err.to_string()))?;
			Some(compute_next_run_at(
				&current.schedule.cron,
				&current.schedule.timezone,
				Utc::now(),
			)?)
		}
		(_, &PipelineStatus::Paused | &PipelineStatus::Deleted) => None,
	};
	let row = sqlx::query(
		r#"
		UPDATE v1_pipelines
		SET status = $3::pipeline_status,
		    next_run_at = $4,
		    updated_at = NOW()
		WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
		RETURNING id, tenant_id, name, status::TEXT, source_config, schedule_cron, schedule_timezone,
		          verification_settings, policy_config, delivery_config, next_run_at, last_scheduled_at, last_run_id,
		          created_at, updated_at
		"#,
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.bind(enum_str(&status))
	.bind(next_run_at)
	.fetch_optional(&mut *tx)
	.await?;
	tx.commit().await?;
	row.map(|row| row_to_pipeline_view(&row)).transpose()
}

pub async fn create_manual_pipeline_run(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
	force: bool,
	reason: Option<String>,
) -> Result<TriggerPipelineResponse> {
	crate::smtp_egress::require_smtp_ready(&config)?;
	let mut tx = pg_pool.begin().await?;
	let pipeline = match fetch_pipeline_row_for_update(&mut tx, pipeline_id).await? {
		Some(row) if row.tenant_id == tenant_id && row.deleted_at.is_none() => row,
		_ => return Err(PipelineRequestError::not_found("Pipeline not found").into()),
	};
	if matches!(
		serde_json::from_value::<PipelineSource>(pipeline.source_config.clone()),
		Ok(PipelineSource::Push { .. })
	) {
		return Err(PipelineRequestError::validation(
			"Push pipelines are triggered through the push endpoint",
		)
		.into());
	}

	let has_active_run = has_active_run(&mut tx, pipeline_id).await?;
	if !force && has_active_run {
		return Err(PipelineRequestError::conflict("Pipeline already has an active run").into());
	}
	let should_queue_only = force && has_active_run;
	let run_id: i64 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_pipeline_runs (pipeline_id, tenant_id, trigger_type, status, source_snapshot, stats)
		VALUES ($1, $2, 'manual', 'queued', $3, $4)
		RETURNING id
		"#,
	)
	.bind(pipeline.id)
	.bind(pipeline.tenant_id)
	.bind(&pipeline.source_config)
	.bind(match reason {
		Some(reason) => json!({ "trigger_reason": reason }),
		None => json!({}),
	})
	.fetch_one(&mut *tx)
	.await?;

	sqlx::query("UPDATE v1_pipelines SET last_run_id = $2, updated_at = NOW() WHERE id = $1")
		.bind(pipeline.id)
		.bind(run_id)
		.execute(&mut *tx)
		.await?;

	tx.commit().await?;
	if !should_queue_only {
		execute_pipeline_run_detached(config, pg_pool, run_id).await?;
	}

	let actual_status: String =
		sqlx::query_scalar("SELECT status::TEXT FROM v1_pipeline_runs WHERE id = $1")
			.bind(run_id)
			.fetch_one(pg_pool)
			.await?;

	Ok(TriggerPipelineResponse {
		run_id,
		status: parse_pipeline_run_status(&actual_status)?,
	})
}

pub async fn create_push_pipeline_run(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
	idempotency_key: String,
	input: PushPipelineInput,
) -> Result<PushPipelineResponse> {
	crate::smtp_egress::require_smtp_ready(&config)?;
	let idempotency_key = idempotency_key.trim();
	if idempotency_key.is_empty() || idempotency_key.len() > 200 {
		return Err(PipelineRequestError::validation(
			"Idempotency-Key is required and must not exceed 200 characters",
		)
		.into());
	}
	if input.rows.is_empty() || input.rows.len() > 10_000 {
		return Err(PipelineRequestError::validation(
			"rows must contain between 1 and 10000 contacts",
		)
		.into());
	}
	let email_column = input.email_column.trim();
	if email_column.is_empty() || email_column.len() > 100 {
		return Err(PipelineRequestError::validation("email_column is invalid").into());
	}
	if input.rows.iter().any(|row| !row.contains_key(email_column)) {
		return Err(PipelineRequestError::validation(
			"Every pushed row must contain the configured email column",
		)
		.into());
	}

	let mut tx = pg_pool.begin().await?;
	let pipeline = match fetch_pipeline_row_for_update(&mut tx, pipeline_id).await? {
		Some(row) if row.tenant_id == tenant_id && row.deleted_at.is_none() => row,
		_ => return Err(PipelineRequestError::not_found("Pipeline not found").into()),
	};
	let source: PipelineSource = serde_json::from_value(pipeline.source_config.clone())?;
	let PipelineSource::Push {
		accepted_format, ..
	} = source
	else {
		return Err(PipelineRequestError::validation("Pipeline source is not push").into());
	};
	if accepted_format.to_ascii_lowercase() != "json" {
		return Err(PipelineRequestError::validation(
			"Only json push pipeline payloads are currently supported",
		)
		.into());
	}
	let status: String = sqlx::query_scalar(
		"SELECT status::TEXT FROM v1_pipelines WHERE id = $1 AND tenant_id = $2",
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.fetch_one(&mut *tx)
	.await?;
	if status != "active" {
		return Err(PipelineRequestError::conflict("Pipeline is not active").into());
	}

	if let Some(row) = sqlx::query(
		"SELECT id, run_id, row_count FROM v1_pipeline_push_batches WHERE pipeline_id = $1 AND idempotency_key = $2",
	)
	.bind(pipeline_id)
	.bind(idempotency_key)
	.fetch_optional(&mut *tx)
	.await?
	{
		let run_id: Option<i64> = row.get("run_id");
		let run_id = run_id.context("Existing push batch is missing its run")?;
		let run_status: String = sqlx::query_scalar(
			"SELECT status::TEXT FROM v1_pipeline_runs WHERE id = $1",
		)
		.bind(run_id)
		.fetch_one(&mut *tx)
		.await?;
		tx.commit().await?;
		return Ok(PushPipelineResponse {
			batch_id: row.get("id"),
			run_id,
			status: parse_pipeline_run_status(&run_status)?,
			accepted_rows: row.get("row_count"),
			replayed: true,
		});
	}

	if has_active_run(&mut tx, pipeline_id).await? {
		return Err(PipelineRequestError::conflict("Pipeline already has an active run").into());
	}
	let rows = serde_json::to_value(&input.rows)?;
	let batch_id: i64 = sqlx::query_scalar(
		"INSERT INTO v1_pipeline_push_batches (pipeline_id, tenant_id, idempotency_key, source_key, email_column, rows, row_count) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id",
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.bind(idempotency_key)
	.bind(input.source_key.as_deref().map(str::trim).filter(|v| !v.is_empty()))
	.bind(email_column)
	.bind(rows)
	.bind(input.rows.len() as i32)
	.fetch_one(&mut *tx)
	.await?;
	let source_snapshot = json!({
		"source": pipeline.source_config,
		"push_batch_id": batch_id,
		"source_key": input.source_key,
	});
	let run_id: i64 = sqlx::query_scalar(
		"INSERT INTO v1_pipeline_runs (pipeline_id, tenant_id, trigger_type, status, source_snapshot, stats) VALUES ($1, $2, 'push', 'queued', $3, $4) RETURNING id",
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.bind(source_snapshot)
	.bind(json!({"accepted_rows": input.rows.len()}))
	.fetch_one(&mut *tx)
	.await?;
	sqlx::query("UPDATE v1_pipeline_push_batches SET run_id = $2 WHERE id = $1")
		.bind(batch_id)
		.bind(run_id)
		.execute(&mut *tx)
		.await?;
	sqlx::query("UPDATE v1_pipelines SET last_run_id = $2, updated_at = NOW() WHERE id = $1")
		.bind(pipeline_id)
		.bind(run_id)
		.execute(&mut *tx)
		.await?;
	tx.commit().await?;

	execute_pipeline_run_detached(config, pg_pool, run_id).await?;
	let run_status: String =
		sqlx::query_scalar("SELECT status::TEXT FROM v1_pipeline_runs WHERE id = $1")
			.bind(run_id)
			.fetch_one(pg_pool)
			.await?;
	Ok(PushPipelineResponse {
		batch_id,
		run_id,
		status: parse_pipeline_run_status(&run_status)?,
		accepted_rows: input.rows.len() as i32,
		replayed: false,
	})
}

pub async fn list_pipeline_runs(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
	limit: i64,
	offset: i64,
) -> Result<Option<(Vec<PipelineRunView>, i64)>> {
	let pipeline = get_pipeline(pg_pool, tenant_id, pipeline_id).await?;
	if pipeline.is_none() {
		return Ok(None);
	}

	let rows = sqlx::query(
		r#"
		SELECT id, pipeline_id, tenant_id, trigger_type, status::TEXT, scheduled_for, started_at,
		       completed_at, job_id, list_id, source_snapshot, stats, billed_emails,
		       result_location, delivery_status::TEXT, delivery_attempts, last_delivery_attempt_at,
		       next_delivery_attempt_at, delivery_error, error_code, error_message, created_at, updated_at
		FROM v1_pipeline_runs
		WHERE pipeline_id = $1 AND tenant_id = $2
		ORDER BY created_at DESC
		LIMIT $3 OFFSET $4
		"#,
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.bind(limit)
	.bind(offset)
	.fetch_all(pg_pool)
	.await?;
	let total: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_pipeline_runs WHERE pipeline_id = $1 AND tenant_id = $2",
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.fetch_one(pg_pool)
	.await?;

	let mut runs = Vec::with_capacity(rows.len());
	for row in rows {
		let run_id: i64 = row.get("id");
		maybe_finalize_pipeline_run(config.as_ref(), pg_pool, run_id).await?;
		let current_row = sqlx::query(
			r#"
			SELECT id, pipeline_id, tenant_id, trigger_type, status::TEXT, scheduled_for, started_at,
			       completed_at, job_id, list_id, source_snapshot, stats, billed_emails,
			       result_location, delivery_status::TEXT, delivery_attempts, last_delivery_attempt_at,
			       next_delivery_attempt_at, delivery_error, error_code, error_message, created_at, updated_at
			FROM v1_pipeline_runs
			WHERE id = $1
			"#,
		)
		.bind(run_id)
		.fetch_one(pg_pool)
		.await?;
		runs.push(row_to_pipeline_run_view(&current_row)?);
	}

	Ok(Some((runs, total)))
}

pub async fn get_pipeline_run(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
	run_id: i64,
) -> Result<Option<PipelineRunView>> {
	let pipeline = get_pipeline(pg_pool, tenant_id, pipeline_id).await?;
	if pipeline.is_none() {
		return Ok(None);
	}

	let owned_run_exists: Option<i64> = sqlx::query_scalar(
		r#"
		SELECT id
		FROM v1_pipeline_runs
		WHERE id = $1 AND pipeline_id = $2 AND tenant_id = $3
		"#,
	)
	.bind(run_id)
	.bind(pipeline_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await?;
	if owned_run_exists.is_none() {
		return Ok(None);
	}

	maybe_finalize_pipeline_run(config.as_ref(), pg_pool, run_id).await?;

	let row = sqlx::query(
		r#"
		SELECT id, pipeline_id, tenant_id, trigger_type, status::TEXT, scheduled_for, started_at,
		       completed_at, job_id, list_id, source_snapshot, stats, billed_emails,
		       result_location, delivery_status::TEXT, delivery_attempts, last_delivery_attempt_at,
		       next_delivery_attempt_at, delivery_error, error_code, error_message, created_at, updated_at
		FROM v1_pipeline_runs
		WHERE id = $1 AND pipeline_id = $2 AND tenant_id = $3
		"#,
	)
	.bind(run_id)
	.bind(pipeline_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await?;
	row.map(|row| row_to_pipeline_run_view(&row)).transpose()
}

pub async fn maybe_finalize_pipeline_run_for_job(
	config: &BackendConfig,
	pg_pool: &PgPool,
	job_id: i32,
) -> Result<()> {
	let run_id = sqlx::query_scalar::<_, i64>(
		r#"
		SELECT id
		FROM v1_pipeline_runs
		WHERE job_id = $1
		ORDER BY id DESC
		LIMIT 1
		"#,
	)
	.bind(job_id)
	.fetch_optional(pg_pool)
	.await?;
	if let Some(run_id) = run_id {
		maybe_finalize_pipeline_run(config, pg_pool, run_id).await?;
	}
	Ok(())
}

pub async fn maybe_finalize_pipeline_run(
	_config: &BackendConfig,
	pg_pool: &PgPool,
	run_id: i64,
) -> Result<()> {
	let row = sqlx::query(
		r#"
		SELECT pr.id, pr.pipeline_id, pr.tenant_id, pr.status::TEXT, pr.job_id, pr.list_id,
		       pr.stats, pr.billed_emails, pr.result_location, pr.delivery_status::TEXT,
		       pr.error_code, pr.error_message, p.delivery_config, t.default_webhook_url,
		       t.webhook_signing_secret
		FROM v1_pipeline_runs pr
		INNER JOIN v1_pipelines p ON p.id = pr.pipeline_id
		INNER JOIN tenants t ON t.id = pr.tenant_id
		WHERE pr.id = $1
		"#,
	)
	.bind(run_id)
	.fetch_optional(pg_pool)
	.await?;
	let Some(row) = row else {
		return Ok(());
	};
	let status = parse_pipeline_run_status(&row.get::<String, _>("status"))?;
	let result_location: Option<Value> = row.get("result_location");
	// Reading a run calls this for every run it returns. Finalizing a finished
	// run again would roll contact state back to that older run and re-send its
	// delivery webhook.
	if status.is_terminal() && result_location.is_some() {
		return Ok(());
	}
	// The executor still owns the run until it moves it to running: it may still
	// be inserting rows, and its own status updates would overwrite a run
	// finalized here, leaving it active forever.
	if matches!(
		status,
		PipelineRunStatus::Queued
			| PipelineRunStatus::Preparing
			| PipelineRunStatus::FetchingSource
			| PipelineRunStatus::Publishing
	) {
		return Ok(());
	}

	let Some(job_id) = row.get::<Option<i32>, _>("job_id") else {
		return Ok(());
	};
	let job_row = sqlx::query("SELECT status::TEXT, completed_at FROM v1_bulk_job WHERE id = $1")
		.bind(job_id)
		.fetch_optional(pg_pool)
		.await?;
	let Some(job_row) = job_row else {
		return Ok(());
	};
	let job_status: String = job_row.get("status");
	if !matches!(job_status.as_str(), "completed" | "failed" | "cancelled") {
		return Ok(());
	}

	let stats = merge_pipeline_run_stats(
		row.get::<Value, _>("stats"),
		build_pipeline_run_stats(pg_pool, row.get::<Option<i32>, _>("list_id"), job_id).await?,
	);
	let final_status = match job_status.as_str() {
		"completed" => PipelineRunStatus::Completed,
		"failed" => PipelineRunStatus::Failed,
		"cancelled" => PipelineRunStatus::Cancelled,
		_ => PipelineRunStatus::Failed,
	};
	let download_url = json!({
		"download_url": row
			.get::<Option<i32>, _>("list_id")
			.map(|list_id| format!("/v1/lists/{list_id}/download"))
	});

	let claim_sql = if status.is_terminal() {
		r#"
		UPDATE v1_pipeline_runs
		SET stats = $2,
		    billed_emails = $3,
		    result_location = $4,
		    updated_at = NOW()
		WHERE id = $1
		  AND status = $5::pipeline_run_status
		  AND result_location IS NULL
		RETURNING id
		"#
	} else {
		r#"
		UPDATE v1_pipeline_runs
		SET status = 'delivering'::pipeline_run_status,
		    stats = $2,
		    billed_emails = $3,
		    result_location = $4,
		    updated_at = NOW()
		WHERE id = $1 AND status = $5::pipeline_run_status
		RETURNING id
		"#
	};
	let claimed = sqlx::query(claim_sql)
		.bind(run_id)
		.bind(&stats)
		.bind(stats["billed_emails"].as_i64().unwrap_or_default() as i32)
		.bind(&download_url)
		.bind(enum_str(&status))
		.fetch_optional(pg_pool)
		.await?;
	if claimed.is_none() {
		return Ok(());
	}

	// Record contact state before marking the run finished: if this fails, a run
	// claimed from 'running' stays 'delivering' and the stalled-run sweep
	// finalizes it again, instead of it ending without contact state.
	update_pipeline_contact_state_for_run(pg_pool, run_id).await?;

	sqlx::query(
		r#"
		UPDATE v1_pipeline_runs
		SET status = $2::pipeline_run_status,
		    completed_at = COALESCE(completed_at, NOW()),
		    stats = $3,
		    billed_emails = $4,
		    result_location = $5,
		    updated_at = NOW()
		WHERE id = $1
		"#,
	)
	.bind(run_id)
	.bind(enum_str(&final_status))
	.bind(&stats)
	.bind(stats["billed_emails"].as_i64().unwrap_or_default() as i32)
	.bind(&download_url)
	.execute(pg_pool)
	.await?;

	if let Err(err) = attempt_pipeline_run_delivery(pg_pool, run_id, false).await {
		warn!(target: LOG_TARGET, run_id = run_id, error = ?err, "Pipeline run delivery failed");
	}

	Ok(())
}

enum PipelineUsageReservation {
	Reserved { usage_event_id: i64 },
	ExceededMonthlyLimit,
}

async fn reserve_pipeline_usage_event(
	pg_pool: &PgPool,
	tenant_ctx: &crate::tenant::context::TenantContext,
	tenant_id: Uuid,
	pipeline_id: i64,
	run_id: i64,
	trigger_type: &str,
	reserved_emails: i32,
	verification: &PipelineVerificationSettings,
	delivery: &PipelineDeliveryConfig,
) -> Result<PipelineUsageReservation> {
	let mut tx = pg_pool.begin().await?;

	if let Some(limit) = tenant_ctx.monthly_email_limit.filter(|value| *value > 0) {
		if Utc::now() >= tenant_ctx.period_reset_at {
			sqlx::query(
				r#"
				UPDATE tenants
				SET used_this_period = 0,
				    period_reset_at = date_trunc('month', NOW()) + INTERVAL '1 month'
				WHERE id = $1 AND period_reset_at <= NOW()
				"#,
			)
			.bind(tenant_id)
			.execute(&mut *tx)
			.await?;
		}

		let request_count = reserved_emails.max(0);
		if request_count > 0 {
			let usage_updated = sqlx::query_scalar::<_, i32>(
				r#"
				UPDATE tenants
				SET used_this_period = used_this_period + $2
				WHERE id = $1
				  AND (
					monthly_email_limit IS NULL
					OR monthly_email_limit <= 0
					OR used_this_period + $2 <= monthly_email_limit
				  )
				RETURNING used_this_period
				"#,
			)
			.bind(tenant_id)
			.bind(request_count)
			.fetch_optional(&mut *tx)
			.await?;

			if usage_updated.is_none() {
				let current_usage = sqlx::query_scalar::<_, Option<i32>>(
					"SELECT used_this_period FROM tenants WHERE id = $1",
				)
				.bind(tenant_id)
				.fetch_one(&mut *tx)
				.await?
				.unwrap_or_default();

				tx.rollback().await?;
				if current_usage >= limit {
					return Ok(PipelineUsageReservation::ExceededMonthlyLimit);
				}

				bail!("Tenant quota reservation failed for pipeline run");
			}
		}
	}

	let usage_event_id: i64 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_usage_events (
			tenant_id, pipeline_id, pipeline_run_id, source, reserved_emails, committed_emails, status, metadata
		)
		VALUES ($1, $2, $3, $4, $5, 0, 'reserved', $6)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(pipeline_id)
	.bind(run_id)
	.bind(format!("pipeline.{trigger_type}"))
	.bind(reserved_emails.max(0))
	.bind(json!({
		"verification": verification,
		"delivery": sanitized_delivery_metadata(delivery),
	}))
	.fetch_one(&mut *tx)
	.await?;

	tx.commit().await?;
	Ok(PipelineUsageReservation::Reserved { usage_event_id })
}

async fn execute_pipeline_run(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	run_id: i64,
) -> Result<()> {
	let result = execute_pipeline_run_inner(config, pg_pool, run_id).await;
	if let Err(err) = &result {
		if let Err(mark_err) =
			mark_pipeline_run_execution_failed(pg_pool, run_id, "execution_error", err).await
		{
			warn!(
				target: LOG_TARGET,
				run_id = run_id,
				error = ?mark_err,
				"Failed to mark pipeline run as failed after execution error"
			);
		}
	}
	result
}

/// Runs `execute_pipeline_run` on its own task and waits for it. Request
/// handlers call this: a handler's future is dropped when the client
/// disconnects, and executing inline would stop the run midway, leaving it
/// active with nothing left to finish it.
async fn execute_pipeline_run_detached(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	run_id: i64,
) -> Result<()> {
	let pg_pool = pg_pool.clone();
	tokio::spawn(async move { execute_pipeline_run(config, &pg_pool, run_id).await })
		.await
		.context("Pipeline run execution task did not complete")?
}

async fn execute_pipeline_run_inner(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	run_id: i64,
) -> Result<()> {
	let run = sqlx::query(
		r#"
		SELECT pr.id, pr.pipeline_id, pr.tenant_id, pr.trigger_type, pr.status::TEXT,
		       pr.scheduled_for, pr.source_snapshot, pr.stats,
		       p.name, p.source_config, p.source_type::TEXT, p.schedule_cron, p.schedule_timezone,
		       p.verification_settings, p.delivery_config
		FROM v1_pipeline_runs pr
		INNER JOIN v1_pipelines p ON p.id = pr.pipeline_id
		WHERE pr.id = $1
		"#,
	)
	.bind(run_id)
	.fetch_optional(pg_pool)
	.await?
	.context("pipeline run not found")?;

	let current_status = parse_pipeline_run_status(&run.get::<String, _>("status"))?;
	if current_status.is_terminal() {
		return Ok(());
	}

	let pipeline_id: i64 = run.get("pipeline_id");
	let tenant_id: Uuid = run.get("tenant_id");
	let source_snapshot: Value = run.get("source_snapshot");
	let source = parse_pipeline_source_snapshot(&source_snapshot)?;
	let normalized_source_snapshot =
		serde_json::to_value(&source).context("failed to serialize pipeline source snapshot")?;
	let verification: PipelineVerificationSettings =
		serde_json::from_value(run.get::<Value, _>("verification_settings")).unwrap_or_default();
	let delivery: PipelineDeliveryConfig =
		serde_json::from_value(run.get::<Value, _>("delivery_config")).unwrap_or_default();

	sqlx::query(
		"UPDATE v1_pipeline_runs SET status = 'preparing'::pipeline_run_status, started_at = COALESCE(started_at, NOW()), updated_at = NOW() WHERE id = $1",
	)
	.bind(run_id)
	.execute(pg_pool)
	.await?;

	let prepared_list = load_pipeline_source(pg_pool, tenant_id, &source, &source_snapshot).await?;
	let (prepared_list, delta_summary) =
		apply_delta_selection(pg_pool, pipeline_id, &verification, prepared_list).await?;
	let tenant_ctx = resolve_tenant_context_by_id(pg_pool, tenant_id).await?;
	let updated_stats = merge_pipeline_run_stats(
		run.get::<Value, _>("stats"),
		json!({
			"source_name": prepared_list.source_name.clone(),
			"source_filename": prepared_list.source_filename.clone(),
			"source_row_count": prepared_list.rows.len(),
			"source_unique_emails": prepared_list.unique_email_count,
			"delta_mode": verification.delta_mode,
			"freshness_days": verification.freshness_days,
			"selected_unique_emails": delta_summary.selected_unique_emails,
			"skipped_unchanged": delta_summary.skipped_unchanged,
			"changed_only_export": verification.delta_mode,
		}),
	);
	sqlx::query(
		"UPDATE v1_pipeline_runs SET status = 'fetching_source'::pipeline_run_status, source_snapshot = $2, stats = $3, updated_at = NOW() WHERE id = $1",
	)
	.bind(run_id)
	.bind(&normalized_source_snapshot)
	.bind(updated_stats)
	.execute(pg_pool)
	.await?;

	let usage_event_id = match reserve_pipeline_usage_event(
		pg_pool,
		&tenant_ctx,
		tenant_id,
		pipeline_id,
		run_id,
		&run.get::<String, _>("trigger_type"),
		prepared_list.unique_email_count,
		&verification,
		&delivery,
	)
	.await?
	{
		PipelineUsageReservation::Reserved { usage_event_id } => usage_event_id,
		PipelineUsageReservation::ExceededMonthlyLimit => {
			fail_pipeline_run(
				pg_pool,
				run_id,
				"insufficient_credits",
				"Tenant quota would be exceeded by this pipeline run",
				&normalized_source_snapshot,
			)
			.await?;
			return Ok(());
		}
	};

	let job_id: i32 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_bulk_job (total_records, tenant_id, status, metadata, created_by)
		VALUES ($1, $2, 'pending'::job_state, $3, 'pipeline')
		RETURNING id
		"#,
	)
	.bind(prepared_list.rows.len() as i32)
	.bind(tenant_id)
	.bind(json!({
		"source": "pipeline",
		"pipeline_id": pipeline_id,
		"pipeline_run_id": run_id,
		"source_type": enum_str(&source.source_type()),
	}))
	.fetch_one(pg_pool)
	.await?;

	let list_id = create_pipeline_list(
		pg_pool,
		tenant_id,
		pipeline_id,
		run_id,
		job_id,
		&run.get::<String, _>("name"),
		&prepared_list,
	)
	.await?;

	sqlx::query(
		"UPDATE v1_pipeline_runs SET status = 'publishing'::pipeline_run_status, job_id = $2, list_id = $3, updated_at = NOW() WHERE id = $1",
	)
	.bind(run_id)
	.bind(job_id)
	.bind(list_id)
	.execute(pg_pool)
	.await?;

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(0);
	let worker_config = config.must_worker_config()?;

	// Every row gets its task row before anything is published. The worker
	// settles the job and list as tasks complete, and a row added after the last
	// completion would leave the list short of its total forever.
	let mut groups_to_publish = Vec::with_capacity(prepared_list.canonical_groups.len());
	for (canonical, indices) in &prepared_list.canonical_groups {
		let primary_index = indices[0];
		let routing_email = prepared_list.rows[primary_index]
			.get(&prepared_list.email_column)
			.and_then(Value::as_str)
			.unwrap_or_default()
			.trim()
			.to_string();
		let primary_extra = json!({
			"list_id": list_id,
			"row_index": primary_index as i32,
			"email_column": prepared_list.email_column,
			"pipeline_id": pipeline_id,
			"pipeline_run_id": run_id,
		});
		let primary_task_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (job_id, payload, extra, task_state, tenant_id, canonical_email, is_duplicate)
			VALUES ($1, $2, $3, 'queued', $4, $5, false)
			RETURNING id
			"#,
		)
		.bind(job_id)
		.bind(json!({
				"input": {"to_email": routing_email},
				"job_id": {"bulk": job_id},
				"webhook": null
			}))
		.bind(primary_extra)
		.bind(tenant_id)
		.bind(canonical)
		.fetch_one(pg_pool)
		.await?;

		let mut duplicate_ids = Vec::new();
		for &dup_index in &indices[1..] {
			let dup_email = prepared_list.rows[dup_index]
				.get(&prepared_list.email_column)
				.and_then(Value::as_str)
				.unwrap_or_default()
				.trim()
				.to_string();
			let dup_extra = json!({
				"list_id": list_id,
				"row_index": dup_index as i32,
				"email_column": prepared_list.email_column,
				"pipeline_id": pipeline_id,
				"pipeline_run_id": run_id,
			});
			let dup_id: i32 = sqlx::query_scalar(
				r#"
				INSERT INTO v1_task_result (
					job_id, payload, extra, task_state, tenant_id,
					canonical_email, is_duplicate, canonical_task_id
				)
				VALUES ($1, $2, $3, 'queued', $4, $5, true, $6)
				RETURNING id
				"#,
			)
			.bind(job_id)
			.bind(json!({
				"input": {"to_email": dup_email},
				"job_id": {"bulk": job_id},
				"webhook": null
			}))
			.bind(dup_extra)
			.bind(tenant_id)
			.bind(canonical)
			.bind(primary_task_id)
			.fetch_one(pg_pool)
			.await?;
			duplicate_ids.push(dup_id);
		}
		groups_to_publish.push((canonical, primary_index, primary_task_id, duplicate_ids));
	}

	for &index in &prepared_list.blank_indices {
		let extra = json!({
			"list_id": list_id,
			"row_index": index as i32,
			"email_column": prepared_list.email_column,
			"pipeline_id": pipeline_id,
			"pipeline_run_id": run_id,
		});
		sqlx::query(
			r#"
			INSERT INTO v1_task_result (
				job_id, payload, extra, result, tenant_id, task_state,
				score, score_category, sub_reason, safe_to_send, reason_codes,
				completed_at, canonical_email, is_duplicate
			)
			VALUES ($1, $2, $3, $4, $5, 'completed', 0, 'invalid', 'invalid_syntax', false, ARRAY['invalid_syntax'], NOW(), NULL, false)
			"#,
		)
		.bind(job_id)
		.bind(json!({
				"input": {"to_email": ""},
				"job_id": {"bulk": job_id},
				"webhook": null
			}))
		.bind(extra)
		.bind(blank_email_result())
		.bind(tenant_id)
		.execute(pg_pool)
		.await?;
	}

	for &index in &prepared_list.invalid_indices {
		let invalid_email = prepared_list.rows[index]
			.get(&prepared_list.email_column)
			.and_then(Value::as_str)
			.unwrap_or_default()
			.trim()
			.to_string();
		let extra = json!({
			"list_id": list_id,
			"row_index": index as i32,
			"email_column": prepared_list.email_column,
			"pipeline_id": pipeline_id,
			"pipeline_run_id": run_id,
		});
		sqlx::query(
			r#"
			INSERT INTO v1_task_result (
				job_id, payload, extra, result, tenant_id, task_state,
				score, score_category, sub_reason, safe_to_send, reason_codes,
				completed_at, canonical_email, is_duplicate
			)
			VALUES ($1, $2, $3, $4, $5, 'completed', 0, 'invalid', 'invalid_syntax', false, ARRAY['invalid_syntax'], NOW(), NULL, false)
			"#,
		)
		.bind(job_id)
		.bind(json!({
				"input": {"to_email": invalid_email},
				"job_id": {"bulk": job_id},
				"webhook": null
			}))
		.bind(extra)
		.bind(invalid_syntax_email_result(&invalid_email))
		.bind(tenant_id)
		.execute(pg_pool)
		.await?;
	}

	// Mark the job and list in progress before publishing: the worker moves them
	// to a final status when the last task finishes, which can happen before the
	// publish loop ends. The status guards keep a run failed in the meantime
	// (by the stalled-run sweep) from being revived.
	sqlx::query(
		"UPDATE v1_bulk_job SET status = 'running'::job_state, updated_at = NOW() WHERE id = $1 AND status = 'pending'::job_state",
	)
	.bind(job_id)
	.execute(pg_pool)
	.await?;
	sqlx::query(
		"UPDATE v1_lists SET status = 'processing'::list_status, updated_at = NOW() WHERE id = $1 AND status = 'uploading'::list_status",
	)
	.bind(list_id)
	.execute(pg_pool)
	.await?;

	let mut published_unique_groups = 0i32;
	for (canonical, primary_index, primary_task_id, duplicate_ids) in groups_to_publish {
		let task = CheckEmailTask {
			input: CheckEmailRequest {
				to_email: prepared_list.rows[primary_index]
					.get(&prepared_list.email_column)
					.and_then(Value::as_str)
					.unwrap_or_default()
					.trim()
					.to_string(),
				..Default::default()
			}
			.to_check_email_input(Arc::clone(&config)),
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				dispatch_id: None,
				lease_id: None,
				decision_policy: None,
				tenant_id: Some(tenant_id.to_string()),
				request_id: None,
				correlation_id: None,
				created_by: Some("pipeline".to_string()),
				retry_policy: None,
				dedupe_key: Some(format!(
					"pipeline:{pipeline_id}:run:{run_id}:row:{primary_index}"
				)),
				task_db_id: Some(primary_task_id),
			}),
		};

		if let Err(err) =
			publish_task(worker_config.channel.clone(), task, properties.clone()).await
		{
			warn!(
				target: LOG_TARGET,
				pipeline_id = pipeline_id,
				run_id = run_id,
				email = %canonical,
				error = ?err,
				"Failed to publish pipeline task"
			);
			let mut all_failed_ids = vec![primary_task_id];
			all_failed_ids.extend(duplicate_ids);
			sqlx::query(
				r#"
				UPDATE v1_task_result
				SET task_state = 'failed'::task_state,
				    error = 'publish_failed',
				    completed_at = NOW(),
				    updated_at = NOW()
				WHERE id = ANY($1)
				"#,
			)
			.bind(&all_failed_ids)
			.execute(pg_pool)
			.await?;
			continue;
		}

		published_unique_groups += 1;
	}

	let unused_reserved = prepared_list.unique_email_count - published_unique_groups;
	if unused_reserved > 0 {
		release_reserved_usage(Some(pg_pool), Some(tenant_id), unused_reserved).await?;
	}
	sqlx::query(
		r#"
		UPDATE v1_usage_events
		SET committed_emails = $2,
		    status = $3,
		    job_id = $4,
		    updated_at = NOW()
		WHERE id = $1
		"#,
	)
	.bind(usage_event_id)
	.bind(published_unique_groups)
	.bind(if published_unique_groups > 0 {
		"committed"
	} else {
		"released"
	})
	.bind(job_id)
	.execute(pg_pool)
	.await?;

	// Rows that failed to publish, or a list with nothing to verify, leave no
	// task for the worker to finish, so nothing else would settle these.
	settle_job_without_outstanding_tasks(pg_pool, job_id).await?;
	settle_list_without_outstanding_rows(pg_pool, list_id, job_id).await?;

	// Only now hand the run over: maybe_finalize_pipeline_run skips runs that
	// are still publishing, so the worker cannot finalize it underneath us. The
	// guard keeps a run the stalled-run sweep failed meanwhile failed.
	sqlx::query(
		"UPDATE v1_pipeline_runs SET status = 'running'::pipeline_run_status, billed_emails = $2, updated_at = NOW() WHERE id = $1 AND status = 'publishing'::pipeline_run_status",
	)
	.bind(run_id)
	.bind(published_unique_groups)
	.execute(pg_pool)
	.await?;

	// The job may have finished while the run was publishing, when the worker's
	// own finalize call skipped it. A failure here leaves the run running with a
	// finished job, which the stalled-run sweep finalizes later.
	if let Err(err) = maybe_finalize_pipeline_run(config.as_ref(), pg_pool, run_id).await {
		warn!(
			target: LOG_TARGET,
			run_id = run_id,
			error = ?err,
			"Failed to finalize pipeline run after publishing"
		);
	}

	Ok(())
}

/// Gives a pipeline job its final status once none of its tasks is still
/// queued or running, using the worker's rules: cancelling becomes cancelled,
/// any failed task makes it failed, otherwise completed.
async fn settle_job_without_outstanding_tasks(pg_pool: &PgPool, job_id: i32) -> Result<()> {
	sqlx::query(
		r#"
		WITH settled AS (
			SELECT j.id,
			       CASE
			           WHEN j.status = 'cancelling'::job_state THEN 'cancelled'::job_state
			           WHEN EXISTS (
			               SELECT 1 FROM v1_task_result t
			               WHERE t.job_id = j.id
			                 AND t.task_state IN ('failed'::task_state, 'dead_lettered'::task_state)
			           ) THEN 'failed'::job_state
			           ELSE 'completed'::job_state
			       END AS final_status
			FROM v1_bulk_job j
			WHERE j.id = $1
			  AND j.status IN ('running'::job_state, 'cancelling'::job_state)
			  AND NOT EXISTS (
			      SELECT 1 FROM v1_task_result t
			      WHERE t.job_id = j.id
			        AND t.task_state IN ('queued'::task_state, 'running'::task_state, 'retrying'::task_state)
			  )
		)
		UPDATE v1_bulk_job j
		SET status = settled.final_status,
		    completed_at = CASE
		        WHEN settled.final_status IN ('completed'::job_state, 'failed'::job_state)
		        THEN COALESCE(j.completed_at, NOW())
		        ELSE j.completed_at
		    END,
		    cancelled_at = CASE
		        WHEN settled.final_status = 'cancelled'::job_state THEN COALESCE(j.cancelled_at, NOW())
		        ELSE j.cancelled_at
		    END,
		    updated_at = NOW()
		FROM settled
		WHERE j.id = settled.id
		"#,
	)
	.bind(job_id)
	.execute(pg_pool)
	.await?;
	Ok(())
}

/// Gives a pipeline list its final status once every row has a result, using
/// the worker's rules: failed if any row failed, otherwise completed.
async fn settle_list_without_outstanding_rows(
	pg_pool: &PgPool,
	list_id: i32,
	job_id: i32,
) -> Result<()> {
	sqlx::query(
		r#"
		WITH tally AS (
			SELECT
				COUNT(*) FILTER (
					WHERE (result IS NOT NULL OR error IS NOT NULL OR task_state = 'cancelled'::task_state)
						AND task_state NOT IN ('queued'::task_state, 'running'::task_state, 'retrying'::task_state)
				) AS processed,
				COUNT(*) FILTER (
					WHERE task_state IN ('failed'::task_state, 'dead_lettered'::task_state)
				) AS failed
			FROM v1_task_result
			WHERE job_id = $2
		)
		UPDATE v1_lists l
		SET status = CASE WHEN tally.failed > 0 THEN 'failed'::list_status ELSE 'completed'::list_status END,
		    error_message = CASE WHEN tally.failed > 0 THEN COALESCE(l.error_message, 'task_failed') ELSE l.error_message END,
		    completed_at = COALESCE(l.completed_at, NOW()),
		    updated_at = NOW()
		FROM tally
		WHERE l.id = $1
		  AND l.status = 'processing'::list_status
		  AND tally.processed >= l.total_rows
		"#,
	)
	.bind(list_id)
	.bind(job_id)
	.execute(pg_pool)
	.await?;
	Ok(())
}

async fn mark_pipeline_run_execution_failed(
	pg_pool: &PgPool,
	run_id: i64,
	error_code: &str,
	err: &anyhow::Error,
) -> Result<()> {
	let row = sqlx::query(
		r#"
		SELECT tenant_id, job_id, list_id, status::TEXT
		FROM v1_pipeline_runs
		WHERE id = $1
		"#,
	)
	.bind(run_id)
	.fetch_optional(pg_pool)
	.await?;
	let Some(row) = row else {
		return Ok(());
	};

	let status = parse_pipeline_run_status(&row.get::<String, _>("status"))?;
	if status.is_terminal() {
		return Ok(());
	}

	let tenant_id: Uuid = row.get("tenant_id");
	let job_id: Option<i32> = row.get("job_id");
	let list_id: Option<i32> = row.get("list_id");
	let reserved_emails = sqlx::query_scalar::<_, Option<i64>>(
		r#"
		SELECT SUM(GREATEST(reserved_emails - committed_emails, 0))
		FROM v1_usage_events
		WHERE pipeline_run_id = $1
		  AND status = 'reserved'
		"#,
	)
	.bind(run_id)
	.fetch_one(pg_pool)
	.await?
	.unwrap_or(0) as i32;
	if reserved_emails > 0 {
		release_reserved_usage(Some(pg_pool), Some(tenant_id), reserved_emails).await?;
	}

	sqlx::query(
		r#"
		UPDATE v1_usage_events
		SET status = 'released',
		    updated_at = NOW()
		WHERE pipeline_run_id = $1
		  AND status = 'reserved'
		"#,
	)
	.bind(run_id)
	.execute(pg_pool)
	.await?;

	if let Some(job_id) = job_id {
		sqlx::query(
			r#"
			UPDATE v1_bulk_job
			SET status = 'failed'::job_state,
			    completed_at = COALESCE(completed_at, NOW()),
			    updated_at = NOW()
			WHERE id = $1
			  AND status NOT IN ('completed'::job_state, 'failed'::job_state, 'cancelled'::job_state)
			"#,
		)
		.bind(job_id)
		.execute(pg_pool)
		.await?;
	}

	if let Some(list_id) = list_id {
		sqlx::query(
			r#"
			UPDATE v1_lists
			SET status = 'failed'::list_status,
			    error_message = COALESCE(error_message, $2),
			    completed_at = COALESCE(completed_at, NOW()),
			    updated_at = NOW()
			WHERE id = $1
			  AND status NOT IN ('completed'::list_status, 'failed'::list_status, 'deleted'::list_status)
			"#,
		)
		.bind(list_id)
		.bind(err.to_string())
		.execute(pg_pool)
		.await?;
	}

	sqlx::query(
		r#"
		UPDATE v1_pipeline_runs
		SET status = 'failed'::pipeline_run_status,
		    error_code = COALESCE(error_code, $3),
		    error_message = COALESCE(error_message, $2),
		    completed_at = COALESCE(completed_at, NOW()),
		    updated_at = NOW()
		WHERE id = $1
		  AND status IN (
			'queued'::pipeline_run_status,
			'preparing'::pipeline_run_status,
			'fetching_source'::pipeline_run_status,
			'publishing'::pipeline_run_status,
			'running'::pipeline_run_status,
			'delivering'::pipeline_run_status
		  )
		"#,
	)
	.bind(run_id)
	.bind(err.to_string())
	.bind(error_code)
	.execute(pg_pool)
	.await?;

	Ok(())
}

async fn fail_pipeline_run(
	pg_pool: &PgPool,
	run_id: i64,
	error_code: &str,
	error_message: &str,
	source_snapshot: &Value,
) -> Result<()> {
	sqlx::query(
		r#"
		UPDATE v1_pipeline_runs
		SET status = 'failed'::pipeline_run_status,
		    source_snapshot = $2,
		    error_code = $3,
		    error_message = $4,
		    completed_at = NOW(),
		    updated_at = NOW()
		WHERE id = $1
		"#,
	)
	.bind(run_id)
	.bind(source_snapshot)
	.bind(error_code)
	.bind(error_message)
	.execute(pg_pool)
	.await?;
	Ok(())
}

async fn create_pipeline_list(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
	run_id: i64,
	job_id: i32,
	pipeline_name: &str,
	prepared: &PreparedList,
) -> Result<i32> {
	let original_data = build_original_data(&prepared.rows);
	let list_name = format!(
		"{} {}",
		pipeline_name,
		Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
	);
	let list_id: i32 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_lists (
			tenant_id, job_id, source_list_id, pipeline_id, pipeline_run_id,
			name, original_filename, file_size_bytes, total_rows, email_column,
			original_headers, original_data, status, unique_emails, deduplicated_count
		)
		VALUES ($1, $2, $3, $4, $5, $6, $7, 0, $8, $9, $10, $11, 'uploading'::list_status, $12, $13)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(job_id)
	.bind(prepared.source_list_id)
	.bind(pipeline_id)
	.bind(run_id)
	.bind(list_name)
	.bind(&prepared.source_filename)
	.bind(prepared.rows.len() as i32)
	.bind(&prepared.email_column)
	.bind(&prepared.headers)
	.bind(&original_data)
	.bind(prepared.unique_email_count)
	.bind(prepared.deduplicated_count)
	.fetch_one(pg_pool)
	.await?;
	Ok(list_id)
}

async fn load_source_list_snapshot(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	source: &PipelineSource,
) -> Result<PreparedList> {
	let PipelineSource::ListSnapshot { list_id } = source else {
		bail!("Only list_snapshot pipeline sources are supported in phase 1");
	};

	let row = sqlx::query(
		r#"
		SELECT id, name, original_filename, total_rows, email_column, original_headers, original_data, status::TEXT
		FROM v1_lists
		WHERE id = $1 AND tenant_id = $2
		"#,
	)
	.bind(*list_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await?
	.context("Source list not found")?;

	let status: String = row.get("status");
	if status == "deleted" {
		bail!("Source list has been deleted");
	}

	let original_data: Value = row.get("original_data");
	let rows = parse_original_rows(&original_data)?;
	let headers: Vec<String> = row.get("original_headers");
	let email_column: String = row.get("email_column");

	let (canonical_groups, blank_indices, invalid_indices) =
		build_canonical_groups(&rows, &email_column);
	let unique_email_count = canonical_groups.len() as i32;
	let deduplicated_count =
		(canonical_groups.values().map(|v| v.len()).sum::<usize>() - canonical_groups.len()) as i32;

	Ok(PreparedList {
		source_list_id: Some(row.get("id")),
		headers,
		email_column,
		rows,
		source_filename: row.get("original_filename"),
		source_name: row.get("name"),
		unique_email_count,
		deduplicated_count,
		canonical_groups,
		blank_indices,
		invalid_indices,
	})
}

async fn load_pipeline_source(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	source: &PipelineSource,
	source_snapshot: &Value,
) -> Result<PreparedList> {
	match source {
		PipelineSource::ListSnapshot { .. } => {
			load_source_list_snapshot(pg_pool, tenant_id, source).await
		}
		PipelineSource::Push { .. } => {
			let batch_id = source_snapshot
				.get("push_batch_id")
				.and_then(Value::as_i64)
				.context("Push pipeline run is missing push_batch_id")?;
			let row = sqlx::query(
				"SELECT source_key, email_column, rows, row_count FROM v1_pipeline_push_batches WHERE id = $1 AND tenant_id = $2",
			)
			.bind(batch_id)
			.bind(tenant_id)
			.fetch_optional(pg_pool)
			.await?
			.context("Push pipeline batch not found")?;
			let rows: Vec<Map<String, Value>> = serde_json::from_value(row.get("rows"))?;
			let email_column: String = row.get("email_column");
			let (canonical_groups, blank_indices, invalid_indices) =
				build_canonical_groups(&rows, &email_column);
			let unique_email_count = canonical_groups.len() as i32;
			let deduplicated_count = (canonical_groups.values().map(Vec::len).sum::<usize>()
				- canonical_groups.len()) as i32;
			let source_key: Option<String> = row.get("source_key");
			Ok(PreparedList {
				source_list_id: None,
				headers: rows
					.first()
					.map(|value| value.keys().cloned().collect())
					.unwrap_or_default(),
				email_column,
				rows,
				source_filename: format!("push-batch-{batch_id}.json"),
				source_name: source_key.unwrap_or_else(|| format!("push-batch-{batch_id}")),
				unique_email_count,
				deduplicated_count,
				canonical_groups,
				blank_indices,
				invalid_indices,
			})
		}
		_ => bail!("Pipeline source type is not implemented"),
	}
}

fn build_canonical_groups(
	rows: &[Map<String, Value>],
	email_column: &str,
) -> (HashMap<String, Vec<usize>>, Vec<usize>, Vec<usize>) {
	let mut canonical_groups: HashMap<String, Vec<usize>> = HashMap::new();
	let mut blank_indices = Vec::new();
	let mut invalid_indices = Vec::new();

	for (index, row) in rows.iter().enumerate() {
		let email = row
			.get(email_column)
			.and_then(Value::as_str)
			.unwrap_or_default()
			.trim()
			.to_string();
		if email.is_empty() {
			blank_indices.push(index);
			continue;
		}
		match canonicalize_email(&email) {
			Some(canonical) => canonical_groups.entry(canonical).or_default().push(index),
			None => invalid_indices.push(index),
		}
	}

	(canonical_groups, blank_indices, invalid_indices)
}

fn parse_original_rows(original_data: &Value) -> Result<Vec<Map<String, Value>>> {
	let original_map = original_data
		.as_object()
		.context("Source list is missing original_data")?;
	let mut indexed_rows = Vec::with_capacity(original_map.len());
	for (key, value) in original_map {
		let index = key
			.parse::<usize>()
			.with_context(|| format!("Invalid row index: {key}"))?;
		let object = value
			.as_object()
			.cloned()
			.context("Source list row is not an object")?;
		indexed_rows.push((index, object));
	}
	indexed_rows.sort_by_key(|(index, _)| *index);
	Ok(indexed_rows.into_iter().map(|(_, row)| row).collect())
}

fn build_original_data(rows: &[Map<String, Value>]) -> Value {
	let mut root = Map::new();
	for (index, row) in rows.iter().enumerate() {
		root.insert(index.to_string(), Value::Object(row.clone()));
	}
	Value::Object(root)
}

async fn apply_delta_selection(
	pg_pool: &PgPool,
	pipeline_id: i64,
	verification: &PipelineVerificationSettings,
	prepared: PreparedList,
) -> Result<(PreparedList, DeltaSelectionSummary)> {
	if !verification.delta_mode {
		let selected_unique_emails = prepared.unique_email_count;
		return Ok((
			prepared,
			DeltaSelectionSummary {
				selected_unique_emails,
				skipped_unchanged: 0,
			},
		));
	}

	let canonical_emails: Vec<String> = prepared.canonical_groups.keys().cloned().collect();
	if canonical_emails.is_empty() {
		return Ok((prepared, DeltaSelectionSummary::default()));
	}

	let rows = sqlx::query(
		r#"
		SELECT canonical_email, source_hash, last_verified_at
		FROM v1_pipeline_contact_state
		WHERE pipeline_id = $1 AND canonical_email = ANY($2)
		"#,
	)
	.bind(pipeline_id)
	.bind(&canonical_emails)
	.fetch_all(pg_pool)
	.await?;

	let mut state_by_email = HashMap::with_capacity(rows.len());
	for row in rows {
		let state = PipelineContactStateRow {
			canonical_email: row.get("canonical_email"),
			source_hash: row.get("source_hash"),
			last_verified_at: row.get("last_verified_at"),
		};
		state_by_email.insert(state.canonical_email.clone(), state);
	}

	let freshness_cutoff = verification
		.freshness_days
		.map(|days| Utc::now() - Duration::days(i64::from(days.max(0))));

	let mut selected_indices = Vec::new();
	let mut skipped_unchanged = 0i32;
	for (canonical_email, indices) in &prepared.canonical_groups {
		let current_hash = hash_canonical_group(&prepared.rows, indices)?;
		let state = state_by_email.get(canonical_email);
		let stale = freshness_cutoff.is_some_and(|cutoff| {
			state
				.and_then(|value| value.last_verified_at)
				.map(|verified_at| verified_at < cutoff)
				.unwrap_or(true)
		});
		let changed = state
			.map(|value| value.source_hash != current_hash)
			.unwrap_or(true);

		if changed || stale {
			selected_indices.extend(indices.iter().copied());
		} else {
			skipped_unchanged += 1;
		}
	}

	selected_indices.extend(prepared.blank_indices.iter().copied());
	selected_indices.extend(prepared.invalid_indices.iter().copied());
	selected_indices.sort_unstable();
	selected_indices.dedup();

	let selected_rows = selected_indices
		.into_iter()
		.map(|index| prepared.rows[index].clone())
		.collect::<Vec<_>>();
	let (canonical_groups, blank_indices, invalid_indices) =
		build_canonical_groups(&selected_rows, &prepared.email_column);
	let unique_email_count = canonical_groups.len() as i32;
	let deduplicated_count =
		(canonical_groups.values().map(|v| v.len()).sum::<usize>() - canonical_groups.len()) as i32;

	Ok((
		PreparedList {
			source_list_id: prepared.source_list_id,
			headers: prepared.headers,
			email_column: prepared.email_column,
			rows: selected_rows,
			source_filename: prepared.source_filename,
			source_name: prepared.source_name,
			unique_email_count,
			deduplicated_count,
			canonical_groups,
			blank_indices,
			invalid_indices,
		},
		DeltaSelectionSummary {
			selected_unique_emails: unique_email_count,
			skipped_unchanged,
		},
	))
}

fn hash_canonical_group(rows: &[Map<String, Value>], indices: &[usize]) -> Result<String> {
	let mut hasher = Sha256::new();
	for index in indices {
		hash_row(&mut hasher, &rows[*index])?;
		hasher.update(b"\n");
	}
	Ok(hex::encode(hasher.finalize()))
}

fn hash_row(hasher: &mut Sha256, row: &Map<String, Value>) -> Result<()> {
	let mut keys = row.keys().cloned().collect::<Vec<_>>();
	keys.sort();
	for key in keys {
		hasher.update(key.as_bytes());
		hasher.update(b"=");
		hasher.update(serde_json::to_vec(
			row.get(&key).context("missing row value while hashing")?,
		)?);
		hasher.update(b";");
	}
	Ok(())
}

async fn deliver_pipeline_run_summary(
	delivery: &PipelineDeliveryConfig,
	default_webhook_url: Option<String>,
	webhook_signing_secret: Option<String>,
	run_id: i64,
	pipeline_id: i64,
	job_id: i32,
	list_id: Option<i32>,
	final_status: &PipelineRunStatus,
	stats: &Value,
) -> Result<()> {
	let webhook = delivery
		.webhook
		.as_ref()
		.map(|webhook| (webhook.url.clone(), webhook.headers.clone()))
		.or_else(|| default_webhook_url.map(|url| (url, HashMap::new())));
	let Some((url, headers)) = webhook else {
		return Ok(());
	};

	let payload = json!({
		"pipeline_id": pipeline_id,
		"run_id": run_id,
		"job_id": job_id,
		"list_id": list_id,
		"status": final_status,
		"stats": stats,
		"download_url": list_id.map(|list_id| format!("/v1/lists/{list_id}/download")),
	});

	let client = validated_webhook_client(&url).await?;
	let mut request = client.post(url).json(&payload);
	for (name, value) in headers {
		request = request.header(&name, value);
	}
	if let Some(secret) = webhook_signing_secret {
		let body = serde_json::to_vec(&payload)?;
		request = request.header(WEBHOOK_SIGNATURE_HEADER, sign_payload(&secret, &body));
	}

	let response = request.send().await?;
	if !response.status().is_success() {
		bail!("Pipeline delivery webhook returned {}", response.status());
	}
	Ok(())
}

async fn attempt_pipeline_run_delivery(pg_pool: &PgPool, run_id: i64, force: bool) -> Result<()> {
	let row = sqlx::query(
		r#"
		SELECT pr.id, pr.pipeline_id, pr.status::TEXT, pr.job_id, pr.list_id, pr.stats,
		       pr.delivery_status::TEXT, pr.delivery_attempts, pr.next_delivery_attempt_at,
		       p.delivery_config, t.default_webhook_url, t.webhook_signing_secret
		FROM v1_pipeline_runs pr
		INNER JOIN v1_pipelines p ON p.id = pr.pipeline_id
		INNER JOIN tenants t ON t.id = pr.tenant_id
		WHERE pr.id = $1
		"#,
	)
	.bind(run_id)
	.fetch_optional(pg_pool)
	.await?;
	let Some(row) = row else {
		return Ok(());
	};

	let run_status = parse_pipeline_run_status(&row.get::<String, _>("status"))?;
	if !run_status.is_terminal() {
		return Ok(());
	}

	let delivery_status = parse_pipeline_delivery_status(&row.get::<String, _>("delivery_status"))?;
	let next_delivery_attempt_at: Option<DateTime<Utc>> = row.get("next_delivery_attempt_at");
	let delivery: PipelineDeliveryConfig =
		serde_json::from_value(row.get("delivery_config")).unwrap_or_default();
	let default_webhook_url: Option<String> = row.get("default_webhook_url");
	let has_webhook_target = delivery.webhook.is_some() || default_webhook_url.is_some();
	if !has_webhook_target {
		if matches!(
			delivery_status,
			PipelineDeliveryStatus::Delivered | PipelineDeliveryStatus::Failed
		) {
			return Ok(());
		}
		sqlx::query(
			r#"
			UPDATE v1_pipeline_runs
			SET delivery_status = 'not_requested'::pipeline_delivery_status,
			    next_delivery_attempt_at = NULL,
			    delivery_error = NULL,
			    updated_at = NOW()
			WHERE id = $1
			"#,
		)
		.bind(run_id)
		.execute(pg_pool)
		.await?;
		return Ok(());
	}

	if !force {
		match delivery_status {
			PipelineDeliveryStatus::Delivered
			| PipelineDeliveryStatus::Failed
			| PipelineDeliveryStatus::RetryScheduled => return Ok(()),
			PipelineDeliveryStatus::Pending if next_delivery_attempt_at.is_some() => return Ok(()),
			PipelineDeliveryStatus::Pending | PipelineDeliveryStatus::NotRequested => {}
		}
	}

	let attempt_number = row.get::<i32, _>("delivery_attempts") + 1;
	let in_flight_retry_at =
		Utc::now() + Duration::seconds(i64::from(delivery.retry_backoff_seconds.max(1)));
	let claim_result = if force {
		sqlx::query(
			r#"
			UPDATE v1_pipeline_runs
			SET delivery_status = 'pending'::pipeline_delivery_status,
			    next_delivery_attempt_at = $2,
			    updated_at = NOW()
			WHERE id = $1
			RETURNING id
			"#,
		)
		.bind(run_id)
		.bind(in_flight_retry_at)
		.fetch_optional(pg_pool)
		.await?
	} else {
		sqlx::query(
			r#"
			UPDATE v1_pipeline_runs
			SET delivery_status = 'pending'::pipeline_delivery_status,
			    next_delivery_attempt_at = $2,
			    updated_at = NOW()
			WHERE id = $1
			  AND delivery_status = $3::pipeline_delivery_status
			  AND next_delivery_attempt_at IS NOT DISTINCT FROM $4
			RETURNING id
			"#,
		)
		.bind(run_id)
		.bind(in_flight_retry_at)
		.bind(enum_str(&delivery_status))
		.bind(next_delivery_attempt_at)
		.fetch_optional(pg_pool)
		.await?
	};
	if claim_result.is_none() {
		return Ok(());
	}

	let delivery_result = deliver_pipeline_run_summary(
		&delivery,
		default_webhook_url,
		row.get("webhook_signing_secret"),
		run_id,
		row.get("pipeline_id"),
		row.get::<Option<i32>, _>("job_id").unwrap_or_default(),
		row.get("list_id"),
		&run_status,
		&row.get::<Value, _>("stats"),
	)
	.await;

	match delivery_result {
		Ok(()) => {
			sqlx::query(
				r#"
				UPDATE v1_pipeline_runs
				SET delivery_status = 'delivered'::pipeline_delivery_status,
				    delivery_attempts = $2,
				    last_delivery_attempt_at = NOW(),
				    next_delivery_attempt_at = NULL,
				    delivery_error = NULL,
				    updated_at = NOW()
				WHERE id = $1
				"#,
			)
			.bind(run_id)
			.bind(attempt_number)
			.execute(pg_pool)
			.await?;
		}
		Err(err) => {
			let retry_delay_seconds = compute_delivery_retry_delay_seconds(
				delivery.retry_backoff_seconds,
				attempt_number,
			);
			let should_retry = attempt_number < delivery.max_attempts;
			sqlx::query(
				r#"
				UPDATE v1_pipeline_runs
				SET delivery_status = $2::pipeline_delivery_status,
				    delivery_attempts = $3,
				    last_delivery_attempt_at = NOW(),
				    next_delivery_attempt_at = $4,
				    delivery_error = $5,
				    updated_at = NOW()
				WHERE id = $1
				"#,
			)
			.bind(run_id)
			.bind(if should_retry {
				"retry_scheduled"
			} else {
				"failed"
			})
			.bind(attempt_number)
			.bind(if should_retry {
				Some(Utc::now() + Duration::seconds(i64::from(retry_delay_seconds)))
			} else {
				None
			})
			.bind(err.to_string())
			.execute(pg_pool)
			.await?;
		}
	}

	Ok(())
}

fn compute_delivery_retry_delay_seconds(base_delay_seconds: i32, attempt_number: i32) -> i32 {
	let base_delay_seconds = base_delay_seconds.clamp(1, 86_400);
	let exponent = attempt_number.saturating_sub(1).min(6) as u32;
	let multiplier = 2i32.saturating_pow(exponent);
	base_delay_seconds
		.saturating_mul(multiplier)
		.clamp(base_delay_seconds, 86_400)
}

async fn update_pipeline_contact_state_for_run(pg_pool: &PgPool, run_id: i64) -> Result<()> {
	let run_row = sqlx::query(
		r#"
		SELECT pr.pipeline_id, pr.list_id, pr.completed_at, l.email_column, l.original_data
		FROM v1_pipeline_runs pr
		INNER JOIN v1_lists l ON l.id = pr.list_id
		WHERE pr.id = $1
		"#,
	)
	.bind(run_id)
	.fetch_optional(pg_pool)
	.await?;
	let Some(run_row) = run_row else {
		return Ok(());
	};

	let pipeline_id: i64 = run_row.get("pipeline_id");
	let list_id: i32 = run_row.get("list_id");
	let completed_at: DateTime<Utc> = run_row
		.get::<Option<DateTime<Utc>>, _>("completed_at")
		.unwrap_or_else(Utc::now);
	let email_column: String = run_row.get("email_column");
	let original_data: Value = run_row.get("original_data");
	let rows = parse_original_rows(&original_data)?;
	let (canonical_groups, _, _) = build_canonical_groups(&rows, &email_column);
	if canonical_groups.is_empty() {
		return Ok(());
	}

	let task_rows = sqlx::query(
		r#"
		SELECT id, canonical_email, completed_at
		FROM v1_task_result
		WHERE job_id = (
			SELECT job_id FROM v1_pipeline_runs WHERE id = $1
		)
		  AND is_duplicate = false
		  AND canonical_email IS NOT NULL
		  AND task_state = 'completed'::task_state
		  AND result IS NOT NULL
		"#,
	)
	.bind(run_id)
	.fetch_all(pg_pool)
	.await?;
	let mut task_by_canonical = HashMap::with_capacity(task_rows.len());
	for row in task_rows {
		let canonical_email: String = row.get("canonical_email");
		let task_id: i32 = row.get("id");
		// When the address was verified, not when the run was finalized: a
		// stalled run can be finalized after a newer run of the same pipeline.
		let verified_at = row
			.get::<Option<DateTime<Utc>>, _>("completed_at")
			.unwrap_or(completed_at);
		task_by_canonical.insert(canonical_email, (task_id, verified_at));
	}

	for (canonical_email, indices) in canonical_groups {
		let Some((task_id, verified_at)) = task_by_canonical.get(&canonical_email).copied() else {
			continue;
		};
		let source_hash = hash_canonical_group(&rows, &indices)?;
		sqlx::query(
			r#"
			INSERT INTO v1_pipeline_contact_state (
				pipeline_id, canonical_email, source_hash, last_run_id,
				last_result_task_id, last_seen_at, last_verified_at
			)
			VALUES ($1, $2, $3, $4, $5, $6, $6)
			ON CONFLICT (pipeline_id, canonical_email)
			DO UPDATE SET
				source_hash = EXCLUDED.source_hash,
				last_run_id = EXCLUDED.last_run_id,
				last_result_task_id = EXCLUDED.last_result_task_id,
				last_seen_at = EXCLUDED.last_seen_at,
				last_verified_at = EXCLUDED.last_verified_at,
				updated_at = NOW()
			-- Runs can finish out of order (a forced run, a recovered run); an
			-- older run must not replace the state a newer one recorded.
			WHERE v1_pipeline_contact_state.last_verified_at IS NULL
			   OR v1_pipeline_contact_state.last_verified_at <= EXCLUDED.last_verified_at
			"#,
		)
		.bind(pipeline_id)
		.bind(canonical_email)
		.bind(source_hash)
		.bind(run_id)
		.bind(task_id)
		.bind(verified_at)
		.execute(pg_pool)
		.await?;
	}

	// Keep the associated list fresh for changed-only exports and downstream downloads.
	sqlx::query(
		"UPDATE v1_lists SET completed_at = COALESCE(completed_at, NOW()), updated_at = NOW() WHERE id = $1",
	)
	.bind(list_id)
	.execute(pg_pool)
	.await?;

	Ok(())
}

async fn build_pipeline_run_stats(
	pg_pool: &PgPool,
	list_id: Option<i32>,
	job_id: i32,
) -> Result<Value> {
	let row = sqlx::query(
		r#"
		SELECT
			COUNT(*) FILTER (WHERE score_category = 'valid') AS total_valid,
			COUNT(*) FILTER (WHERE score_category = 'risky') AS total_risky,
			COUNT(*) FILTER (WHERE score_category = 'unknown') AS total_unknown,
			COUNT(*) FILTER (WHERE score_category = 'invalid') AS total_invalid,
			COUNT(*) FILTER (WHERE result IS NOT NULL OR error IS NOT NULL OR task_state = 'cancelled') AS total_processed,
			COUNT(*) FILTER (WHERE task_state IN ('failed', 'dead_lettered')) AS total_failed
		FROM v1_task_result
		WHERE job_id = $1
		"#,
	)
	.bind(job_id)
	.fetch_one(pg_pool)
	.await?;

	// A run that failed before its usage was committed has no usage event linked
	// to the job; that bills nothing rather than failing every read of the run.
	let billed_emails = sqlx::query_scalar::<_, Option<i32>>(
		"SELECT committed_emails FROM v1_usage_events WHERE job_id = $1 ORDER BY id DESC LIMIT 1",
	)
	.bind(job_id)
	.fetch_optional(pg_pool)
	.await?
	.flatten()
	.unwrap_or(0);

	Ok(json!({
		"job_id": job_id,
		"list_id": list_id,
		"total_valid": row.get::<Option<i64>, _>("total_valid").unwrap_or(0),
		"total_risky": row.get::<Option<i64>, _>("total_risky").unwrap_or(0),
		"total_unknown": row.get::<Option<i64>, _>("total_unknown").unwrap_or(0),
		"total_invalid": row.get::<Option<i64>, _>("total_invalid").unwrap_or(0),
		"total_processed": row.get::<Option<i64>, _>("total_processed").unwrap_or(0),
		"total_failed": row.get::<Option<i64>, _>("total_failed").unwrap_or(0),
		"billed_emails": billed_emails,
	}))
}

fn merge_pipeline_run_stats(existing: Value, computed: Value) -> Value {
	let mut merged = existing.as_object().cloned().unwrap_or_default();
	if let Some(computed_map) = computed.as_object() {
		for (key, value) in computed_map {
			merged.insert(key.clone(), value.clone());
		}
	}
	Value::Object(merged)
}

/// How far past its first run a schedule is checked for gaps. Day-of-month
/// and month fields only repeat yearly, and daylight-saving changes happen
/// within a year, so a year and a week covers every gap, including the one
/// into the next cycle. An hourly schedule fires about 8,900 times in that
/// span, under the run cap below.
const SCHEDULE_VALIDATION_HORIZON_DAYS: i64 = 366 + 7;
/// Upper bound on run times inspected, so validating a dense schedule stays cheap.
const SCHEDULE_VALIDATION_MAX_RUNS: usize = 10_000;

/// Rejects schedules whose runs are ever closer than `min_interval_seconds`.
///
/// Every gap in the cycle is checked, not just the next one, so whether a
/// schedule is accepted does not depend on the time of day it is saved.
fn validate_cron_expression(
	expression: &str,
	timezone: &str,
	min_interval_seconds: i32,
	now_utc: DateTime<Utc>,
) -> Result<()> {
	let tz = parse_timezone(timezone)?;
	let schedule = parse_schedule(expression)?;
	let mut upcoming = schedule
		.after(&now_utc.with_timezone(&tz))
		.take(SCHEDULE_VALIDATION_MAX_RUNS);
	let first = upcoming
		.next()
		.context("Cron expression does not produce any future run times")?;
	let horizon = first + Duration::days(SCHEDULE_VALIDATION_HORIZON_DAYS);
	let mut previous = first;
	let mut has_second_run = false;
	for next in upcoming {
		has_second_run = true;
		if next.signed_duration_since(previous).num_seconds() < i64::from(min_interval_seconds) {
			bail!(
				"Cron schedule must be at least {} seconds apart",
				min_interval_seconds
			);
		}
		if next > horizon {
			break;
		}
		previous = next;
	}
	if !has_second_run {
		bail!("Cron expression must produce at least two future run times");
	}
	Ok(())
}

/// The first run of the schedule after `now_utc`.
///
/// This does not apply the minimum-interval policy: that is enforced when a
/// schedule is saved, and re-checking it here would pause a pipeline whose
/// schedule was accepted earlier.
pub fn compute_next_run_at(
	expression: &str,
	timezone: &str,
	now_utc: DateTime<Utc>,
) -> Result<DateTime<Utc>> {
	let tz = parse_timezone(timezone)?;
	let now = now_utc.with_timezone(&tz);
	let schedule = parse_schedule(expression)?;
	let next = schedule
		.after(&now)
		.next()
		.context("Cron expression did not produce a next run")?;
	Ok(next.with_timezone(&Utc))
}

/// Takes a connection rather than the pool so `update_pipeline` can validate on
/// the transaction that holds the pipeline row lock instead of waiting for a
/// second pooled connection while holding it.
async fn validate_pipeline_input(
	conn: &mut sqlx::PgConnection,
	tenant_id: Uuid,
	source: &PipelineSource,
	schedule: &PipelineSchedule,
	policy: &PipelinePolicyConfig,
	delivery: &PipelineDeliveryConfig,
	min_interval_seconds: i32,
) -> std::result::Result<(), PipelineRequestError> {
	validate_cron_expression(
		&schedule.cron,
		&schedule.timezone,
		min_interval_seconds,
		Utc::now(),
	)
	.map_err(|err| PipelineRequestError::validation(err.to_string()))?;
	if policy.missed_run_window_hours < 1 {
		return Err(PipelineRequestError::validation(
			"Pipeline missed_run_window_hours must be at least 1",
		));
	}
	if let Some(policy_id) = policy.policy_id {
		let exists: bool = sqlx::query_scalar(
			"SELECT EXISTS(SELECT 1 FROM v1_score_policies WHERE id = $1 AND tenant_id = $2)",
		)
		.bind(policy_id)
		.bind(tenant_id)
		.fetch_one(&mut *conn)
		.await?;
		if !exists {
			return Err(PipelineRequestError::validation(
				"Referenced score policy does not exist for this tenant",
			));
		}
	}
	if delivery.max_attempts < 1 {
		return Err(PipelineRequestError::validation(
			"Pipeline delivery max_attempts must be at least 1",
		));
	}
	if delivery.retry_backoff_seconds < 1 {
		return Err(PipelineRequestError::validation(
			"Pipeline delivery retry_backoff_seconds must be at least 1",
		));
	}
	if let Some(webhook) = &delivery.webhook {
		validate_webhook_url(&webhook.url)
			.map_err(|err| PipelineRequestError::validation(err.to_string()))?;
	}
	match source {
		PipelineSource::ListSnapshot { list_id } => {
			let exists: bool = sqlx::query_scalar::<_, bool>(
				"SELECT EXISTS(SELECT 1 FROM v1_lists WHERE id = $1 AND tenant_id = $2 AND status <> 'deleted'::list_status)",
			)
			.bind(*list_id)
			.bind(tenant_id)
			.fetch_one(&mut *conn)
			.await?;
			if !exists {
				return Err(PipelineRequestError::validation(
					"Referenced source list does not exist for this tenant",
				));
			}
		}
		PipelineSource::Push {
			accepted_format, ..
		} => {
			if accepted_format.trim().to_ascii_lowercase() != "json" {
				return Err(PipelineRequestError::validation(
					"Push pipeline accepted_format must be json",
				));
			}
		}
		_ => {
			return Err(PipelineRequestError::validation(
				"Only list_snapshot and push sources are supported",
			));
		}
	}
	Ok(())
}

pub(crate) fn validate_webhook_url(url: &str) -> Result<()> {
	let parsed = reqwest::Url::parse(url).context("Invalid webhook URL")?;
	match parsed.scheme() {
		"https" => {}
		"http" => {
			bail!("Webhook URL must use HTTPS. HTTP is not allowed.");
		}
		other => bail!("Webhook URL scheme '{}' is not allowed. Use HTTPS.", other),
	}
	if let Some(host) = parsed.host_str() {
		if host == "localhost"
			|| host == "127.0.0.1"
			|| host == "::1"
			|| host == "0.0.0.0"
			|| host.ends_with(".local")
			|| host == "[::1]"
		{
			bail!("Webhook URL must not target localhost or loopback addresses");
		}
		if let Ok(ip) = host
			.trim_start_matches('[')
			.trim_end_matches(']')
			.parse::<std::net::IpAddr>()
		{
			if is_private_ip(ip) {
				bail!("Webhook URL must not target private or reserved IP addresses");
			}
		}
	} else {
		bail!("Webhook URL must have a valid host");
	}
	Ok(())
}

pub(crate) async fn validate_resolved_webhook_target(
	url: &str,
) -> Result<Vec<std::net::SocketAddr>> {
	let parsed = reqwest::Url::parse(url).context("Invalid webhook URL")?;
	let host = parsed
		.host_str()
		.context("Webhook URL must have a valid host")?;
	let port = parsed
		.port_or_known_default()
		.context("Webhook URL must use a known port")?;
	let addrs = check_if_email_exists::network::resolve_public_addresses(
		host.trim_start_matches('[').trim_end_matches(']'),
		port,
	)
	.await
	.with_context(|| format!("Invalid webhook destination: {host}"))?;
	Ok(addrs)
}

pub(crate) async fn validated_webhook_client(url: &str) -> Result<reqwest::Client> {
	validate_webhook_url(url)?;
	let addrs = validate_resolved_webhook_target(url).await?;
	let parsed = reqwest::Url::parse(url)?;
	pinned_webhook_client(parsed.host_str().context("Missing webhook host")?, &addrs)
}

fn pinned_webhook_client(host: &str, addrs: &[std::net::SocketAddr]) -> Result<reqwest::Client> {
	Ok(reqwest::Client::builder()
		.no_proxy()
		.timeout(std::time::Duration::from_secs(10))
		.redirect(reqwest::redirect::Policy::none())
		.resolve_to_addrs(host.trim_start_matches('[').trim_end_matches(']'), addrs)
		.build()?)
}

/// A five-field cron schedule with standard day semantics.
///
/// When both day-of-month and day-of-week are restricted, standard cron runs
/// on days that match either field, but the `cron` crate requires both, so
/// "0 9 1 * 1" would only fire on Mondays that fall on the 1st. Such an
/// expression is split into one schedule per day field, and their run times
/// are merged in order.
struct CronSchedule(Vec<Schedule>);

impl CronSchedule {
	fn after<'a, Z>(&'a self, after: &DateTime<Z>) -> impl Iterator<Item = DateTime<Z>> + 'a
	where
		Z: chrono::TimeZone + 'a,
	{
		let mut runs: Vec<_> = self
			.0
			.iter()
			.map(|schedule| schedule.after(after).peekable())
			.collect();
		std::iter::from_fn(move || {
			let earliest = runs
				.iter_mut()
				.filter_map(|run| run.peek().cloned())
				.min()?;
			// A day matching both fields fires once.
			for run in runs.iter_mut() {
				while run.peek() == Some(&earliest) {
					run.next();
				}
			}
			Some(earliest)
		})
	}
}

fn parse_schedule(expression: &str) -> Result<CronSchedule> {
	let parts: Vec<&str> = expression.split_whitespace().collect();
	let [minute, hour, day_of_month, month, day_of_week] = parts.as_slice() else {
		bail!("Cron expression must use standard 5-field syntax");
	};
	let day_of_week = translate_day_of_week_field(day_of_week)?;
	let build = |day_of_month: &str, day_of_week: &str| {
		Schedule::from_str(&format!(
			"0 {minute} {hour} {day_of_month} {month} {day_of_week} *"
		))
		.context("Invalid cron expression")
	};
	let unrestricted = |field: &str| field == "*" || field == "?";
	if unrestricted(day_of_month) || unrestricted(&day_of_week) {
		return Ok(CronSchedule(vec![build(day_of_month, &day_of_week)?]));
	}
	Ok(CronSchedule(vec![
		build(day_of_month, "*")?,
		build("*", &day_of_week)?,
	]))
}

/// Rewrites a standard day-of-week field for the `cron` crate.
///
/// Standard cron numbers days 0-7 with 0 and 7 both Sunday and 1 Monday; the
/// crate numbers them 1 (Sunday) to 7 (Saturday). Passing numbers through made
/// "0 9 * * 1" run on Sundays and rejected "0". Numeric items, ranges and steps
/// are expanded to the crate's numbers; day names already mean the same day to
/// both and pass through unchanged.
fn translate_day_of_week_field(field: &str) -> Result<String> {
	let items = field
		.split(',')
		.map(translate_day_of_week_item)
		.collect::<Result<Vec<_>>>()?;
	Ok(items.join(","))
}

fn translate_day_of_week_item(item: &str) -> Result<String> {
	if item == "*" || item == "?" || item.chars().any(|c| c.is_ascii_alphabetic()) {
		return Ok(item.to_string());
	}
	let (base, step) = match item.split_once('/') {
		Some((base, step)) => (base, parse_cron_number(step, "step")?),
		None => (item, 1),
	};
	if step == 0 {
		bail!("Day-of-week step must be at least 1");
	}
	let (first, last) = match base.split_once('-') {
		Some((first, last)) => (parse_day_of_week(first)?, parse_day_of_week(last)?),
		None if base == "*" || base == "?" => (0, 7),
		// "N/step" runs from N to the end of the week, as in cronie.
		None if item.contains('/') => (parse_day_of_week(base)?, 7),
		None => {
			let day = parse_day_of_week(base)?;
			(day, day)
		}
	};
	if first > last {
		bail!("Day-of-week range {} runs backwards", base);
	}
	let mut days: Vec<u32> = (first..=last)
		.step_by(step as usize)
		.map(|day| day % 7 + 1)
		.collect();
	days.sort_unstable();
	days.dedup();
	Ok(days
		.iter()
		.map(u32::to_string)
		.collect::<Vec<_>>()
		.join(","))
}

fn parse_day_of_week(value: &str) -> Result<u32> {
	let day = parse_cron_number(value, "day of week")?;
	if day > 7 {
		bail!("Day of week must be between 0 and 7, got {}", day);
	}
	Ok(day)
}

fn parse_cron_number(value: &str, what: &str) -> Result<u32> {
	value
		.parse()
		.with_context(|| format!("Invalid {} in cron expression: {:?}", what, value))
}

fn parse_timezone(timezone: &str) -> Result<Tz> {
	Tz::from_str(timezone).with_context(|| format!("Invalid timezone: {}", timezone))
}

fn enum_str<T: Serialize>(value: &T) -> String {
	serde_json::to_string(value)
		.unwrap_or_else(|_| "\"unknown\"".to_string())
		.trim_matches('"')
		.to_string()
}

fn parse_pipeline_status(value: &str) -> Result<PipelineStatus> {
	match value {
		"active" => Ok(PipelineStatus::Active),
		"paused" => Ok(PipelineStatus::Paused),
		"deleted" => Ok(PipelineStatus::Deleted),
		_ => bail!("Unknown pipeline status: {}", value),
	}
}

fn parse_pipeline_run_status(value: &str) -> Result<PipelineRunStatus> {
	match value {
		"queued" => Ok(PipelineRunStatus::Queued),
		"preparing" => Ok(PipelineRunStatus::Preparing),
		"fetching_source" => Ok(PipelineRunStatus::FetchingSource),
		"publishing" => Ok(PipelineRunStatus::Publishing),
		"running" => Ok(PipelineRunStatus::Running),
		"delivering" => Ok(PipelineRunStatus::Delivering),
		"completed" => Ok(PipelineRunStatus::Completed),
		"failed" => Ok(PipelineRunStatus::Failed),
		"cancelled" => Ok(PipelineRunStatus::Cancelled),
		"skipped" => Ok(PipelineRunStatus::Skipped),
		_ => bail!("Unknown pipeline run status: {}", value),
	}
}

fn parse_pipeline_delivery_status(value: &str) -> Result<PipelineDeliveryStatus> {
	match value {
		"not_requested" => Ok(PipelineDeliveryStatus::NotRequested),
		"pending" => Ok(PipelineDeliveryStatus::Pending),
		"delivered" => Ok(PipelineDeliveryStatus::Delivered),
		"retry_scheduled" => Ok(PipelineDeliveryStatus::RetryScheduled),
		"failed" => Ok(PipelineDeliveryStatus::Failed),
		_ => bail!("Unknown pipeline delivery status: {}", value),
	}
}

fn row_to_pipeline_view(row: &sqlx::postgres::PgRow) -> Result<PipelineView> {
	Ok(PipelineView {
		id: row.get("id"),
		tenant_id: row.get("tenant_id"),
		name: row.get("name"),
		status: parse_pipeline_status(&row.get::<String, _>("status"))?,
		source: serde_json::from_value(row.get("source_config"))
			.context("invalid source config")?,
		schedule: PipelineSchedule {
			cron: row.get("schedule_cron"),
			timezone: row.get("schedule_timezone"),
		},
		verification: serde_json::from_value(row.get("verification_settings")).unwrap_or_default(),
		policy: serde_json::from_value(row.get("policy_config")).unwrap_or_default(),
		delivery: serde_json::from_value(row.get("delivery_config")).unwrap_or_default(),
		next_run_at: row.get("next_run_at"),
		last_scheduled_at: row.get("last_scheduled_at"),
		last_run_id: row.get("last_run_id"),
		created_at: row.get("created_at"),
		updated_at: row.get("updated_at"),
	})
}

fn row_to_pipeline_run_view(row: &sqlx::postgres::PgRow) -> Result<PipelineRunView> {
	Ok(PipelineRunView {
		id: row.get("id"),
		pipeline_id: row.get("pipeline_id"),
		tenant_id: row.get("tenant_id"),
		trigger_type: row.get("trigger_type"),
		status: parse_pipeline_run_status(&row.get::<String, _>("status"))?,
		scheduled_for: row.get("scheduled_for"),
		started_at: row.get("started_at"),
		completed_at: row.get("completed_at"),
		job_id: row.get("job_id"),
		list_id: row.get("list_id"),
		source_snapshot: row.get("source_snapshot"),
		stats: row.get("stats"),
		billed_emails: row.get("billed_emails"),
		result_location: row.get("result_location"),
		delivery_status: parse_pipeline_delivery_status(&row.get::<String, _>("delivery_status"))?,
		delivery_attempts: row.get("delivery_attempts"),
		last_delivery_attempt_at: row.get("last_delivery_attempt_at"),
		next_delivery_attempt_at: row.get("next_delivery_attempt_at"),
		delivery_error: row.get("delivery_error"),
		error_code: row.get("error_code"),
		error_message: row.get("error_message"),
		created_at: row.get("created_at"),
		updated_at: row.get("updated_at"),
	})
}

async fn has_active_run(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	pipeline_id: i64,
) -> Result<bool> {
	let count: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*)
		FROM v1_pipeline_runs
		WHERE pipeline_id = $1
		  AND status IN (
			'queued'::pipeline_run_status,
			'preparing'::pipeline_run_status,
			'fetching_source'::pipeline_run_status,
			'publishing'::pipeline_run_status,
			'running'::pipeline_run_status,
			'delivering'::pipeline_run_status
		  )
		"#,
	)
	.bind(pipeline_id)
	.fetch_one(&mut **tx)
	.await?;
	Ok(count > 0)
}

struct PipelineRowForUpdate {
	id: i64,
	tenant_id: Uuid,
	source_config: Value,
	schedule_cron: String,
	schedule_timezone: String,
	policy_config: Value,
	deleted_at: Option<DateTime<Utc>>,
}

async fn fetch_pipeline_row_for_update<'a>(
	tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
	pipeline_id: i64,
) -> Result<Option<PipelineRowForUpdate>> {
	let row = sqlx::query(
		r#"
		SELECT id, tenant_id, source_config, schedule_cron, schedule_timezone, policy_config, deleted_at
		FROM v1_pipelines
		WHERE id = $1
		FOR UPDATE
		"#,
	)
	.bind(pipeline_id)
	.fetch_optional(&mut **tx)
	.await?;

	let Some(row) = row else {
		return Ok(None);
	};

	Ok(Some(PipelineRowForUpdate {
		id: row.get("id"),
		tenant_id: row.get("tenant_id"),
		source_config: row.get("source_config"),
		schedule_cron: row.get("schedule_cron"),
		schedule_timezone: row.get("schedule_timezone"),
		policy_config: row.get("policy_config"),
		deleted_at: row.get("deleted_at"),
	}))
}

fn invalid_syntax_email_result(to_email: &str) -> Value {
	json!({
		"input": to_email,
		"is_reachable": Reachable::Invalid,
		"score": {
			"score": 0,
			"category": "invalid",
			"sub_reason": "invalid_syntax",
			"safe_to_send": false,
			"reason_codes": ["invalid_syntax"]
		}
	})
}

fn blank_email_result() -> Value {
	invalid_syntax_email_result("")
}

fn sanitized_delivery_metadata(delivery: &PipelineDeliveryConfig) -> Value {
	json!({
		"dashboard": delivery.dashboard,
		"has_webhook": delivery.webhook.is_some(),
		"max_attempts": delivery.max_attempts,
		"retry_backoff_seconds": delivery.retry_backoff_seconds,
	})
}

fn parse_pipeline_source_snapshot(source_snapshot: &Value) -> Result<PipelineSource> {
	let payload = source_snapshot
		.get("source")
		.cloned()
		.unwrap_or_else(|| source_snapshot.clone());
	serde_json::from_value(payload).context("invalid pipeline source snapshot")
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::{TimeZone, Timelike};

	#[tokio::test]
	async fn webhook_connection_uses_the_validated_address_without_another_dns_lookup() {
		use tokio::io::{AsyncReadExt, AsyncWriteExt};
		let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
		let address = listener.local_addr().unwrap();
		let server = tokio::spawn(async move {
			let (mut socket, _) = listener.accept().await.unwrap();
			let mut request = [0; 2048];
			let count = socket.read(&mut request).await.unwrap();
			assert!(String::from_utf8_lossy(&request[..count]).contains("rebind.invalid"));
			socket
				.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
				.await
				.unwrap();
		});
		// Exercise address pinning independently of the public-address validation.
		let client = pinned_webhook_client("rebind.invalid", &[address]).unwrap();
		let response = client
			.get(format!("http://rebind.invalid:{}/", address.port()))
			.send()
			.await
			.unwrap();
		assert_eq!(response.status(), reqwest::StatusCode::OK);
		server.await.unwrap();
		assert!(validated_webhook_client("https://127.0.0.1/hook")
			.await
			.is_err());
		assert!(validated_webhook_client("https://[::1]/hook")
			.await
			.is_err());
	}

	#[test]
	fn test_compute_next_run_at_accepts_five_field_cron() {
		let now = Utc.with_ymd_and_hms(2026, 3, 28, 12, 0, 0).unwrap();
		let next = compute_next_run_at("0 13 * * *", "UTC", now).unwrap();
		assert!(next > now);
		assert_eq!(next.date_naive(), now.date_naive());
		assert_eq!(next.hour(), 13);
		assert_eq!(next.minute(), 0);
		assert_eq!(next.second(), 0);
	}

	#[test]
	fn test_compute_next_run_at_rolls_to_next_day() {
		let now = Utc.with_ymd_and_hms(2026, 3, 28, 13, 30, 0).unwrap();
		let next = compute_next_run_at("0 13 * * *", "UTC", now).unwrap();
		assert!(next > now);
		assert_eq!(
			next.date_naive(),
			(now + chrono::Duration::days(1)).date_naive()
		);
		assert_eq!(next.hour(), 13);
		assert_eq!(next.minute(), 0);
		assert_eq!(next.second(), 0);
	}

	#[test]
	fn test_compute_delivery_retry_delay_clamps_large_base_delay() {
		assert_eq!(compute_delivery_retry_delay_seconds(120_000, 1), 86_400);
		assert_eq!(compute_delivery_retry_delay_seconds(120_000, 4), 86_400);
	}

	#[test]
	fn test_validate_cron_expression_rejects_too_frequent_schedule() {
		let err = validate_cron_expression("* * * * *", "UTC", 3600, Utc::now()).unwrap_err();
		assert!(err.to_string().contains("at least 3600 seconds"));
	}

	/// Weekdays (1 = Monday .. 7 = Sunday) on which a schedule fires during the
	/// two weeks after Saturday 2026-03-28.
	fn fire_weekdays(expression: &str) -> Vec<u32> {
		use chrono::Datelike;
		let now = Utc.with_ymd_and_hms(2026, 3, 28, 12, 0, 0).unwrap();
		let mut days: Vec<u32> = parse_schedule(expression)
			.unwrap()
			.after(&now)
			.take_while(|fire| *fire < now + Duration::days(14))
			.map(|fire| fire.weekday().number_from_monday())
			.collect();
		days.sort_unstable();
		days.dedup();
		days
	}

	#[test]
	fn test_parse_schedule_uses_standard_day_of_week_numbers() {
		// Standard cron numbers days 0-7 with 0 and 7 both Sunday; the UI hint
		// promises "0 9 * * 1" runs on Monday.
		let cases: &[(&str, &[u32])] = &[
			("0 9 * * 1", &[1]),
			("0 9 * * 0", &[7]),
			("0 9 * * 7", &[7]),
			("0 9 * * 6", &[6]),
			("0 9 * * 1-5", &[1, 2, 3, 4, 5]),
			("0 9 * * 5-7", &[5, 6, 7]),
			("0 9 * * 0,3", &[3, 7]),
			("0 9 * * */2", &[2, 4, 6, 7]),
			("0 9 * * 1-5/2", &[1, 3, 5]),
			("0 9 * * 1,SAT", &[1, 6]),
			("0 9 * * MON", &[1]),
			("0 9 * * sun", &[7]),
			("0 9 * * MON-FRI", &[1, 2, 3, 4, 5]),
			("0 9 * * *", &[1, 2, 3, 4, 5, 6, 7]),
		];
		for (expression, weekdays) in cases {
			assert_eq!(
				fire_weekdays(expression),
				weekdays.to_vec(),
				"weekdays for {:?}",
				expression
			);
		}
		for invalid in ["0 9 * * 8", "0 9 * * 5-1", "0 9 * * 1/0", "0 9 * * 1-"] {
			assert!(
				parse_schedule(invalid).is_err(),
				"{:?} should be rejected",
				invalid
			);
		}
	}

	#[test]
	fn test_validate_cron_expression_sees_gaps_across_month_boundaries() {
		// Saved on Jan 31 after 09:00, the first runs are Feb 1 and Mar 1, a
		// month apart; the 1-day gap from Mar 31 to Apr 1 comes later.
		let saved_at = Utc.with_ymd_and_hms(2026, 1, 31, 12, 0, 0).unwrap();
		let two_days = 2 * 24 * 3600;
		assert!(validate_cron_expression("0 9 1,31 * *", "UTC", two_days, saved_at).is_err());
		assert!(validate_cron_expression("0 9 1,15 * *", "UTC", two_days, saved_at).is_ok());
	}

	#[test]
	fn test_parse_schedule_runs_when_either_day_field_matches() {
		use chrono::Datelike;
		let now = Utc.with_ymd_and_hms(2026, 3, 28, 12, 0, 0).unwrap();
		let fire_dates = |expression: &str| -> Vec<(u32, u32)> {
			parse_schedule(expression)
				.unwrap()
				.after(&now)
				.take_while(|fire| *fire < now + Duration::days(36))
				.map(|fire| (fire.month(), fire.day()))
				.collect()
		};

		// Standard cron ORs the two day fields when both are restricted: the
		// 1st of each month, and every Monday. Run times stay ordered and
		// a day matching both fields fires once.
		assert_eq!(
			fire_dates("0 9 1 * 1"),
			vec![(3, 30), (4, 1), (4, 6), (4, 13), (4, 20), (4, 27), (5, 1),]
		);
		// With one day field left as "*", the other alone decides.
		assert_eq!(fire_dates("0 9 1 * *"), vec![(4, 1), (5, 1)]);
		assert_eq!(fire_dates("0 9 * * 1").len(), 5);
	}

	#[test]
	fn test_validate_cron_expression_checks_every_gap_regardless_of_save_time() {
		// 09:00 and 10:00 are an hour apart. Checking only the next two runs
		// accepted this when saved between 09:00 and 10:00 (next runs 10:00 and
		// 09:00 tomorrow, 23 hours apart) and rejected it at other times.
		for hour in [8, 9, 12] {
			let now = Utc.with_ymd_and_hms(2026, 3, 28, hour, 30, 0).unwrap();
			assert!(
				validate_cron_expression("0 9,10 * * *", "UTC", 7200, now).is_err(),
				"accepted a one-hour gap when saved at {}",
				now
			);
		}

		// Monday and Tuesday runs are a day apart; saved on a Tuesday afternoon the
		// next two runs (Friday, Monday) are three days apart.
		let tuesday = Utc.with_ymd_and_hms(2026, 3, 31, 12, 0, 0).unwrap();
		assert!(validate_cron_expression("0 9 * * 1,2,5", "UTC", 2 * 86_400, tuesday).is_err());
		assert!(validate_cron_expression("0 9 * * 1,5", "UTC", 2 * 86_400, tuesday).is_ok());
		assert!(validate_cron_expression("0 9 * * 1", "UTC", 3600, tuesday).is_ok());
		assert!(validate_cron_expression("0 9 31 2 *", "UTC", 3600, tuesday).is_err());
	}

	#[test]
	fn test_build_canonical_groups_deduplicates_and_tracks_blank_and_invalid_rows() {
		let mut row_one = Map::new();
		row_one.insert(
			"email".to_string(),
			Value::String("User@gmail.com".to_string()),
		);
		let mut row_two = Map::new();
		row_two.insert(
			"email".to_string(),
			Value::String("u.s.e.r+tag@googlemail.com".to_string()),
		);
		let mut row_three = Map::new();
		row_three.insert("email".to_string(), Value::String("".to_string()));
		let mut row_four = Map::new();
		row_four.insert(
			"email".to_string(),
			Value::String("not-an-email".to_string()),
		);
		let rows = vec![row_one, row_two, row_three, row_four];
		let (groups, blanks, invalids) = build_canonical_groups(&rows, "email");
		assert_eq!(groups.len(), 1);
		assert_eq!(groups.get("user@gmail.com").unwrap().len(), 2);
		assert_eq!(blanks, vec![2]);
		assert_eq!(invalids, vec![3]);
	}

	#[test]
	fn test_compute_delivery_retry_delay_seconds_backoff_and_cap() {
		assert_eq!(compute_delivery_retry_delay_seconds(300, 1), 300);
		assert_eq!(compute_delivery_retry_delay_seconds(300, 2), 600);
		assert_eq!(compute_delivery_retry_delay_seconds(300, 3), 1200);
		assert_eq!(compute_delivery_retry_delay_seconds(300, 10), 19_200);
		assert_eq!(compute_delivery_retry_delay_seconds(20_000, 10), 86_400);
	}

	#[test]
	fn test_merge_pipeline_run_stats_preserves_phase_two_metadata() {
		let merged = merge_pipeline_run_stats(
			json!({
				"delta_mode": true,
				"skipped_unchanged": 4
			}),
			json!({
				"total_valid": 7,
				"billed_emails": 7
			}),
		);

		assert_eq!(merged["delta_mode"], true);
		assert_eq!(merged["skipped_unchanged"], 4);
		assert_eq!(merged["total_valid"], 7);
		assert_eq!(merged["billed_emails"], 7);
	}

	#[test]
	fn test_hash_canonical_group_is_stable_for_key_order() {
		let mut row_one = Map::new();
		row_one.insert("name".to_string(), Value::String("Alice".to_string()));
		row_one.insert(
			"email".to_string(),
			Value::String("alice@example.com".to_string()),
		);
		let mut row_two = Map::new();
		row_two.insert(
			"email".to_string(),
			Value::String("alice@example.com".to_string()),
		);
		row_two.insert("name".to_string(), Value::String("Alice".to_string()));
		let rows = vec![row_one, row_two];

		let hash_one = hash_canonical_group(&rows, &[0]).unwrap();
		let hash_two = hash_canonical_group(&rows, &[1]).unwrap();
		assert_eq!(hash_one, hash_two);
	}

	#[test]
	fn test_is_private_ip_covers_reserved_ipv6_ranges() {
		assert!(is_private_ip("fe80::1".parse().unwrap()));
		assert!(is_private_ip("fd00::1".parse().unwrap()));
		assert!(is_private_ip("2001:db8::1".parse().unwrap()));
		assert!(is_private_ip("ff01::1".parse().unwrap()));
		assert!(is_private_ip("fec0::1".parse().unwrap()));
		assert!(is_private_ip("::ffff:127.0.0.1".parse().unwrap()));
	}
}
