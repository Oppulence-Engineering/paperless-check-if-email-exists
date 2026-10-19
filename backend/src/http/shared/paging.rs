//! Bounds for list endpoints' paging query parameters.
//!
//! Postgres rejects a negative LIMIT or OFFSET, which surfaced as a 500, and
//! a zero limit made cursor pagination report more rows without a cursor.

use crate::http::ReacherResponseError;
use warp::http::StatusCode;

const DEFAULT_PAGE_LIMIT: i64 = 50;
const MAX_PAGE_LIMIT: i64 = 200;

/// Page size between 1 and 200, 50 when omitted.
pub fn clamp_limit(limit: Option<i64>) -> i64 {
	limit.unwrap_or(DEFAULT_PAGE_LIMIT).clamp(1, MAX_PAGE_LIMIT)
}

/// Page size from `clamp_limit` and an offset of at least 0.
pub fn clamp_page(limit: Option<i64>, offset: Option<i64>) -> (i64, i64) {
	(clamp_limit(limit), offset.unwrap_or(0).max(0))
}

/// Validates a `task_state` filter so an unknown value is a 400, not a
/// failed enum cast.
pub fn validate_task_state(raw: Option<String>) -> Result<Option<String>, warp::Rejection> {
	match raw {
		Some(state) => {
			let normalized = state.to_lowercase();
			match normalized.as_str() {
				"queued" | "running" | "completed" | "retrying" | "failed" | "cancelled"
				| "dead_lettered" => Ok(Some(normalized)),
				_ => Err(warp::reject::custom(ReacherResponseError::new(
					StatusCode::BAD_REQUEST,
					"invalid task state. Expected queued|running|completed|retrying|failed|cancelled|dead_lettered",
				))),
			}
		}
		None => Ok(None),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn page_bounds_stay_valid_for_postgres() {
		assert_eq!(clamp_page(None, None), (50, 0));
		assert_eq!(clamp_page(Some(-5), Some(-10)), (1, 0));
		assert_eq!(clamp_page(Some(0), Some(30)), (1, 30));
		assert_eq!(clamp_page(Some(10_000), None), (200, 0));
	}

	#[test]
	fn task_state_filter_rejects_unknown_states() {
		assert_eq!(
			validate_task_state(Some("Dead_Lettered".into())).unwrap(),
			Some("dead_lettered".to_string())
		);
		assert_eq!(validate_task_state(None).unwrap(), None);
		let rejection = validate_task_state(Some("done".into())).unwrap_err();
		assert_eq!(
			rejection.find::<ReacherResponseError>().map(|e| e.code),
			Some(StatusCode::BAD_REQUEST)
		);
	}
}
