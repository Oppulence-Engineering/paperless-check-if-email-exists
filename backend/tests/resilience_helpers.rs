#![allow(dead_code)]

use lapin::{Connection, ConnectionProperties};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::process::{Command as StdCommand, Stdio};
use std::sync::LazyLock;
use tokio::process::Command;
use tokio::sync::Mutex;
use tokio::time::{sleep, timeout, Duration, Instant};
use uuid::Uuid;

const POSTGRES_IMAGE: &str = "postgres:16";
const RABBITMQ_IMAGE: &str = "rabbitmq:3.8.22-management";
const POSTGRES_WAIT_TIMEOUT: Duration = Duration::from_secs(60);
const RABBITMQ_WAIT_TIMEOUT: Duration = Duration::from_secs(300);
const RABBITMQ_CONNECT_ATTEMPT_TIMEOUT: Duration = Duration::from_secs(5);
const POLL_INTERVAL: Duration = Duration::from_millis(500);

static RABBITMQ_START_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

pub struct OwnedPostgres {
	name: String,
	db_name: String,
	db_url: String,
}

pub struct OwnedRabbitMq {
	name: String,
	amqp_url: String,
}

pub struct OwnedResilienceEnv {
	pub postgres: OwnedPostgres,
	pub rabbitmq: OwnedRabbitMq,
}

impl OwnedPostgres {
	pub async fn start() -> Self {
		let name = format!("reacher-pg-{}", Uuid::new_v4().simple());
		let db_name = "reacher_owned".to_string();
		let db_url = match start_postgres_container(&name, &db_name).await {
			Ok(db_url) => db_url,
			Err(err) => {
				docker_rm_force(&name);
				panic!("start owned postgres: {}", err);
			}
		};
		Self {
			name,
			db_name,
			db_url,
		}
	}

	pub fn db_url(&self) -> &str {
		&self.db_url
	}

	pub async fn pool(&self) -> PgPool {
		PgPoolOptions::new()
			.max_connections(10)
			.connect(&self.db_url)
			.await
			.expect("connect owned postgres")
	}

	pub async fn migrate_to_head(&self) {
		let pool = self.pool().await;
		sqlx::migrate!("./migrations")
			.run(&pool)
			.await
			.expect("run migrations on owned postgres");
		pool.close().await;
	}

	pub async fn stop(&self) {
		docker(&["stop", &self.name])
			.await
			.expect("stop owned postgres");
	}

	pub async fn kill(&self) {
		docker(&["kill", &self.name])
			.await
			.expect("kill owned postgres");
	}

	pub async fn restart(&self) {
		docker(&["start", &self.name])
			.await
			.expect("restart owned postgres");
		wait_for_postgres(&self.db_url)
			.await
			.expect("postgres ready after restart");
	}
}

impl Drop for OwnedPostgres {
	fn drop(&mut self) {
		docker_rm_force(&self.name);
	}
}

impl OwnedRabbitMq {
	pub async fn start() -> Self {
		let name = format!("reacher-rabbit-{}", Uuid::new_v4().simple());
		let _startup_guard = RABBITMQ_START_LOCK.lock().await;
		let amqp_url = match start_rabbitmq_container(&name).await {
			Ok(amqp_url) => amqp_url,
			Err(err) => {
				docker_rm_force(&name);
				panic!("start owned rabbitmq: {}", err);
			}
		};
		Self { name, amqp_url }
	}

	pub fn amqp_url(&self) -> &str {
		&self.amqp_url
	}

	pub async fn stop(&self) {
		docker(&["stop", &self.name])
			.await
			.expect("stop owned rabbitmq");
	}

	pub async fn kill(&self) {
		docker(&["kill", &self.name])
			.await
			.expect("kill owned rabbitmq");
	}

	pub async fn restart(&self) {
		docker(&["start", &self.name])
			.await
			.expect("restart owned rabbitmq");
		wait_for_rabbitmq_container(&self.name)
			.await
			.expect("rabbitmq container healthy after restart");
		wait_for_rabbitmq(&self.amqp_url)
			.await
			.expect("rabbitmq ready after restart");
	}
}

impl Drop for OwnedRabbitMq {
	fn drop(&mut self) {
		docker_rm_force(&self.name);
	}
}

impl OwnedResilienceEnv {
	pub async fn start() -> Self {
		let postgres = OwnedPostgres::start().await;
		postgres.migrate_to_head().await;
		let rabbitmq = OwnedRabbitMq::start().await;
		Self { postgres, rabbitmq }
	}
}

async fn start_postgres_container(name: &str, db_name: &str) -> Result<String, String> {
	docker(&[
		"run",
		"-d",
		"--name",
		name,
		"-e",
		"POSTGRES_USER=postgres",
		"-e",
		"POSTGRES_PASSWORD=postgres",
		"-e",
		&format!("POSTGRES_DB={db_name}"),
		"-p",
		"127.0.0.1::5432",
		POSTGRES_IMAGE,
	])
	.await?;
	let host_port = docker_mapped_port(name, "5432/tcp").await?;
	let db_url = format!("postgres://postgres:postgres@127.0.0.1:{host_port}/{db_name}");
	wait_for_postgres(&db_url).await?;
	Ok(db_url)
}

async fn start_rabbitmq_container(name: &str) -> Result<String, String> {
	docker(&[
		"run",
		"-d",
		"--name",
		name,
		"-p",
		"127.0.0.1::5672",
		"--health-cmd",
		"rabbitmq-diagnostics -q ping",
		"--health-start-period",
		"30s",
		"--health-interval",
		"10s",
		"--health-timeout",
		"15s",
		"--health-retries",
		"30",
		RABBITMQ_IMAGE,
	])
	.await?;
	let host_port = docker_mapped_port(name, "5672/tcp").await?;
	let amqp_url = format!("amqp://guest:guest@127.0.0.1:{host_port}");
	wait_for_rabbitmq_container(name).await?;
	wait_for_rabbitmq(&amqp_url).await?;
	Ok(amqp_url)
}

async fn docker(args: &[&str]) -> Result<String, String> {
	let output = Command::new("docker")
		.args(args)
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.output()
		.await
		.map_err(|err| format!("failed to run docker {:?}: {err}", args))?;
	if !output.status.success() {
		return Err(format!(
			"docker {:?} failed: {}",
			args,
			String::from_utf8_lossy(&output.stderr)
		));
	}
	Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn docker_rm_force(container_name: &str) {
	let _ = StdCommand::new("docker")
		.args(["rm", "-f", container_name])
		.stdout(Stdio::null())
		.stderr(Stdio::null())
		.status();
}

async fn docker_mapped_port(container_name: &str, port: &str) -> Result<u16, String> {
	let output = docker(&["port", container_name, port]).await?;
	let first_line = output
		.lines()
		.next()
		.ok_or_else(|| format!("docker port returned no lines for {container_name} {port}"))?;
	let port = first_line
		.rsplit(':')
		.next()
		.ok_or_else(|| format!("failed to parse mapped port from '{first_line}'"))?;
	port.parse::<u16>()
		.map_err(|err| format!("invalid mapped port '{port}': {err}"))
}

async fn docker_logs_tail(container_name: &str) -> String {
	Command::new("docker")
		.args(["logs", "--tail", "80", container_name])
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.output()
		.await
		.map(|output| {
			let stdout = String::from_utf8_lossy(&output.stdout);
			let stderr = String::from_utf8_lossy(&output.stderr);
			format!("{stdout}{stderr}").trim().to_string()
		})
		.unwrap_or_else(|err| format!("failed to read docker logs: {err}"))
}

async fn wait_for_rabbitmq_container(container_name: &str) -> Result<(), String> {
	let deadline = Instant::now() + RABBITMQ_WAIT_TIMEOUT;
	loop {
		let status = docker(&[
			"inspect",
			"-f",
			"{{.State.Running}} {{if .State.Health}}{{.State.Health.Status}}{{else}}no-health{{end}}",
			container_name,
		])
		.await?;
		if status == "true healthy" || status == "true no-health" {
			return Ok(());
		}
		if status.starts_with("false") {
			let logs = docker_logs_tail(container_name).await;
			return Err(format!(
				"rabbitmq container {container_name} exited before readiness; status: {status}; logs:\n{logs}"
			));
		}
		if Instant::now() >= deadline {
			let logs = docker_logs_tail(container_name).await;
			return Err(format!(
				"rabbitmq container {container_name} was not healthy within {:?}; last status: {status}; logs:\n{logs}",
				RABBITMQ_WAIT_TIMEOUT
			));
		}
		sleep(POLL_INTERVAL).await;
	}
}

async fn wait_for_postgres(db_url: &str) -> Result<(), String> {
	let deadline = Instant::now() + POSTGRES_WAIT_TIMEOUT;
	loop {
		match PgPoolOptions::new()
			.max_connections(1)
			.acquire_timeout(Duration::from_secs(2))
			.connect(db_url)
			.await
		{
			Ok(pool) => {
				pool.close().await;
				return Ok(());
			}
			Err(err) if Instant::now() < deadline => {
				let _ = err;
				sleep(POLL_INTERVAL).await;
			}
			Err(err) => return Err(format!("postgres not ready at {db_url}: {err}")),
		}
	}
}

async fn wait_for_rabbitmq(amqp_url: &str) -> Result<(), String> {
	let deadline = Instant::now() + RABBITMQ_WAIT_TIMEOUT;
	loop {
		match timeout(
			RABBITMQ_CONNECT_ATTEMPT_TIMEOUT,
			Connection::connect(amqp_url, ConnectionProperties::default()),
		)
		.await
		{
			Ok(Ok(connection)) => {
				let _ = connection.close(0, "test cleanup").await;
				return Ok(());
			}
			Ok(Err(err)) if Instant::now() < deadline => {
				let _ = err;
				sleep(POLL_INTERVAL).await;
			}
			Err(_) if Instant::now() < deadline => {
				sleep(POLL_INTERVAL).await;
			}
			Ok(Err(err)) => {
				return Err(format!(
					"rabbitmq not ready at {amqp_url} within {:?}: {err}",
					RABBITMQ_WAIT_TIMEOUT
				));
			}
			Err(_) => {
				return Err(format!(
					"rabbitmq connection attempts to {amqp_url} timed out within {:?}",
					RABBITMQ_WAIT_TIMEOUT
				));
			}
		}
	}
}
