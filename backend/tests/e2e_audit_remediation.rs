#[path = "../src/bin/prune_db.rs"]
#[allow(dead_code)]
mod legacy_prune;
mod test_helpers;

use reacher_backend::config::ThrottleConfig;
use reacher_backend::tenant::auth::resolve_tenant_context_by_id;
use reacher_backend::tenant::quota::{check_and_increment_quota, QuotaCheckResult};
use reacher_backend::throttle::reserve_tenant_rate;
use serial_test::serial;
use test_helpers::{insert_tenant, test_db_url, TestDb};

#[tokio::test]
#[serial]
async fn mailbox_identity_migration_preserves_alias_history_and_local_case() {
	use reacher_backend::http::v1::lists::canonicalize::canonicalize_email;
	use reacher_backend::suppression::has_active_suppression;
	let db = test_helpers::migrate_scratch_db_to_head(
		|version| version < "20261020000002",
		Some(
			std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("tests/fixtures/audit_identity_v1.sql"),
		),
	)
	.await;
	let tenant = uuid::Uuid::parse_str("22222222-2222-4222-8222-222222222222").unwrap();
	for input in [
		"A.B+old@GoogleMail.com",
		"ab@gmail.com",
		"Sales@COMPANY.EXAMPLE",
		"sales@company.example",
		" First.Last+Tag@COMPANY.EXAMPLE ",
		"",
		"a@@b",
		"@b",
		"Ä.B+tag@GMAIL.com",
		"\u{2003}User@EXAMPLE.COM\u{2003}",
	] {
		let sql_identity: Option<String> = sqlx::query_scalar("SELECT mailbox_identity($1)")
			.bind(input)
			.fetch_one(db.pool())
			.await
			.unwrap();
		assert_eq!(sql_identity, canonicalize_email(input), "{input}");
	}
	assert!(
		has_active_suppression(db.pool(), tenant, "a.b+new@gmail.com")
			.await
			.unwrap()
	);
	assert!(
		has_active_suppression(db.pool(), tenant, "Sales@company.example")
			.await
			.unwrap()
	);
	assert!(
		!has_active_suppression(db.pool(), tenant, "sales@company.example")
			.await
			.unwrap()
	);
	let aliases: Vec<(String, String, bool)> = sqlx::query_as(
		"SELECT email, status, expires_at IS NULL FROM v1_suppression_entries WHERE canonical_email = 'ab@gmail.com' ORDER BY email")
		.fetch_all(db.pool()).await.unwrap();
	assert_eq!(aliases.len(), 3, "no alias history may be deleted");
	assert_eq!(
		aliases
			.iter()
			.filter(|(_, status, _)| status == "active")
			.count(),
		1
	);
	assert!(aliases
		.iter()
		.any(|(email, status, permanent)| email == "a.b@gmail.com"
			&& status == "active"
			&& *permanent));
	let original_events: i64 = sqlx::query_scalar(
		"SELECT count(*) FROM v1_suppression_events WHERE event_type = 'created'",
	)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(original_events, 4);
	let migration_events: i64 = sqlx::query_scalar(
		"SELECT count(*) FROM v1_suppression_events WHERE event_type = 'identity_migrated' AND actor_id = 'mailbox_identity_v2'",
	)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(migration_events, 4);
	let history: (String, String) = sqlx::query_as(
		"SELECT canonical_email, extra->>'identity_v1_canonical' FROM v1_task_result",
	)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(
		history,
		(
			"Sales@company.example".into(),
			"sales@company.example".into()
		)
	);
	let outcomes: Vec<String> = sqlx::query_scalar(
		"SELECT canonical_email FROM v1_contact_outcomes ORDER BY canonical_email COLLATE \"C\"",
	)
	.fetch_all(db.pool())
	.await
	.unwrap();
	assert_eq!(outcomes, vec!["Sales@company.example", "ab@gmail.com"]);
}

#[tokio::test]
#[serial]
async fn deployed_casefold_history_recovers_distinct_suppressions() {
	let db = test_helpers::migrate_scratch_db_to_head(
		|version| version < "20261021000000",
		Some(
			std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("tests/fixtures/mailbox_identity_case_distinct.sql"),
		),
	)
	.await;
	let rows: Vec<(String, String)> = sqlx::query_as(
        "SELECT canonical_email, status FROM v1_suppression_entries ORDER BY canonical_email COLLATE \"C\"",
    ).fetch_all(db.pool()).await.unwrap();
	assert_eq!(
		rows,
		vec![
			("Sales@company.example".into(), "active".into()),
			("sales@company.example".into(), "active".into()),
		]
	);
	let backups: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM mailbox_identity_v3_backup WHERE table_name = 'v1_suppression_entries'",
    ).fetch_one(db.pool()).await.unwrap();
	assert_eq!(backups, 2);
	let legacy: String =
		sqlx::query_scalar("SELECT canonical_email FROM v1_task_result WHERE job_id IS NULL")
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_eq!(
		legacy, "legacy@company.example",
		"retain keys without a supplied address"
	);

	let independent: i64 = sqlx::query_scalar("SELECT count(*) FROM v1_task_result WHERE job_id = 9901 AND is_duplicate IS FALSE AND canonical_task_id IS NULL")
        .fetch_one(db.pool()).await.unwrap();
	assert_eq!(
		independent, 2,
		"restore independently verified case-distinct mailboxes"
	);
	let counts: (i32, i32) = sqlx::query_as(
		"SELECT unique_emails, deduplicated_count FROM v1_lists WHERE job_id = 9901",
	)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(counts, (2, 0));
	let outcomes: i64 = sqlx::query_scalar("SELECT count(*) FROM verification_outcome_evidence")
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(outcomes, 2, "retain evidence for both distinct mailboxes");

	sqlx::query(r#"INSERT INTO v1_task_result (tenant_id, payload, canonical_email, task_state) VALUES ('33333333-3333-4333-8333-333333333333', '{"input":{"to_email":"Support@company.example"}}', 'support@company.example', 'queued')"#)
        .execute(db.pool()).await.unwrap();
	let mut tx = db.pool().begin().await.unwrap();
	let error = sqlx::raw_sql(include_str!(
		"../migrations/20261022000000_restore_mailbox_identity_contract.up.sql"
	))
	.execute(&mut *tx)
	.await
	.unwrap_err();
	assert!(error
		.to_string()
		.contains("requires affected jobs to finish"));
	tx.rollback().await.unwrap();
	let identity: String = sqlx::query_scalar(
		"SELECT canonical_email FROM v1_task_result WHERE task_state = 'queued'",
	)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(
		identity, "support@company.example",
		"live work must not be retargeted"
	);
}

#[tokio::test]
#[serial]
async fn identity_recovery_preserves_a_later_suppression_deactivation() {
	let db = test_helpers::migrate_scratch_db_to_head(
		|version| version < "20261022000000",
		Some(
			std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
				.join("tests/fixtures/mailbox_identity_deactivated_v3.sql"),
		),
	)
	.await;
	let active: i64 =
		sqlx::query_scalar("SELECT count(*) FROM v1_suppression_entries WHERE status = 'active'")
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_eq!(
		active, 0,
		"do not revive aliases after their survivor was deactivated"
	);
}

#[tokio::test]
#[serial]
async fn every_suppression_writer_uses_the_same_identity_and_retains_supplied_address() {
	use reacher_backend::suppression::has_active_suppression;
	use serde_json::json;
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "shared-mailbox-identity", None, 0).await;
	let (key, _) = test_helpers::insert_api_key(db.pool(), tenant).await;
	let config = test_helpers::build_test_config(
		test_helpers::ConfigProfile::PseudoWorker,
		Some(&test_db_url()),
		None,
	)
	.await;
	let routes = reacher_backend::http::create_routes(config);
	for (path, address, canonical, body) in [
		(
			"/v1/suppressions",
			"A.B+tag@GoogleMail.com",
			"ab@gmail.com",
			json!({"emails":["A.B+tag@GoogleMail.com"]}),
		),
		(
			"/v1/suppressions/import",
			"C.D+tag@gmail.com",
			"cd@gmail.com",
			json!({"emails":["C.D+tag@gmail.com"]}),
		),
		(
			"/v1/outcomes",
			"E.F+tag@gmail.com",
			"ef@gmail.com",
			json!({"provider":"audit", "outcomes":[{"email":"E.F+tag@gmail.com","event_type":"bounced"}]}),
		),
	] {
		let response = warp::test::request()
			.method("POST")
			.path(path)
			.header("authorization", format!("Bearer {key}"))
			.json(&body)
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 200, "{}: {:?}", path, response.body());
		assert!(has_active_suppression(db.pool(), tenant, canonical)
			.await
			.unwrap());
		let stored: String = sqlx::query_scalar("SELECT email FROM v1_suppression_entries WHERE tenant_id = $1 AND canonical_email = $2 AND status = 'active'")
			.bind(tenant).bind(canonical).fetch_one(db.pool()).await.unwrap();
		assert_eq!(stored, address);
		let checked = warp::test::request()
			.path(&format!("/v1/suppressions/check?email={canonical}"))
			.header("authorization", format!("Bearer {key}"))
			.reply(&routes)
			.await;
		assert_eq!(checked.status(), 200, "{:?}", checked.body());
		assert_eq!(
			serde_json::from_slice::<serde_json::Value>(checked.body()).unwrap()["suppressed"],
			true
		);
	}
	sqlx::query("UPDATE tenants SET settings = '{\"auto_suppress_below_score\":30}' WHERE id = $1")
		.bind(tenant)
		.execute(db.pool())
		.await
		.unwrap();
	reacher_backend::worker::actions::evaluate_post_completion_actions(
		db.pool(),
		tenant,
		"G.H+tag@GoogleMail.com",
		Some(10),
		None,
	)
	.await
	.unwrap();
	assert!(has_active_suppression(db.pool(), tenant, "gh@gmail.com")
		.await
		.unwrap());
	let campaign = reacher_backend::outcomes::ingest_outcomes(
		db.pool(),
		tenant,
		&Default::default(),
		&[reacher_backend::outcomes::IngestOutcome {
			email: "I.J+tag@GoogleMail.com".into(),
			outcome_type: reacher_backend::outcomes::OutcomeType::HardBounce,
			occurred_at: chrono::Utc::now(),
			source: Some("identity-test".into()),
			campaign_id: None,
			metadata: None,
		}],
	)
	.await;
	assert_eq!(campaign.accepted, 1);
	assert!(has_active_suppression(db.pool(), tenant, "ij@gmail.com")
		.await
		.unwrap());
	let supplied: String = sqlx::query_scalar("SELECT email FROM verification_outcomes WHERE tenant_id = $1 AND canonical_email = 'ij@gmail.com'")
		.bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(supplied, "I.J+tag@GoogleMail.com");
	let response = warp::test::request()
		.method("POST")
		.path("/v1/suppressions")
		.header("authorization", format!("Bearer {key}"))
		.json(&json!({"emails":["Sales@COMPANY.EXAMPLE", "A.B+other@gmail.com"]}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 200);
	assert!(
		has_active_suppression(db.pool(), tenant, "Sales@company.example")
			.await
			.unwrap()
	);
	assert!(
		!has_active_suppression(db.pool(), tenant, "sales@company.example")
			.await
			.unwrap()
	);
}

#[tokio::test]
#[serial]
async fn list_dispatch_preserves_routing_addresses_and_only_groups_gmail_aliases() {
	use reacher_backend::worker::consume::CHECK_EMAIL_QUEUE;
	use reacher_backend::worker::do_work::CheckEmailTask;
	let db = TestDb::start().await;
	let broker = test_helpers::TestRabbitMq::start().await;
	let tenant = insert_tenant(db.pool(), "list-identity", Some(100), 0).await;
	let (key, _) = test_helpers::insert_api_key(db.pool(), tenant).await;
	let config = test_helpers::build_test_config(
		test_helpers::ConfigProfile::WorkerRabbit,
		Some(&test_db_url()),
		Some(&broker.amqp_url),
	)
	.await;
	let channel = config.must_worker_config().unwrap().channel;
	channel
		.queue_purge(CHECK_EMAIL_QUEUE.into(), Default::default())
		.await
		.unwrap();
	let routes = reacher_backend::http::create_routes(config);
	let body = concat!("--IdentityBoundary\r\nContent-Disposition: form-data; name=\"file\"; filename=\"contacts.csv\"\r\n",
		"Content-Type: text/csv\r\n\r\nemail\r\nSales@COMPANY.EXAMPLE\r\nsales@company.example\r\n",
		"A.B+tag@GoogleMail.com\r\nab@gmail.com\r\n--IdentityBoundary--\r\n");
	let response = warp::test::request()
		.method("POST")
		.path("/v1/lists")
		.header("authorization", format!("Bearer {key}"))
		.header(
			"content-type",
			"multipart/form-data; boundary=IdentityBoundary",
		)
		.body(body)
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 202, "{:?}", response.body());
	let mut dispatched = vec![];
	for _ in 0..3 {
		assert!(reacher_backend::worker::dispatch::publish_next(
			db.pool(),
			&channel,
			"check_email"
		)
		.await
		.unwrap());
		let delivery = channel
			.basic_get(CHECK_EMAIL_QUEUE.into(), Default::default())
			.await
			.unwrap()
			.unwrap();
		let task: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();
		dispatched.push(task.input.to_email);
		delivery.ack(Default::default()).await.unwrap();
	}
	dispatched.sort();
	assert_eq!(
		dispatched,
		vec![
			"A.B+tag@GoogleMail.com",
			"Sales@COMPANY.EXAMPLE",
			"sales@company.example"
		]
	);
	assert!(channel
		.basic_get(CHECK_EMAIL_QUEUE.into(), Default::default())
		.await
		.unwrap()
		.is_none());
	let rows: Vec<(String, String, bool)> = sqlx::query_as(
		"SELECT payload->'input'->>'to_email', canonical_email, is_duplicate FROM v1_task_result WHERE tenant_id = $1 ORDER BY (extra->>'row_index')::int")
		.bind(tenant).fetch_all(db.pool()).await.unwrap();
	assert_eq!(
		rows,
		vec![
			(
				"Sales@COMPANY.EXAMPLE".into(),
				"Sales@company.example".into(),
				false
			),
			(
				"sales@company.example".into(),
				"sales@company.example".into(),
				false
			),
			(
				"A.B+tag@GoogleMail.com".into(),
				"ab@gmail.com".into(),
				false
			),
			("ab@gmail.com".into(), "ab@gmail.com".into(), true),
		]
	);
}

#[tokio::test]
#[serial]
async fn mailbox_history_does_not_fold_local_case_or_boost_ambiguous_legacy_outcomes() {
	use serde_json::json;
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "identity-history", None, 0).await;
	let (key, _) = test_helpers::insert_api_key(db.pool(), tenant).await;
	let config = test_helpers::build_test_config(
		test_helpers::ConfigProfile::PseudoWorker,
		Some(&test_db_url()),
		None,
	)
	.await;
	let routes = reacher_backend::http::create_routes(config);
	for (email, canonical) in [
		("Sales@COMPANY.EXAMPLE", Some("Sales@company.example")),
		("sales@company.example", Some("sales@company.example")),
		("Sales@company.example", None),
		("A.B+tag@GoogleMail.com", None),
	] {
		let mut result = test_helpers::safe_result();
		result["input"] = json!(email);
		sqlx::query("INSERT INTO v1_task_result (tenant_id,payload,result,canonical_email,task_state,completed_at) VALUES ($1,$2,$3,$4,'completed',NOW() - INTERVAL '1 day')")
			.bind(tenant).bind(json!({"input":{"to_email":email}})).bind(result).bind(canonical)
			.execute(db.pool()).await.unwrap();
	}
	for (email, expected) in [
		("Sales@company.example", 2),
		("sales@company.example", 1),
		("ab@gmail.com", 1),
	] {
		let response = warp::test::request()
			.path(&format!("/v1/emails/{email}/history"))
			.header("authorization", format!("Bearer {key}"))
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 200, "{:?}", response.body());
		let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
		assert_eq!(body["total"], expected, "{body}");
		assert_eq!(body["history"].as_array().unwrap().len(), expected as usize);
	}
	sqlx::query("INSERT INTO verification_outcomes (tenant_id,canonical_email,outcome_type,occurred_at,source) VALUES ($1,'sales@company.example','delivered',NOW() - INTERVAL '1 hour','legacy')")
		.bind(tenant).execute(db.pool()).await.unwrap();
	let legacy = reacher_backend::outcomes::enrich_outcome_context(
		Some(db.pool()),
		Some(tenant),
		"sales@company.example",
		chrono::Utc::now(),
	)
	.await;
	assert_eq!(
		legacy.delivered_count, 0,
		"old case-folded positive evidence cannot raise confidence"
	);
	reacher_backend::outcomes::ingest_outcomes(
		db.pool(),
		tenant,
		&Default::default(),
		&[reacher_backend::outcomes::IngestOutcome {
			email: "Sales@COMPANY.EXAMPLE".into(),
			outcome_type: reacher_backend::outcomes::OutcomeType::Delivered,
			occurred_at: chrono::Utc::now() - chrono::Duration::minutes(1),
			source: Some("current".into()),
			campaign_id: None,
			metadata: None,
		}],
	)
	.await;
	let fresh = reacher_backend::outcomes::enrich_outcome_context(
		Some(db.pool()),
		Some(tenant),
		"Sales@company.example",
		chrono::Utc::now(),
	)
	.await;
	assert_eq!(fresh.delivered_count, 1);
	let lower = reacher_backend::outcomes::enrich_outcome_context(
		Some(db.pool()),
		Some(tenant),
		"sales@company.example",
		chrono::Utc::now(),
	)
	.await;
	assert_eq!(lower.delivered_count, 0);
}

#[tokio::test]
#[serial]
async fn finalization_fails_closed_on_count_errors_and_rolls_back_related_states() {
	use reacher_backend::worker::do_work::sync_task_entities;
	use serde_json::json;
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "finalization-fault", None, 0).await;
	let config = test_helpers::build_test_config(
		test_helpers::ConfigProfile::PseudoWorker,
		Some(&test_db_url()),
		None,
	)
	.await;
	let job = test_helpers::insert_job(db.pool(), Some(tenant), 2, "running").await;
	let task = test_helpers::insert_task(
		db.pool(),
		job,
		"completed",
		Some(tenant),
		Some(json!({})),
		None,
	)
	.await;
	sync_task_entities(&config, Some(task), Some(job))
		.await
		.unwrap();
	reacher_backend::worker::job_lifecycle::finalize_bulk_job_if_done(db.pool(), job)
		.await
		.unwrap();
	let status: String = sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE id = $1")
		.bind(job)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(status, "running", "missing rows cannot make a job complete");
	test_helpers::insert_task(
		db.pool(),
		job,
		"completed",
		Some(tenant),
		Some(json!({})),
		None,
	)
	.await;

	sqlx::raw_sql("CREATE SCHEMA audit_count_fault; CREATE TABLE audit_count_fault._sqlx_migrations AS TABLE public._sqlx_migrations; CREATE FUNCTION audit_count_fault.fail_count() RETURNS boolean LANGUAGE plpgsql VOLATILE AS $$ BEGIN RAISE EXCEPTION 'audit count failure'; END $$; CREATE VIEW audit_count_fault.v1_task_result WITH (security_barrier = true) AS SELECT * FROM public.v1_task_result WHERE audit_count_fault.fail_count();")
		.execute(db.pool()).await.unwrap();
	let mut broken_url = reqwest::Url::parse(&test_db_url()).unwrap();
	broken_url
		.query_pairs_mut()
		.append_pair("options", "-c search_path=audit_count_fault,public");
	let broken_config = test_helpers::build_test_config(
		test_helpers::ConfigProfile::PseudoWorker,
		Some(broken_url.as_str()),
		None,
	)
	.await;
	let failed = sync_task_entities(&broken_config, None, Some(job))
		.await
		.unwrap_err();
	assert!(
		failed.to_string().contains("audit count failure"),
		"{}",
		failed
	);
	let status: String = sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE id = $1")
		.bind(job)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(status, "running");
	broken_config.get_pg_pool().unwrap().close().await;
	sqlx::raw_sql("DROP SCHEMA audit_count_fault CASCADE")
		.execute(db.pool())
		.await
		.unwrap();

	let list = test_helpers::insert_list(
		db.pool(),
		tenant,
		job,
		"finalization",
		"processing",
		2,
		&["email"],
		json!({}),
	)
	.await;
	sqlx::query("UPDATE v1_task_result SET extra = jsonb_build_object('list_id',$2::INTEGER) WHERE job_id = $1")
		.bind(job).bind(list).execute(db.pool()).await.unwrap();
	sqlx::query("UPDATE v1_task_result SET dispatch_id = gen_random_uuid(), completion_done = false WHERE id = $1")
		.bind(task).execute(db.pool()).await.unwrap();
	reacher_backend::worker::job_lifecycle::finalize_bulk_job_if_done(db.pool(), job)
		.await
		.unwrap();
	reacher_backend::worker::job_lifecycle::finalize_list_if_done(db.pool(), list)
		.await
		.unwrap();
	let states: (String, String) = sqlx::query_as(
		"SELECT j.status::TEXT, l.status::TEXT FROM v1_bulk_job j JOIN v1_lists l ON l.job_id = j.id WHERE j.id = $1")
		.bind(job).fetch_one(db.pool()).await.unwrap();
	assert_eq!(
		states,
		("running".into(), "processing".into()),
		"creator-side finalization must wait for durable completion effects"
	);
	sqlx::query("UPDATE v1_task_result SET completion_done = true WHERE id = $1")
		.bind(task)
		.execute(db.pool())
		.await
		.unwrap();
	sqlx::raw_sql("CREATE FUNCTION audit_fail_finalize() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'audit finalization failure'; END $$; CREATE TRIGGER audit_fail_finalize BEFORE UPDATE ON v1_bulk_job FOR EACH ROW EXECUTE FUNCTION audit_fail_finalize();")
		.execute(db.pool()).await.unwrap();
	let failed = sync_task_entities(&config, Some(task), Some(job)).await;
	sqlx::raw_sql(
		"DROP TRIGGER audit_fail_finalize ON v1_bulk_job; DROP FUNCTION audit_fail_finalize()",
	)
	.execute(db.pool())
	.await
	.unwrap();
	assert!(failed.is_err());
	let list_status: String = sqlx::query_scalar("SELECT status::TEXT FROM v1_lists WHERE id = $1")
		.bind(list)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(
		list_status, "processing",
		"a failed job update must roll back list completion"
	);
	sync_task_entities(&config, Some(task), Some(job))
		.await
		.unwrap();
	let status: String = sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE id = $1")
		.bind(job)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(status, "completed");
	sqlx::query("UPDATE v1_bulk_job SET status = 'cancelled' WHERE id = $1")
		.bind(job)
		.execute(db.pool())
		.await
		.unwrap();
	sync_task_entities(&config, Some(task), Some(job))
		.await
		.unwrap();
	let status: String = sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE id = $1")
		.bind(job)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(
		status, "cancelled",
		"reconciliation cannot replace a terminal state"
	);
}

#[tokio::test]
#[serial]
async fn bulk_publication_does_not_overwrite_a_completion_that_wins_the_race() {
	let db = TestDb::start().await;
	let broker = test_helpers::TestRabbitMq::start().await;
	let tenant = insert_tenant(db.pool(), "fast-bulk-completion", Some(100), 0).await;
	let (key, _) = test_helpers::insert_api_key(db.pool(), tenant).await;
	let config = test_helpers::build_test_config(
		test_helpers::ConfigProfile::WorkerRabbit,
		Some(&test_db_url()),
		Some(&broker.amqp_url),
	)
	.await;
	let routes = reacher_backend::http::create_routes(config);
	// A committed completion can precede the handler's final publication update.
	sqlx::raw_sql("CREATE FUNCTION audit_fast_completion() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN UPDATE v1_task_result SET task_state = 'completed', result = '{}'::jsonb WHERE id = NEW.id; UPDATE v1_bulk_job SET status = 'completed', completed_at = NOW() WHERE id = NEW.job_id; RETURN NEW; END $$; CREATE TRIGGER audit_fast_completion AFTER INSERT ON v1_task_result FOR EACH ROW EXECUTE FUNCTION audit_fast_completion();")
		.execute(db.pool()).await.unwrap();
	let response = warp::test::request()
		.method("POST")
		.path("/v1/bulk")
		.header("authorization", format!("Bearer {key}"))
		.json(&serde_json::json!({"input":["invalid"]}))
		.reply(&routes)
		.await;
	sqlx::raw_sql("DROP TRIGGER audit_fast_completion ON v1_task_result; DROP FUNCTION audit_fast_completion()").execute(db.pool()).await.unwrap();
	assert_eq!(response.status(), 200, "{:?}", response.body());
	let status: String =
		sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE tenant_id = $1")
			.bind(tenant)
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_eq!(status, "completed");
}

#[tokio::test]
#[serial]
async fn duplicate_completion_cannot_copy_across_case_distinct_mailboxes() {
	use reacher_backend::worker::do_work::sync_task_entities;
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "duplicate-identity", None, 0).await;
	let config = test_helpers::build_test_config(
		test_helpers::ConfigProfile::PseudoWorker,
		Some(&test_db_url()),
		None,
	)
	.await;
	let job = test_helpers::insert_job(db.pool(), Some(tenant), 3, "running").await;
	let primary = test_helpers::insert_task(
		db.pool(),
		job,
		"completed",
		Some(tenant),
		Some(serde_json::json!({"input":"Sales@company.example"})),
		None,
	)
	.await;
	sqlx::query(
		"UPDATE v1_task_result SET canonical_email = 'Sales@company.example' WHERE id = $1",
	)
	.bind(primary)
	.execute(db.pool())
	.await
	.unwrap();
	for email in ["Sales@company.example", "sales@company.example"] {
		sqlx::query("INSERT INTO v1_task_result (job_id,tenant_id,payload,canonical_email,is_duplicate,canonical_task_id,task_state) VALUES ($1,$2,'{}',$3,true,$4,'queued')")
			.bind(job).bind(tenant).bind(email).bind(primary).execute(db.pool()).await.unwrap();
	}
	sync_task_entities(&config, Some(primary), Some(job))
		.await
		.unwrap();
	let rows: Vec<(String,String,bool)> = sqlx::query_as("SELECT canonical_email,task_state::TEXT,result IS NOT NULL FROM v1_task_result WHERE canonical_task_id = $1 ORDER BY canonical_email COLLATE \"C\"")
		.bind(primary).fetch_all(db.pool()).await.unwrap();
	assert_eq!(
		rows,
		vec![
			("Sales@company.example".into(), "completed".into(), true),
			("sales@company.example".into(), "queued".into(), false)
		]
	);
	let status: String = sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE id = $1")
		.bind(job)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(status, "running");
}

#[tokio::test]
#[serial]
async fn quota_reservation_and_work_commit_together_across_concurrent_pools() {
	use reacher_backend::tenant::quota::reserve_quota_in_tx;
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "transactional-quota", Some(1), 0).await;
	let context = resolve_tenant_context_by_id(db.pool(), tenant)
		.await
		.unwrap();
	let mut tx = db.pool().begin().await.unwrap();
	assert_eq!(
		reserve_quota_in_tx(&mut tx, &context, 1).await.unwrap(),
		QuotaCheckResult::Allowed
	);
	assert!(
		sqlx::query("INSERT INTO v1_bulk_job (tenant_id,total_records) VALUES ($1,NULL)")
			.bind(tenant)
			.execute(&mut *tx)
			.await
			.is_err()
	);
	tx.rollback().await.unwrap();
	let used: i32 = sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id = $1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(used, 0);

	let other_pool = sqlx::PgPool::connect(&test_db_url()).await.unwrap();
	async fn create_one(
		pool: &sqlx::PgPool,
		context: &reacher_backend::tenant::context::TenantContext,
	) -> QuotaCheckResult {
		let mut tx = pool.begin().await.unwrap();
		let result = reserve_quota_in_tx(&mut tx, context, 1).await.unwrap();
		if result == QuotaCheckResult::Allowed {
			sqlx::query("INSERT INTO v1_bulk_job (tenant_id,total_records) VALUES ($1,1)")
				.bind(context.tenant_id)
				.execute(&mut *tx)
				.await
				.unwrap();
			tx.commit().await.unwrap();
		}
		result
	}
	let (first, second) = tokio::join!(
		create_one(db.pool(), &context),
		create_one(&other_pool, &context)
	);
	assert_eq!(
		usize::from(first == QuotaCheckResult::Allowed)
			+ usize::from(second == QuotaCheckResult::Allowed),
		1
	);
	assert!(
		matches!(first, QuotaCheckResult::ExceededMonthlyLimit { .. })
			|| matches!(second, QuotaCheckResult::ExceededMonthlyLimit { .. })
	);
	let (used,jobs): (i32,i64) = sqlx::query_as("SELECT used_this_period,(SELECT COUNT(*) FROM v1_bulk_job WHERE tenant_id = $1) FROM tenants WHERE id = $1").bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!((used, jobs), (1, 1));
	other_pool.close().await;
}

#[tokio::test]
#[serial]
async fn tenant_rate_limits_are_atomic_across_independent_pools() {
	let db = TestDb::start().await;
	let pool_a = sqlx::PgPool::connect(&test_db_url()).await.unwrap();
	let pool_b = sqlx::PgPool::connect(&test_db_url()).await.unwrap();
	let tenant = insert_tenant(db.pool(), "shared-rate", Some(100), 0).await;
	let limits = ThrottleConfig {
		max_requests_per_hour: Some(10),
		max_requests_per_day: Some(1),
		..ThrottleConfig::new_without_throttle()
	};
	let (a, b) = tokio::join!(
		reserve_tenant_rate(&pool_a, tenant, &limits),
		reserve_tenant_rate(&pool_b, tenant, &limits),
	);
	assert_eq!(
		usize::from(a.unwrap().is_none()) + usize::from(b.unwrap().is_none()),
		1
	);
	let counts: Vec<i64> = sqlx::query_scalar(
		"SELECT used FROM tenant_rate_windows WHERE tenant_id = $1 ORDER BY window_seconds",
	)
	.bind(tenant)
	.fetch_all(db.pool())
	.await
	.unwrap();
	assert_eq!(
		counts,
		vec![1, 1],
		"a denied request must not reserve another window"
	);

	let other = insert_tenant(db.pool(), "other-rate", Some(100), 0).await;
	assert!(reserve_tenant_rate(&pool_b, other, &limits)
		.await
		.unwrap()
		.is_none());
	sqlx::query("UPDATE tenant_rate_windows SET resets_at = NOW() - INTERVAL '1 second' WHERE tenant_id = $1")
		.bind(tenant).execute(db.pool()).await.unwrap();
	assert!(reserve_tenant_rate(&pool_b, tenant, &limits)
		.await
		.unwrap()
		.is_none());
	assert!(
		reserve_tenant_rate(&pool_a, tenant, &ThrottleConfig::new_without_throttle())
			.await
			.unwrap()
			.is_none()
	);
	pool_a.close().await;
	assert!(reserve_tenant_rate(&pool_a, tenant, &limits).await.is_err());
	pool_b.close().await;
}

#[tokio::test]
#[serial]
async fn quota_does_not_admit_on_missing_tenant_or_failed_database() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "exhausted-audit", Some(10), 10).await;
	let mut context = resolve_tenant_context_by_id(db.pool(), tenant)
		.await
		.unwrap();
	let broken = sqlx::PgPool::connect(&test_db_url()).await.unwrap();
	broken.close().await;
	assert_eq!(
		check_and_increment_quota(Some(&broken), &context).await,
		QuotaCheckResult::Unavailable
	);
	context.tenant_id = Some(uuid::Uuid::new_v4());
	assert_eq!(
		check_and_increment_quota(Some(db.pool()), &context).await,
		QuotaCheckResult::Unavailable
	);
}

#[tokio::test]
#[serial]
async fn expired_suppression_is_reactivated_with_a_new_audit_event() {
	use reacher_backend::suppression::{has_active_suppression, insert_active_suppression};
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "expired-suppression", None, 0).await;
	let email = "target@audit.example";
	assert!(insert_active_suppression(
		db.pool(),
		tenant,
		email,
		"bounce",
		"test",
		"outcome",
		"test"
	)
	.await
	.unwrap());
	sqlx::query("UPDATE v1_suppression_entries SET expires_at = NOW() - INTERVAL '1 day' WHERE tenant_id = $1")
		.bind(tenant).execute(db.pool()).await.unwrap();
	assert!(!has_active_suppression(db.pool(), tenant, email)
		.await
		.unwrap());
	assert!(insert_active_suppression(
		db.pool(),
		tenant,
		email,
		"complaint",
		"test",
		"outcome",
		"test"
	)
	.await
	.unwrap());
	assert!(has_active_suppression(db.pool(), tenant, email)
		.await
		.unwrap());
	let events: Vec<String> = sqlx::query_scalar(
		"SELECT event_type FROM v1_suppression_events WHERE tenant_id = $1 ORDER BY id",
	)
	.bind(tenant)
	.fetch_all(db.pool())
	.await
	.unwrap();
	assert_eq!(events, vec!["created", "reactivated"]);
}

#[tokio::test]
#[serial]
async fn failed_outcome_suppression_rolls_back_and_retry_repairs_it() {
	use reacher_backend::outcomes::{
		default_outcome_policy_rules, ingest_outcomes, IngestOutcome, OutcomeType, RuleAction,
	};
	use reacher_backend::suppression::has_active_suppression;
	let db = TestDb::start().await;
	let pool = db.pool();
	let tenant = insert_tenant(pool, "outcome-atomic", None, 0).await;
	let event = IngestOutcome {
		email: "complaint@audit.example".into(),
		outcome_type: OutcomeType::Complaint,
		occurred_at: chrono::Utc::now(),
		source: Some("audit".into()),
		campaign_id: None,
		metadata: None,
	};
	let mut policy = default_outcome_policy_rules();
	sqlx::raw_sql("CREATE FUNCTION audit_fail_suppression() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'audit injected failure'; END $$; CREATE TRIGGER audit_fail_suppression BEFORE INSERT ON v1_suppression_entries FOR EACH ROW EXECUTE FUNCTION audit_fail_suppression();")
		.execute(pool).await.unwrap();
	let first = ingest_outcomes(pool, tenant, &policy, &[event.clone()]).await;
	sqlx::raw_sql("DROP TRIGGER audit_fail_suppression ON v1_suppression_entries; DROP FUNCTION audit_fail_suppression();")
		.execute(pool).await.unwrap();
	assert_eq!((first.accepted, first.rejected), (0, 1));
	let count: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM verification_outcomes WHERE tenant_id = $1")
			.bind(tenant)
			.fetch_one(pool)
			.await
			.unwrap();
	assert_eq!(
		count, 0,
		"failed action must not leave a deduplicated outcome behind"
	);
	let retry = ingest_outcomes(pool, tenant, &policy, &[event.clone()]).await;
	assert_eq!((retry.accepted, retry.suppressed), (1, 1));
	assert!(has_active_suppression(pool, tenant, &event.email)
		.await
		.unwrap());
	let duplicate = ingest_outcomes(pool, tenant, &policy, &[event]).await;
	assert_eq!((duplicate.accepted, duplicate.suppressed), (1, 0));

	policy.soft_bounce = RuleAction::SuppressAfter {
		threshold_count: 2,
		threshold_window_days: 30,
	};
	let first = [IngestOutcome {
		email: "soft@audit.example".into(),
		outcome_type: OutcomeType::SoftBounce,
		occurred_at: chrono::Utc::now(),
		source: Some("first".into()),
		campaign_id: None,
		metadata: None,
	}];
	let second = [IngestOutcome {
		source: Some("second".into()),
		..first[0].clone()
	}];
	let (a, b) = tokio::join!(
		ingest_outcomes(pool, tenant, &policy, &first),
		ingest_outcomes(pool, tenant, &policy, &second),
	);
	assert_eq!(a.accepted + b.accepted, 2);
	assert!(has_active_suppression(pool, tenant, "soft@audit.example")
		.await
		.unwrap());
}

#[tokio::test]
#[serial]
async fn legacy_pruning_rolls_back_child_deletion_when_parent_delete_fails() {
	let db = TestDb::start().await;
	let pool = db.pool();
	let job: i32 =
		sqlx::query_scalar("INSERT INTO bulk_jobs (total_records) VALUES (1) RETURNING id")
			.fetch_one(pool)
			.await
			.unwrap();
	sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, '{}')")
		.bind(job)
		.execute(pool)
		.await
		.unwrap();
	sqlx::raw_sql("CREATE FUNCTION audit_fail_prune() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'audit parent delete failure'; END $$; CREATE TRIGGER audit_fail_prune BEFORE DELETE ON bulk_jobs FOR EACH ROW EXECUTE FUNCTION audit_fail_prune();")
		.execute(pool).await.unwrap();
	let failed = legacy_prune::prune_jobs(pool, &[job]).await;
	sqlx::raw_sql("DROP TRIGGER audit_fail_prune ON bulk_jobs; DROP FUNCTION audit_fail_prune();")
		.execute(pool)
		.await
		.unwrap();
	assert!(failed.is_err());
	let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM email_results WHERE job_id = $1")
		.bind(job)
		.fetch_one(pool)
		.await
		.unwrap();
	assert_eq!(count, 1);
	legacy_prune::prune_jobs(pool, &[job]).await.unwrap();
	let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM bulk_jobs WHERE id = $1")
		.bind(job)
		.fetch_one(pool)
		.await
		.unwrap();
	assert_eq!(count, 0);
}

#[tokio::test]
#[serial]
async fn failed_idempotency_reclaim_has_one_owner_and_fences_late_completion() {
	use reacher_backend::http::idempotency::{
		check_idempotency_key, complete_idempotency_key, fail_idempotency_key, hash_request_body,
		IdempotencyCheck,
	};
	let db = TestDb::start().await;
	let pool = db.pool();
	let hash = hash_request_body(b"request");
	let first = check_idempotency_key(pool, "audit", "retry", "/v1/check_email", &hash, "first")
		.await
		.unwrap();
	let IdempotencyCheck::New { record_id } = first else {
		panic!("expected new operation");
	};
	fail_idempotency_key(pool, record_id).await.unwrap();
	let (a, b) = tokio::join!(
		check_idempotency_key(pool, "audit", "retry", "/v1/check_email", &hash, "a"),
		check_idempotency_key(pool, "audit", "retry", "/v1/check_email", &hash, "b"),
	);
	let replies = [a.unwrap(), b.unwrap()];
	assert_eq!(
		replies
			.iter()
			.filter(|r| matches!(r, IdempotencyCheck::New { .. }))
			.count(),
		1
	);
	assert_eq!(
		replies
			.iter()
			.filter(|r| matches!(r, IdempotencyCheck::InProgress))
			.count(),
		1
	);
	complete_idempotency_key(pool, record_id, 200, b"late old response")
		.await
		.unwrap();
	assert!(matches!(
		check_idempotency_key(pool, "audit", "retry", "/v1/check_email", &hash, "reader")
			.await
			.unwrap(),
		IdempotencyCheck::InProgress
	));
}

#[tokio::test]
#[serial]
async fn list_deletion_is_atomic_and_rechecks_lifecycle_after_waiting_for_retry() {
	use reacher_backend::http::v1::lists::delete::delete_list_records;
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "atomic-delete", None, 0).await;
	let job: i32 = sqlx::query_scalar("INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES (1, $1, 'completed') RETURNING id")
		.bind(tenant).fetch_one(db.pool()).await.unwrap();
	let list: i32 = sqlx::query_scalar("INSERT INTO v1_lists (tenant_id, job_id, name, original_filename, file_size_bytes, email_column, status) VALUES ($1, $2, 'audit', 'audit.csv', 1, 'email', 'uploading') RETURNING id")
		.bind(tenant).bind(job).fetch_one(db.pool()).await.unwrap();
	let task: i32 = sqlx::query_scalar("INSERT INTO v1_task_result (job_id, tenant_id, payload, extra, task_state) VALUES ($1, $2, '{}', $3, 'completed') RETURNING id")
		.bind(job).bind(tenant).bind(serde_json::json!({"list_id": list})).fetch_one(db.pool()).await.unwrap();
	assert_eq!(
		delete_list_records(db.pool(), tenant, list)
			.await
			.unwrap_err()
			.code,
		warp::http::StatusCode::CONFLICT
	);
	sqlx::query("UPDATE v1_lists SET status = 'completed' WHERE id = $1")
		.bind(list)
		.execute(db.pool())
		.await
		.unwrap();
	sqlx::raw_sql("CREATE OR REPLACE FUNCTION audit_fail_list_delete() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'injected list delete failure'; END $$; CREATE TRIGGER audit_fail_list_delete BEFORE DELETE ON v1_lists FOR EACH ROW EXECUTE FUNCTION audit_fail_list_delete();")
		.execute(db.pool()).await.unwrap();
	assert!(delete_list_records(db.pool(), tenant, list).await.is_err());
	let remains: bool =
		sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM v1_task_result WHERE id = $1)")
			.bind(task)
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert!(
		remains,
		"failed parent deletion must roll back child deletion"
	);
	sqlx::raw_sql(
		"DROP TRIGGER audit_fail_list_delete ON v1_lists; DROP FUNCTION audit_fail_list_delete()",
	)
	.execute(db.pool())
	.await
	.unwrap();
	let mut retry = db.pool().begin().await.unwrap();
	sqlx::query("SELECT id FROM v1_bulk_job WHERE id = $1 FOR UPDATE")
		.bind(job)
		.fetch_one(&mut *retry)
		.await
		.unwrap();
	let pool = db.pool().clone();
	let mut deletion = tokio::spawn(async move { delete_list_records(&pool, tenant, list).await });
	assert!(
		tokio::time::timeout(std::time::Duration::from_millis(30), &mut deletion)
			.await
			.is_err()
	);
	sqlx::query("UPDATE v1_bulk_job SET status = 'running' WHERE id = $1")
		.bind(job)
		.execute(&mut *retry)
		.await
		.unwrap();
	retry.commit().await.unwrap();
	assert_eq!(
		deletion.await.unwrap().unwrap_err().code,
		warp::http::StatusCode::CONFLICT
	);
	sqlx::query("UPDATE v1_bulk_job SET status = 'completed' WHERE id = $1")
		.bind(job)
		.execute(db.pool())
		.await
		.unwrap();
	delete_list_records(db.pool(), tenant, list).await.unwrap();
	let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result WHERE id = $1")
		.bind(task)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(remaining, 0);
}

#[tokio::test]
#[serial]
async fn newly_added_suppression_is_read_from_primary_despite_replica_lag() {
	use reacher_backend::config::BackendConfig;
	use reacher_backend::config::{PostgresConfig, StorageConfig};
	use reacher_backend::decision::types::RecommendedAction;
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "primary-safety-read", None, 0).await;
	sqlx::raw_sql("CREATE SCHEMA IF NOT EXISTS audit_lagging_replica; CREATE OR REPLACE VIEW audit_lagging_replica.v1_suppression_entries AS SELECT * FROM public.v1_suppression_entries WHERE false;")
		.execute(db.pool()).await.unwrap();
	let mut replica = reqwest::Url::parse(&test_db_url()).unwrap();
	replica
		.query_pairs_mut()
		.append_pair("options", "-c search_path=audit_lagging_replica,public");
	let mut config = BackendConfig::empty();
	config.storage = Some(StorageConfig::Postgres(PostgresConfig {
		db_url: test_db_url(),
		read_replica_url: Some(replica.to_string()),
		extra: None,
	}));
	config.connect().await.unwrap();
	let email = "target@audit.example";
	reacher_backend::suppression::insert_active_suppression(
		db.pool(),
		tenant,
		email,
		"manual",
		"test",
		"test",
		"test",
	)
	.await
	.unwrap();
	assert!(!reacher_backend::suppression::has_active_suppression(
		&config.get_read_pg_pool().unwrap(),
		tenant,
		email
	)
	.await
	.unwrap());
	let output = reacher_backend::sandbox::sandbox_check(email);
	let prepared = reacher_backend::scoring::response::prepare_verification_response(
		&config,
		&output,
		Some(tenant),
		chrono::Utc::now(),
		false,
	)
	.await
	.unwrap();
	assert_eq!(
		prepared.recommendation.unwrap().action,
		RecommendedAction::Drop
	);
}

#[tokio::test]
#[serial]
async fn requested_policy_survives_queue_serialization_and_is_stored_and_replayed() {
	use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
	use reacher_backend::decision::custom_policy::resolve_requested_policy;
	use reacher_backend::decision::types::{PolicyDecision, PolicyMode};
	use reacher_backend::http::shared::check_email::handle_check_email;
	use reacher_backend::http::CheckEmailRequest;
	use reacher_backend::scoring::response::{
		prepare_verification_response, prepare_verification_response_with_policy,
	};
	use reacher_backend::worker::do_work::{
		check_email_and_prepare_result_with_config, CheckEmailJobId, CheckEmailTask, TaskMetadata,
	};
	use std::sync::Arc;
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "policy-request", Some(100), 0).await;
	sqlx::query("UPDATE tenants SET default_policy_mode = 'growth' WHERE id = $1")
		.bind(tenant)
		.execute(db.pool())
		.await
		.unwrap();
	let mut config = BackendConfig::empty();
	config.storage = Some(StorageConfig::Postgres(PostgresConfig {
		db_url: test_db_url(),
		read_replica_url: None,
		extra: None,
	}));
	config.connect().await.unwrap();
	let config = Arc::new(config);
	let mut output = reacher_backend::sandbox::sandbox_check("sales@audit.example");
	output.misc.as_mut().unwrap().is_role_account = true;
	let baseline =
		prepare_verification_response(&config, &output, Some(tenant), chrono::Utc::now(), false)
			.await
			.unwrap();
	assert_eq!(
		baseline.policy_evaluation.unwrap().decision,
		PolicyDecision::Send
	);
	let strict = resolve_requested_policy(
		Some(db.pool()),
		Some(tenant),
		Some(PolicyMode::EnterpriseStrict),
		None,
	)
	.await
	.unwrap();
	let explicit = prepare_verification_response_with_policy(
		&config,
		&output,
		Some(tenant),
		chrono::Utc::now(),
		false,
		strict.as_ref(),
	)
	.await
	.unwrap();
	assert_eq!(
		explicit.policy_evaluation.unwrap().decision,
		PolicyDecision::Review
	);
	for (key, status, decision) in [
		("active-one", "active", "send"),
		("named", "inactive", "drop"),
	] {
		sqlx::query("INSERT INTO tenant_policy_profiles (tenant_id, profile_key, display_name, status, rules) VALUES ($1,$2,$2,$3,$4)")
			.bind(tenant).bind(key).bind(status).bind(serde_json::json!({"rules":[],"default_decision":decision})).execute(db.pool()).await.unwrap();
	}
	let named = resolve_requested_policy(
		Some(db.pool()),
		Some(tenant),
		Some(PolicyMode::Custom),
		Some("named"),
	)
	.await
	.unwrap();
	assert!(resolve_requested_policy(
		Some(db.pool()),
		Some(uuid::Uuid::new_v4()),
		Some(PolicyMode::Custom),
		Some("named")
	)
	.await
	.is_err());
	let task = CheckEmailTask {
		verification_backend: Default::default(),
		input: CheckEmailRequest {
			to_email: "invalid".into(),
			..Default::default()
		}
		.to_check_email_input(Arc::clone(&config)),
		job_id: CheckEmailJobId::SingleShot,
		webhook: None,
		metadata: Some(TaskMetadata {
			dispatch_id: None,
			lease_id: None,
			decision_policy: named.clone(),
			tenant_id: Some(tenant.to_string()),
			request_id: None,
			correlation_id: None,
			created_by: None,
			retry_policy: None,
			dedupe_key: None,
			task_db_id: None,
		}),
	};
	let serialized = serde_json::to_vec(&task).unwrap();
	sqlx::query("UPDATE tenant_policy_profiles SET rules = $2 WHERE tenant_id = $1 AND profile_key = 'named'")
		.bind(tenant).bind(serde_json::json!({"rules":[],"default_decision":"send"})).execute(db.pool()).await.unwrap();
	let restored: CheckEmailTask = serde_json::from_slice(&serialized).unwrap();
	assert_eq!(restored.metadata.as_ref().unwrap().decision_policy, named);
	let queued = check_email_and_prepare_result_with_config(&restored, Some(Arc::clone(&config)))
		.await
		.unwrap();
	assert_eq!(
		queued
			.response
			.policy_evaluation
			.as_ref()
			.unwrap()
			.policy_profile_key
			.as_deref(),
		Some("named")
	);
	let snapshot = prepare_verification_response_with_policy(
		&config,
		&output,
		Some(tenant),
		chrono::Utc::now(),
		false,
		restored.metadata.as_ref().unwrap().decision_policy.as_ref(),
	)
	.await
	.unwrap();
	assert_eq!(
		snapshot.policy_evaluation.unwrap().decision,
		PolicyDecision::Drop
	);
	let context = resolve_tenant_context_by_id(db.pool(), tenant)
		.await
		.unwrap();
	let request = CheckEmailRequest {
		to_email: "invalid".into(),
		policy_mode: Some(PolicyMode::EnterpriseStrict),
		..Default::default()
	};
	let bytes = serde_json::to_vec(&request).unwrap();
	let first = handle_check_email(
		Arc::clone(&config),
		&bytes,
		&request,
		&context,
		"/v1/check_email",
		Some("policy-replay".into()),
	)
	.await
	.unwrap();
	let replay = handle_check_email(
		Arc::clone(&config),
		&bytes,
		&request,
		&context,
		"/v1/check_email",
		Some("policy-replay".into()),
	)
	.await
	.unwrap();
	assert_eq!(first.body, replay.body);
	let value: serde_json::Value = serde_json::from_slice(&first.body).unwrap();
	assert_eq!(value["policy_evaluation"]["mode"], "enterprise_strict");
	let stored: Vec<String> =
		sqlx::query_scalar("SELECT policy_mode FROM v1_task_result WHERE tenant_id = $1")
			.bind(tenant)
			.fetch_all(db.pool())
			.await
			.unwrap();
	assert_eq!(stored, vec!["enterprise_strict"]);
}
