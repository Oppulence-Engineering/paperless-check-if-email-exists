#![allow(dead_code)]

/// Shared test infrastructure for E2E tests.
/// Uses per-process testcontainers by default to avoid cross-binary collisions.
/// Explicit TEST_DATABASE_URL / TEST_AMQP_URL always win so CI can route tests
/// to job-level service containers. Set USE_LOCAL_TEST_INFRA=1 to also opt into
/// the default local ports instead.
use lapin::{Connection, ConnectionProperties};
use reacher_backend::config::{
	BackendConfig, PipelinesConfig, PostgresConfig, RabbitMQConfig, StorageConfig, WorkerConfig,
};
use sqlx::migrate::Migrator;
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::{LazyLock, Mutex};
use testcontainers::runners::AsyncRunner;
use testcontainers::ContainerAsync;
use testcontainers::ImageExt;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::rabbitmq::RabbitMq;
use tokio::sync::OnceCell;
use tokio::time::{sleep, Duration};
use uuid::Uuid;

static SHARED_AMQP_URL: OnceCell<String> = OnceCell::const_new();
static SHARED_DB_URL: OnceCell<String> = OnceCell::const_new();
static TEST_DB_SCHEMA_READY: OnceCell<()> = OnceCell::const_new();
static CURRENT_TEST_DB_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));
static CURRENT_TEST_AMQP_URL: LazyLock<Mutex<Option<String>>> = LazyLock::new(|| Mutex::new(None));

const DEFAULT_TEST_DB_URL: &str = "postgres://postgres:postgres@127.0.0.1:25432/reacher_test";
const DEFAULT_TEST_AMQP_URL: &str = "amqp://guest:guest@127.0.0.1:35672";
const USE_LOCAL_TEST_INFRA_ENV: &str = "USE_LOCAL_TEST_INFRA";
const V410_FIXTURE_MAX_MIGRATION: &str = "20260320000003_list_deduplication";
pub const TEST_SECRET: &str = "test-secret";
pub const ADMIN_SECRET: &str = "admin-secret";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigProfile {
	Public,
	DbOnly,
	BearerTenant,
	AdminSecret,
	PseudoWorker,
	WorkerRabbit,
	PipelineEnabled,
}

#[derive(Debug, Clone)]
pub struct TenantApiKeysFixture {
	pub tenant_id: Uuid,
	pub full_access_key: String,
	pub bulk_key: String,
	pub lists_key: String,
	pub verify_key: String,
	pub pipelines_key: String,
	pub revoked_key_id: Uuid,
	pub revoked_key: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationFixture {
	V410,
}

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

	if let Ok(url) = std::env::var("TEST_DATABASE_URL") {
		ensure_test_db_schema(&url)
			.await
			.expect("TEST_DATABASE_URL schema should be ready");
		set_current_test_db_url(&url);
		return url;
	}

	if use_local_test_infra() {
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

pub struct UpgradedScratchDb {
	pool: PgPool,
	db_url: String,
	admin_db_url: String,
	db_name: String,
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

		if let Ok(url) = std::env::var("TEST_AMQP_URL") {
			set_current_test_amqp_url(&url);
			return Self {
				amqp_url: url,
				_container: None,
			};
		}

		if use_local_test_infra() {
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

impl UpgradedScratchDb {
	pub fn pool(&self) -> &PgPool {
		&self.pool
	}

	pub fn db_url(&self) -> &str {
		&self.db_url
	}
}

impl Drop for UpgradedScratchDb {
	fn drop(&mut self) {
		let admin_db_url = self.admin_db_url.clone();
		let db_name = self.db_name.clone();
		let pool = self.pool.clone();
		if let Ok(handle) = tokio::runtime::Handle::try_current() {
			handle.spawn(async move {
				pool.close().await;
				let _ = drop_scratch_database(&admin_db_url, &db_name).await;
			});
		}
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

	sqlx::migrate!("./migrations")
		.run(&pool)
		.await
		.expect("Failed to run migrations on existing test database");

	// Clean up data from previous runs (keep schema)
	let _ = sqlx::query("DELETE FROM job_comments").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM job_events").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM v1_suppression_entries")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM idempotency_keys")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM reputation_cache")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM tenant_domains")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_finder_result")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_finder_result")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_finder_job")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_usage_events")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_pipeline_contact_state")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_pipeline_runs")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_pipelines").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM v1_lists").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM v1_task_result")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM v1_bulk_job").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM email_results")
		.execute(&pool)
		.await;
	let _ = sqlx::query("DELETE FROM bulk_jobs").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM api_keys").execute(&pool).await;
	let _ = sqlx::query("DELETE FROM tenants").execute(&pool).await;

	TestDb {
		pool,
		db_url: url,
		_container: None,
	}
}

fn fixture_seed_path(fixture: MigrationFixture) -> PathBuf {
	match fixture {
		MigrationFixture::V410 => Path::new(env!("CARGO_MANIFEST_DIR"))
			.join("tests")
			.join("fixtures")
			.join("migration_upgrade")
			.join("v4.1.0_seed.sql"),
	}
}

fn fixture_max_migration(fixture: MigrationFixture) -> &'static str {
	match fixture {
		MigrationFixture::V410 => V410_FIXTURE_MAX_MIGRATION,
	}
}

fn rewrite_database_name(url: &str, database_name: &str) -> String {
	let (prefix, suffix) = url
		.rsplit_once('/')
		.expect("database url should include a database name");
	match suffix.split_once('?') {
		Some((_db, query)) => format!("{prefix}/{database_name}?{query}"),
		None => format!("{prefix}/{database_name}"),
	}
}

fn materialize_release_migrations(fixture: MigrationFixture) -> Result<PathBuf, String> {
	let migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
	let temp_dir = std::env::temp_dir().join(format!(
		"reacher-release-migrations-{}",
		Uuid::new_v4().simple()
	));
	fs::create_dir_all(&temp_dir).map_err(|err| {
		format!(
			"failed to create migration tempdir {}: {err}",
			temp_dir.display()
		)
	})?;
	let max_prefix = fixture_max_migration(fixture);

	for entry in fs::read_dir(&migrations_dir).map_err(|err| {
		format!(
			"failed to read migrations dir {}: {err}",
			migrations_dir.display()
		)
	})? {
		let entry = entry.map_err(|err| format!("failed to read migration entry: {err}"))?;
		let file_name = entry.file_name();
		let file_name = file_name.to_string_lossy();
		let keep = file_name == "README.md"
			|| file_name
				.strip_suffix(".up.sql")
				.or_else(|| file_name.strip_suffix(".down.sql"))
				.map(|prefix| prefix <= max_prefix)
				.unwrap_or(false);
		if keep {
			fs::copy(entry.path(), temp_dir.join(file_name.as_ref())).map_err(|err| {
				format!(
					"failed to copy migration {} into {}: {err}",
					entry.path().display(),
					temp_dir.display()
				)
			})?;
		}
	}

	Ok(temp_dir)
}

fn split_sql_statements(sql: &str) -> Vec<String> {
	let mut statements = Vec::new();
	let mut current = String::new();
	let mut in_single_quote = false;
	let mut in_double_quote = false;
	let mut chars = sql.chars().peekable();

	while let Some(ch) = chars.next() {
		match ch {
			'\'' if !in_double_quote => {
				current.push(ch);
				if in_single_quote {
					if chars.peek() == Some(&'\'') {
						current.push(chars.next().expect("escaped single quote"));
					} else {
						in_single_quote = false;
					}
				} else {
					in_single_quote = true;
				}
			}
			'"' if !in_single_quote => {
				in_double_quote = !in_double_quote;
				current.push(ch);
			}
			';' if !in_single_quote && !in_double_quote => {
				let trimmed = current.trim();
				if !trimmed.is_empty() {
					statements.push(trimmed.to_string());
				}
				current.clear();
			}
			_ => current.push(ch),
		}
	}

	let trimmed = current.trim();
	if !trimmed.is_empty() {
		statements.push(trimmed.to_string());
	}
	statements
}

async fn apply_sql_fixture(pool: &PgPool, path: &Path) -> Result<(), String> {
	let sql = fs::read_to_string(path)
		.map_err(|err| format!("failed to read fixture {}: {err}", path.display()))?;
	for statement in split_sql_statements(&sql) {
		sqlx::query(&statement).execute(pool).await.map_err(|err| {
			format!(
				"failed to execute fixture statement from {}: {err}",
				path.display()
			)
		})?;
	}
	Ok(())
}

async fn drop_scratch_database(admin_db_url: &str, db_name: &str) -> Result<(), String> {
	let pool = PgPoolOptions::new()
		.max_connections(1)
		.connect(admin_db_url)
		.await
		.map_err(|err| format!("failed to connect admin db for cleanup: {err}"))?;
	sqlx::query(
		r#"
		SELECT pg_terminate_backend(pid)
		FROM pg_stat_activity
		WHERE datname = $1 AND pid <> pg_backend_pid()
		"#,
	)
	.bind(db_name)
	.execute(&pool)
	.await
	.map_err(|err| format!("failed to terminate scratch db connections: {err}"))?;
	sqlx::query(&format!(r#"DROP DATABASE IF EXISTS "{db_name}""#))
		.execute(&pool)
		.await
		.map_err(|err| format!("failed to drop scratch db {db_name}: {err}"))?;
	pool.close().await;
	Ok(())
}

pub async fn restore_migration_fixture_to_head(fixture: MigrationFixture) -> UpgradedScratchDb {
	let base_db_url = ensure_test_db_url().await;
	let admin_db_url = rewrite_database_name(&base_db_url, "postgres");
	let db_name = format!("reacher_upgrade_{}", Uuid::new_v4().simple());
	let scratch_db_url = rewrite_database_name(&base_db_url, &db_name);
	let admin_pool = PgPoolOptions::new()
		.max_connections(1)
		.connect(&admin_db_url)
		.await
		.expect("connect admin db for scratch restore");
	sqlx::query(&format!(r#"CREATE DATABASE "{db_name}""#))
		.execute(&admin_pool)
		.await
		.expect("create scratch database");
	admin_pool.close().await;

	let pool = PgPoolOptions::new()
		.max_connections(10)
		.connect(&scratch_db_url)
		.await
		.expect("connect scratch database");
	let release_migrations_dir =
		materialize_release_migrations(fixture).expect("materialize release migrations");
	let release_migrator = Migrator::new(release_migrations_dir.as_path())
		.await
		.expect("load release migrator");
	release_migrator
		.run(&pool)
		.await
		.expect("run release migrations");
	let _ = fs::remove_dir_all(&release_migrations_dir);

	apply_sql_fixture(&pool, &fixture_seed_path(fixture))
		.await
		.expect("apply upgrade fixture seed");
	sqlx::migrate!("./migrations")
		.run(&pool)
		.await
		.expect("run head migrations after fixture restore");

	UpgradedScratchDb {
		pool,
		db_url: scratch_db_url,
		admin_db_url,
		db_name,
	}
}

pub async fn insert_tenant(
	pool: &PgPool,
	slug: &str,
	monthly_limit: Option<i32>,
	used: i32,
) -> Uuid {
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

pub async fn insert_tenant_with_status(pool: &PgPool, slug: &str, status: &str) -> Uuid {
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

pub async fn insert_api_key(pool: &PgPool, tenant_id: Uuid) -> (String, Uuid) {
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
	tenant_id: Uuid,
	scopes: &[&str],
) -> (String, Uuid) {
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
	tenant_id: Uuid,
	status: &str,
) -> (String, Uuid) {
	let (full_key, prefix, hash) = reacher_backend::tenant::auth::generate_api_key();
	let row = sqlx::query(
		"INSERT INTO api_keys (tenant_id, key_prefix, key_hash, name, status) VALUES ($1, $2, $3, 'test-key', $4::api_key_status) RETURNING id",
	)
	.bind(tenant_id).bind(&prefix).bind(&hash).bind(status)
	.fetch_one(pool).await.expect("insert_api_key_with_status failed");
	(full_key, row.get("id"))
}

pub async fn insert_tenant_with_keys(pool: &PgPool, slug: &str) -> TenantApiKeysFixture {
	let tenant_id = insert_tenant(pool, slug, Some(10_000), 0).await;
	insert_keys_for_existing_tenant(pool, tenant_id).await
}

pub async fn insert_keys_for_existing_tenant(
	pool: &PgPool,
	tenant_id: Uuid,
) -> TenantApiKeysFixture {
	let (full_access_key, _) = insert_api_key(pool, tenant_id).await;
	let (bulk_key, _) = insert_api_key_with_scopes(pool, tenant_id, &["bulk"]).await;
	let (lists_key, _) = insert_api_key_with_scopes(pool, tenant_id, &["lists"]).await;
	let (verify_key, _) = insert_api_key_with_scopes(pool, tenant_id, &["verify"]).await;
	let (pipelines_key, _) = insert_api_key_with_scopes(
		pool,
		tenant_id,
		&["pipelines.read", "pipelines.write", "pipelines.trigger"],
	)
	.await;
	let (revoked_key, revoked_key_id) =
		insert_api_key_with_status(pool, tenant_id, "revoked").await;

	TenantApiKeysFixture {
		tenant_id,
		full_access_key,
		bulk_key,
		lists_key,
		verify_key,
		pipelines_key,
		revoked_key_id,
		revoked_key,
	}
}

pub async fn insert_job(
	pool: &PgPool,
	tenant_id: Option<Uuid>,
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
	tenant_id: Option<Uuid>,
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

pub async fn insert_scored_task(
	pool: &PgPool,
	job_id: i32,
	tenant_id: Option<Uuid>,
	payload_email: &str,
	extra: Option<serde_json::Value>,
	result_json: Option<serde_json::Value>,
	task_state: &str,
	score: Option<i16>,
	score_category: Option<&str>,
	sub_reason: Option<&str>,
	safe_to_send: Option<bool>,
	reason_codes: Option<Vec<String>>,
	canonical_email: Option<&str>,
	is_duplicate: bool,
) -> i32 {
	let payload = serde_json::json!({
		"input": {"to_email": payload_email},
		"job_id": {"bulk": job_id},
		"webhook": null
	});
	let row = sqlx::query(
		r#"
		INSERT INTO v1_task_result (
			job_id,
			payload,
			extra,
			result,
			error,
			task_state,
			tenant_id,
			score,
			score_category,
			sub_reason,
			safe_to_send,
			reason_codes,
			canonical_email,
			is_duplicate,
			completed_at
		)
		VALUES (
			$1, $2, $3, $4, NULL, $5::task_state, $6, $7, $8, $9, $10, $11, $12, $13,
			CASE
				WHEN $5::task_state IN ('completed', 'failed', 'cancelled', 'dead_lettered') THEN NOW()
				ELSE NULL
			END
		)
		RETURNING id
		"#,
	)
	.bind(job_id)
	.bind(&payload)
	.bind(extra)
	.bind(result_json)
	.bind(task_state)
	.bind(tenant_id)
	.bind(score)
	.bind(score_category)
	.bind(sub_reason)
	.bind(safe_to_send)
	.bind(reason_codes)
	.bind(canonical_email)
	.bind(is_duplicate)
	.fetch_one(pool)
	.await
	.expect("insert_scored_task failed");
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

pub async fn insert_list(
	pool: &PgPool,
	tenant_id: Uuid,
	job_id: i32,
	name: &str,
	status: &str,
	total_rows: i32,
	original_headers: &[&str],
	original_rows: serde_json::Value,
) -> i32 {
	let row = sqlx::query(
		r#"
		INSERT INTO v1_lists (
			tenant_id,
			job_id,
			name,
			original_filename,
			file_size_bytes,
			total_rows,
			email_column,
			original_headers,
			original_data,
			status,
			unique_emails,
			deduplicated_count
		)
		VALUES ($1, $2, $3, 'fixture.csv', 128, $4, 'email', $5, $6, $7::list_status, $8, 0)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(job_id)
	.bind(name)
	.bind(total_rows)
	.bind(
		original_headers
			.iter()
			.map(|value| value.to_string())
			.collect::<Vec<_>>(),
	)
	.bind(original_rows)
	.bind(status)
	.bind(total_rows)
	.fetch_one(pool)
	.await
	.expect("insert_list failed");
	row.get("id")
}

pub async fn insert_pipeline(
	pool: &PgPool,
	tenant_id: Uuid,
	name: &str,
	source_config: serde_json::Value,
) -> i64 {
	let row = sqlx::query(
		r#"
		INSERT INTO v1_pipelines (
			tenant_id,
			name,
			status,
			source_type,
			source_config,
			schedule_cron,
			schedule_timezone,
			verification_settings,
			delivery_config,
			next_run_at
		)
		VALUES (
			$1, $2, 'active'::pipeline_status, 'list_snapshot'::pipeline_source_type, $3,
			'0 * * * *', 'UTC', '{}'::jsonb, '{}'::jsonb, NOW() + INTERVAL '1 hour'
		)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(name)
	.bind(source_config)
	.fetch_one(pool)
	.await
	.expect("insert_pipeline failed");
	row.get("id")
}

pub async fn insert_pipeline_run(
	pool: &PgPool,
	pipeline_id: i64,
	tenant_id: Uuid,
	status: &str,
	job_id: Option<i32>,
	list_id: Option<i32>,
) -> i64 {
	let row = sqlx::query(
		r#"
		INSERT INTO v1_pipeline_runs (
			pipeline_id,
			tenant_id,
			trigger_type,
			status,
			job_id,
			list_id,
			source_snapshot,
			stats
		)
		VALUES ($1, $2, 'manual', $3::pipeline_run_status, $4, $5, '{}'::jsonb, '{}'::jsonb)
		RETURNING id
		"#,
	)
	.bind(pipeline_id)
	.bind(tenant_id)
	.bind(status)
	.bind(job_id)
	.bind(list_id)
	.fetch_one(pool)
	.await
	.expect("insert_pipeline_run failed");
	row.get("id")
}

pub async fn insert_comment(
	pool: &PgPool,
	tenant_id: Uuid,
	job_id: Option<i32>,
	list_id: Option<i32>,
	body: &str,
) -> i64 {
	let row = sqlx::query(
		"INSERT INTO job_comments (tenant_id, job_id, list_id, body, author) VALUES ($1, $2, $3, $4, 'Harness') RETURNING id",
	)
	.bind(tenant_id)
	.bind(job_id)
	.bind(list_id)
	.bind(body)
	.fetch_one(pool)
	.await
	.expect("insert_comment failed");
	row.get("id")
}

pub async fn insert_suppression(pool: &PgPool, tenant_id: Uuid, email: &str, reason: &str) -> i32 {
	let row = sqlx::query(
		"INSERT INTO v1_suppression_entries (tenant_id, email, reason) VALUES ($1, $2, $3::suppression_reason) RETURNING id",
	)
	.bind(tenant_id)
	.bind(email)
	.bind(reason)
	.fetch_one(pool)
	.await
	.expect("insert_suppression failed");
	row.get("id")
}

pub async fn insert_reputation_cache(
	pool: &PgPool,
	domain: &str,
	response: serde_json::Value,
	score: i16,
	risk_level: &str,
) {
	sqlx::query(
		r#"
		INSERT INTO reputation_cache (domain, response, score, risk_level, expires_at)
		VALUES ($1, $2, $3, $4, NOW() + INTERVAL '30 days')
		ON CONFLICT (domain) DO UPDATE
		SET response = EXCLUDED.response,
		    score = EXCLUDED.score,
		    risk_level = EXCLUDED.risk_level,
		    expires_at = EXCLUDED.expires_at,
		    updated_at = NOW()
		"#,
	)
	.bind(domain)
	.bind(response)
	.bind(score)
	.bind(risk_level)
	.execute(pool)
	.await
	.expect("insert_reputation_cache failed");
}

pub async fn insert_domain(
	pool: &PgPool,
	tenant_id: Uuid,
	domain: &str,
	is_active: bool,
	is_verified: bool,
) -> Uuid {
	let row = sqlx::query(
		r#"
		INSERT INTO tenant_domains (tenant_id, domain, is_active, is_verified, notes)
		VALUES ($1, $2, $3, $4, 'fixture')
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(domain)
	.bind(is_active)
	.bind(is_verified)
	.fetch_one(pool)
	.await
	.expect("insert_domain failed");
	row.get("id")
}

pub async fn insert_finder_job(
	pool: &PgPool,
	tenant_id: Uuid,
	bulk_job_id: i32,
	status: &str,
	domain: &str,
	candidates_checked: i32,
	best_match_email: Option<&str>,
) -> i32 {
	let row = sqlx::query(
		r#"
		INSERT INTO v1_finder_job (
			tenant_id,
			bulk_job_id,
			first_name,
			last_name,
			domain,
			normalized_first_name,
			normalized_last_name,
			status,
			domain_has_mx,
			domain_is_catch_all,
			candidates_checked,
			best_match_email,
			best_match_score,
			best_match_confidence,
			completed_at
		)
		VALUES (
			$1, $2, 'Jane', 'Doe', $3, 'jane', 'doe', $4::job_state, true, false, $5, $6, 95, 'high',
			CASE WHEN $4::job_state = 'completed' THEN NOW() ELSE NULL END
		)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(bulk_job_id)
	.bind(domain)
	.bind(status)
	.bind(candidates_checked)
	.bind(best_match_email)
	.fetch_one(pool)
	.await
	.expect("insert_finder_job failed");
	row.get("id")
}

pub async fn insert_finder_result(
	pool: &PgPool,
	finder_job_id: i32,
	task_result_id: Option<i32>,
	candidate_email: &str,
	pattern: &str,
	score: i16,
	score_category: &str,
	result: Option<serde_json::Value>,
) -> i32 {
	let row = sqlx::query(
		r#"
		INSERT INTO v1_finder_result (
			finder_job_id,
			task_result_id,
			candidate_email,
			pattern,
			rank_position,
			score,
			score_category,
			sub_reason,
			result
		)
		VALUES ($1, $2, $3, $4, 1, $5, $6, 'deliverable', $7)
		RETURNING id
		"#,
	)
	.bind(finder_job_id)
	.bind(task_result_id)
	.bind(candidate_email)
	.bind(pattern)
	.bind(score)
	.bind(score_category)
	.bind(result)
	.fetch_one(pool)
	.await
	.expect("insert_finder_result failed");
	row.get("id")
}

pub async fn insert_legacy_bulk_job(pool: &PgPool, total_records: i32) -> i32 {
	let row = sqlx::query("INSERT INTO bulk_jobs (total_records) VALUES ($1) RETURNING id")
		.bind(total_records)
		.fetch_one(pool)
		.await
		.expect("insert_legacy_bulk_job failed");
	row.get("id")
}

pub async fn insert_legacy_email_result(
	pool: &PgPool,
	job_id: i32,
	result: serde_json::Value,
) -> i32 {
	let row =
		sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2) RETURNING id")
			.bind(job_id)
			.bind(result)
			.fetch_one(pool)
			.await
			.expect("insert_legacy_email_result failed");
	row.get("id")
}

pub async fn build_test_config(
	profile: ConfigProfile,
	db_url: Option<&str>,
	amqp_url: Option<&str>,
) -> Arc<BackendConfig> {
	let mut config = BackendConfig::empty();
	match profile {
		ConfigProfile::Public => return Arc::new(config),
		ConfigProfile::DbOnly | ConfigProfile::BearerTenant => {
			config.header_secret = Some(TEST_SECRET.to_string());
			config.storage = Some(StorageConfig::Postgres(PostgresConfig {
				db_url: db_url.expect("db_url required").to_string(),
				read_replica_url: None,
				extra: None,
			}));
			config.connect().await.expect("db config connect");
		}
		ConfigProfile::AdminSecret => {
			config.header_secret = Some(ADMIN_SECRET.to_string());
			config.storage = Some(StorageConfig::Postgres(PostgresConfig {
				db_url: db_url.expect("db_url required").to_string(),
				read_replica_url: None,
				extra: None,
			}));
			config.connect().await.expect("admin config connect");
		}
		ConfigProfile::PseudoWorker => {
			config.header_secret = Some(TEST_SECRET.to_string());
			config.storage = Some(StorageConfig::Postgres(PostgresConfig {
				db_url: db_url.expect("db_url required").to_string(),
				read_replica_url: None,
				extra: None,
			}));
			config.connect().await.expect("pseudo worker connect");
			config.worker.enable = true;
		}
		ConfigProfile::WorkerRabbit | ConfigProfile::PipelineEnabled => {
			config.header_secret = Some(TEST_SECRET.to_string());
			config.storage = Some(StorageConfig::Postgres(PostgresConfig {
				db_url: db_url.expect("db_url required").to_string(),
				read_replica_url: None,
				extra: None,
			}));
			config.worker = WorkerConfig {
				enable: true,
				rabbitmq: Some(RabbitMQConfig {
					url: amqp_url.expect("amqp_url required").to_string(),
					concurrency: 4,
				}),
				webhook: None,
			};
			if matches!(profile, ConfigProfile::PipelineEnabled) {
				config.pipelines = PipelinesConfig {
					enable: true,
					..PipelinesConfig::default()
				};
			}
			config.connect().await.expect("worker config connect");
		}
	}

	Arc::new(config)
}

pub fn safe_result() -> serde_json::Value {
	serde_json::json!({"input":"test@example.com","is_reachable":"safe","misc":{"is_disposable":false,"is_role_account":false,"is_b2c":false},"mx":{"accepts_email":true,"records":[]},"smtp":{"can_connect_smtp":true,"has_full_inbox":false,"is_catch_all":false,"is_deliverable":true,"is_disabled":false},"syntax":{"address":"test@example.com","domain":"example.com","is_valid_syntax":true,"username":"test"}})
}

pub fn invalid_result() -> serde_json::Value {
	serde_json::json!({"input":"bad@invalid","is_reachable":"invalid","misc":{"is_disposable":false,"is_role_account":false,"is_b2c":false},"mx":{"accepts_email":false,"records":[]},"smtp":{"can_connect_smtp":false,"has_full_inbox":false,"is_catch_all":false,"is_deliverable":false,"is_disabled":false},"syntax":{"address":null,"domain":"","is_valid_syntax":false,"username":""}})
}
