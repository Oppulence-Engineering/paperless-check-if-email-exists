pub mod patterns;

use crate::config::BackendConfig;
use crate::http::ReacherResponseError;
use check_if_email_exists::check_email;
use check_if_email_exists::provider::detect_provider;
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
	pub confidence: Option<ConfidenceExplanation>,
	pub result: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct FinderBestMatch {
	pub email: String,
	pub score: i16,
	pub confidence: String,
	pub pattern: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Pattern quality scores used in confidence calculation.
/// Higher-ranked patterns (e.g. first.last) are more commonly used in real
/// organizations, so matches against them carry higher confidence.
fn pattern_quality(pattern: &str) -> i16 {
	match pattern {
		"first.last" => 10,
		"firstlast" => 9,
		"first_last" => 9,
		"first-last" => 9,
		"f.last" => 7,
		"flast" => 7,
		"f_last" => 7,
		"first.l" => 6,
		"firstl" => 6,
		"last.first" => 5,
		"lastf" => 5,
		"first" => 3,
		"last" => 3,
		_ => 1,
	}
}

#[derive(Debug, Clone, Serialize)]
pub struct ConfidenceExplanation {
	pub score: i16,
	pub level: String,
	pub factors: Vec<String>,
}

/// Builds a human-readable confidence explanation for a finder candidate.
pub fn explain_confidence(
	email_score: i16,
	pattern: &str,
	domain_precheck: &DomainPrecheck,
) -> ConfidenceExplanation {
	let pq = pattern_quality(pattern);
	let mut raw = pq * 10;
	let mut factors = Vec::new();

	factors.push(format!(
		"pattern_quality: {} ({})",
		pq,
		if pq >= 9 {
			"very common"
		} else if pq >= 7 {
			"common"
		} else if pq >= 5 {
			"uncommon"
		} else {
			"rare"
		}
	));

	if !domain_precheck.has_mx_records {
		raw -= 20;
		factors.push("no_mx_records: -20".to_string());
	}

	if domain_precheck.is_catch_all {
		raw -= 15;
		factors.push("catch_all_domain: -15".to_string());
	}

	if email_score >= 80 {
		raw += 10;
		factors.push(format!("email_score_bonus: +10 (score={})", email_score));
	} else if email_score < 50 {
		raw -= 10;
		factors.push(format!("email_score_penalty: -10 (score={})", email_score));
	}

	let score = raw.clamp(0, 100);
	let level = match score {
		80..=100 => "high",
		60..=79 => "medium",
		30..=59 => "low",
		_ => "very_low",
	}
	.to_string();

	ConfidenceExplanation {
		score,
		level,
		factors,
	}
}

pub async fn precheck_domain(
	config: Arc<BackendConfig>,
	domain: &str,
) -> Result<DomainPrecheck, ReacherResponseError> {
	// Known consumer-provider domains are explicit aliases in the core provider
	// catalog. They are stable enough to short-circuit without a live DNS probe,
	// which keeps finder behavior deterministic in tests and avoids an extra
	// network round-trip in production.
	if detect_provider(domain, None).is_some() {
		return Ok(DomainPrecheck {
			has_mx_records: true,
			is_catch_all: false,
		});
	}

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
	// Fetch domain precheck info for confidence explanations
	let domain_row =
		sqlx::query("SELECT domain_has_mx, domain_is_catch_all FROM v1_finder_job WHERE id = $1")
			.bind(finder_job_id)
			.fetch_optional(pg_pool)
			.await
			.map_err(ReacherResponseError::from)?;
	let domain_precheck = domain_row
		.map(|r| DomainPrecheck {
			has_mx_records: r.get("domain_has_mx"),
			is_catch_all: r.get("domain_is_catch_all"),
		})
		.unwrap_or(DomainPrecheck {
			has_mx_records: true,
			is_catch_all: false,
		});

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
	let mut has_failed = false;

	for row in &rows {
		let task_state = row
			.get::<Option<String>, _>("task_state")
			.unwrap_or_else(|| "queued".to_string());
		if matches!(task_state.as_str(), "queued" | "running" | "retrying") {
			all_terminal = false;
		}
		if matches!(task_state.as_str(), "failed" | "dead_lettered") {
			has_failed = true;
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

			let pattern: String = row.get("pattern");
			let confidence = Some(explain_confidence(score, &pattern, &domain_precheck));
			completed_results.push(FinderCandidateResult {
				email: row.get("candidate_email"),
				pattern,
				score,
				category,
				sub_reason,
				is_reachable,
				confidence,
				result: result.clone(),
			});
		}
	}

	rank_candidates(&mut completed_results);

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

	let best_match = best_match_from_ranked(&completed_results, &domain_precheck);

	if all_terminal {
		let final_status = if has_failed { "failed" } else { "completed" };
		sqlx::query(
			r#"
			UPDATE v1_finder_job
			SET status = $5::job_state,
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
		.bind(final_status)
		.execute(pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	}

	Ok((completed_results, best_match, all_terminal))
}

/// Orders candidates best first: by score, then by how common the pattern is,
/// then by email so the order is stable. Candidates often tie on score (every
/// address on a catch-all domain scores the same), and the pattern is the only
/// signal left to choose between them.
fn rank_candidates(results: &mut [FinderCandidateResult]) {
	results.sort_by(|left, right| {
		right
			.score
			.cmp(&left.score)
			.then_with(|| pattern_quality(&right.pattern).cmp(&pattern_quality(&left.pattern)))
			.then_with(|| left.email.cmp(&right.email))
	});
}

/// The top-ranked candidate, if its score is good enough to suggest. On a
/// catch-all domain the server accepts every address, so a good score does not
/// show that this mailbox exists and the match is reported with low confidence.
fn best_match_from_ranked(
	ranked: &[FinderCandidateResult],
	domain_precheck: &DomainPrecheck,
) -> Option<FinderBestMatch> {
	let candidate = ranked.first()?;
	let confidence = confidence_for_score(candidate.score)?;
	let confidence = if domain_precheck.is_catch_all {
		"low"
	} else {
		confidence
	};
	Some(FinderBestMatch {
		email: candidate.email.clone(),
		score: candidate.score,
		confidence: confidence.to_string(),
		pattern: candidate.pattern.clone(),
	})
}

#[cfg(test)]
mod tests {
	use super::{
		best_match_from_ranked, precheck_domain, rank_candidates, DomainPrecheck,
		FinderCandidateResult,
	};
	use crate::config::BackendConfig;
	use std::sync::Arc;

	fn scored(email: &str, pattern: &str, score: i16) -> FinderCandidateResult {
		FinderCandidateResult {
			email: email.to_string(),
			pattern: pattern.to_string(),
			score,
			category: "valid".to_string(),
			sub_reason: "deliverable".to_string(),
			is_reachable: "safe".to_string(),
			confidence: None,
			result: None,
		}
	}

	const PLAIN_DOMAIN: DomainPrecheck = DomainPrecheck {
		has_mx_records: true,
		is_catch_all: false,
	};
	const CATCH_ALL_DOMAIN: DomainPrecheck = DomainPrecheck {
		has_mx_records: true,
		is_catch_all: true,
	};

	#[test]
	fn ranking_breaks_score_ties_by_pattern_quality_not_alphabet() {
		// Alphabetically "doe.john" sorts first, but first.last is the more common
		// corporate pattern. A higher score still beats a better pattern.
		let mut results = vec![
			scored("doe.john@example.com", "last.first", 80),
			scored("john@example.com", "first", 80),
			scored("john.doe@example.com", "first.last", 80),
			scored("jdoe@example.com", "flast", 85),
		];
		rank_candidates(&mut results);
		let order: Vec<&str> = results.iter().map(|r| r.email.as_str()).collect();
		assert_eq!(
			order,
			vec![
				"jdoe@example.com",
				"john.doe@example.com",
				"doe.john@example.com",
				"john@example.com",
			]
		);
	}

	#[test]
	fn best_match_confidence_follows_score_and_is_low_on_catch_all_domains() {
		let strong = vec![scored("john.doe@example.com", "first.last", 95)];
		let best = best_match_from_ranked(&strong, &PLAIN_DOMAIN).unwrap();
		assert_eq!(best.email, "john.doe@example.com");
		assert_eq!(best.confidence, "high");

		// A catch-all domain accepts every candidate, so the same score proves nothing
		// about which address exists.
		let best = best_match_from_ranked(&strong, &CATCH_ALL_DOMAIN).unwrap();
		assert_eq!(best.email, "john.doe@example.com");
		assert_eq!(best.confidence, "low");

		let weak = vec![scored("john.doe@example.com", "first.last", 60)];
		assert!(best_match_from_ranked(&weak, &PLAIN_DOMAIN).is_none());
		assert!(best_match_from_ranked(&weak, &CATCH_ALL_DOMAIN).is_none());
		assert!(best_match_from_ranked(&[], &PLAIN_DOMAIN).is_none());
	}

	#[tokio::test]
	async fn precheck_short_circuits_known_provider_domains() {
		let precheck = precheck_domain(Arc::new(BackendConfig::empty()), "gmail.com")
			.await
			.unwrap();

		assert_eq!(
			precheck,
			DomainPrecheck {
				has_mx_records: true,
				is_catch_all: false,
			}
		);
	}
}
