mod test_helpers;
use lapin::{options::*, types::FieldTable};
use reacher_backend::config::{BackendConfig, RabbitMQConfig};
use reacher_backend::worker::consume::{run_worker, CHECK_EMAIL_QUEUE};
use std::sync::Arc;
use testcontainers::{core::IntoContainerPort, runners::AsyncRunner, ImageExt};
use testcontainers_modules::rabbitmq::RabbitMq;
use tokio::time::{sleep, timeout, Duration};

async fn connect(url: &str, db_url: &str) -> Arc<BackendConfig> {
	let mut config = BackendConfig::empty();
	config.backend_name = "recovery-test".into();
	config.worker.enable = true;
	config.smtp_health_enabled = false;
	config.storage = Some(reacher_backend::config::StorageConfig::Postgres(
		reacher_backend::config::PostgresConfig {
			db_url: db_url.into(),
			read_replica_url: None,
			extra: None,
		},
	));
	config.worker.rabbitmq = Some(RabbitMQConfig {
		queue: "check_email".into(),
		url: url.into(),
		concurrency: 1,
	});
	config
		.connect()
		.await
		.expect("API startup tolerates a broker outage");
	Arc::new(config)
}

async fn wait_for_consumer(config: &BackendConfig) {
	timeout(Duration::from_secs(90), async {
		loop {
			if let Ok(worker) = config.must_worker_config() {
				if let Ok(queue) = worker
					.channel
					.queue_declare(
						CHECK_EMAIL_QUEUE.into(),
						QueueDeclareOptions {
							passive: true,
							..Default::default()
						},
						FieldTable::default(),
					)
					.await
				{
					if queue.consumer_count() == 1 {
						break;
					}
				}
			}
			sleep(Duration::from_millis(100)).await;
		}
	})
	.await
	.expect("worker must register a consumer after recovery");
}

#[tokio::test]
async fn worker_survives_startup_outage_connection_loss_and_consumer_cancellation() {
	// This broker belongs only to this test. Never stop shared infrastructure.
	let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
	let fixed_port = listener.local_addr().unwrap().port();
	drop(listener);
	let broker = RabbitMq::default()
		.with_mapped_port(fixed_port, 5672.tcp())
		.start()
		.await
		.unwrap();
	let port = broker.get_host_port_ipv4(5672).await.unwrap();
	let url = format!("amqp://guest:guest@127.0.0.1:{port}/%2f?heartbeat=2");
	broker.stop().await.unwrap();
	let db = test_helpers::TestDb::start().await;
	let config = connect(&url, db.db_url()).await;
	assert!(config.must_worker_config().is_err());
	let readiness = reacher_backend::http::create_routes(config.clone());
	assert_eq!(
		warp::test::request()
			.path("/readyz")
			.reply(&readiness)
			.await
			.status(),
		503
	);
	let worker = tokio::spawn(run_worker(config.clone()));
	broker.start().await.unwrap();
	wait_for_consumer(&config).await;
	assert_eq!(
		warp::test::request()
			.path("/readyz")
			.reply(&readiness)
			.await
			.status(),
		200
	);
	broker.stop().await.unwrap();
	timeout(Duration::from_secs(15), async {
		loop {
			if config
				.must_worker_config()
				.map(|w| !w.channel.status().connected())
				.unwrap_or(true)
			{
				break;
			}
			sleep(Duration::from_millis(100)).await;
		}
	})
	.await
	.unwrap();
	assert!(
		!worker.is_finished(),
		"connection loss must not exit the combined API process"
	);
	assert_eq!(
		warp::test::request()
			.path("/readyz")
			.reply(&readiness)
			.await
			.status(),
		503
	);
	broker.start().await.unwrap();
	wait_for_consumer(&config).await;
	let channel = config.must_worker_config().unwrap().channel;
	channel
		.basic_cancel(
			"recovery-test-check_email".into(),
			BasicCancelOptions::default(),
		)
		.await
		.unwrap();
	sleep(Duration::from_millis(100)).await;
	wait_for_consumer(&config).await;
	assert!(!worker.is_finished());
	worker.abort();
	let _ = worker.await;
}
