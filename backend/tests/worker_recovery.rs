use lapin::{options::*, types::FieldTable};
use reacher_backend::config::{BackendConfig, RabbitMQConfig};
use reacher_backend::worker::consume::{run_worker, CHECK_EMAIL_QUEUE};
use std::sync::Arc;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::rabbitmq::RabbitMq;
use tokio::time::{sleep, timeout, Duration};

async fn connect(url: &str) -> Arc<BackendConfig> {
	let mut config = BackendConfig::empty();
	config.backend_name = "recovery-test".into();
	config.worker.enable = true;
	config.worker.rabbitmq = Some(RabbitMQConfig {
		url: url.into(),
		concurrency: 1,
	});
	// ContainerAsync::start returns before the restarted broker is listening.
	timeout(Duration::from_secs(30), async {
		loop {
			if config.connect().await.is_ok() {
				break;
			}
			sleep(Duration::from_millis(100)).await;
		}
	})
	.await
	.expect("test broker must become available");
	Arc::new(config)
}

async fn wait_for_consumer(config: &BackendConfig) {
	let channel = config.must_worker_config().unwrap().channel;
	timeout(Duration::from_secs(10), async {
		loop {
			let queue = channel
				.queue_declare(
					CHECK_EMAIL_QUEUE,
					QueueDeclareOptions {
						passive: true,
						..Default::default()
					},
					FieldTable::default(),
				)
				.await
				.unwrap();
			if queue.consumer_count() == 1 {
				break;
			}
			sleep(Duration::from_millis(50)).await;
		}
	})
	.await
	.expect("worker must register a consumer");
}

#[tokio::test]
async fn worker_reports_broker_loss_and_can_restart() {
	// This broker belongs only to this test. Never stop shared test infrastructure.
	let broker = RabbitMq::default().start().await.unwrap();
	let port = broker.get_host_port_ipv4(5672).await.unwrap();
	let url = format!("amqp://guest:guest@127.0.0.1:{port}/%2f?heartbeat=2");
	let config = connect(&url).await;
	let worker = tokio::spawn(run_worker(config.clone()));
	wait_for_consumer(&config).await;
	assert!(
		!worker.is_finished(),
		"worker must remain supervised while idle"
	);

	broker.stop().await.unwrap();
	let result = timeout(Duration::from_secs(15), worker)
		.await
		.expect("broker loss must terminate the worker")
		.unwrap();
	assert!(
		result.is_err(),
		"worker failure must reach the process supervisor"
	);

	broker.start().await.unwrap();
	let config = connect(&url).await;
	let worker = tokio::spawn(run_worker(config.clone()));
	wait_for_consumer(&config).await;
	let channel = config.must_worker_config().unwrap().channel;
	channel
		.basic_cancel("recovery-test-check_email", BasicCancelOptions::default())
		.await
		.unwrap();
	let result = timeout(Duration::from_secs(5), worker)
		.await
		.unwrap()
		.unwrap();
	assert!(result
		.unwrap_err()
		.to_string()
		.contains("ended unexpectedly"));

	channel.close(200, "test startup failure").await.unwrap();
	assert!(run_worker(config)
		.await
		.unwrap_err()
		.to_string()
		.contains("Starting RabbitMQ consumer"));
}
