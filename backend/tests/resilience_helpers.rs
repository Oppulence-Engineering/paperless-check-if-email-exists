#![allow(dead_code)]

use lapin::{Connection, ConnectionProperties};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::process::Stdio;
use tokio::process::Command;
use tokio::time::{sleep, Duration, Instant};
use uuid::Uuid;

const POSTGRES_IMAGE: &str = "postgres:16";
const RABBITMQ_IMAGE: &str = "rabbitmq:3.8.22-management";
const WAIT_TIMEOUT: Duration = Duration::from_secs(60);
const POLL_INTERVAL: Duration = Duration::from_millis(500);

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
		docker(&[
			"run",
			"-d",
			"--name",
			&name,
			"-e",
			"POSTGRES_USER=postgres",
			"-e",
			"POSTGRES_PASSWORD=postgres",
			"-e",
			&format!("POSTGRES_DB={db_name}"),
			"-P",
			POSTGRES_IMAGE,
		])
		.await
		.expect("start owned postgres");
		let host_port = docker_mapped_port(&name, "5432/tcp")
			.await
			.expect("postgres mapped port");
		let db_url = format!("postgres://postgres:postgres@127.0.0.1:{host_port}/{db_name}");
		wait_for_postgres(&db_url).await.expect("postgres ready");
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
		let name = self.name.clone();
		if let Ok(handle) = tokio::runtime::Handle::try_current() {
			handle.spawn(async move {
				let _ = docker(&["rm", "-f", &name]).await;
			});
		}
	}
}

impl OwnedRabbitMq {
	pub async fn start() -> Self {
		let name = format!("reacher-rabbit-{}", Uuid::new_v4().simple());
		docker(&["run", "-d", "--name", &name, "-P", RABBITMQ_IMAGE])
			.await
			.expect("start owned rabbitmq");
		let host_port = docker_mapped_port(&name, "5672/tcp")
			.await
			.expect("rabbitmq mapped port");
		let amqp_url = format!("amqp://guest:guest@127.0.0.1:{host_port}");
		wait_for_rabbitmq(&amqp_url).await.expect("rabbitmq ready");
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
		wait_for_rabbitmq(&self.amqp_url)
			.await
			.expect("rabbitmq ready after restart");
	}
}

impl Drop for OwnedRabbitMq {
	fn drop(&mut self) {
		let name = self.name.clone();
		if let Ok(handle) = tokio::runtime::Handle::try_current() {
			handle.spawn(async move {
				let _ = docker(&["rm", "-f", &name]).await;
			});
		}
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

async fn wait_for_postgres(db_url: &str) -> Result<(), String> {
	let deadline = Instant::now() + WAIT_TIMEOUT;
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
	let deadline = Instant::now() + WAIT_TIMEOUT;
	loop {
		match Connection::connect(amqp_url, ConnectionProperties::default()).await {
			Ok(connection) => {
				let _ = connection.close(0, "test cleanup").await;
				return Ok(());
			}
			Err(err) if Instant::now() < deadline => {
				let _ = err;
				sleep(POLL_INTERVAL).await;
			}
			Err(err) => return Err(format!("rabbitmq not ready at {amqp_url}: {err}")),
		}
	}
}
