/// Shared test infrastructure for E2E tests.
/// Uses TEST_DATABASE_URL if set (fast, no Docker), otherwise testcontainers.
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::rabbitmq::RabbitMq;

/// Wrapper that holds a PgPool. If using testcontainers, also holds the container.
pub struct TestDb {
	pool: PgPool,
	// Option because when using TEST_DATABASE_URL we don't have a container
	_container: Option<ContainerAsync<Postgres>>,
}

pub struct TestRabbitMq {
	pub amqp_url: String,
	_container: Option<ContainerAsync<RabbitMq>>,
}

impl TestRabbitMq {
	pub async fn start() -> Self {
		// Check for existing RabbitMQ via env var
		if let Ok(url) = std::env::var("TEST_AMQP_URL") {
			return Self { amqp_url: url, _container: None };
		}
		let container = RabbitMq::default()
			.start()
			.await
			.expect("Failed to start RabbitMQ container");
		let host_port = container
			.get_host_port_ipv4(5672)
			.await
			.expect("Failed to get RabbitMQ port");
		Self {
			amqp_url: format!("amqp://guest:guest@127.0.0.1:{}", host_port),
			_container: Some(container),
		}
	}
}

impl TestDb {
	pub async fn start() -> Self {
		// Fast path: use existing Postgres via env var (works under llvm-cov)
		if let Ok(url) = std::env::var("TEST_DATABASE_URL") {
			let pool = PgPoolOptions::new()
				.max_connections(5)
				.connect(&url)
				.await
				.expect("Failed to connect to TEST_DATABASE_URL");

			// Clean up data from previous runs (keep schema)
			let _ = sqlx::query("DELETE FROM job_events").execute(&pool).await;
			let _ = sqlx::query("DELETE FROM idempotency_keys").execute(&pool).await;
			let _ = sqlx::query("DELETE FROM v1_task_result").execute(&pool).await;
			let _ = sqlx::query("DELETE FROM v1_bulk_job").execute(&pool).await;
			let _ = sqlx::query("DELETE FROM api_keys").execute(&pool).await;
			let _ = sqlx::query("DELETE FROM tenants").execute(&pool).await;

			return Self { pool, _container: None };
		}

		// Slow path: testcontainers
		let container = Postgres::default()
			.start()
			.await
			.expect("Failed to start Postgres container");
		let host_port = container
			.get_host_port_ipv4(5432)
			.await
			.expect("Failed to get Postgres port");
		let db_url = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", host_port);
		let pool = PgPoolOptions::new()
			.max_connections(5)
			.connect(&db_url)
			.await
			.expect("Failed to connect to test database");

		sqlx::migrate!("./migrations")
			.run(&pool)
			.await
			.expect("Failed to run migrations");

		Self { pool, _container: Some(container) }
	}

	pub fn pool(&self) -> &PgPool {
		&self.pool
	}

	pub fn pool_owned(&self) -> PgPool {
		self.pool.clone()
	}
}

pub async fn insert_tenant(pool: &PgPool, slug: &str, monthly_limit: Option<i32>, used: i32) -> uuid::Uuid {
	let row = sqlx::query(
		"INSERT INTO tenants (name, slug, contact_email, plan_tier, status, monthly_email_limit, used_this_period) VALUES ($1, $2, $3, 'starter', 'active', $4, $5) RETURNING id",
	)
	.bind(format!("Tenant {}", slug))
	.bind(slug)
	.bind(format!("{}@test.com", slug))
	.bind(monthly_limit)
	.bind(used)
	.fetch_one(pool).await.expect("insert_tenant failed");
	row.get("id")
}

pub async fn insert_tenant_with_status(pool: &PgPool, slug: &str, status: &str) -> uuid::Uuid {
	let row = sqlx::query(
		"INSERT INTO tenants (name, slug, contact_email, plan_tier, status) VALUES ($1, $2, $3, 'free', $4::tenant_status) RETURNING id",
	)
	.bind(format!("Tenant {}", slug))
	.bind(slug)
	.bind(format!("{}@test.com", slug))
	.bind(status)
	.fetch_one(pool).await.expect("insert_tenant_with_status failed");
	row.get("id")
}

pub async fn insert_api_key(pool: &PgPool, tenant_id: uuid::Uuid) -> (String, uuid::Uuid) {
	let (full_key, prefix, hash) = reacher_backend::tenant::auth::generate_api_key();
	let row = sqlx::query(
		"INSERT INTO api_keys (tenant_id, key_prefix, key_hash, name, status) VALUES ($1, $2, $3, 'test-key', 'active') RETURNING id",
	)
	.bind(tenant_id).bind(&prefix).bind(&hash)
	.fetch_one(pool).await.expect("insert_api_key failed");
	(full_key, row.get("id"))
}

pub async fn insert_api_key_with_status(pool: &PgPool, tenant_id: uuid::Uuid, status: &str) -> (String, uuid::Uuid) {
	let (full_key, prefix, hash) = reacher_backend::tenant::auth::generate_api_key();
	let row = sqlx::query(
		"INSERT INTO api_keys (tenant_id, key_prefix, key_hash, name, status) VALUES ($1, $2, $3, 'test-key', $4::api_key_status) RETURNING id",
	)
	.bind(tenant_id).bind(&prefix).bind(&hash).bind(status)
	.fetch_one(pool).await.expect("insert_api_key_with_status failed");
	(full_key, row.get("id"))
}

pub async fn insert_job(pool: &PgPool, tenant_id: Option<uuid::Uuid>, total_records: i32, status: &str) -> i32 {
	let row = sqlx::query(
		"INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES ($1, $2, $3::job_state) RETURNING id",
	)
	.bind(total_records).bind(tenant_id).bind(status)
	.fetch_one(pool).await.expect("insert_job failed");
	row.get("id")
}

pub async fn insert_task(pool: &PgPool, job_id: i32, state: &str, tenant_id: Option<uuid::Uuid>, result_json: Option<serde_json::Value>, error_text: Option<&str>) -> i32 {
	let payload = serde_json::json!({"input": {"to_email": "t@e.com"}, "job_id": {"bulk": job_id}, "webhook": null});
	let row = sqlx::query(
		"INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id, result, error) VALUES ($1, $2, $3::task_state, $4, $5, $6) RETURNING id",
	)
	.bind(job_id).bind(&payload).bind(state).bind(tenant_id).bind(&result_json).bind(error_text)
	.fetch_one(pool).await.expect("insert_task failed");
	row.get("id")
}

pub async fn insert_event(pool: &PgPool, job_id: i32, task_id: Option<i32>, event_type: &str) -> i64 {
	let row = sqlx::query(
		"INSERT INTO job_events (job_id, task_id, event_type, actor) VALUES ($1, $2, $3, 'test') RETURNING id",
	)
	.bind(job_id).bind(task_id).bind(event_type)
	.fetch_one(pool).await.expect("insert_event failed");
	row.get("id")
}

pub fn safe_result() -> serde_json::Value {
	serde_json::json!({"input":"test@example.com","is_reachable":"safe","misc":{"is_disposable":false,"is_role_account":false,"is_b2c":false},"mx":{"accepts_email":true,"records":[]},"smtp":{"can_connect_smtp":true,"has_full_inbox":false,"is_catch_all":false,"is_deliverable":true,"is_disabled":false},"syntax":{"address":"test@example.com","domain":"example.com","is_valid_syntax":true,"username":"test"}})
}

pub fn invalid_result() -> serde_json::Value {
	serde_json::json!({"input":"bad@invalid","is_reachable":"invalid","misc":{"is_disposable":false,"is_role_account":false,"is_b2c":false},"mx":{"accepts_email":false,"records":[]},"smtp":{"can_connect_smtp":false,"has_full_inbox":false,"is_catch_all":false,"is_deliverable":false,"is_disabled":false},"syntax":{"address":null,"domain":"","is_valid_syntax":false,"username":""}})
}
