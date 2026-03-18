// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! This file implements the `POST /v1/bulk` endpoint.

use std::sync::Arc;

use check_if_email_exists::LOG_TARGET;
use futures::stream::StreamExt;
use futures::stream::TryStreamExt;
use lapin::Channel;
use lapin::{options::*, BasicProperties};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info};
use warp::http::StatusCode;
use warp::Filter;

use super::with_worker_db;
use crate::config::BackendConfig;
use crate::http::resolve_tenant;
use crate::http::v0::check_email::post::with_config;
use crate::http::CheckEmailRequest;
use crate::http::ReacherResponseError;
use crate::tenant::context::TenantContext;
use crate::tenant::quota::{check_and_increment_quota_for_count, QuotaCheckResult};
use crate::worker::consume::CHECK_EMAIL_QUEUE;
use crate::worker::do_work::CheckEmailJobId;
use crate::worker::do_work::CheckEmailTask;
use crate::worker::do_work::TaskMetadata;
use crate::worker::do_work::TaskWebhook;

/// POST v1/bulk endpoint request body.
#[derive(Debug, Deserialize)]
struct Request {
	input: Vec<String>,
	webhook: Option<TaskWebhook>,
}

/// POST v1/bulk endpoint response body.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Response {
	job_id: i32,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	body: Request,
) -> Result<impl warp::Reply, warp::Rejection> {
	if body.input.is_empty() {
		return Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "Empty input").into());
	}

	// Atomically check and increment quota for all emails in the bulk request.
	let email_count = body.input.len() as i32;
	match check_and_increment_quota_for_count(Some(&pg_pool), &tenant_ctx, email_count).await {
		QuotaCheckResult::Allowed => {}
		QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used,
			resets_at,
		} => {
			return Err(ReacherResponseError::new(
				StatusCode::TOO_MANY_REQUESTS,
				format!(
					"Monthly email limit of {} reached ({} used). Resets at {}",
					limit,
					used,
					resets_at.format("%Y-%m-%d %H:%M:%S UTC")
				),
			)
			.into());
		}
	}

	let tenant_id = tenant_ctx.tenant_id;

	// Create job entry with status = 'pending'
	let rec = sqlx::query!(
		r#"
		INSERT INTO v1_bulk_job (total_records, tenant_id, status)
		VALUES ($1, $2, 'pending')
		RETURNING id
		"#,
		body.input.len() as i32,
		tenant_id,
	)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	// Pre-create task result rows with task_state = 'queued'
	let mut task_ids: Vec<i32> = Vec::with_capacity(body.input.len());
	for email in &body.input {
		let input = CheckEmailRequest {
			to_email: email.clone(),
			..Default::default()
		}
		.to_check_email_input(Arc::clone(&config));

		let payload_json = serde_json::to_value(&CheckEmailTask {
			input,
			job_id: CheckEmailJobId::Bulk(rec.id),
			webhook: body.webhook.clone(),
			metadata: None,
		})
		.map_err(ReacherResponseError::from)?;

		let task_row = sqlx::query_scalar!(
			r#"
			INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id)
			VALUES ($1, $2, 'queued', $3)
			RETURNING id
			"#,
			rec.id,
			payload_json,
			tenant_id,
		)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		task_ids.push(task_row);
	}

	// Publish tasks to RabbitMQ with metadata.task_db_id
	let n = body.input.len();
	let webhook = body.webhook.clone();
	let emails_with_ids: Vec<(String, i32)> = body.input.into_iter().zip(task_ids).collect();
	let stream = futures::stream::iter(emails_with_ids.into_iter());

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(1); // Low priority

	let job_id = rec.id;
	stream
		.map::<Result<_, ReacherResponseError>, _>(Ok)
		.try_for_each_concurrent(10, |(to_email, task_db_id)| {
			let config = Arc::clone(&config);
			let webhook = webhook.clone();
			let properties = properties.clone();
			async move {
				let input = CheckEmailRequest {
					to_email,
					..Default::default()
				}
				.to_check_email_input(Arc::clone(&config));

				let task = CheckEmailTask {
					input,
					job_id: CheckEmailJobId::Bulk(job_id),
					webhook,
					metadata: Some(TaskMetadata {
						tenant_id: tenant_id.map(|id| id.to_string()),
						request_id: None,
						correlation_id: None,
						created_by: None,
						retry_policy: None,
						dedupe_key: None,
						task_db_id: Some(task_db_id),
					}),
				};

				publish_task(
					config
						.must_worker_config()
						.map_err(ReacherResponseError::from)?
						.channel,
					task,
					properties,
				)
				.await
			}
		})
		.await?;

	// Update job to 'running'
	sqlx::query!(
		"UPDATE v1_bulk_job SET status = 'running', updated_at = NOW() WHERE id = $1",
		job_id,
	)
	.execute(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	// Record job.created event
	let _ = sqlx::query!(
		r#"
		INSERT INTO job_events (job_id, event_type, event_data, actor)
		VALUES ($1, 'job.created', $2, 'api')
		"#,
		job_id,
		serde_json::json!({ "total_records": n }),
	)
	.execute(&pg_pool)
	.await;

	info!(
		target: LOG_TARGET,
		queue = CHECK_EMAIL_QUEUE,
		"Added {n} emails",
	);
	Ok(warp::reply::json(&Response { job_id }))
}

/// Publish a task to the "check_email" queue.
pub async fn publish_task(
	channel: Arc<Channel>,
	task: CheckEmailTask,
	properties: BasicProperties,
) -> Result<(), ReacherResponseError> {
	let payload = serde_json::to_vec(&task).map_err(ReacherResponseError::from)?;

	channel
		.basic_publish(
			"",
			CHECK_EMAIL_QUEUE,
			BasicPublishOptions::default(),
			&payload,
			properties,
		)
		.await
		.map_err(ReacherResponseError::from)?
		.await
		.map_err(ReacherResponseError::from)?;

	debug!(
		target: LOG_TARGET,
		email = ?task.input.to_email,
		queue = CHECK_EMAIL_QUEUE,
		"Published task"
	);

	Ok(())
}

/// Create the v1 bulk endpoint.
///
/// Creates a tenant-scoped bulk job for async processing.
#[utoipa::path(
	post,
	path = "/v1/bulk",
	tag = "v1",
	params((
		"Idempotency-Key" = Option<String>, Header, description = "Optional idempotency key")
	),
	responses((status = 200, description = "Bulk job created"))
)]
pub fn v1_create_bulk_job(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "bulk")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::content_length_limit(1024 * 1024 * 50))
		.and(warp::body::json())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
