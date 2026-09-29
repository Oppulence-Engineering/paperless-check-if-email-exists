mod test_helpers;

#[cfg(test)]
mod tests {
	use crate::test_helpers::{
		build_test_config, insert_api_key_with_scopes, insert_job, insert_list, insert_scored_task,
		insert_tenant, ConfigProfile, TestDb,
	};
	use reacher_backend::http::create_routes;
	use serial_test::serial;
	use warp::http::StatusCode;
	use warp::test::request;

	fn result_json(
		email: &str,
		reachable: &str,
		syntax_valid: bool,
		role_account: bool,
	) -> serde_json::Value {
		serde_json::json!({
			"input": email,
			"is_reachable": reachable,
			"syntax": {"is_valid_syntax": syntax_valid},
			"misc": {
				"is_disposable": false,
				"is_role_account": role_account,
				"is_spam_trap_domain": false
			},
			"smtp": {
				"is_catch_all": false,
				"has_full_inbox": false
			}
		})
	}

	#[tokio::test]
	#[serial]
	async fn creates_gets_downloads_and_reuses_remediation_plan() {
		let db = TestDb::start().await;
		let tenant_id = insert_tenant(db.pool(), "remediation-main", Some(10_000), 0).await;
		let (key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["lists"]).await;
		let job_id = insert_job(db.pool(), Some(tenant_id), 5, "completed").await;
		let list_id = insert_list(
			db.pool(),
			tenant_id,
			job_id,
			"Remediation fixture",
			"completed",
			5,
			&["email", "name"],
			serde_json::json!({
				"0": {"email": "user@Example.COM", "name": "Case"},
				"1": {"email": "billing@example.com", "name": "Role"},
				"2": {"email": "bad", "name": "Invalid"},
				"3": {"email": "good@example.com", "name": "Safe"},
				"4": {"email": "suppressed@example.com", "name": "Suppressed"}
			}),
		)
		.await;

		sqlx::query(
			"INSERT INTO v1_suppression_entries (tenant_id, email, canonical_email, reason) VALUES ($1, $2, $2, 'manual'::suppression_reason)",
		)
		.bind(tenant_id)
		.bind("suppressed@example.com")
		.execute(db.pool())
		.await
		.unwrap();

		insert_scored_task(
			db.pool(),
			job_id,
			Some(tenant_id),
			"user@Example.COM",
			Some(serde_json::json!({"list_id": list_id, "row_index": 0, "email_column": "email"})),
			Some(result_json("user@Example.COM", "safe", true, false)),
			"completed",
			Some(95),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("user@example.com"),
			false,
		)
		.await;
		insert_scored_task(
			db.pool(),
			job_id,
			Some(tenant_id),
			"billing@example.com",
			Some(serde_json::json!({"list_id": list_id, "row_index": 1, "email_column": "email"})),
			Some(result_json("billing@example.com", "safe", true, true)),
			"completed",
			Some(80),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["role_account".to_string()]),
			Some("billing@example.com"),
			false,
		)
		.await;
		insert_scored_task(
			db.pool(),
			job_id,
			Some(tenant_id),
			"bad",
			Some(serde_json::json!({"list_id": list_id, "row_index": 2, "email_column": "email"})),
			Some(result_json("bad", "invalid", false, false)),
			"completed",
			Some(0),
			Some("invalid"),
			Some("invalid_syntax"),
			Some(false),
			Some(vec!["invalid_syntax".to_string()]),
			Some("bad"),
			false,
		)
		.await;
		insert_scored_task(
			db.pool(),
			job_id,
			Some(tenant_id),
			"good@example.com",
			Some(serde_json::json!({"list_id": list_id, "row_index": 3, "email_column": "email"})),
			Some(result_json("good@example.com", "safe", true, false)),
			"completed",
			Some(99),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("good@example.com"),
			false,
		)
		.await;
		insert_scored_task(
			db.pool(),
			job_id,
			Some(tenant_id),
			"suppressed@example.com",
			Some(serde_json::json!({"list_id": list_id, "row_index": 4, "email_column": "email"})),
			Some(result_json("suppressed@example.com", "safe", true, false)),
			"completed",
			Some(99),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("suppressed@example.com"),
			false,
		)
		.await;

		let config = build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await;
		let routes = create_routes(config);
		let created = request()
			.path(&format!("/v1/lists/{}/remediation-plan", list_id))
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({}))
			.reply(&routes)
			.await;
		assert_eq!(
			created.status(),
			StatusCode::CREATED,
			"{:?}",
			created.body()
		);
		let body: serde_json::Value = serde_json::from_slice(created.body()).unwrap();
		// Case-normalized user@Example.COM is fixed. A role account that is
		// still safe_to_send stays safe; invalid syntax and the active
		// suppression are dropped.
		assert_eq!(body["summary_counts"]["fixed"], 1, "{body}");
		assert_eq!(body["summary_counts"]["safe"], 2, "{body}");
		assert_eq!(body["summary_counts"]["review"], 0, "{body}");
		assert_eq!(body["summary_counts"]["drop"], 2, "{body}");
		let plan_id = body["id"].as_i64().unwrap();

		let repeated = request()
			.path(&format!("/v1/lists/{}/remediation-plan", list_id))
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({}))
			.reply(&routes)
			.await;
		assert_eq!(
			repeated.status(),
			StatusCode::CREATED,
			"{:?}",
			repeated.body()
		);
		let repeated_body: serde_json::Value = serde_json::from_slice(repeated.body()).unwrap();
		assert_eq!(repeated_body["id"], plan_id);

		let fetched = request()
			.path(&format!("/v1/lists/{}/remediation-plan", list_id))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(fetched.status(), StatusCode::OK, "{:?}", fetched.body());
		let fetched_body: serde_json::Value = serde_json::from_slice(fetched.body()).unwrap();
		assert_eq!(fetched_body["id"], plan_id);

		let export = request()
			.path(&format!("/v1/lists/{}/remediation-exports", list_id))
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"plan_id": plan_id,
				"partitions": ["safe_to_send"]
			}))
			.reply(&routes)
			.await;
		assert_eq!(export.status(), StatusCode::CREATED, "{:?}", export.body());
		let export_body: serde_json::Value = serde_json::from_slice(export.body()).unwrap();
		let export_id = export_body["id"].as_i64().unwrap();

		let download = request()
			.path(&format!(
				"/v1/lists/{}/remediation-exports/{}/download",
				list_id, export_id
			))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(download.status(), StatusCode::OK, "{:?}", download.body());
		let csv = String::from_utf8(download.body().to_vec()).unwrap();
		assert!(csv.contains("remediation_classification"));
		assert!(csv.contains("user@example.com"));
		assert!(csv.contains("good@example.com"));
		assert!(csv.contains("billing@example.com"));
		assert!(!csv.contains("suppressed@example.com"));
		assert!(!csv.contains("bad,"));
	}

	#[tokio::test]
	#[serial]
	async fn rejects_incomplete_list_unless_partial_is_requested() {
		let db = TestDb::start().await;
		let tenant_id = insert_tenant(db.pool(), "remediation-partial", Some(10_000), 0).await;
		let (key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["lists"]).await;
		let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
		let list_id = insert_list(
			db.pool(),
			tenant_id,
			job_id,
			"Incomplete remediation fixture",
			"processing",
			1,
			&["email"],
			serde_json::json!({"0": {"email": "pending@example.com"}}),
		)
		.await;
		insert_scored_task(
			db.pool(),
			job_id,
			Some(tenant_id),
			"pending@example.com",
			Some(serde_json::json!({"list_id": list_id, "row_index": 0, "email_column": "email"})),
			None,
			"running",
			None,
			None,
			None,
			None,
			None,
			Some("pending@example.com"),
			false,
		)
		.await;

		let config = build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await;
		let routes = create_routes(config);
		let rejected = request()
			.path(&format!("/v1/lists/{}/remediation-plan", list_id))
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({}))
			.reply(&routes)
			.await;
		assert_eq!(
			rejected.status(),
			StatusCode::BAD_REQUEST,
			"{:?}",
			rejected.body()
		);

		let partial = request()
			.path(&format!("/v1/lists/{}/remediation-plan", list_id))
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"allow_partial": true}))
			.reply(&routes)
			.await;
		assert_eq!(
			partial.status(),
			StatusCode::CREATED,
			"{:?}",
			partial.body()
		);
		let body: serde_json::Value = serde_json::from_slice(partial.body()).unwrap();
		assert_eq!(body["summary_counts"]["review"], 1);
	}
}
