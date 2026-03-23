mod test_helpers;

#[cfg(test)]
mod admin_jobs_tests {
	use crate::test_helpers::TestDb;
	use reacher_backend::config::{
		BackendConfig, PostgresConfig, RabbitMQConfig, StorageConfig, WorkerConfig,
	};
	use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
	use serial_test::serial;
	use sqlx::Row;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	async fn worker_config() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("s".into());
		let db = crate::test_helpers::test_db_url();
		let rmq = crate::test_helpers::test_amqp_url();
		c.storage = Some(StorageConfig::Postgres(PostgresConfig { read_replica_url: None,
			db_url: db,
			extra: None,
		}));
		c.worker = WorkerConfig {
			enable: true,
			rabbitmq: Some(RabbitMQConfig {
				url: rmq,
				concurrency: 4,
			}),
			webhook: None,
		};
		c.connect().await.unwrap();
		Arc::new(c)
	}

	async fn setup_job(pool: &sqlx::PgPool) -> (uuid::Uuid, i32) {
		let tid: uuid::Uuid = sqlx::query("INSERT INTO tenants (name,slug,contact_email) VALUES ('AJ','admin-jobs-t','a@j.com') RETURNING id")
			.fetch_one(pool).await.unwrap().get("id");
		let jid: i32 = sqlx::query("INSERT INTO v1_bulk_job (total_records,tenant_id,status) VALUES (2,$1,'running'::job_state) RETURNING id")
			.bind(tid).fetch_one(pool).await.unwrap().get("id");
		sqlx::query("INSERT INTO v1_task_result (job_id,payload,task_state,tenant_id,result) VALUES ($1,$2,'completed'::task_state,$3,$4)")
			.bind(jid).bind(serde_json::json!({})).bind(tid)
			.bind(serde_json::json!({"is_reachable":"safe","input":"t@e.com"}))
			.execute(pool).await.unwrap();
		sqlx::query(
			"INSERT INTO job_events (job_id,event_type,actor) VALUES ($1,'job.created','test')",
		)
		.bind(jid)
		.execute(pool)
		.await
		.unwrap();
		(tid, jid)
	}

	#[tokio::test]
	#[serial]
	async fn test_list_jobs() {
		let db = TestDb::start().await;
		setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/admin/jobs")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["total"].as_i64().unwrap() >= 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_list_jobs_status_filter() {
		let db = TestDb::start().await;
		setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/admin/jobs?status=running")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_list_jobs_tenant_filter() {
		let db = TestDb::start().await;
		let (tid, _) = setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/admin/jobs?tenant_id={}", tid))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_list_jobs_pagination() {
		let db = TestDb::start().await;
		setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/admin/jobs?limit=1&offset=0")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["jobs"].as_array().unwrap().len() <= 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_get_job() {
		let db = TestDb::start().await;
		let (_, jid) = setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/admin/jobs/{}", jid))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["job_id"], jid);
	}

	#[tokio::test]
	#[serial]
	async fn test_get_job_not_found() {
		let _db = TestDb::start().await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/admin/jobs/999999")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	#[serial]
	async fn test_get_job_events() {
		let db = TestDb::start().await;
		let (_, jid) = setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/admin/jobs/{}/events", jid))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["total"].as_i64().unwrap() >= 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_get_job_events_pagination() {
		let db = TestDb::start().await;
		let (_, jid) = setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/admin/jobs/{}/events?limit=1&offset=0", jid))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_get_job_results() {
		let db = TestDb::start().await;
		let (_, jid) = setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/admin/jobs/{}/results", jid))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["total"].as_i64().unwrap() >= 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_get_job_results_state_filter() {
		let db = TestDb::start().await;
		let (_, jid) = setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/admin/jobs/{}/results?state=completed", jid))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_list_tenant_jobs() {
		let db = TestDb::start().await;
		let (tid, _) = setup_job(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/admin/tenants/{}/jobs", tid))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["total"].as_i64().unwrap() >= 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_no_auth() {
		let c = worker_config().await;
		let r = request()
			.path("/v1/admin/jobs")
			.method("GET")
			.reply(&create_routes(c))
			.await;
		assert_ne!(r.status(), StatusCode::OK);
	}
}

#[cfg(test)]
mod admin_quota_tests {
	use crate::test_helpers::TestDb;
	use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
	use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
	use serial_test::serial;
	use sqlx::Row;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	async fn cfg() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("s".into());
		let db = crate::test_helpers::test_db_url();
		c.storage = Some(StorageConfig::Postgres(PostgresConfig { read_replica_url: None,
			db_url: db,
			extra: None,
		}));
		c.connect().await.unwrap();
		Arc::new(c)
	}

	#[tokio::test]
	#[serial]
	async fn test_get_quota() {
		let db = TestDb::start().await;
		let tid: uuid::Uuid = sqlx::query("INSERT INTO tenants (name,slug,contact_email,monthly_email_limit,used_this_period) VALUES ('Q','quota-get-nm','q@q.com',1000,50) RETURNING id")
			.fetch_one(db.pool()).await.unwrap().get("id");
		let c = cfg().await;
		let r = request()
			.path(&format!("/v1/admin/tenants/{}/quota", tid))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["monthly_email_limit"], 1000);
		assert_eq!(b["used_this_period"], 50);
	}

	#[tokio::test]
	#[serial]
	async fn test_update_quota() {
		let db = TestDb::start().await;
		let tid: uuid::Uuid = sqlx::query("INSERT INTO tenants (name,slug,contact_email) VALUES ('Q2','quota-upd-nm','q2@q.com') RETURNING id")
			.fetch_one(db.pool()).await.unwrap().get("id");
		let c = cfg().await;
		let r = request()
			.path(&format!("/v1/admin/tenants/{}/quota", tid))
			.method("PATCH")
			.header(REACHER_SECRET_HEADER, "s")
			.json(&serde_json::json!({"monthly_email_limit":5000}))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["monthly_email_limit"], 5000);
	}

	#[tokio::test]
	#[serial]
	async fn test_reset_quota() {
		let db = TestDb::start().await;
		let tid: uuid::Uuid = sqlx::query("INSERT INTO tenants (name,slug,contact_email,used_this_period) VALUES ('Q3','quota-rst-nm','q3@q.com',100) RETURNING id")
			.fetch_one(db.pool()).await.unwrap().get("id");
		let c = cfg().await;
		let r = request()
			.path(&format!("/v1/admin/tenants/{}/quota/reset", tid))
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["used_this_period"], 0);
	}

	#[tokio::test]
	#[serial]
	async fn test_quota_not_found() {
		let _db = TestDb::start().await;
		let c = cfg().await;
		let r = request()
			.path("/v1/admin/tenants/00000000-0000-0000-0000-000000000000/quota")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "s")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::NOT_FOUND);
	}
}

#[cfg(test)]
mod tenant_self_tests {
	use crate::test_helpers::TestDb;
	use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
	use reacher_backend::http::create_routes;
	use reacher_backend::tenant::auth::generate_api_key;
	use serial_test::serial;
	use sqlx::Row;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	async fn cfg() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("s".into());
		let db = crate::test_helpers::test_db_url();
		c.storage = Some(StorageConfig::Postgres(PostgresConfig { read_replica_url: None,
			db_url: db,
			extra: None,
		}));
		c.connect().await.unwrap();
		Arc::new(c)
	}

	async fn create_tenant_key(pool: &sqlx::PgPool) -> (uuid::Uuid, String) {
		let tid: uuid::Uuid = sqlx::query("INSERT INTO tenants (name,slug,contact_email,monthly_email_limit,used_this_period) VALUES ('Self','self-test-nm','s@t.com',1000,10) RETURNING id")
			.fetch_one(pool).await.unwrap().get("id");
		let (key, prefix, hash) = generate_api_key();
		sqlx::query("INSERT INTO api_keys (tenant_id,key_prefix,key_hash,name,status) VALUES ($1,$2,$3,'test','active')")
			.bind(tid).bind(&prefix).bind(&hash).execute(pool).await.unwrap();
		(tid, key)
	}

	#[tokio::test]
	#[serial]
	async fn test_get_me() {
		let db = TestDb::start().await;
		let (_, key) = create_tenant_key(db.pool()).await;
		let c = cfg().await;
		let r = request()
			.path("/v1/me")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["tenant_name"], "Self");
	}

	#[tokio::test]
	#[serial]
	async fn test_get_settings() {
		let db = TestDb::start().await;
		let (_, key) = create_tenant_key(db.pool()).await;
		let c = cfg().await;
		let r = request()
			.path("/v1/me/settings")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_update_settings() {
		let db = TestDb::start().await;
		let (_, key) = create_tenant_key(db.pool()).await;
		let c = cfg().await;
		let r = request()
			.path("/v1/me/settings")
			.method("PATCH")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"default_webhook_url":"https://hook.test"}))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_get_usage() {
		let db = TestDb::start().await;
		let (_, key) = create_tenant_key(db.pool()).await;
		let c = cfg().await;
		let r = request()
			.path("/v1/me/usage")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["used_this_period"], 10);
	}

	#[tokio::test]
	#[serial]
	async fn test_webhook_get_put_delete() {
		let db = TestDb::start().await;
		let (_, key) = create_tenant_key(db.pool()).await;
		let c = cfg().await;
		let routes = create_routes(Arc::clone(&c));

		let r = request()
			.path("/v1/me/webhook")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);

		let r = request()
			.path("/v1/me/webhook")
			.method("PATCH")
			.header("Authorization", format!("Bearer {}", key))
			.json(
				&serde_json::json!({"default_webhook_url":"https://w.t","webhook_signing_secret":"s"}),
			)
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);

		let r = request()
			.path("/v1/me/webhook")
			.method("DELETE")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_account_api_keys_crud() {
		let db = TestDb::start().await;
		let (_, key) = create_tenant_key(db.pool()).await;
		let c = cfg().await;
		let routes = create_routes(Arc::clone(&c));

		let r = request()
			.path("/v1/me/api-keys")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);

		let r = request()
			.path("/v1/me/api-keys")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"name":"New"}))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::CREATED);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		let kid = b["id"].as_str().unwrap().to_string();

		let r = request()
			.path(&format!("/v1/me/api-keys/{}", kid))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);

		let r = request()
			.path(&format!("/v1/me/api-keys/{}", kid))
			.method("PATCH")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"name":"Renamed"}))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);

		let r = request()
			.path(&format!("/v1/me/api-keys/{}", kid))
			.method("DELETE")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_me_no_auth() {
		let c = cfg().await;
		let r = request()
			.path("/v1/me")
			.method("GET")
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::UNAUTHORIZED);
	}

	#[tokio::test]
	#[serial]
	async fn test_domains_crud() {
		let db = TestDb::start().await;
		let (_, key) = create_tenant_key(db.pool()).await;
		let c = cfg().await;
		let routes = create_routes(Arc::clone(&c));

		let r = request()
			.path("/v1/me/domains")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);

		let r = request()
			.path("/v1/me/domains")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"domain":"example.com"}))
			.reply(&routes)
			.await;
		assert!(
			r.status() == StatusCode::CREATED || r.status() == StatusCode::OK,
			"Got {}",
			r.status()
		);

		let r = request()
			.path("/v1/me/domains/example.com")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);

		let r = request()
			.path("/v1/me/domains/example.com")
			.method("DELETE")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}
}
