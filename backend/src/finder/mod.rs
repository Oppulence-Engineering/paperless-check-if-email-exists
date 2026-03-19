pub mod patterns;

use crate::config::BackendConfig;
use crate::http::ReacherResponseError;
use check_if_email_exists::check_email;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;

#[derive(Debug, Clone, Serialize)]
pub struct FinderCandidateResult {
	pub email: String,
	pub pattern: String,
	pub score: i16,
	pub category: String,
	pub sub_reason: String,
	pub is_reachable: String,
	pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FinderBestMatch {
	pub email: String,
	pub score: i16,
	pub confidence: String,
	pub pattern: String,
}

#[derive(Debug, Clone, Copy)]
pub struct DomainPrecheck {
	pub has_mx_records: bool,
	pub is_catch_all: bool,
}

pub fn require_tenant_id(tenant_id: Option<uuid::Uuid>) -> Result<uuid::Uuid, warp::Rejection> {
	tenant_id.ok_or_else(|| {
		ReacherResponseError::new(
			StatusCode::UNAUTHORIZED,
			"Tenant authentication is required for this endpoint",
		)
		.into()
	})
}

pub fn confidence_for_score(score: i16) -> Option<&'static str> {
	match score {
		90..=100 => Some("high"),
		70..=89 => Some("medium"),
		_ => None,
	}
}

pub async fn precheck_domain(
	config: Arc<BackendConfig>,
	domain: &str,
) -> Result<DomainPrecheck, ReacherResponseError> {
	let probe_local = format!("reacher_probe_{}", uuid::Uuid::new_v4().simple());
	let probe_email = format!("{}@{}", probe_local, domain);
	let input = crate::http::CheckEmailRequest {
		to_email: probe_email,
		..Default::default()
	}
	.to_check_email_input(config);
	let output = check_email(&input).await;

	let has_mx_records = output
		.mx
		.as_ref()
		.ok()
		.and_then(|mx| mx.lookup.as_ref().ok())
		.map(|lookup| lookup.iter().next().is_some())
		.unwrap_or(false);
	let is_catch_all = output
		.smtp
		.as_ref()
		.ok()
		.map(|smtp| smtp.is_catch_all)
		.unwrap_or(false);

	Ok(DomainPrecheck {
		has_mx_records,
		is_catch_all,
	})
}

pub async fn sync_finder_results(
	pg_pool: &PgPool,
	finder_job_id: i32,
) -> Result<(Vec<FinderCandidateResult>, Option<FinderBestMatch>, bool), ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT
			fr.id,
			fr.candidate_email,
			fr.pattern,
			tr.result,
			tr.score,
			tr.score_category,
			tr.sub_reason,
			tr.task_state::TEXT AS task_state
		FROM v1_finder_result fr
		LEFT JOIN v1_task_result tr ON tr.id = fr.task_result_id
		WHERE fr.finder_job_id = $1
		ORDER BY fr.id ASC
		"#,
	)
	.bind(finder_job_id)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let mut completed_results = Vec::new();
	let mut all_terminal = true;

	for row in &rows {
		let task_state = row
			.get::<Option<String>, _>("task_state")
			.unwrap_or_else(|| "queued".to_string());
		if matches!(task_state.as_str(), "queued" | "running" | "retrying") {
			all_terminal = false;
		}
		let result: Option<serde_json::Value> = row.get("result");
		let score = row.get::<Option<i16>, _>("score").or_else(|| {
			result
				.as_ref()
				.and_then(|value| value.get("score"))
				.and_then(|value| value.get("score"))
				.and_then(|value| value.as_i64())
				.map(|value| value as i16)
		});
		if let Some(score) = score {
			let category = row
				.get::<Option<String>, _>("score_category")
				.or_else(|| {
					result
						.as_ref()
						.and_then(|value| value.get("score"))
						.and_then(|value| value.get("category"))
						.and_then(|value| value.as_str())
						.map(ToOwned::to_owned)
				})
				.unwrap_or_else(|| "unknown".to_string());
			let sub_reason = row
				.get::<Option<String>, _>("sub_reason")
				.or_else(|| {
					result
						.as_ref()
						.and_then(|value| value.get("score"))
						.and_then(|value| value.get("sub_reason"))
						.and_then(|value| value.as_str())
						.map(ToOwned::to_owned)
				})
				.unwrap_or_else(|| "unknown".to_string());
			let is_reachable = result
				.as_ref()
				.and_then(|value| value.get("is_reachable"))
				.and_then(|value| value.as_str())
				.unwrap_or(match task_state.as_str() {
					"failed" | "dead_lettered" => "unknown",
					_ => "unknown",
				})
				.to_string();

			completed_results.push(FinderCandidateResult {
				email: row.get("candidate_email"),
				pattern: row.get("pattern"),
				score,
				category,
				sub_reason,
				is_reachable,
				result: result.clone(),
			});
		}
	}

	completed_results.sort_by(|left, right| {
		right
			.score
			.cmp(&left.score)
			.then_with(|| left.email.cmp(&right.email))
	});

	for (index, result) in completed_results.iter().enumerate() {
		sqlx::query(
			r#"
			UPDATE v1_finder_result
			SET rank_position = $2,
			    score = $3,
			    score_category = $4,
			    sub_reason = $5,
			    result = $6,
			    updated_at = NOW()
			WHERE finder_job_id = $1 AND candidate_email = $7 AND pattern = $8
			"#,
		)
		.bind(finder_job_id)
		.bind((index + 1) as i32)
		.bind(result.score)
		.bind(&result.category)
		.bind(&result.sub_reason)
		.bind(&result.result)
		.bind(&result.email)
		.bind(&result.pattern)
		.execute(pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	}

	let best_match = completed_results.first().and_then(|candidate| {
		confidence_for_score(candidate.score).map(|confidence| FinderBestMatch {
			email: candidate.email.clone(),
			score: candidate.score,
			confidence: confidence.to_string(),
			pattern: candidate.pattern.clone(),
		})
	});

	if all_terminal {
		sqlx::query(
			r#"
			UPDATE v1_finder_job
			SET status = 'completed'::job_state,
			    best_match_email = $2,
			    best_match_score = $3,
			    best_match_confidence = $4,
			    completed_at = COALESCE(completed_at, NOW()),
			    updated_at = NOW()
			WHERE id = $1
			"#,
		)
		.bind(finder_job_id)
		.bind(best_match.as_ref().map(|match_| match_.email.clone()))
		.bind(best_match.as_ref().map(|match_| i32::from(match_.score)))
		.bind(best_match.as_ref().map(|match_| match_.confidence.clone()))
		.execute(pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	}

	Ok((completed_results, best_match, all_terminal))
}
