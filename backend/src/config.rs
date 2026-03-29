// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::bounce_risk::BounceRiskService;
use crate::storage::{postgres::PostgresStorage, StorageAdapter};
use crate::throttle::ThrottleManager;
use crate::worker::do_work::TaskWebhook;
use crate::worker::setup_rabbit_mq;
use anyhow::{bail, Context};
use check_if_email_exists::smtp::verif_method::{
	EverythingElseVerifMethod, GmailVerifMethod, HotmailB2BVerifMethod, HotmailB2CVerifMethod,
	MimecastVerifMethod, ProofpointVerifMethod, VerifMethod, VerifMethodSmtpConfig,
	YahooVerifMethod, DEFAULT_PROXY_ID,
};
use check_if_email_exists::{CheckEmailInputProxy, WebdriverConfig, LOG_TARGET};
use config::Config;
use dashmap::DashMap;
use lapin::Channel;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tracing::warn;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct BackendConfig {
	/// Name of the backend.
	pub backend_name: String,

	// Fields from VerifMethodSmtpConfig
	pub from_email: String,
	pub hello_name: String,
	/// Timeout for each SMTP connection, in seconds. Leaving it commented out
	/// will not set a timeout, i.e. the connection will wait indefinitely.
	pub smtp_timeout: Option<u64>,
	// This field is deprecated, but kept for backwards compatibility. If set,
	// it will be moved to the "default" proxy in the `verif_method.proxies`
	// field.
	pub proxy: Option<CheckEmailInputProxy>,

	/// Overrides over the default verification method provided above.
	pub overrides: OverridesConfig,

	/// Webdriver configuration.
	pub webdriver_addr: String,
	pub webdriver: WebdriverConfig,

	/** Backend-specific config*/
	/// Backend host
	pub http_host: String,
	/// Backend port
	pub http_port: u16,
	/// Shared secret between a trusted client and the backend.
	pub header_secret: Option<String>,
	/// Sentry DSN to report errors to
	pub sentry_dsn: Option<String>,

	/// Worker configuration, only present if the backend is a worker.
	pub worker: WorkerConfig,

	/// Configuration on where to store the email verification results.
	pub storage: Option<StorageConfig>,

	/// Whether to enable the Commercial License Trial. Setting this to true
	pub commercial_license_trial: Option<CommercialLicenseTrialConfig>,

	/// Throttle configuration for all requests
	pub throttle: ThrottleConfig,

	/// Scheduled re-verification configuration
	#[serde(default)]
	pub reverification: ReverificationConfig,

	/// Scheduled pipeline configuration
	#[serde(default)]
	pub pipelines: PipelinesConfig,
	/// Bounce-risk enrichment configuration.
	#[serde(default)]
	pub bounce_risk: BounceRiskConfig,

	// Internal fields, not part of the configuration.
	#[serde(skip)]
	channel: Option<Arc<Channel>>,

	#[serde(skip)]
	storage_adapter: Arc<StorageAdapter>,

	#[serde(skip)]
	throttle_manager: Arc<ThrottleManager>,

	/// Per-tenant throttle managers, keyed by tenant UUID.
	/// Legacy/open-mode callers use the global `throttle_manager` above.
	#[serde(skip)]
	tenant_throttle_managers: Arc<DashMap<Uuid, Arc<ThrottleManager>>>,

	#[serde(skip)]
	bounce_risk_service: Arc<BounceRiskService>,
}

impl BackendConfig {
	/// Create an empty BackendConfig. This is useful for testing purposes.
	pub fn empty() -> Self {
		Self {
			backend_name: "".to_string(),
			webdriver_addr: "".to_string(),
			webdriver: WebdriverConfig::default(),
			from_email: "".to_string(),
			hello_name: "".to_string(),
			smtp_timeout: None,
			proxy: None,
			overrides: OverridesConfig::default(),
			http_host: "127.0.0.1".to_string(),
			http_port: 8080,
			header_secret: None,
			sentry_dsn: None,
			worker: WorkerConfig::default(),
			storage: Some(StorageConfig::Noop),
			commercial_license_trial: None,
			throttle: ThrottleConfig::new_without_throttle(),
			reverification: ReverificationConfig::default(),
			pipelines: PipelinesConfig::default(),
			bounce_risk: BounceRiskConfig::default(),
			channel: None,
			storage_adapter: Arc::new(StorageAdapter::Noop),
			throttle_manager: Arc::new(
				ThrottleManager::new(ThrottleConfig::new_without_throttle()),
			),
			tenant_throttle_managers: Arc::new(DashMap::new()),
			bounce_risk_service: Arc::new(BounceRiskService::default()),
		}
	}

	pub fn get_verif_method(&self) -> VerifMethod {
		let mut proxies = self.overrides.proxies.clone();
		if let Some(proxy) = self.proxy.as_ref() {
			proxies.insert(DEFAULT_PROXY_ID.to_string(), proxy.clone());
		}

		let default_smtp_config = VerifMethodSmtpConfig {
			from_email: self.from_email.clone(),
			hello_name: self.hello_name.clone(),
			proxy: self.proxy.as_ref().map(|_| DEFAULT_PROXY_ID.to_string()),
			smtp_timeout: self.smtp_timeout.map(Duration::from_secs),
			..Default::default()
		};

		VerifMethod {
			proxies,
			gmail: self
				.overrides
				.gmail
				.clone()
				.unwrap_or_else(|| GmailVerifMethod::Smtp(default_smtp_config.clone())),
			hotmailb2b: self
				.overrides
				.hotmailb2b
				.clone()
				.unwrap_or_else(|| HotmailB2BVerifMethod::Smtp(default_smtp_config.clone())),
			hotmailb2c: self
				.overrides
				.hotmailb2c
				.clone()
				.unwrap_or(HotmailB2CVerifMethod::Headless),
			mimecast: self
				.overrides
				.mimecast
				.clone()
				.unwrap_or_else(|| MimecastVerifMethod::Smtp(default_smtp_config.clone())),
			proofpoint: self
				.overrides
				.proofpoint
				.clone()
				.unwrap_or_else(|| ProofpointVerifMethod::Smtp(default_smtp_config.clone())),
			yahoo: self
				.overrides
				.yahoo
				.clone()
				.unwrap_or(YahooVerifMethod::Headless),
			everything_else: EverythingElseVerifMethod::Smtp(default_smtp_config),
		}
	}

	/// Get the worker configuration.
	///
	/// # Panics
	///
	/// Panics if the worker configuration is missing.
	pub fn must_worker_config(&self) -> Result<MustWorkerConfig, anyhow::Error> {
		match (self.worker.enable, &self.worker.rabbitmq, &self.channel) {
			(true, Some(rabbitmq), Some(channel)) => Ok(MustWorkerConfig {
				channel: channel.clone(),
				rabbitmq: rabbitmq.clone(),
				webhook: self.worker.webhook.clone(),
			}),

			(true, _, _) => bail!("Worker configuration is missing"),
			_ => bail!("Calling must_worker_config on a non-worker backend"),
		}
	}

	/// Attempt connection to the Postgres database and RabbitMQ. Also populates
	/// the internal `pg_pool` and `channel` fields with the connections.
	pub async fn connect(&mut self) -> Result<(), anyhow::Error> {
		match &self.storage {
			Some(StorageConfig::Postgres(config)) => {
				let storage = PostgresStorage::new(
					&config.db_url,
					config.read_replica_url.as_deref(),
					config.extra.clone(),
				)
				.await
				.with_context(|| format!("Connecting to postgres DB {}", config.db_url))?;

				self.storage_adapter = Arc::new(StorageAdapter::Postgres(storage));
			}
			_ => {
				self.storage_adapter = Arc::new(StorageAdapter::Noop);
			}
		}

		let channel = if self.worker.enable {
			let rabbitmq_config = self.worker.rabbitmq.as_ref().ok_or_else(|| {
				anyhow::anyhow!("Worker configuration is missing the rabbitmq configuration")
			})?;
			let channel = setup_rabbit_mq(&self.backend_name, rabbitmq_config).await?;
			Some(Arc::new(channel))
		} else {
			None
		};
		self.channel = channel;

		// Initialize throttle manager
		self.throttle_manager = Arc::new(ThrottleManager::new(self.throttle.clone()));
		self.refresh_bounce_risk_service();

		Ok(())
	}

	/// Get the Postgres connection pool, if the storage is Postgres.
	pub fn get_storage_adapter(&self) -> Arc<StorageAdapter> {
		self.storage_adapter.clone()
	}

	/// Get the Postgres connection pool, if the storage is Postgres.
	pub fn get_pg_pool(&self) -> Option<PgPool> {
		match self.storage_adapter.as_ref() {
			StorageAdapter::Postgres(storage) => Some(storage.pg_pool.clone()),
			StorageAdapter::Noop => None,
		}
	}

	/// Get the read-only Postgres connection pool (replica if configured, primary otherwise).
	pub fn get_read_pg_pool(&self) -> Option<PgPool> {
		match self.storage_adapter.as_ref() {
			StorageAdapter::Postgres(storage) => Some(storage.read_pool.clone()),
			StorageAdapter::Noop => None,
		}
	}

	pub fn get_throttle_manager(&self) -> Arc<ThrottleManager> {
		self.throttle_manager.clone()
	}

	pub fn get_bounce_risk_service(&self) -> Arc<BounceRiskService> {
		self.bounce_risk_service.clone()
	}

	pub fn refresh_bounce_risk_service(&mut self) {
		self.bounce_risk_service = Arc::new(BounceRiskService::new(self.bounce_risk.clone()));
	}

	/// Get or create a per-tenant ThrottleManager. If the tenant_id is None
	/// (legacy mode), returns the global throttle manager.
	pub fn get_tenant_throttle_manager(
		&self,
		tenant_id: Option<Uuid>,
		tenant_throttle_config: &ThrottleConfig,
	) -> Arc<ThrottleManager> {
		match tenant_id {
			Some(id) => self
				.tenant_throttle_managers
				.entry(id)
				.or_insert_with(|| Arc::new(ThrottleManager::new(tenant_throttle_config.clone())))
				.clone(),
			None => self.throttle_manager.clone(),
		}
	}
}

#[derive(Debug, Default, Deserialize, Clone, Serialize)]
pub struct OverridesConfig {
	pub proxies: HashMap<String, CheckEmailInputProxy>,
	pub gmail: Option<GmailVerifMethod>,
	pub hotmailb2b: Option<HotmailB2BVerifMethod>,
	pub hotmailb2c: Option<HotmailB2CVerifMethod>,
	pub mimecast: Option<MimecastVerifMethod>,
	pub proofpoint: Option<ProofpointVerifMethod>,
	pub yahoo: Option<YahooVerifMethod>,
}

#[derive(Debug, Default, Deserialize, Clone, Serialize)]
pub struct WorkerConfig {
	pub enable: bool,
	pub rabbitmq: Option<RabbitMQConfig>,
	/// Optional webhook configuration to send email verification results.
	pub webhook: Option<TaskWebhook>,
}

/// Worker configuration that must be present if worker.enable is true. Used as
/// a domain type to ensure that the worker configuration is present.
#[derive(Debug, Clone)]
pub struct MustWorkerConfig {
	pub channel: Arc<Channel>,
	pub rabbitmq: RabbitMQConfig,
	pub webhook: Option<TaskWebhook>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RabbitMQConfig {
	pub url: String,
	/// Total number of concurrent messages that the worker can process.
	pub concurrency: u16,
}

#[derive(Debug, Default, Deserialize, Clone, Serialize)]
pub struct ThrottleConfig {
	pub max_requests_per_second: Option<u32>,
	pub max_requests_per_minute: Option<u32>,
	pub max_requests_per_hour: Option<u32>,
	pub max_requests_per_day: Option<u32>,
}

impl ThrottleConfig {
	/// Create a new ThrottleConfig with no throttling.
	pub fn new_without_throttle() -> Self {
		Self {
			max_requests_per_second: None,
			max_requests_per_minute: None,
			max_requests_per_hour: None,
			max_requests_per_day: None,
		}
	}
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ReverificationConfig {
	#[serde(default)]
	pub enable: bool,
	#[serde(default = "default_reverification_interval")]
	pub interval_seconds: u64,
	#[serde(default = "default_staleness_days")]
	pub default_staleness_days: i32,
	#[serde(default = "default_reverification_batch_size")]
	pub default_batch_size: i32,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct PipelinesConfig {
	#[serde(default)]
	pub enable: bool,
	#[serde(default = "default_pipeline_tick_seconds")]
	pub tick_seconds: u64,
	#[serde(default = "default_pipeline_max_due_per_tick")]
	pub max_due_per_tick: i32,
	#[serde(default = "default_pipeline_max_missed_hours")]
	pub max_missed_run_age_hours: i32,
	#[serde(default = "default_pipeline_min_interval_seconds")]
	pub min_interval_seconds: i32,
}

fn default_pipeline_tick_seconds() -> u64 {
	60
}
fn default_pipeline_max_due_per_tick() -> i32 {
	25
}
fn default_pipeline_max_missed_hours() -> i32 {
	24
}
fn default_pipeline_min_interval_seconds() -> i32 {
	3600
}

impl Default for PipelinesConfig {
	fn default() -> Self {
		Self {
			enable: false,
			tick_seconds: default_pipeline_tick_seconds(),
			max_due_per_tick: default_pipeline_max_due_per_tick(),
			max_missed_run_age_hours: default_pipeline_max_missed_hours(),
			min_interval_seconds: default_pipeline_min_interval_seconds(),
		}
	}
}

fn default_reverification_interval() -> u64 {
	3600
}
fn default_staleness_days() -> i32 {
	30
}
fn default_reverification_batch_size() -> i32 {
	100
}

impl Default for ReverificationConfig {
	fn default() -> Self {
		Self {
			enable: false,
			interval_seconds: default_reverification_interval(),
			default_staleness_days: default_staleness_days(),
			default_batch_size: default_reverification_batch_size(),
		}
	}
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct BounceRiskConfig {
	#[serde(default = "default_bounce_risk_enabled")]
	pub enabled: bool,
	#[serde(default = "default_bounce_risk_config_path")]
	pub config_path: String,
	#[serde(default = "default_bounce_risk_reload_interval_seconds")]
	pub reload_interval_seconds: u64,
	#[serde(default = "default_bounce_risk_history_limit")]
	pub history_limit: u32,
	#[serde(default = "default_bounce_risk_network_timeout_ms")]
	pub network_timeout_ms: u64,
	#[serde(default = "default_bounce_risk_website_probe_timeout_ms")]
	pub website_probe_timeout_ms: u64,
}

fn default_bounce_risk_enabled() -> bool {
	false
}

fn default_bounce_risk_config_path() -> String {
	"bounce_risk.toml".to_string()
}

fn default_bounce_risk_reload_interval_seconds() -> u64 {
	30
}

fn default_bounce_risk_history_limit() -> u32 {
	10
}

fn default_bounce_risk_network_timeout_ms() -> u64 {
	5_000
}

fn default_bounce_risk_website_probe_timeout_ms() -> u64 {
	3_000
}

impl Default for BounceRiskConfig {
	fn default() -> Self {
		Self {
			enabled: default_bounce_risk_enabled(),
			config_path: default_bounce_risk_config_path(),
			reload_interval_seconds: default_bounce_risk_reload_interval_seconds(),
			history_limit: default_bounce_risk_history_limit(),
			network_timeout_ms: default_bounce_risk_network_timeout_ms(),
			website_probe_timeout_ms: default_bounce_risk_website_probe_timeout_ms(),
		}
	}
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageConfig {
	/// Store the email verification results in the Postgres database.
	Postgres(PostgresConfig),
	/// Don't store the email verification results.
	#[default]
	Noop,
}

#[derive(Debug, Default, Deserialize, Clone, PartialEq, Serialize)]
pub struct PostgresConfig {
	pub db_url: String,
	#[serde(default)]
	pub read_replica_url: Option<String>,
	pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CommercialLicenseTrialConfig {
	pub api_token: String,
	pub url: String,
}

/// Load the worker configuration from the worker_config.toml file and from the
/// environment.
pub async fn load_config() -> Result<BackendConfig, anyhow::Error> {
	let cfg = Config::builder()
		.add_source(config::File::with_name("backend_config"))
		.add_source(config::Environment::with_prefix("RCH").separator("__"));

	let mut cfg = cfg.build()?.try_deserialize::<BackendConfig>()?;
	cfg.refresh_bounce_risk_service();

	// Perform additional checks

	// 1. Make sure that if the worker is enabled, a Postgres database is configured.
	if cfg.worker.enable {
		warn!(target: LOG_TARGET, "The worker feature is currently in beta. Please send any feedback to amaury@reacher.email.");

		match &cfg.storage {
			Some(StorageConfig::Postgres(_)) => {}
			_ => bail!("When worker mode is enabled, a Postgres database must be configured."),
		}
	}

	// 2. Validate the verif_method proxies, meaning that for each email
	// provider's verification method, the proxy (if set) must exist in the
	// `proxies` field.
	cfg.get_verif_method().validate_proxies()?;

	Ok(cfg)
}

#[cfg(test)]
mod tests {
	use super::*;
	use serial_test::serial;
	use std::{env, time::Duration};
	use {
		EverythingElseVerifMethod, GmailVerifMethod, HotmailB2BVerifMethod, VerifMethodSmtpConfig,
	};

	#[tokio::test]
	#[serial]
	async fn test_proxies() {
		env::set_var("RCH__OVERRIDES__PROXIES__PROXY3__HOST", "test-proxy");
		env::set_var("RCH__OVERRIDES__PROXIES__PROXY3__PORT", "1234");
		let cfg = load_config().await.unwrap();
		// Proxies
		assert_eq!(cfg.get_verif_method().proxies.len(), 1);
		assert_eq!(
			cfg.get_verif_method().proxies.get("proxy3").unwrap().host,
			"test-proxy"
		);
		assert_eq!(
			cfg.get_verif_method().proxies.get("proxy3").unwrap().port,
			1234
		);

		env::remove_var("RCH__OVERRIDES__PROXIES__PROXY3__HOST");
		env::remove_var("RCH__OVERRIDES__PROXIES__PROXY3__PORT");
	}

	#[tokio::test]
	#[serial]
	async fn test_default_proxy() {
		env::set_var("RCH__PROXY__HOST", "test-default-proxy");
		env::set_var("RCH__PROXY__PORT", "5678");
		let cfg = load_config().await.unwrap();
		// Proxies
		assert_eq!(cfg.get_verif_method().proxies.len(), 1);
		assert_eq!(
			cfg.get_verif_method()
				.proxies
				.get(DEFAULT_PROXY_ID)
				.unwrap()
				.host,
			"test-default-proxy"
		);
		assert_eq!(
			cfg.get_verif_method()
				.proxies
				.get(DEFAULT_PROXY_ID)
				.unwrap()
				.port,
			5678
		);

		env::remove_var("RCH__PROXY__HOST");
		env::remove_var("RCH__PROXY__PORT");
	}

	#[test]
	fn test_deserialize_verif_method() {
		let toml = r#"
[proxies]
# Allow inline
proxy1 = { host = "my-proxy1", port = 1051, username = "user1", password = "pass1" }
[proxies.proxy2]
host = "my-proxy2"
port = 1052
username = "user2"
password = "pass2"

[hotmailb2c]
type = "headless"

[yahoo]
type = "headless"

[gmail]
type = "smtp"
from_email = "from@email.com"
hello_name = "email.com"
proxy = "proxy1"
smtp_port = 465
retries = 3
smtp_timeout = { secs = 23, nanos = 0 }

# Allow skipping internal fields
[hotmailb2b]
type = "smtp"

# Allow skipping whole section
# [everything_else.Smtp]
"#;

		let verif_method: VerifMethod = toml::from_str(toml).unwrap();
		assert_eq!(verif_method.proxies.len(), 2);
		assert_eq!(
			verif_method.proxies.get("proxy1").unwrap().host,
			"my-proxy1"
		);
		assert_eq!(verif_method.proxies.get("proxy1").unwrap().port, 1051);
		assert_eq!(
			verif_method
				.proxies
				.get("proxy1")
				.unwrap()
				.username
				.as_deref(),
			Some("user1")
		);
		assert_eq!(
			verif_method
				.proxies
				.get("proxy1")
				.unwrap()
				.password
				.as_deref(),
			Some("pass1")
		);

		assert_eq!(
			verif_method.proxies.get("proxy2").unwrap().host,
			"my-proxy2"
		);
		assert_eq!(verif_method.proxies.get("proxy2").unwrap().port, 1052);
		assert_eq!(
			verif_method
				.proxies
				.get("proxy2")
				.unwrap()
				.username
				.as_deref(),
			Some("user2")
		);
		assert_eq!(
			verif_method
				.proxies
				.get("proxy2")
				.unwrap()
				.password
				.as_deref(),
			Some("pass2")
		);

		assert_eq!(
			verif_method.gmail,
			GmailVerifMethod::Smtp(VerifMethodSmtpConfig {
				from_email: "from@email.com".to_string(),
				hello_name: "email.com".to_string(),
				smtp_port: 465,
				retries: 3,
				proxy: Some("proxy1".to_string()),
				smtp_timeout: Some(Duration::from_secs(23)),
			})
		);

		assert_eq!(
			verif_method.hotmailb2b,
			HotmailB2BVerifMethod::Smtp(VerifMethodSmtpConfig {
				from_email: "reacher@gmail.com".to_string(),
				hello_name: "gmail.com".to_string(),
				smtp_port: 25,
				retries: 1,
				proxy: None,
				smtp_timeout: None,
			})
		);

		assert_eq!(
			verif_method.everything_else,
			EverythingElseVerifMethod::Smtp(Default::default())
		);
	}

	#[tokio::test]
	async fn test_env_vars() {
		env::set_var("RCH__BACKEND_NAME", "test-backend");
		env::set_var("RCH__STORAGE__POSTGRES__DB_URL", "test2");
		let cfg = load_config().await.unwrap();
		assert_eq!(cfg.backend_name, "test-backend");
		assert_eq!(
			cfg.storage,
			Some(StorageConfig::Postgres(PostgresConfig {
				db_url: "test2".to_string(),
				read_replica_url: None,
				extra: None,
			}))
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_bounce_risk_env_vars() {
		env::set_var("RCH__BOUNCE_RISK__ENABLED", "true");
		env::set_var(
			"RCH__BOUNCE_RISK__CONFIG_PATH",
			"/tmp/reacher-bounce-risk.toml",
		);
		let cfg = load_config().await.unwrap();
		assert!(cfg.bounce_risk.enabled);
		assert_eq!(
			cfg.bounce_risk.config_path,
			"/tmp/reacher-bounce-risk.toml".to_string()
		);
		env::remove_var("RCH__BOUNCE_RISK__ENABLED");
		env::remove_var("RCH__BOUNCE_RISK__CONFIG_PATH");
	}

	#[tokio::test]
	async fn test_serialize_storage_config() {
		let storage_config = StorageConfig::Postgres(PostgresConfig {
			db_url: "postgres://localhost:5432/test1".to_string(),
			read_replica_url: None,
			extra: None,
		});

		let expected = r#"[postgres]
db_url = "postgres://localhost:5432/test1"
"#;

		assert_eq!(expected, toml::to_string(&storage_config).unwrap(),);
	}
}
