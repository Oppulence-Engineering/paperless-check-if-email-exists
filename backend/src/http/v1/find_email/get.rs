use crate::config::BackendConfig;
use crate::finder::{sync_finder_results, FinderBestMatch, FinderCandidateResult};
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct AlternativeContact {
	email: String,
	score: i16,
	pattern: String,
	confidence: Option<crate::finder::ConfidenceExplanation>,
}

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	bulk_job_id: i32,
	status: String,
	domain_has_mx: bool,
	domain_is_catch_all: bool,
	candidates_checked: i32,
	results: Vec<FinderCandidateResult>,
	best_match: Option<FinderBestMatch>,
	alternatives: Vec<AlternativeContact>,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::FIND)?;

	let row = sqlx::query(
		r#"
		SELECT id, bulk_job_id, status::TEXT AS status, domain_has_mx, domain_is_catch_all, candidates_checked
		FROM v1_finder_job
		WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)
		"#,
	)
	.bind(job_id)
	.bind(tenant_ctx.tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let row = row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Finder job not found",
		))
	})?;

	let (results, best_match, all_terminal) = sync_finder_results(&pg_pool, job_id)
		.await
		.map_err(warp::reject::custom)?;

	let status = if all_terminal {
		// sync_finder_results has just stored the final status, which is
		// failed when any candidate check failed.
		sqlx::query_scalar::<_, String>("SELECT status::TEXT FROM v1_finder_job WHERE id = $1")
			.bind(job_id)
			.fetch_one(&pg_pool)
			.await
			.map_err(ReacherResponseError::from)?
	} else {
		row.get::<String, _>("status")
	};

	let alternatives = build_alternatives(&results, best_match.as_ref());

	Ok(warp::reply::json(&Response {
		job_id: row.get("id"),
		bulk_job_id: row.get("bulk_job_id"),
		status,
		domain_has_mx: row.get("domain_has_mx"),
		domain_is_catch_all: row.get("domain_is_catch_all"),
		candidates_checked: row.get("candidates_checked"),
		results,
		best_match,
		alternatives,
	}))
}

/// Candidates other than the best match that scored >= 50, at most four.
/// Results are sorted by score, but a top candidate below the best-match
/// threshold is not a best match and stays in the list.
fn build_alternatives(
	results: &[FinderCandidateResult],
	best_match: Option<&FinderBestMatch>,
) -> Vec<AlternativeContact> {
	let skip = usize::from(best_match.is_some());
	results
		.iter()
		.skip(skip)
		.filter(|c| c.score >= 50)
		.take(4)
		.map(|c| AlternativeContact {
			email: c.email.clone(),
			score: c.score,
			pattern: c.pattern.clone(),
			confidence: c.confidence.clone(),
		})
		.collect()
}

/// GET /v1/find_email/{job_id}
#[utoipa::path(
	get,
	path = "/v1/find_email/{job_id}",
	tag = "v1",
	params(("job_id" = i32, Path, description = "Finder job identifier")),
	responses((status = 200, description = "Finder job result"))
)]
pub fn v1_get_find_email(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "find_email" / i32)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;

	fn candidate(email: &str, score: i16) -> FinderCandidateResult {
		FinderCandidateResult {
			email: email.to_string(),
			pattern: "first.last".to_string(),
			score,
			category: "risky".to_string(),
			sub_reason: "catch_all".to_string(),
			is_reachable: "risky".to_string(),
			confidence: None,
			result: None,
		}
	}

	fn emails(alternatives: &[AlternativeContact]) -> Vec<&str> {
		alternatives.iter().map(|a| a.email.as_str()).collect()
	}

	#[test]
	fn top_candidate_is_an_alternative_when_no_best_match_is_chosen() {
		// Scores below 70 give no best match, so the top candidate must not be skipped.
		let results = [
			candidate("jane.doe@acme.test", 65),
			candidate("jdoe@acme.test", 55),
		];
		assert_eq!(
			emails(&build_alternatives(&results, None)),
			["jane.doe@acme.test", "jdoe@acme.test"]
		);

		let best = FinderBestMatch {
			email: "jane.doe@acme.test".to_string(),
			score: 95,
			confidence: "high".to_string(),
			pattern: "first.last".to_string(),
		};
		let results = [
			candidate("jane.doe@acme.test", 95),
			candidate("jdoe@acme.test", 55),
		];
		assert_eq!(
			emails(&build_alternatives(&results, Some(&best))),
			["jdoe@acme.test"]
		);
	}
}
