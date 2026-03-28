use crate::config::{BackendConfig, PipelinesConfig};
use crate::http::v1::bulk::post::publish_task;
use crate::http::v1::lists::canonicalize::canonicalize_email;
use crate::http::CheckEmailRequest;
use crate::tenant::auth::resolve_tenant_context_by_id;
use crate::tenant::quota::release_reserved_usage;
use crate::tenant::webhook::{sign_payload, WEBHOOK_SIGNATURE_HEADER};
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskMetadata};
use anyhow::{bail, Context, Result};
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
}

impl Default for PipelinePolicyConfig {
	fn default() -> Self {
		Self {
			missed_run_window_hours: default_missed_run_window_hours(),
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

#[derive(Debug, Clone)]
pub struct PipelineListQuery {
	pub status: Option<PipelineStatus>,
	pub limit: i64,
	pub offset: i64,
}

#[derive(Debug)]
struct PreparedList {
	source_list_id: i32,
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

	recover_stranded_pending_deliveries(pg_pool).await?;
	retry_due_pipeline_deliveries(pg_pool, config.pipelines.max_due_per_tick).await?;

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
		let next_future_run = compute_next_run_at(
			&pipeline.schedule_cron,
			&pipeline.schedule_timezone,
			Utc::now(),
			pipeline_config.min_interval_seconds,
		)?;
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
		WITH candidates AS (
			SELECT pr.id, pr.pipeline_id
			FROM v1_pipeline_runs pr
			INNER JOIN v1_pipelines p ON p.id = pr.pipeline_id
			INNER JOIN tenants t ON t.id = pr.tenant_id
			WHERE pr.status = 'queued'::pipeline_run_status
			  AND pr.started_at IS NULL
			  AND pr.updated_at < NOW() - INTERVAL '5 minutes'
			  AND p.deleted_at IS NULL
			  AND t.status = 'active'::tenant_status
			ORDER BY pr.created_at ASC
			LIMIT $1
			FOR UPDATE SKIP LOCKED
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
		pg_pool,
		tenant_id,
		&input.source,
		&input.schedule,
		&input.policy,
		&input.delivery,
		pipeline_config.min_interval_seconds,
	)
	.await?;

	let next_run_at = match input.status {
		PipelineStatus::Active => Some(compute_next_run_at(
			&input.schedule.cron,
			&input.schedule.timezone,
			Utc::now(),
			pipeline_config.min_interval_seconds,
		)?),
		PipelineStatus::Paused | PipelineStatus::Deleted => None,
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

	let current = match get_pipeline(pg_pool, tenant_id, pipeline_id).await? {
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
		pg_pool,
		tenant_id,
		&next_source,
		&next_schedule,
		&next_policy,
		&next_delivery,
		pipeline_config.min_interval_seconds,
	)
	.await?;

	let next_run_at = match next_status {
		PipelineStatus::Active if current.status == PipelineStatus::Active && !schedule_changed => {
			current.next_run_at.or(Some(compute_next_run_at(
				&next_schedule.cron,
				&next_schedule.timezone,
				Utc::now(),
				pipeline_config.min_interval_seconds,
			)?))
		}
		PipelineStatus::Active => Some(compute_next_run_at(
			&next_schedule.cron,
			&next_schedule.timezone,
			Utc::now(),
			pipeline_config.min_interval_seconds,
		)?),
		PipelineStatus::Paused | PipelineStatus::Deleted => None,
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
	.fetch_optional(pg_pool)
	.await?;

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
	let current = match get_pipeline(pg_pool, tenant_id, pipeline_id).await? {
		Some(p) => p,
		None => return Ok(None),
	};
	let next_run_at = match status {
		PipelineStatus::Active => Some(compute_next_run_at(
			&current.schedule.cron,
			&current.schedule.timezone,
			Utc::now(),
			min_interval_seconds,
		)?),
		PipelineStatus::Paused | PipelineStatus::Deleted => None,
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
	.fetch_optional(pg_pool)
	.await?;
	row.map(|row| row_to_pipeline_view(&row)).transpose()
}

pub async fn create_manual_pipeline_run(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	tenant_id: Uuid,
	pipeline_id: i64,
	force: bool,
) -> Result<TriggerPipelineResponse> {
	let mut tx = pg_pool.begin().await?;
	let pipeline = match fetch_pipeline_row_for_update(&mut tx, pipeline_id).await? {
		Some(row) if row.tenant_id == tenant_id && row.deleted_at.is_none() => row,
		_ => return Err(PipelineRequestError::not_found("Pipeline not found").into()),
	};

	if !force {
		ensure_no_active_run(&mut tx, pipeline_id).await?;
	}
	let run_id: i64 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_pipeline_runs (pipeline_id, tenant_id, trigger_type, status, source_snapshot, stats)
		VALUES ($1, $2, 'manual', 'queued', $3, '{}'::jsonb)
		RETURNING id
		"#,
	)
	.bind(pipeline.id)
	.bind(pipeline.tenant_id)
	.bind(&pipeline.source_config)
	.fetch_one(&mut *tx)
	.await?;

	sqlx::query("UPDATE v1_pipelines SET last_run_id = $2, updated_at = NOW() WHERE id = $1")
		.bind(pipeline.id)
		.bind(run_id)
		.execute(&mut *tx)
		.await?;

	tx.commit().await?;
	execute_pipeline_run(config, pg_pool, run_id).await?;

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
	let run_id = sqlx::query_scalar::<_, Option<i64>>(
		r#"
		SELECT id
		FROM v1_pipeline_runs
		WHERE job_id = $1
		ORDER BY id DESC
		LIMIT 1
		"#,
	)
	.bind(job_id)
	.fetch_one(pg_pool)
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

	let result_location: Option<Value> = row.get("result_location");
	let claimed = if status.is_terminal() && result_location.is_some() {
		Some(run_id)
	} else {
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
		sqlx::query(claim_sql)
			.bind(run_id)
			.bind(&stats)
			.bind(stats["billed_emails"].as_i64().unwrap_or_default() as i32)
			.bind(&download_url)
			.bind(enum_str(&status))
			.fetch_optional(pg_pool)
			.await?
			.map(|_| run_id)
	};
	if claimed.is_none() {
		return Ok(());
	}

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

	update_pipeline_contact_state_for_run(pg_pool, run_id).await?;
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
		if let Err(mark_err) = mark_pipeline_run_execution_failed(pg_pool, run_id, err).await {
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

async fn execute_pipeline_run_inner(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	run_id: i64,
) -> Result<()> {
	let run = sqlx::query(
		r#"
		SELECT pr.id, pr.pipeline_id, pr.tenant_id, pr.trigger_type, pr.status::TEXT,
		       pr.scheduled_for, pr.source_snapshot,
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

	let prepared_list = load_source_list_snapshot(pg_pool, tenant_id, &source).await?;
	let (prepared_list, delta_summary) =
		apply_delta_selection(pg_pool, pipeline_id, &verification, prepared_list).await?;
	let tenant_ctx = resolve_tenant_context_by_id(pg_pool, tenant_id).await?;
	let source_snapshot = json!({
		"source": source,
		"source_name": prepared_list.source_name,
		"source_filename": prepared_list.source_filename,
		"row_count": prepared_list.rows.len(),
		"unique_emails": prepared_list.unique_email_count,
		"delta_mode": verification.delta_mode,
		"freshness_days": verification.freshness_days,
		"changed_only_export": verification.delta_mode,
		"selected_unique_emails": delta_summary.selected_unique_emails,
		"skipped_unchanged": delta_summary.skipped_unchanged
	});
	sqlx::query(
		"UPDATE v1_pipeline_runs SET status = 'fetching_source'::pipeline_run_status, source_snapshot = $2, stats = $3, updated_at = NOW() WHERE id = $1",
	)
	.bind(run_id)
	.bind(&source_snapshot)
	.bind(json!({
		"delta_mode": verification.delta_mode,
		"freshness_days": verification.freshness_days,
		"selected_unique_emails": delta_summary.selected_unique_emails,
		"skipped_unchanged": delta_summary.skipped_unchanged,
		"changed_only_export": verification.delta_mode,
	}))
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
				&source_snapshot,
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

	let mut published_unique_groups = 0i32;
	let mut published_any = false;
	let mut had_failures = false;

	for (canonical, indices) in &prepared_list.canonical_groups {
		let primary_index = indices[0];
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
				"input": {"to_email": canonical},
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

		let task = CheckEmailTask {
			input: CheckEmailRequest {
				to_email: canonical.clone(),
				..Default::default()
			}
			.to_check_email_input(Arc::clone(&config)),
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
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
			had_failures = true;
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

		published_any = true;
		published_unique_groups += 1;
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

	let final_job_status = if published_any {
		"running"
	} else if had_failures {
		"failed"
	} else {
		"completed"
	};
	let final_list_status = if published_any {
		"processing"
	} else {
		"completed"
	};
	sqlx::query(
		"UPDATE v1_bulk_job SET status = $2::job_state, completed_at = CASE WHEN $2 IN ('completed', 'failed') THEN NOW() ELSE NULL END, updated_at = NOW() WHERE id = $1",
	)
	.bind(job_id)
	.bind(final_job_status)
	.execute(pg_pool)
	.await?;
	sqlx::query(
		"UPDATE v1_lists SET status = $2::list_status, completed_at = CASE WHEN $2 = 'completed' THEN NOW() ELSE NULL END, updated_at = NOW() WHERE id = $1",
	)
	.bind(list_id)
	.bind(final_list_status)
	.execute(pg_pool)
	.await?;

	sqlx::query(
		"UPDATE v1_pipeline_runs SET status = $2::pipeline_run_status, billed_emails = $3, updated_at = NOW() WHERE id = $1",
	)
	.bind(run_id)
	.bind("running")
	.bind(published_unique_groups)
	.execute(pg_pool)
	.await?;

	if final_job_status != "running" {
		maybe_finalize_pipeline_run(config.as_ref(), pg_pool, run_id).await?;
	}

	Ok(())
}

async fn mark_pipeline_run_execution_failed(
	pg_pool: &PgPool,
	run_id: i64,
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
		    error_code = COALESCE(error_code, 'execution_error'),
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
		source_list_id: row.get("id"),
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

	validate_webhook_url(&url)?;
	validate_resolved_webhook_target(&url).await?;
	let client = reqwest::Client::builder()
		.timeout(std::time::Duration::from_secs(10))
		.redirect(reqwest::redirect::Policy::none())
		.build()?;
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

	let delivery: PipelineDeliveryConfig =
		serde_json::from_value(row.get("delivery_config")).unwrap_or_default();
	let default_webhook_url: Option<String> = row.get("default_webhook_url");
	let has_webhook_target = delivery.webhook.is_some() || default_webhook_url.is_some();
	if !has_webhook_target {
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

	let delivery_status = parse_pipeline_delivery_status(&row.get::<String, _>("delivery_status"))?;
	let next_delivery_attempt_at: Option<DateTime<Utc>> = row.get("next_delivery_attempt_at");
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
			RETURNING id
			"#,
		)
		.bind(run_id)
		.bind(in_flight_retry_at)
		.bind(enum_str(&delivery_status))
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
	let exponent = attempt_number.saturating_sub(1).min(6) as u32;
	let multiplier = 2i32.saturating_pow(exponent);
	base_delay_seconds
		.saturating_mul(multiplier)
		.clamp(base_delay_seconds.max(1), 86_400)
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
		SELECT id, canonical_email
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
		task_by_canonical.insert(canonical_email, task_id);
	}

	for (canonical_email, indices) in canonical_groups {
		let Some(task_id) = task_by_canonical.get(&canonical_email).copied() else {
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
			"#,
		)
		.bind(pipeline_id)
		.bind(canonical_email)
		.bind(source_hash)
		.bind(run_id)
		.bind(task_id)
		.bind(completed_at)
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

	let billed_emails = sqlx::query_scalar::<_, Option<i32>>(
		"SELECT committed_emails FROM v1_usage_events WHERE job_id = $1 ORDER BY id DESC LIMIT 1",
	)
	.bind(job_id)
	.fetch_one(pg_pool)
	.await?
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

fn validate_cron_expression(
	expression: &str,
	timezone: &str,
	min_interval_seconds: i32,
) -> Result<()> {
	let tz = parse_timezone(timezone)?;
	let schedule = parse_schedule(expression)?;
	let now = Utc::now().with_timezone(&tz);
	let mut upcoming = schedule.after(&now);
	let first = upcoming
		.next()
		.context("Cron expression does not produce any future run times")?;
	let second = upcoming
		.next()
		.context("Cron expression must produce at least two future run times")?;
	let interval = second.signed_duration_since(first).num_seconds();
	if interval < i64::from(min_interval_seconds) {
		bail!(
			"Cron schedule must be at least {} seconds apart",
			min_interval_seconds
		);
	}
	Ok(())
}

pub fn compute_next_run_at(
	expression: &str,
	timezone: &str,
	now_utc: DateTime<Utc>,
	min_interval_seconds: i32,
) -> Result<DateTime<Utc>> {
	validate_cron_expression(expression, timezone, min_interval_seconds)?;
	let tz = parse_timezone(timezone)?;
	let now = now_utc.with_timezone(&tz);
	let schedule = parse_schedule(expression)?;
	let next = schedule
		.after(&now)
		.next()
		.context("Cron expression did not produce a next run")?;
	Ok(next.with_timezone(&Utc))
}

async fn validate_pipeline_input(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	source: &PipelineSource,
	schedule: &PipelineSchedule,
	policy: &PipelinePolicyConfig,
	delivery: &PipelineDeliveryConfig,
	min_interval_seconds: i32,
) -> std::result::Result<(), PipelineRequestError> {
	validate_cron_expression(&schedule.cron, &schedule.timezone, min_interval_seconds)
		.map_err(|err| PipelineRequestError::validation(err.to_string()))?;
	if policy.missed_run_window_hours < 1 {
		return Err(PipelineRequestError::validation(
			"Pipeline missed_run_window_hours must be at least 1",
		));
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
			.fetch_one(pg_pool)
			.await?;
			if !exists {
				return Err(PipelineRequestError::validation(
					"Referenced source list does not exist for this tenant",
				));
			}
		}
		_ => {
			return Err(PipelineRequestError::validation(
				"Only list_snapshot sources are supported in phase 1",
			));
		}
	}
	Ok(())
}

fn validate_webhook_url(url: &str) -> Result<()> {
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
		if let Ok(ip) = host.parse::<std::net::IpAddr>() {
			if is_private_ip(ip) {
				bail!("Webhook URL must not target private or reserved IP addresses");
			}
		}
	} else {
		bail!("Webhook URL must have a valid host");
	}
	Ok(())
}

async fn validate_resolved_webhook_target(url: &str) -> Result<()> {
	let parsed = reqwest::Url::parse(url).context("Invalid webhook URL")?;
	let host = parsed
		.host_str()
		.context("Webhook URL must have a valid host")?;
	let port = parsed
		.port_or_known_default()
		.context("Webhook URL must use a known port")?;
	let addrs = tokio::net::lookup_host((host, port))
		.await
		.with_context(|| format!("Failed to resolve webhook host: {host}"))?
		.collect::<Vec<_>>();
	if addrs.is_empty() {
		bail!("Webhook URL host did not resolve to any addresses");
	}
	if addrs.iter().any(|addr| is_private_ip(addr.ip())) {
		bail!("Webhook URL must not resolve to private or reserved IP addresses");
	}
	Ok(())
}

fn is_private_ip(ip: std::net::IpAddr) -> bool {
	match ip {
		std::net::IpAddr::V4(v4) => {
			v4.is_loopback()
				|| v4.is_private()
				|| v4.is_link_local()
				|| v4.is_broadcast()
				|| v4.is_multicast()
				|| v4.is_unspecified()
				|| v4.octets()[0] == 169 && v4.octets()[1] == 254 // link-local
				|| v4.octets()[0] == 100 && (v4.octets()[1] & 0xC0) == 64 // CGN 100.64/10
				|| v4.octets()[0] == 192 && v4.octets()[1] == 0 && v4.octets()[2] == 0 // 192.0.0.0/24 IETF protocol assignments
				|| v4.octets()[0] == 192 && v4.octets()[1] == 0 && v4.octets()[2] == 2 // 192.0.2.0/24 TEST-NET-1
				|| v4.octets()[0] == 198 && v4.octets()[1] == 18 // 198.18.0.0/15 benchmark
				|| v4.octets()[0] == 198 && v4.octets()[1] == 19
				|| v4.octets()[0] == 198 && v4.octets()[1] == 51 && v4.octets()[2] == 100 // 198.51.100.0/24 TEST-NET-2
				|| v4.octets()[0] == 203 && v4.octets()[1] == 0 && v4.octets()[2] == 113 // 203.0.113.0/24 TEST-NET-3
				|| v4.octets()[0] >= 240 // reserved + limited broadcast block
		}
		std::net::IpAddr::V6(v6) => {
			let first_segment = v6.segments()[0];
			let mapped_private = v6
				.to_ipv4_mapped()
				.is_some_and(|mapped| is_private_ip(std::net::IpAddr::V4(mapped)));
			v6.is_loopback()
				|| v6.is_unspecified()
				|| v6.is_multicast()
				|| (first_segment & 0xffc0) == 0xfe80 // fe80::/10 link-local
				|| (first_segment & 0xfe00) == 0xfc00 // fc00::/7 unique local
				|| (first_segment & 0xffc0) == 0xfec0 // fec0::/10 site-local (deprecated but non-global)
				|| (first_segment == 0x2001 && v6.segments()[1] == 0x0db8) // 2001:db8::/32 documentation
				|| mapped_private
		}
	}
}

fn parse_schedule(expression: &str) -> Result<Schedule> {
	let parts: Vec<&str> = expression.split_whitespace().collect();
	if parts.len() != 5 {
		bail!("Cron expression must use standard 5-field syntax");
	}
	let cron_expression = format!("0 {} *", expression.trim());
	Schedule::from_str(&cron_expression).context("Invalid cron expression")
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

async fn ensure_no_active_run(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	pipeline_id: i64,
) -> Result<()> {
	if has_active_run(tx, pipeline_id).await? {
		return Err(PipelineRequestError::conflict("Pipeline already has an active run").into());
	}
	Ok(())
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

	#[test]
	fn test_compute_next_run_at_accepts_five_field_cron() {
		let now = Utc.with_ymd_and_hms(2026, 3, 28, 12, 0, 0).unwrap();
		let next = compute_next_run_at("0 13 * * *", "UTC", now, 3600).unwrap();
		assert!(next > now);
		assert_eq!(next.date_naive(), now.date_naive());
		assert_eq!(next.hour(), 13);
		assert_eq!(next.minute(), 0);
		assert_eq!(next.second(), 0);
	}

	#[test]
	fn test_compute_next_run_at_rolls_to_next_day() {
		let now = Utc.with_ymd_and_hms(2026, 3, 28, 13, 30, 0).unwrap();
		let next = compute_next_run_at("0 13 * * *", "UTC", now, 3600).unwrap();
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
	fn test_validate_cron_expression_rejects_too_frequent_schedule() {
		let err = validate_cron_expression("* * * * *", "UTC", 3600).unwrap_err();
		assert!(err.to_string().contains("at least 3600 seconds"));
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
