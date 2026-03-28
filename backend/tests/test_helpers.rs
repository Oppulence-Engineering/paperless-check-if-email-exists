/// Shared test infrastructure for E2E tests.
/// Uses per-process testcontainers by default to avoid cross-binary collisions.
/// Set USE_LOCAL_TEST_INFRA=1 to opt into TEST_DATABASE_URL / TEST_AMQP_URL or
/// the default local ports instead.
use lapin::{Connection, ConnectionProperties};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::sync::{LazyLock, Mutex};
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers::ImageExt;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::rabbitmq::RabbitMq;
use tokio::sync::OnceCell;
use tokio::time::{sleep, Duration};

static SHARED_AMQP_URL: OnceCell<String> = OnceCell::const_new();
static SHARED_DB_URL: OnceCell<String> = OnceCell::const_new();
static TEST_DB_SCHEMA_READY: OnceCell<()> = OnceCell::const_new();
static CURRENT_TEST_DB_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static CURRENT_TEST_AMQP_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

const DEFAULT_TEST_DB_URL: &str = "postgres://postgres:postgres@127.0.0.1:25432/reacher_test";
const DEFAULT_TEST_AMQP_URL: &str = "amqp://guest:guest@127.0.0.1:35672";
const USE_LOCAL_TEST_INFRA_ENV: &str = "USE_LOCAL_TEST_INFRA";

fn use_local_test_infra() -> bool {
	std::env::var(USE_LOCAL_TEST_INFRA_ENV)
		.ok()
		.map(|value| {
			matches!(
				value.trim().to_ascii_lowercase().as_str(),
				"1" | "true" | "yes" | "on"
			)
		})
		.unwrap_or(false)
}

fn set_current_test_db_url(url: &str) {
	*CURRENT_TEST_DB_URL.lock().expect("db url mutex poisoned") = Some(url.to_string());
}

fn set_current_test_amqp_url(url: &str) {
	*CURRENT_TEST_AMQP_URL
		.lock()
		.expect("amqp url mutex poisoned") = Some(url.to_string());
}

pub fn test_db_url() -> String {
	CURRENT_TEST_DB_URL
		.lock()
		.expect("db url mutex poisoned")
		.clone()
		.or_else(|| std::env::var("TEST_DATABASE_URL").ok())
		.unwrap_or_else(|| DEFAULT_TEST_DB_URL.to_string())
}

pub fn test_amqp_url() -> String {
	CURRENT_TEST_AMQP_URL
		.lock()
		.expect("amqp url mutex poisoned")
		.clone()
		.or_else(|| std::env::var("TEST_AMQP_URL").ok())
		.unwrap_or_else(|| DEFAULT_TEST_AMQP_URL.to_string())
}

pub async fn ensure_test_db_url() -> String {
	if let Some(url) = CURRENT_TEST_DB_URL
		.lock()
		.expect("db url mutex poisoned")
		.clone()
	{
		ensure_test_db_schema(&url)
			.await
			.expect("current test database schema should be ready");
		return url;
	}

	if use_local_test_infra() {
		if let Ok(url) = std::env::var("TEST_DATABASE_URL") {
			ensure_test_db_schema(&url)
				.await
				.expect("TEST_DATABASE_URL schema should be ready");
			set_current_test_db_url(&url);
			return url;
		}

		if PgPoolOptions::new()
			.max_connections(1)
			.acquire_timeout(Duration::from_secs(2))
			.connect(DEFAULT_TEST_DB_URL)
			.await
			.is_ok()
		{
			if ensure_test_db_schema(DEFAULT_TEST_DB_URL).await.is_ok() {
				set_current_test_db_url(DEFAULT_TEST_DB_URL);
				return DEFAULT_TEST_DB_URL.to_string();
			}
		}
	}

	let db_url = SHARED_DB_URL
		.get_or_try_init(|| async {
			let mut last_error = None;
			for attempt in 1..=3 {
				match Postgres::default()
					.with_startup_timeout(Duration::from_secs(120))
					.start()
					.await
				{
					Ok(container) => {
						let host_port = container
							.get_host_port_ipv4(5432)
							.await
							.map_err(|err| format!("Failed to get Postgres port: {err}"))?;
						let db_url =
							format!("postgres://postgres:postgres@127.0.0.1:{host_port}/postgres");

						ensure_test_db_schema(&db_url).await.map_err(|err| {
							format!("Failed to prepare shared test database schema: {err}")
						})?;

						set_current_test_db_url(&db_url);
						let _ = Box::leak(Box::new(container));
						return Ok::<String, String>(db_url);
					}
					Err(err) => {
						last_error = Some(err.to_string());
						if attempt < 3 {
							sleep(Duration::from_secs(2)).await;
						}
					}
				}
			}

			Err(format!(
				"Failed to start Postgres container: {}",
				last_error.unwrap_or_else(|| "unknown error".to_string())
			))
		})
		.await
		.expect("shared Postgres init")
		.clone();

	set_current_test_db_url(&db_url);
	db_url
}

async fn ensure_test_db_schema(url: &str) -> Result<(), String> {
	TEST_DB_SCHEMA_READY
		.get_or_try_init(|| async move {
			let pool = PgPoolOptions::new()
				.max_connections(10)
				.connect(url)
				.await
				.map_err(|err| format!("Failed to connect for test migrations: {err}"))?;

			sqlx::migrate!("./migrations")
				.run(&pool)
				.await
				.map_err(|err| format!("Failed to run test migrations: {err}"))?;

			Ok::<(), String>(())
		})
		.await
		.map(|_| ())
}

pub async fn ensure_test_amqp_url() -> String {
	TestRabbitMq::start().await.amqp_url
}

/// Wrapper that holds a PgPool. If using testcontainers, also holds the container.
pub struct TestDb {
	pool: PgPool,
	db_url: String,
	// Option because when using TEST_DATABASE_URL we don't have a container
	_container: Option<ContainerAsync<Postgres>>,
}

pub struct TestRabbitMq {
	pub amqp_url: String,
	_container: Option<ContainerAsync<RabbitMq>>,
}

impl TestRabbitMq {
	pub async fn start() -> Self {
		if let Some(url) = CURRENT_TEST_AMQP_URL
			.lock()
			.expect("amqp url mutex poisoned")
			.clone()
		{
			return Self {
				amqp_url: url,
				_container: None,
			};
		}

		if use_local_test_infra() {
			if let Ok(url) = std::env::var("TEST_AMQP_URL") {
				set_current_test_amqp_url(&url);
				return Self {
					amqp_url: url,
					_container: None,
				};
			}

			if Connection::connect(DEFAULT_TEST_AMQP_URL, ConnectionProperties::default())
				.await
				.is_ok()
			{
				set_current_test_amqp_url(DEFAULT_TEST_AMQP_URL);
				return Self {
					amqp_url: DEFAULT_TEST_AMQP_URL.to_string(),
					_container: None,
				};
			}
		}

		let amqp_url = SHARED_AMQP_URL
			.get_or_try_init(|| async {
				let mut last_error = None;
				for attempt in 1..=5 {
					match RabbitMq::default()
						.with_startup_timeout(Duration::from_secs(300))
						.start()
						.await
					{
						Ok(container) => {
							let host_port = container
								.get_host_port_ipv4(5672)
								.await
								.map_err(|err| format!("Failed to get RabbitMQ port: {err}"))?;
							let amqp_url = format!("amqp://guest:guest@127.0.0.1:{}", host_port);
							set_current_test_amqp_url(&amqp_url);
							let _ = Box::leak(Box::new(container));
							return Ok::<String, String>(amqp_url);
						}
						Err(err) => {
							last_error = Some(err.to_string());
							if attempt < 5 {
								sleep(Duration::from_secs(2)).await;
							}
						}
					}
				}
				Err(format!(
					"Failed to start RabbitMQ container: {}",
					last_error.unwrap_or_else(|| "unknown error".to_string())
				))
			})
			.await
			.expect("shared RabbitMQ init")
			.clone();

		Self {
			amqp_url,
			_container: None,
		}
	}
}

impl TestDb {
	pub async fn start() -> Self {
		connect_existing_db(ensure_test_db_url().await).await
	}

	pub fn pool(&self) -> &PgPool {
		&self.pool
	}

	pub fn pool_owned(&self) -> PgPool {
		self.pool.clone()
	}

	pub fn db_url(&self) -> &str {
		&self.db_url
	}
}

async fn connect_existing_db(url: String) -> TestDb {
	ensure_test_db_schema(&url)
		.await
		.expect("existing test database schema should be ready");

	let pool = PgPoolOptions::new()
		.max_connections(10)
		.connect(&url)
		.await
		.expect("Failed to connect to existing test database");

	// Clean up data from previous runs (keep schema)
	let _ = sqlx::query("DELETE FROM job_comments").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM job_events").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM idempotency_keys")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM reputation_cache")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_finder_result")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_finder_job")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_lists").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM v1_task_result")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_bulk_job").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM api_keys").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM tenants").execute(&pool).await;

	TestDb {
		pool,
		db_url: url,
		_container: None,
	}
}

pub async fn insert_tenant(
	pool: &PgPool,
	slug: &str,
	monthly_limit: Option<i32>,
	used: i32,
) -> uuid::Uuid {
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

pub async fn insert_api_key_with_scopes(
	pool: &PgPool,
	tenant_id: uuid::Uuid,
	scopes: &[&str],
) -> (String, uuid::Uuid) {
	let (full_key, prefix, hash) = reacher_backend::tenant::auth::generate_api_key();
	let scope_vec: Vec<String> = scopes.iter().map(|s| s.to_string()).collect();
	let row = sqlx::query(
		"INSERT INTO api_keys (tenant_id, key_prefix, key_hash, name, status, scopes) VALUES ($1, $2, $3, 'scoped-key', 'active', $4) RETURNING id",
	)
	.bind(tenant_id).bind(&prefix).bind(&hash).bind(&scope_vec)
	.fetch_one(pool).await.expect("insert_api_key_with_scopes failed");
	(full_key, row.get("id"))
}

pub async fn insert_api_key_with_status(
	pool: &PgPool,
	tenant_id: uuid::Uuid,
	status: &str,
) -> (String, uuid::Uuid) {
	let (full_key, prefix, hash) = reacher_backend::tenant::auth::generate_api_key();
	let row = sqlx::query(
		"INSERT INTO api_keys (tenant_id, key_prefix, key_hash, name, status) VALUES ($1, $2, $3, 'test-key', $4::api_key_status) RETURNING id",
	)
	.bind(tenant_id).bind(&prefix).bind(&hash).bind(status)
	.fetch_one(pool).await.expect("insert_api_key_with_status failed");
	(full_key, row.get("id"))
}

pub async fn insert_job(
	pool: &PgPool,
	tenant_id: Option<uuid::Uuid>,
	total_records: i32,
	status: &str,
) -> i32 {
	let row = sqlx::query(
		"INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES ($1, $2, $3::job_state) RETURNING id",
	)
	.bind(total_records).bind(tenant_id).bind(status)
	.fetch_one(pool).await.expect("insert_job failed");
	row.get("id")
}

pub async fn insert_task(
	pool: &PgPool,
	job_id: i32,
	state: &str,
	tenant_id: Option<uuid::Uuid>,
	result_json: Option<serde_json::Value>,
	error_text: Option<&str>,
) -> i32 {
	let payload = serde_json::json!({"input": {"to_email": "t@e.com"}, "job_id": {"bulk": job_id}, "webhook": null});
	let row = sqlx::query(
		"INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id, result, error) VALUES ($1, $2, $3::task_state, $4, $5, $6) RETURNING id",
	)
	.bind(job_id).bind(&payload).bind(state).bind(tenant_id).bind(&result_json).bind(error_text)
	.fetch_one(pool).await.expect("insert_task failed");
	row.get("id")
}

pub async fn insert_event(
	pool: &PgPool,
	job_id: i32,
	task_id: Option<i32>,
	event_type: &str,
) -> i64 {
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
