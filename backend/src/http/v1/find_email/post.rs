use crate::config::BackendConfig;
use crate::finder::patterns::{generate_candidates, normalize_domain, normalize_name};
use crate::finder::{precheck_domain, require_tenant_id};
use crate::http::v0::check_email::post::with_config;
use crate::http::v1::bulk::post::publish_task;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, CheckEmailRequest, ReacherResponseError};
use crate::tenant::context::TenantContext;
use crate::tenant::quota::{check_and_increment_quota_for_count, QuotaCheckResult};
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskMetadata};
use check_if_email_exists::LOG_TARGET;
use lapin::BasicProperties;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct Request {
	first_name: String,
	last_name: String,
	domain: String,
}

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	bulk_job_id: i32,
	status: String,
	candidates_checked: i32,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	body: Request,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let normalized_first = normalize_name(&body.first_name);
	let normalized_last = normalize_name(&body.last_name);
	let normalized_domain = normalize_domain(&body.domain);
	if normalized_first.is_empty() || normalized_last.is_empty() || normalized_domain.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"first_name, last_name, and domain are required",
		)
		.into());
	}

	let candidates = generate_candidates(&body.first_name, &body.last_name, &body.domain);
	if candidates.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Unable to generate email candidates",
		)
		.into());
	}

	let precheck = precheck_domain(Arc::clone(&config), &normalized_domain)
		.await
		.map_err(warp::reject::custom)?;

	if precheck.has_mx_records {
		match check_and_increment_quota_for_count(
			Some(&pg_pool),
			&tenant_ctx,
			candidates.len() as i32,
		)
		.await
		{
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
				.into())
			}
		}
	}

	let bulk_status = if precheck.has_mx_records {
		"pending"
	} else {
		"completed"
	};
	let total_records = if precheck.has_mx_records {
		candidates.len() as i32
	} else {
		0
	};

	let bulk_job_id: i32 = sqlx::query_scalar(
		"INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES ($1, $2, $3::job_state) RETURNING id",
	)
	.bind(total_records)
	.bind(tenant_id)
	.bind(bulk_status)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let finder_status = if precheck.has_mx_records {
		"running"
	} else {
		"completed"
	};
	let finder_job_id: i32 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_finder_job (
			tenant_id, bulk_job_id, first_name, last_name, domain,
			normalized_first_name, normalized_last_name, status,
			domain_has_mx, domain_is_catch_all, candidates_checked, completed_at
		)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8::job_state, $9, $10, $11, CASE WHEN $8 = 'completed' THEN NOW() ELSE NULL END)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(bulk_job_id)
	.bind(&body.first_name)
	.bind(&body.last_name)
	.bind(&normalized_domain)
	.bind(&normalized_first)
	.bind(&normalized_last)
	.bind(finder_status)
	.bind(precheck.has_mx_records)
	.bind(precheck.is_catch_all)
	.bind(total_records)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	if !precheck.has_mx_records {
		return Ok(warp::reply::with_status(
			warp::reply::json(&Response {
				job_id: finder_job_id,
				bulk_job_id,
				status: "completed".to_string(),
				candidates_checked: 0,
			}),
			StatusCode::ACCEPTED,
		));
	}

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(1);

	for candidate in candidates {
		let task_payload = CheckEmailRequest {
			to_email: candidate.email.clone(),
			..Default::default()
		}
		.to_check_email_input(Arc::clone(&config));
		let task_row_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id)
			VALUES ($1, $2, 'queued', $3)
			RETURNING id
			"#,
		)
		.bind(bulk_job_id)
		.bind(serde_json::json!({
			"input": task_payload,
			"job_id": {"bulk": bulk_job_id},
			"webhook": null
		}))
		.bind(tenant_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		sqlx::query(
			r#"
			INSERT INTO v1_finder_result (finder_job_id, task_result_id, candidate_email, pattern)
			VALUES ($1, $2, $3, $4)
			"#,
		)
		.bind(finder_job_id)
		.bind(task_row_id)
		.bind(&candidate.email)
		.bind(&candidate.pattern)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		let task = CheckEmailTask {
			input: CheckEmailRequest {
				to_email: candidate.email.clone(),
				..Default::default()
			}
			.to_check_email_input(Arc::clone(&config)),
			job_id: CheckEmailJobId::Bulk(bulk_job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: Some(tenant_id.to_string()),
				request_id: None,
				correlation_id: None,
				created_by: Some("finder".to_string()),
				retry_policy: None,
				dedupe_key: Some(format!("finder:{}:{}", finder_job_id, candidate.pattern)),
				task_db_id: Some(task_row_id),
			}),
		};
		publish_task(
			config
				.must_worker_config()
				.map_err(ReacherResponseError::from)?
				.channel,
			task,
			properties.clone(),
		)
		.await?;
	}

	sqlx::query(
		"UPDATE v1_bulk_job SET status = 'running'::job_state, updated_at = NOW() WHERE id = $1",
	)
	.bind(bulk_job_id)
	.execute(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::with_status(
		warp::reply::json(&Response {
			job_id: finder_job_id,
			bulk_job_id,
			status: "running".to_string(),
			candidates_checked: total_records,
		}),
		StatusCode::ACCEPTED,
	))
}

/// POST /v1/find_email
#[utoipa::path(
	post,
	path = "/v1/find_email",
	tag = "v1",
	responses((status = 202, description = "Finder job accepted"))
)]
pub fn v1_find_email(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "find_email")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json::<Request>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
