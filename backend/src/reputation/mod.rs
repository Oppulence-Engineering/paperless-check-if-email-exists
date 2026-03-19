pub mod checker;
pub mod dns_records;
pub mod dnsbl;
pub mod models;
pub mod scorer;

use sqlx::PgPool;
use std::time::Duration;
use tracing::warn;

pub fn spawn_cache_cleanup(pg_pool: PgPool) {
	tokio::spawn(async move {
		loop {
			tokio::time::sleep(Duration::from_secs(60 * 60)).await;
			if let Err(err) =
				sqlx::query("DELETE FROM reputation_cache WHERE expires_at < NOW()")
					.execute(&pg_pool)
					.await
			{
				warn!(error = ?err, "Failed to cleanup reputation cache");
			}
		}
	});
}
