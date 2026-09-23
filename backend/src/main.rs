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

//! Main entry point of the `reacher_backend` binary. It has two `main`
//! functions, depending on whether the `bulk` feature is enabled or not.

use check_if_email_exists::{initialize_crypto_provider, setup_sentry, LOG_TARGET};
use reacher_backend::config::load_config;
use reacher_backend::http::run_warp_server;
use reacher_backend::worker::run_worker;
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{interval_at, Instant, MissedTickBehavior};
use tracing::{debug, info, warn};

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

async fn prune_expired_results(pool: &PgPool) -> Result<(), sqlx::Error> {
	// ponytail: Delete at most 1000 rows per table each hour; increase the batch if a backlog grows.
	sqlx::query(
		"DELETE FROM v1_finder_job WHERE id IN (\
		 SELECT f.id FROM v1_finder_job f JOIN tenants t ON t.id = f.tenant_id \
		 WHERE f.status IN ('completed', 'failed', 'cancelled') \
		 AND COALESCE(f.completed_at, f.created_at) < NOW() - t.result_retention_days * INTERVAL '1 day' \
		 ORDER BY f.id LIMIT 1000)",
	)
	.execute(pool)
	.await?;
	sqlx::query(
		"DELETE FROM v1_task_result WHERE id IN (\
		 SELECT r.id FROM v1_task_result r \
		 LEFT JOIN v1_bulk_job j ON j.id = r.job_id \
		 JOIN tenants t ON t.id = COALESCE(r.tenant_id, j.tenant_id) \
		 WHERE (r.task_state IN ('completed', 'failed', 'cancelled', 'dead_lettered') \
		 OR r.result IS NOT NULL OR r.error IS NOT NULL) \
		 AND COALESCE(r.completed_at, r.created_at) < NOW() - t.result_retention_days * INTERVAL '1 day' \
		 ORDER BY r.id LIMIT 1000)",
	)
	.execute(pool)
	.await?;
	Ok(())
}

/// Run a HTTP server using warp with bulk endpoints.
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	// Initialize logging.
	tracing_subscriber::fmt::init();

	// Initialize the crypto provider for TLS connections (required before any TLS usage).
	initialize_crypto_provider();

	info!(target: LOG_TARGET, version=?CARGO_PKG_VERSION, "Running Reacher");
	let mut config = load_config().await?;
	config.connect().await?;

	// SECURITY: Don't log full config as it likely contains secrets (DB_URL, etc)
	debug!(target: LOG_TARGET, "{:#?}", config.get_verif_method());

	// Setup sentry bug tracking.
	let _guard: sentry::ClientInitGuard;
	if let Some(sentry_config) = &config.sentry_dsn {
		_guard = setup_sentry(sentry_config);
	}

	let config = Arc::new(config);
	if let Some(pool) = config.get_pg_pool() {
		tokio::spawn(async move {
			let period = Duration::from_secs(3600);
			let mut timer = interval_at(Instant::now() + period, period);
			timer.set_missed_tick_behavior(MissedTickBehavior::Skip);
			loop {
				timer.tick().await;
				if let Err(error) = prune_expired_results(&pool).await {
					warn!(error = ?error, "Failed to prune expired tenant results");
				}
			}
		});
	}

	let server_future = run_warp_server(Arc::clone(&config));
	let worker_future = async {
		if config.worker.enable {
			run_worker(config).await?;
		}
		Ok(())
	};

	tokio::try_join!(server_future, worker_future)?;

	info!("Shutting down...");

	Ok(())
}
