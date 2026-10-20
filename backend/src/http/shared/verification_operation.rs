use super::check_email::CheckEmailResponse;
use crate::{
	config::BackendConfig,
	decision::custom_policy::DecisionPolicy,
	http::{
		idempotency::complete_idempotency_key, v0::check_email::post::CheckEmailRequest,
		ReacherResponseError,
	},
	tenant::{
		context::TenantContext,
		quota::{reserve_quota_in_tx, QuotaCheckResult},
	},
	worker::{
		dispatch,
		do_work::{CheckEmailJobId, CheckEmailTask, TaskMetadata},
	},
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use warp::http::StatusCode;

const HTTP_WAIT: Duration = Duration::from_secs(20);

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct PendingVerification {
	pub operation_id: Uuid,
	pub status: String,
	pub status_url: String,
}

fn pending(id: Uuid) -> Result<CheckEmailResponse, ReacherResponseError> {
	Ok(CheckEmailResponse {
		status_code: StatusCode::ACCEPTED,
		body: serde_json::to_vec(&PendingVerification {
			operation_id: id,
			status: "pending".into(),
			status_url: format!("/v1/operations/{id}"),
		})
		.map_err(ReacherResponseError::from)?,
	})
}

pub async fn read_operation(
	pool: &PgPool,
	id: Uuid,
	tenant_ctx: &TenantContext,
) -> Result<CheckEmailResponse, ReacherResponseError> {
	let row = sqlx::query("SELECT result, error, completion_done FROM v1_task_result WHERE operation_id = $1 AND tenant_id IS NOT DISTINCT FROM $2::UUID")
		.bind(id).bind(tenant_ctx.tenant_id).fetch_optional(pool).await.map_err(ReacherResponseError::from)?
		.ok_or_else(|| ReacherResponseError::new(StatusCode::NOT_FOUND, "Verification operation not found"))?;
	if !row.get::<bool, _>("completion_done") {
		return pending(id);
	}
	let result: Option<serde_json::Value> = row.get("result");
	let (status_code, result) = match result {
		Some(result) => (StatusCode::OK, result),
		None => (
			StatusCode::INTERNAL_SERVER_ERROR,
			serde_json::json!({
				"error": row.get::<Option<String>, _>("error").unwrap_or_else(|| "Verification failed".into()),
				"operation_id": id,
				"status": "failed"
			}),
		),
	};
	Ok(CheckEmailResponse {
		status_code,
		body: serde_json::to_vec(&result).map_err(ReacherResponseError::from)?,
	})
}

/// Admission has no network effect: quota, identity, dispatch and replay commit together.
pub async fn admit_operation(
	config: Arc<BackendConfig>,
	pool: &PgPool,
	body: &CheckEmailRequest,
	tenant_ctx: &TenantContext,
	policy: Option<DecisionPolicy>,
	idempotency_id: Option<i64>,
) -> Result<(Uuid, Uuid), ReacherResponseError> {
	crate::smtp_egress::require_smtp_ready(&config)?;
	let id = Uuid::new_v4();
	let response = pending(id)?;
	let mut tx = pool.begin().await.map_err(ReacherResponseError::from)?;
	if let Some(owner) = idempotency_id {
		let current: Option<i64> = sqlx::query_scalar(
			"SELECT id FROM idempotency_keys WHERE id = $1 AND status = 'processing' FOR UPDATE",
		)
		.bind(owner)
		.fetch_optional(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
		if current.is_none() {
			return Err(ReacherResponseError::new(
				StatusCode::CONFLICT,
				"Request ownership changed; retry with the same key",
			));
		}
	}
	match reserve_quota_in_tx(&mut tx, tenant_ctx, 1)
		.await
		.map_err(ReacherResponseError::from)?
	{
		QuotaCheckResult::Allowed => {}
		QuotaCheckResult::Unavailable => {
			return Err(ReacherResponseError::new(
				StatusCode::SERVICE_UNAVAILABLE,
				"Quota service unavailable; no verification was dispatched.",
			))
		}
		QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used,
			resets_at,
		} => {
			return Err(ReacherResponseError::new(
				StatusCode::TOO_MANY_REQUESTS,
				format!(
					"Monthly email limit of {limit} reached ({used} used). Resets at {resets_at}"
				),
			))
		}
	}
	let task_id: i32 = sqlx::query_scalar("INSERT INTO v1_task_result (operation_id, payload, task_state, tenant_id, quota_period_end) VALUES ($1, $2, 'queued', $3, (SELECT period_reset_at FROM tenants WHERE id = $3 AND monthly_email_limit > 0)) RETURNING id")
		.bind(id).bind(serde_json::json!({"input":body.to_email})).bind(tenant_ctx.tenant_id)
		.fetch_one(&mut *tx).await.map_err(ReacherResponseError::from)?;
	let task = CheckEmailTask {
		verification_backend: config.verification_backend,
		input: body.to_check_email_input(Arc::clone(&config)),
		job_id: CheckEmailJobId::SingleShot,
		webhook: None,
		metadata: Some(TaskMetadata {
			dispatch_id: None,
			lease_id: None,
			decision_policy: policy,
			tenant_id: tenant_ctx.tenant_id.map(|id| id.to_string()),
			request_id: Some(id.to_string()),
			correlation_id: None,
			created_by: Some("api".into()),
			retry_policy: None,
			dedupe_key: None,
			task_db_id: Some(task_id),
		}),
	};
	let dispatch_id = dispatch::enqueue(&mut tx, task, crate::worker::consume::MAX_QUEUE_PRIORITY)
		.await
		.map_err(ReacherResponseError::from)?;
	if let Some(owner) = idempotency_id {
		sqlx::query("UPDATE idempotency_keys SET operation_id = $2 WHERE id = $1")
			.bind(owner)
			.bind(id)
			.execute(&mut *tx)
			.await
			.map_err(ReacherResponseError::from)?;
		complete_idempotency_key(
			&mut *tx,
			owner,
			StatusCode::ACCEPTED.as_u16(),
			&response.body,
		)
		.await?;
	}
	tx.commit().await.map_err(ReacherResponseError::from)?;
	Ok((id, dispatch_id))
}

pub async fn wait_for_operation(
	pool: &PgPool,
	id: Uuid,
	tenant_ctx: &TenantContext,
) -> Result<CheckEmailResponse, ReacherResponseError> {
	let result = tokio::time::timeout(HTTP_WAIT, async {
		loop {
			let response = read_operation(pool, id, tenant_ctx).await?;
			if response.status_code != StatusCode::ACCEPTED {
				return Ok::<_, ReacherResponseError>(response);
			}
			tokio::time::sleep(Duration::from_millis(100)).await;
		}
	})
	.await;
	match result {
		Ok(response) => response,
		Err(_) => pending(id),
	}
}
