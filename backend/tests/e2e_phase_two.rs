mod test_helpers;

#[cfg(test)]
mod tests {
	use crate::test_helpers::{insert_api_key, insert_tenant, TestDb, TestRabbitMq};
	use reacher_backend::config::{
		BackendConfig, PostgresConfig, RabbitMQConfig, StorageConfig, WorkerConfig,
	};
	use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
	use reqwest::multipart::{Form, Part};
	use serial_test::serial;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	async fn db_config(db_url: &str, secret: &str) -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.header_secret = Some(secret.to_string());
		config.storage = Some(StorageConfig::Postgres(PostgresConfig { read_replica_url: None,
			db_url: db_url.to_string(),
			extra: None,
		}));
		config.connect().await.expect("db config connect");
		Arc::new(config)
	}

	async fn worker_config(db_url: &str, amqp_url: &str, secret: &str) -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.header_secret = Some(secret.to_string());
		config.storage = Some(StorageConfig::Postgres(PostgresConfig { read_replica_url: None,
			db_url: db_url.to_string(),
			extra: None,
		}));
		config.worker = WorkerConfig {
			enable: true,
			rabbitmq: Some(RabbitMQConfig {
				url: amqp_url.to_string(),
				concurrency: 2,
			}),
			webhook: None,
		};
		config.connect().await.expect("worker config connect");
		Arc::new(config)
	}

	fn scored_result(
		email: &str,
		reachable: &str,
		score: i16,
		category: &str,
		sub_reason: &str,
	) -> serde_json::Value {
		let safe_to_send = category == "valid";
		let mut reason_codes = Vec::new();
		if sub_reason != "deliverable" {
			reason_codes.push(sub_reason);
		}
		if reason_codes.is_empty() {
			reason_codes.push("deliverable");
		}
		serde_json::json!({
			"input": email,
			"is_reachable": reachable,
			"misc": {
				"is_disposable": false,
				"is_role_account": false
			},
			"mx": {
				"accepts_mail": true
			},
			"smtp": {
				"can_connect_smtp": true,
				"is_catch_all": false,
				"is_deliverable": true,
				"is_disabled": false,
				"has_full_inbox": false
			},
			"syntax": {
				"domain": "example.com",
				"is_valid_syntax": true,
				"username": email.split('@').next().unwrap_or_default()
			},
			"score": {
				"score": score,
				"category": category,
				"sub_reason": sub_reason,
				"safe_to_send": safe_to_send,
				"reason_codes": reason_codes,
				"signals": {
					"valid_syntax": true,
					"reachable": reachable,
					"has_mx_records": true,
					"smtp_error": false,
					"smtp_can_connect": true,
					"smtp_is_deliverable": true,
					"smtp_is_disabled": false,
					"smtp_is_catch_all": false,
					"smtp_has_full_inbox": false,
					"is_disposable": false,
					"is_role_account": false,
					"is_spam_trap_domain": false,
					"is_free_provider": false,
					"has_domain_suggestion": false
				}
			}
		})
	}

	fn task_payload(email: &str, job_id: i32) -> serde_json::Value {
		serde_json::json!({
			"input": {"to_email": email},
			"job_id": {"bulk": job_id},
			"webhook": null
		})
	}

	fn spawn_server(config: Arc<BackendConfig>) -> String {
		let routes = create_routes(config);
		let (addr, server) = warp::serve(routes).bind_ephemeral(([127, 0, 0, 1], 0));
		tokio::task::spawn(server);
		format!("http://{}", addr)
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_check_email_includes_score() {
		let mut config = BackendConfig::empty();
		config.header_secret = Some("phase2".to_string());

		let response = request()
			.method("POST")
			.path("/v1/check_email")
			.header(REACHER_SECRET_HEADER, "phase2")
			.json(&serde_json::json!({ "to_email": "bad" }))
			.reply(&create_routes(Arc::new(config)))
			.await;

		assert_eq!(response.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
		assert_eq!(body["score"]["score"], 0);
		assert_eq!(body["score"]["category"], "invalid");
		assert!(body["score"]["signals"].is_object());
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_find_email_no_mx_short_circuits_without_quota_charge() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;
		let tenant_id = insert_tenant(db.pool(), "finder-no-mx", Some(100), 0).await;
		let (api_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let config = worker_config(db.db_url(), &rmq.amqp_url, "phase2").await;

		let response = request()
			.method("POST")
			.path("/v1/find_email")
			.header("authorization", format!("Bearer {}", api_key))
			.json(&serde_json::json!({
				"first_name": "John",
				"last_name": "Doe",
				"domain": "invalid.invalid"
			}))
			.reply(&create_routes(Arc::clone(&config)))
			.await;

		assert_eq!(response.status(), StatusCode::ACCEPTED);
		let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
		assert_eq!(body["status"], "completed");
		assert_eq!(body["candidates_checked"], 0);

		let used_this_period: i32 =
			sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id = $1")
				.bind(tenant_id)
				.fetch_one(db.pool())
				.await
				.unwrap();
		assert_eq!(used_this_period, 0);

		let job_id = body["job_id"].as_i64().unwrap();
		let get_response = request()
			.method("GET")
			.path(&format!("/v1/find_email/{}", job_id))
			.header("authorization", format!("Bearer {}", api_key))
			.reply(&create_routes(config))
			.await;
		assert_eq!(get_response.status(), StatusCode::OK);
		let get_body: serde_json::Value = serde_json::from_slice(get_response.body()).unwrap();
		assert_eq!(get_body["status"], "completed");
		assert_eq!(get_body["candidates_checked"], 0);
		assert_eq!(get_body["results"].as_array().unwrap().len(), 0);
		assert!(get_body["best_match"].is_null());
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_find_email_charges_quota_for_generated_candidate_count() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;
		let tenant_id = insert_tenant(db.pool(), "finder-quota", Some(100), 0).await;
		let (api_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let config = worker_config(db.db_url(), &rmq.amqp_url, "phase2").await;

		let response = request()
			.method("POST")
			.path("/v1/find_email")
			.header("authorization", format!("Bearer {}", api_key))
			.json(&serde_json::json!({
				"first_name": "John",
				"last_name": "Doe",
				"domain": "gmail.com"
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(
			response.status(),
			StatusCode::ACCEPTED,
			"{:?}",
			response.body()
		);
		let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
		assert_eq!(body["status"], "running");
		assert_eq!(body["candidates_checked"], 13);

		let used_this_period: i32 =
			sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id = $1")
				.bind(tenant_id)
				.fetch_one(db.pool())
				.await
				.unwrap();
		assert_eq!(used_this_period, 13);

		let queued_tasks: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result WHERE tenant_id = $1")
				.bind(tenant_id)
				.fetch_one(db.pool())
				.await
				.unwrap();
		assert_eq!(queued_tasks, 13);
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_jobs_download_csv_streams_large_scored_job() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;
		let config = worker_config(db.db_url(), &rmq.amqp_url, "phase2").await;
		let job_id: i32 = sqlx::query_scalar(
			"INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES ($1, NULL, 'completed'::job_state) RETURNING id",
		)
		.bind(1001_i32)
		.fetch_one(db.pool())
		.await
		.unwrap();

		for index in 0..1001 {
			let email = format!("user{}@example.com", index);
			let payload = serde_json::json!({
				"input": {"to_email": email},
				"job_id": {"bulk": job_id},
				"webhook": null
			});
			sqlx::query(
				r#"
				INSERT INTO v1_task_result (
					job_id, payload, task_state, result, error, score, score_category, sub_reason, safe_to_send, reason_codes, completed_at
				)
				VALUES ($1, $2, 'completed'::task_state, $3, NULL, 95, 'valid', 'deliverable', true, ARRAY['deliverable'], NOW())
				"#,
			)
			.bind(job_id)
			.bind(payload)
			.bind(scored_result(
				&format!("user{}@example.com", index),
				"safe",
				95,
				"valid",
				"deliverable",
			))
			.execute(db.pool())
			.await
			.unwrap();
		}

		let response = request()
			.method("GET")
			.path(&format!("/v1/jobs/{}/download?format=csv", job_id))
			.header(REACHER_SECRET_HEADER, "phase2")
			.reply(&create_routes(config))
			.await;

		assert_eq!(response.status(), StatusCode::OK);
		let body = String::from_utf8(response.body().to_vec()).unwrap();
		assert!(body
			.starts_with("input,is_reachable,score,category,sub_reason,safe_to_send,reason_codes"));
		assert!(body.contains(",95,valid,deliverable,"));
		assert_eq!(body.lines().count(), 1002);
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_jobs_results_and_ndjson_download_include_scored_payloads() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;
		let config = worker_config(db.db_url(), &rmq.amqp_url, "phase2").await;
		let job_id: i32 = sqlx::query_scalar(
			"INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES ($1, NULL, 'completed'::job_state) RETURNING id",
		)
		.bind(2_i32)
		.fetch_one(db.pool())
		.await
		.unwrap();

		for (index, score, category) in [(0, 95_i16, "valid"), (1, 75_i16, "risky")] {
			sqlx::query(
				r#"
				INSERT INTO v1_task_result (
					job_id, payload, task_state, result, error, score, score_category, sub_reason, safe_to_send, reason_codes, completed_at
				)
				VALUES ($1, $2, 'completed'::task_state, $3, NULL, $4, $5, 'deliverable', $6, ARRAY['deliverable'], NOW())
				"#,
			)
			.bind(job_id)
			.bind(task_payload(&format!("user{}@example.com", index), job_id))
			.bind(scored_result(
				&format!("user{}@example.com", index),
				"safe",
				score,
				category,
				"deliverable",
			))
			.bind(i32::from(score))
			.bind(category)
			.bind(category == "valid")
			.execute(db.pool())
			.await
			.unwrap();
		}

		let page = request()
			.method("GET")
			.path(&format!("/v1/jobs/{}/results?limit=10", job_id))
			.header(REACHER_SECRET_HEADER, "phase2")
			.reply(&create_routes(Arc::clone(&config)))
			.await;

		assert_eq!(page.status(), StatusCode::OK);
		let page_body: serde_json::Value = serde_json::from_slice(page.body()).unwrap();
		assert_eq!(page_body["results"].as_array().unwrap().len(), 2);
		assert_eq!(page_body["results"][0]["result"]["score"]["score"], 95);
		assert_eq!(
			page_body["results"][1]["result"]["score"]["category"],
			"risky"
		);

		let ndjson = request()
			.method("GET")
			.path(&format!("/v1/jobs/{}/download?format=json", job_id))
			.header(REACHER_SECRET_HEADER, "phase2")
			.reply(&create_routes(config))
			.await;

		assert_eq!(ndjson.status(), StatusCode::OK);
		assert_eq!(
			ndjson
				.headers()
				.get("content-type")
				.and_then(|value| value.to_str().ok()),
			Some("application/x-ndjson")
		);
		let body = String::from_utf8(ndjson.body().to_vec()).unwrap();
		let lines: Vec<&str> = body.lines().collect();
		assert_eq!(lines.len(), 2);
		let first: serde_json::Value = serde_json::from_str(lines[0]).unwrap();
		let second: serde_json::Value = serde_json::from_str(lines[1]).unwrap();
		assert_eq!(first["input"], "user0@example.com");
		assert_eq!(first["score"]["score"], 95);
		assert_eq!(second["score"]["category"], "risky");
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_find_email_get_returns_ranked_results_and_best_match() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;
		let tenant_id = insert_tenant(db.pool(), "finder-ranked", Some(100), 0).await;
		let (api_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let config = worker_config(db.db_url(), &rmq.amqp_url, "phase2").await;

		let bulk_job_id: i32 = sqlx::query_scalar(
			"INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES ($1, $2, 'running'::job_state) RETURNING id",
		)
		.bind(2_i32)
		.bind(tenant_id)
		.fetch_one(db.pool())
		.await
		.unwrap();
		let finder_job_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_finder_job (
				tenant_id, bulk_job_id, first_name, last_name, domain,
				normalized_first_name, normalized_last_name, status,
				domain_has_mx, domain_is_catch_all, candidates_checked
			)
			VALUES ($1, $2, 'John', 'Doe', 'example.com', 'john', 'doe', 'running'::job_state, true, false, 2)
			RETURNING id
			"#,
		)
		.bind(tenant_id)
		.bind(bulk_job_id)
		.fetch_one(db.pool())
		.await
		.unwrap();

		let high_task_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (
				job_id, payload, tenant_id, task_state, result, score, score_category, sub_reason, safe_to_send, reason_codes, completed_at
			)
			VALUES ($1, $2, $3, 'completed'::task_state, $4, 95, 'valid', 'deliverable', true, ARRAY['deliverable'], NOW())
			RETURNING id
			"#,
		)
		.bind(bulk_job_id)
		.bind(task_payload("john.doe@example.com", bulk_job_id))
		.bind(tenant_id)
		.bind(scored_result(
			"john.doe@example.com",
			"safe",
			95,
			"valid",
			"deliverable",
		))
		.fetch_one(db.pool())
		.await
		.unwrap();
		let low_task_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (
				job_id, payload, tenant_id, task_state, result, score, score_category, sub_reason, safe_to_send, reason_codes, completed_at
			)
			VALUES ($1, $2, $3, 'completed'::task_state, $4, 75, 'risky', 'deliverable', false, ARRAY['deliverable'], NOW())
			RETURNING id
			"#,
		)
		.bind(bulk_job_id)
		.bind(task_payload("jdoe@example.com", bulk_job_id))
		.bind(tenant_id)
		.bind(scored_result(
			"jdoe@example.com",
			"risky",
			75,
			"risky",
			"deliverable",
		))
		.fetch_one(db.pool())
		.await
		.unwrap();

		sqlx::query(
			r#"
			INSERT INTO v1_finder_result (finder_job_id, task_result_id, candidate_email, pattern)
			VALUES
				($1, $2, 'john.doe@example.com', 'first.last'),
				($1, $3, 'jdoe@example.com', 'flast')
			"#,
		)
		.bind(finder_job_id)
		.bind(high_task_id)
		.bind(low_task_id)
		.execute(db.pool())
		.await
		.unwrap();

		let response = request()
			.method("GET")
			.path(&format!("/v1/find_email/{}", finder_job_id))
			.header("authorization", format!("Bearer {}", api_key))
			.reply(&create_routes(Arc::clone(&config)))
			.await;

		assert_eq!(response.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
		assert_eq!(body["status"], "completed");
		assert_eq!(body["results"].as_array().unwrap().len(), 2);
		assert_eq!(body["results"][0]["email"], "john.doe@example.com");
		assert_eq!(body["results"][0]["score"], 95);
		assert_eq!(body["results"][1]["email"], "jdoe@example.com");
		assert_eq!(body["best_match"]["email"], "john.doe@example.com");
		assert_eq!(body["best_match"]["confidence"], "high");

		let stored = sqlx::query(
			"SELECT status::TEXT AS status, best_match_email, best_match_score, best_match_confidence FROM v1_finder_job WHERE id = $1",
		)
		.bind(finder_job_id)
		.fetch_one(db.pool())
		.await
		.unwrap();
		let status: String = sqlx::Row::get(&stored, "status");
		let best_match_email: Option<String> = sqlx::Row::get(&stored, "best_match_email");
		let best_match_score: Option<i16> = sqlx::Row::get(&stored, "best_match_score");
		let best_match_confidence: Option<String> =
			sqlx::Row::get(&stored, "best_match_confidence");
		assert_eq!(status, "completed");
		assert_eq!(best_match_email.as_deref(), Some("john.doe@example.com"));
		assert_eq!(best_match_score, Some(95));
		assert_eq!(best_match_confidence.as_deref(), Some("high"));
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_lists_upload_detail_and_download_round_trip() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;
		let tenant_id = insert_tenant(db.pool(), "lists-roundtrip", Some(100), 0).await;
		let (api_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let config = worker_config(db.db_url(), &rmq.amqp_url, "phase2").await;
		let server_url = spawn_server(Arc::clone(&config));
		let client = reqwest::Client::new();
		let csv = "name,email,company\nAlice,alice@example.com,Acme\nBob,,Widgets";
		let upload = client
			.post(format!("{}/v1/lists", server_url))
			.bearer_auth(&api_key)
			.multipart(
				Form::new().text("name", "My List").part(
					"file",
					Part::bytes(csv.as_bytes().to_vec())
						.file_name("contacts.csv")
						.mime_str("text/csv")
						.unwrap(),
				),
			)
			.send()
			.await
			.unwrap();
		let upload_status = upload.status();
		let upload_text = upload.text().await.unwrap();
		assert_eq!(
			upload_status,
			reqwest::StatusCode::ACCEPTED,
			"{}",
			upload_text
		);
		let upload_body: serde_json::Value = serde_json::from_str(&upload_text).unwrap();
		let list_id = upload_body["list_id"].as_i64().unwrap() as i32;
		let job_id = upload_body["job_id"].as_i64().unwrap() as i32;
		assert_eq!(upload_body["email_column"], "email");

		let task_id: i32 = sqlx::query_scalar(
			"SELECT id FROM v1_task_result WHERE (extra->>'list_id')::INTEGER = $1 AND (extra->>'row_index')::INTEGER = 0",
		)
		.bind(list_id)
		.fetch_one(db.pool())
		.await
		.unwrap();
		sqlx::query(
			r#"
			UPDATE v1_task_result
			SET task_state = 'completed'::task_state,
			    result = $2,
			    score = 95,
			    score_category = 'valid',
			    sub_reason = 'deliverable',
			    safe_to_send = true,
			    reason_codes = ARRAY['deliverable'],
			    completed_at = NOW()
			WHERE id = $1
			"#,
		)
		.bind(task_id)
		.bind(scored_result(
			"alice@example.com",
			"safe",
			95,
			"valid",
			"deliverable",
		))
		.execute(db.pool())
		.await
		.unwrap();
		sqlx::query(
			"UPDATE v1_bulk_job SET status = 'completed'::job_state, completed_at = NOW(), updated_at = NOW() WHERE id = $1",
		)
		.bind(job_id)
		.execute(db.pool())
		.await
		.unwrap();

		let detail = client
			.get(format!("{}/v1/lists/{}", server_url, list_id))
			.bearer_auth(&api_key)
			.send()
			.await
			.unwrap();
		assert_eq!(detail.status(), reqwest::StatusCode::OK);
		let detail_body: serde_json::Value = detail.json().await.unwrap();
		assert_eq!(detail_body["summary"]["total_valid"], 1);
		assert_eq!(detail_body["summary"]["total_invalid"], 1);
		assert_eq!(detail_body["summary"]["total_processed"], 2);

		let download = client
			.get(format!(
				"{}/v1/lists/{}/download?format=csv",
				server_url, list_id
			))
			.bearer_auth(&api_key)
			.send()
			.await
			.unwrap();
		let download_status = download.status();
		let csv_body = download.text().await.unwrap();
		assert_eq!(download_status, reqwest::StatusCode::OK, "{}", csv_body);
		assert!(csv_body.starts_with(
			"name,email,company,is_reachable,score,category,safe_to_send,reason_codes,is_disposable,smtp_is_deliverable,error"
		));
		assert!(csv_body
			.contains("Alice,alice@example.com,Acme,safe,95,valid,true,deliverable,false,true,"));
		assert!(csv_body.contains("Bob,,Widgets,invalid,0,invalid"));
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_lists_index_filter_download_and_delete() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;
		let tenant_id = insert_tenant(db.pool(), "lists-filtered", Some(100), 0).await;
		let (api_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let config = worker_config(db.db_url(), &rmq.amqp_url, "phase2").await;

		let job_id: i32 = sqlx::query_scalar(
			"INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES ($1, $2, 'completed'::job_state) RETURNING id",
		)
		.bind(2_i32)
		.bind(tenant_id)
		.fetch_one(db.pool())
		.await
		.unwrap();
		let list_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_lists (
				tenant_id, job_id, name, original_filename, file_size_bytes, total_rows,
				email_column, original_headers, original_data, status, completed_at
			)
			VALUES ($1, $2, 'Seeded List', 'seeded.csv', 64, 2, 'email', $3, $4, 'completed'::list_status, NOW())
			RETURNING id
			"#,
		)
		.bind(tenant_id)
		.bind(job_id)
		.bind(vec![
			"name".to_string(),
			"email".to_string(),
			"company".to_string(),
		])
		.bind(serde_json::json!({
			"0": {"name": "Alice", "email": "alice@example.com", "company": "Acme"},
			"1": {"name": "Bob", "email": "bob@example.com", "company": "Widgets"}
		}))
		.fetch_one(db.pool())
		.await
		.unwrap();

		for (row_index, email, reachable, score, category, sub_reason) in [
			(
				0_i32,
				"alice@example.com",
				"safe",
				95_i16,
				"valid",
				"deliverable",
			),
			(
				1_i32,
				"bob@example.com",
				"invalid",
				0_i16,
				"invalid",
				"invalid_recipient",
			),
		] {
			let reason_codes_val: Vec<String> = if sub_reason == "deliverable" {
				vec!["deliverable".to_string()]
			} else {
				vec![sub_reason.to_string()]
			};
			sqlx::query(
				r#"
				INSERT INTO v1_task_result (
					job_id, payload, extra, tenant_id, task_state, result, error,
					score, score_category, sub_reason, safe_to_send, reason_codes, completed_at
				)
				VALUES ($1, $2, $3, $4, 'completed'::task_state, $5, NULL, $6, $7, $8, $9, $10, NOW())
				"#,
			)
			.bind(job_id)
			.bind(task_payload(email, job_id))
			.bind(serde_json::json!({
				"list_id": list_id,
				"row_index": row_index,
				"email_column": "email"
			}))
			.bind(tenant_id)
			.bind(scored_result(email, reachable, score, category, sub_reason))
			.bind(i32::from(score))
			.bind(category)
			.bind(sub_reason)
			.bind(category == "valid")
			.bind(&reason_codes_val)
			.execute(db.pool())
			.await
			.unwrap();
		}

		let lists = request()
			.method("GET")
			.path("/v1/lists?limit=10")
			.header("authorization", format!("Bearer {}", api_key))
			.reply(&create_routes(Arc::clone(&config)))
			.await;

		assert_eq!(lists.status(), StatusCode::OK);
		let lists_body: serde_json::Value = serde_json::from_slice(lists.body()).unwrap();
		assert_eq!(lists_body["total"], 1);
		assert_eq!(lists_body["lists"][0]["id"], list_id);
		assert_eq!(lists_body["lists"][0]["name"], "Seeded List");
		assert!(lists_body["lists"][0]["created_at"].is_string());
		assert!(lists_body["lists"][0]["completed_at"].is_string());

		let filtered_download = request()
			.method("GET")
			.path(&format!(
				"/v1/lists/{}/download?format=csv&filter=valid",
				list_id
			))
			.header("authorization", format!("Bearer {}", api_key))
			.reply(&create_routes(Arc::clone(&config)))
			.await;

		assert_eq!(filtered_download.status(), StatusCode::OK);
		let csv_body = String::from_utf8(filtered_download.body().to_vec()).unwrap();
		assert!(csv_body.starts_with(
			"name,email,company,is_reachable,score,category,safe_to_send,reason_codes,is_disposable,smtp_is_deliverable,error"
		));
		assert!(csv_body
			.contains("Alice,alice@example.com,Acme,safe,95,valid,true,deliverable,false,true,"));
		assert!(!csv_body.contains("Bob,bob@example.com,Widgets"));
		assert_eq!(csv_body.lines().count(), 2);

		let delete = request()
			.method("DELETE")
			.path(&format!("/v1/lists/{}", list_id))
			.header("authorization", format!("Bearer {}", api_key))
			.reply(&create_routes(Arc::clone(&config)))
			.await;

		assert_eq!(delete.status(), StatusCode::OK);
		let delete_body: serde_json::Value = serde_json::from_slice(delete.body()).unwrap();
		assert_eq!(delete_body["deleted"], true);

		let list_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_lists WHERE id = $1")
			.bind(list_id)
			.fetch_one(db.pool())
			.await
			.unwrap();
		let job_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_bulk_job WHERE id = $1")
			.bind(job_id)
			.fetch_one(db.pool())
			.await
			.unwrap();
		let task_count: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM v1_task_result WHERE (extra->>'list_id')::INTEGER = $1",
		)
		.bind(list_id)
		.fetch_one(db.pool())
		.await
		.unwrap();
		assert_eq!(list_count, 0);
		assert_eq!(job_count, 0);
		assert_eq!(task_count, 0);
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_lists_upload_enforces_free_plan_row_limit() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;
		let tenant_id: uuid::Uuid = sqlx::query_scalar(
			r#"
			INSERT INTO tenants (
				name, slug, contact_email, plan_tier, status, monthly_email_limit, used_this_period
			)
			VALUES ($1, $2, $3, 'free', 'active', $4, 0)
			RETURNING id
			"#,
		)
		.bind("Free Plan List Tenant")
		.bind("lists-free-limit")
		.bind("lists-free-limit@test.com")
		.bind(10_000_i32)
		.fetch_one(db.pool())
		.await
		.unwrap();
		let (api_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let config = worker_config(db.db_url(), &rmq.amqp_url, "phase2").await;
		let server_url = spawn_server(config);
		let client = reqwest::Client::new();

		let mut csv = String::from("name,email\n");
		for index in 0..1001 {
			csv.push_str(&format!("User {index},user{index}@example.com\n"));
		}

		let upload = client
			.post(format!("{}/v1/lists", server_url))
			.bearer_auth(&api_key)
			.multipart(
				Form::new().part(
					"file",
					Part::bytes(csv.into_bytes())
						.file_name("too-many.csv")
						.mime_str("text/csv")
						.unwrap(),
				),
			)
			.send()
			.await
			.unwrap();

		let status = upload.status();
		let body = upload.text().await.unwrap();
		assert_eq!(status, reqwest::StatusCode::BAD_REQUEST, "{}", body);
		assert!(body.contains("plan allows 1000"), "{}", body);

		let list_count: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM v1_lists WHERE tenant_id = $1")
				.bind(tenant_id)
				.fetch_one(db.pool())
				.await
				.unwrap();
		let job_count: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM v1_bulk_job WHERE tenant_id = $1")
				.bind(tenant_id)
				.fetch_one(db.pool())
				.await
				.unwrap();
		assert_eq!(list_count, 0);
		assert_eq!(job_count, 0);
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_reputation_second_call_is_cached() {
		let db = TestDb::start().await;
		let config = db_config(db.db_url(), "phase2").await;

		let first = request()
			.method("POST")
			.path("/v1/reputation/check")
			.header(REACHER_SECRET_HEADER, "phase2")
			.json(&serde_json::json!({ "domain": "example.com", "force_refresh": false }))
			.reply(&create_routes(Arc::clone(&config)))
			.await;
		assert_eq!(first.status(), StatusCode::OK);
		let first_body: serde_json::Value = serde_json::from_slice(first.body()).unwrap();
		assert_eq!(first_body["cached"], false);

		let second = request()
			.method("POST")
			.path("/v1/reputation/check")
			.header(REACHER_SECRET_HEADER, "phase2")
			.json(&serde_json::json!({ "domain": "example.com", "force_refresh": false }))
			.reply(&create_routes(config))
			.await;
		assert_eq!(second.status(), StatusCode::OK);
		let second_body: serde_json::Value = serde_json::from_slice(second.body()).unwrap();
		assert_eq!(second_body["cached"], true);
	}
}
