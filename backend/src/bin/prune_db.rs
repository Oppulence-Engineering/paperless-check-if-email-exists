use sqlx::PgPool;
use sqlx::Result;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
	// A .env file is optional: containers pass the variables directly.
	dotenv::dotenv().ok();
	tracing_subscriber::fmt::init();

	let db_url = std::env::var("DATABASE_URL").expect("Unable to read DATABASE_URL env var");
	let dry_mode: bool = std::env::var("DRY_RUN").is_ok();
	let days_old_str = std::env::var("DAYS_OLD").expect("Unable to read DAYS_OLD env var");
	let days_old: i32 = days_old_str
		.parse()
		.expect("Unable to parse DAYS_OLD as integer");

	let pool = PgPool::connect(&db_url).await?;

	// Fetch the list of job IDs that match the criteria
	let query = format!(
		"SELECT b.id
        FROM bulk_jobs b
        JOIN (
            SELECT job_id, COUNT(*) as total_processed
            FROM email_results
            GROUP BY job_id
        ) e ON b.id = e.job_id
        WHERE b.total_records = e.total_processed
        AND b.created_at <= current_date - interval '{days_old} days'"
	);

	let job_ids_to_delete: Vec<(i32,)> = sqlx::query_as(&query).fetch_all(&pool).await?;

	match (dry_mode, job_ids_to_delete.is_empty()) {
		(true, _) => info!("Job ids to delete {:?}", job_ids_to_delete),
		(false, true) => info!("No jobs to delete"),
		(false, false) => {
			let ids: Vec<i32> = job_ids_to_delete.iter().map(|&(id,)| id).collect();
			prune_jobs(&pool, &ids).await?;
			info!("Deleted jobs and results for IDs {:?}", ids);
		}
	}

	Ok(())
}

pub async fn prune_jobs(pool: &PgPool, ids: &[i32]) -> Result<()> {
	let mut tx = pool.begin().await?;
	sqlx::query("DELETE FROM email_results WHERE job_id = ANY($1::int[])")
		.bind(ids)
		.execute(&mut *tx)
		.await?;
	sqlx::query("DELETE FROM bulk_jobs WHERE id = ANY($1::int[])")
		.bind(ids)
		.execute(&mut *tx)
		.await?;
	tx.commit().await
}
