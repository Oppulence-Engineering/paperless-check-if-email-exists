use check_if_email_exists::LOG_TARGET;
use sqlx::PgPool;
use tracing::debug;

/// After a task completes, evaluate tenant-level auto-suppression rules
/// and add qualifying addresses to the suppression list.
///
/// Current rules (read from `tenants.settings` JSONB):
/// - `auto_suppress_below_score`: suppress emails scoring below this threshold
/// - `auto_suppress_categories`: suppress emails in these categories (e.g. ["invalid", "risky"])
pub async fn evaluate_post_completion_actions(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	email: &str,
	score: Option<i16>,
	category: Option<&str>,
) {
	let settings = match sqlx::query_scalar::<_, Option<serde_json::Value>>(
		"SELECT settings FROM tenants WHERE id = $1",
	)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await
	{
		Ok(Some(Some(s))) => s,
		Ok(_) => return, // no settings configured
		Err(e) => {
			debug!(
				target: LOG_TARGET,
				tenant_id = %tenant_id,
				error = ?e,
				"Failed to load tenant settings for auto-suppression"
			);
			return;
		}
	};

	let should_suppress = check_auto_suppress(&settings, score, category);

	if should_suppress {
		let normalized_email = email.trim().to_lowercase();
		if normalized_email.is_empty() {
			return;
		}
		let result = sqlx::query(
			r#"
			INSERT INTO v1_suppression_entries (tenant_id, email, reason, source)
			VALUES ($1, $2, 'auto_invalid', 'auto_action')
			ON CONFLICT (tenant_id, email) DO NOTHING
			"#,
		)
		.bind(tenant_id)
		.bind(&normalized_email)
		.execute(pg_pool)
		.await;

		let domain = normalized_email.split('@').nth(1).unwrap_or("unknown");
		match result {
			Ok(r) if r.rows_affected() > 0 => {
				debug!(
					target: LOG_TARGET,
					domain = domain,
					"Auto-suppressed based on conditional action rules"
				);
			}
			Err(e) => {
				debug!(
					target: LOG_TARGET,
					domain = domain,
					error = ?e,
					"Failed to auto-suppress"
				);
			}
			_ => {}
		}
	}
}

fn check_auto_suppress(
	settings: &serde_json::Value,
	score: Option<i16>,
	category: Option<&str>,
) -> bool {
	// Check score threshold
	if let Some(threshold) = settings
		.get("auto_suppress_below_score")
		.and_then(|v| v.as_i64())
	{
		if let Some(s) = score {
			if (s as i64) < threshold {
				return true;
			}
		}
	}

	// Check category list
	if let Some(categories) = settings
		.get("auto_suppress_categories")
		.and_then(|v| v.as_array())
	{
		if let Some(cat) = category {
			if categories.iter().any(|c| c.as_str() == Some(cat)) {
				return true;
			}
		}
	}

	false
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	#[test]
	fn auto_suppress_below_score_threshold() {
		let settings = json!({"auto_suppress_below_score": 30});
		assert!(check_auto_suppress(&settings, Some(20), None));
		assert!(!check_auto_suppress(&settings, Some(50), None));
		assert!(!check_auto_suppress(&settings, None, None));
	}

	#[test]
	fn auto_suppress_by_category() {
		let settings = json!({"auto_suppress_categories": ["invalid", "risky"]});
		assert!(check_auto_suppress(&settings, None, Some("invalid")));
		assert!(check_auto_suppress(&settings, None, Some("risky")));
		assert!(!check_auto_suppress(&settings, None, Some("valid")));
		assert!(!check_auto_suppress(&settings, None, None));
	}

	#[test]
	fn auto_suppress_combined_rules() {
		let settings = json!({
			"auto_suppress_below_score": 30,
			"auto_suppress_categories": ["invalid"]
		});
		// Score matches
		assert!(check_auto_suppress(&settings, Some(10), Some("valid")));
		// Category matches
		assert!(check_auto_suppress(&settings, Some(50), Some("invalid")));
		// Neither matches
		assert!(!check_auto_suppress(&settings, Some(50), Some("valid")));
	}

	#[test]
	fn no_settings_no_suppression() {
		let settings = json!({});
		assert!(!check_auto_suppress(&settings, Some(0), Some("invalid")));
	}
}
