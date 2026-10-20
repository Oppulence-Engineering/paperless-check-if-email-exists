use super::do_work::{do_check_email_work, CheckEmailTask, TaskError};
use super::single_shot::send_single_shot_reply;
use crate::config::{BackendConfig, RabbitMQConfig};
use crate::worker::do_work::CheckEmailJobId;
use anyhow::{bail, Context};
use check_if_email_exists::LOG_TARGET;
use futures::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Channel, Connection, ConnectionProperties};
use sentry_anyhow::capture_anyhow;
use std::sync::Arc;
use tokio::time::{timeout, Duration};
use tracing::{debug, error, info, trace};

/// Our RabbitMQ only has one queue: "check_email".
pub const CHECK_EMAIL_QUEUE: &str = "check_email";
pub const MAX_QUEUE_PRIORITY: u8 = 5;
const RABBITMQ_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// Longest a throttled bulk delivery is held before being requeued. Per-hour
/// and per-day throttle windows can be far longer, and holding an unacked
/// delivery past RabbitMQ's consumer ack timeout (30 minutes by default)
/// gets the whole channel closed.
const MAX_THROTTLE_REQUEUE_DELAY: Duration = Duration::from_secs(60);
/// Shortest hold, so a window that has just rolled over (delay of zero)
/// cannot turn into a tight requeue loop.
const MIN_THROTTLE_REQUEUE_DELAY: Duration = Duration::from_millis(100);

fn throttle_requeue_delay(throttle_delay: Duration) -> Duration {
	throttle_delay.clamp(MIN_THROTTLE_REQUEUE_DELAY, MAX_THROTTLE_REQUEUE_DELAY)
}

/// Set up the RabbitMQ connection and declare the "check_email" queue.
///
/// The check channel is used to consume messages from the queue. It has a
/// global prefetch limit set to the concurrency limit.
///
/// Returns a the lapin Channel.
pub async fn setup_rabbit_mq(
	backend_name: &str,
	config: &RabbitMQConfig,
) -> Result<Channel, anyhow::Error> {
	let options = ConnectionProperties::default().with_connection_name(backend_name.into());

	let conn = timeout(
		RABBITMQ_CONNECT_TIMEOUT,
		Connection::connect(&config.url, options),
	)
	.await
	.with_context(|| {
		format!(
			"Timed out connecting to rabbitmq within {:?}",
			RABBITMQ_CONNECT_TIMEOUT
		)
	})?
	.context("Connecting to rabbitmq")?;
	let channel = conn.create_channel().await?;

	info!(target: LOG_TARGET, backend=?backend_name,connected=conn.status().connected(), "Connected to AMQP broker");

	let mut queue_args = FieldTable::default();
	queue_args.insert("x-max-priority".into(), MAX_QUEUE_PRIORITY.into());

	// Assert all queue is declared.
	channel
		.queue_declare(
			CHECK_EMAIL_QUEUE.into(),
			QueueDeclareOptions {
				durable: true,
				..Default::default()
			},
			queue_args.clone(),
		)
		.await?;

	// Publisher confirms: without them a publish on this channel "succeeds"
	// as soon as the frame is written to the socket, and a broker crash right
	// after silently loses the task. With them, publish_payload waits for the
	// broker's ack (for persistent messages: once written to disk).
	channel
		.confirm_select(ConfirmSelectOptions::default())
		.await?;

	// Set up prefetch (concurrency) limit using qos
	channel
		.basic_qos(
			config.concurrency,
			// Set global to true to apply to all consumers, even though in our
			// case there's only one consumer.
			// ref: https://www.rabbitmq.com/docs/consumer-prefetch#independent-consumers
			BasicQosOptions { global: true },
		)
		.await?;

	info!(target: LOG_TARGET, queues=?CHECK_EMAIL_QUEUE, concurrency=?config.concurrency, "Worker will start consuming messages");

	Ok(channel)
}

/// Consume messages until the worker fails, allowing the process supervisor to restart it.
pub async fn run_worker(config: Arc<BackendConfig>) -> Result<(), anyhow::Error> {
	consume_check_email(config).await
}

/// Consume "check_email" queue.
async fn consume_check_email(config: Arc<BackendConfig>) -> Result<(), anyhow::Error> {
	let config_clone = Arc::clone(&config);
	let worker_config = config_clone.must_worker_config()?;
	let channel = worker_config.channel;
	let throttle = config.get_throttle_manager();

	let mut consumer = match channel
		.basic_consume(
			CHECK_EMAIL_QUEUE.into(),
			format!("{}-{}", &config_clone.backend_name, CHECK_EMAIL_QUEUE).into(),
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await
	{
		Ok(consumer) => consumer,
		Err(err) => {
			error!(
				target: LOG_TARGET,
				error = ?err,
				"Failed to start RabbitMQ consumer"
			);
			return Err(err).context("Starting RabbitMQ consumer");
		}
	};

	// Loop over the incoming messages
	while let Some(delivery) = consumer.next().await {
		let delivery = match delivery {
			Ok(delivery) => delivery,
			Err(err) => {
				error!(
					target: LOG_TARGET,
					error = ?err,
					"RabbitMQ consumer failed"
				);
				return Err(err).context("Consuming RabbitMQ delivery");
			}
		};
		let payload = match serde_json::from_slice::<CheckEmailTask>(&delivery.data) {
			Ok(payload) => payload,
			Err(err) => {
				error!(
					target: LOG_TARGET,
					error = ?err,
					"Rejecting malformed RabbitMQ message"
				);
				capture_anyhow(&anyhow::Error::from(err));
				if let Err(reject_err) =
					delivery.reject(BasicRejectOptions { requeue: false }).await
				{
					error!(
						target: LOG_TARGET,
						error = ?reject_err,
						"Failed to reject malformed RabbitMQ message"
					);
					capture_anyhow(&anyhow::Error::from(reject_err));
				}
				continue;
			}
		};
		debug!(target: LOG_TARGET, email=?payload.input.to_email, "Consuming message");

		// Check if we should throttle before fetching the next message
		if let Some(throttle_result) = throttle.try_acquire().await {
			// This line below will log every time the worker fetches from
			// RabbitMQ. It's noisy
			trace!(target: LOG_TARGET, wait=?throttle_result.delay, email=?payload.input.to_email, "Too many requests {}, throttling", throttle_result.limit_type);

			// Durable operations must retain their delivery while waiting for capacity.
			match payload.job_id {
				CheckEmailJobId::SingleShot
					if payload
						.metadata
						.as_ref()
						.and_then(|m| m.task_db_id)
						.is_none() =>
				{
					debug!(target: LOG_TARGET, email=payload.input.to_email, job_id=?payload.job_id, "Rejecting single-shot email because of throttling");
					delivery
						.reject(BasicRejectOptions { requeue: false })
						.await?;

					if let Err(error) = send_single_shot_reply(
						Arc::clone(&channel),
						&delivery,
						&Err(TaskError::Throttle(throttle_result)),
					)
					.await
					{
						error!(target: LOG_TARGET, error=?error, "Failed to send throttle reply");
						capture_anyhow(&error);
					}
				}
				_ => {
					// Put the message back so that this or another worker can
					// pick it up, but only once the throttle window has passed:
					// requeueing right away hands the same message straight
					// back to us in a hot loop for the whole window. Holding the
					// delivery unacked meanwhile keeps its prefetch slot busy,
					// which also stops this worker pulling more messages.
					let wait = throttle_requeue_delay(throttle_result.delay);
					tokio::spawn(async move {
						tokio::time::sleep(wait).await;
						if let Err(err) =
							delivery.reject(BasicRejectOptions { requeue: true }).await
						{
							error!(target: LOG_TARGET, error=?err, "Failed to requeue throttled message");
						}
					});
					debug!(target: LOG_TARGET, email=payload.input.to_email, job_id=?payload.job_id, wait=?wait, "Requeueing message after throttle delay");
				}
			}

			continue;
		}

		let config_clone2 = Arc::clone(&config_clone);
		let channel_clone2 = Arc::clone(&channel);

		info!(
			target: LOG_TARGET,
			email=payload.input.to_email,
			job_id=?payload.job_id,
			"Starting task"
		);
		tokio::spawn(async move {
			if let Err(e) =
				do_check_email_work(&payload, delivery, channel_clone2, config_clone2).await
			{
				error!(
					target: LOG_TARGET,
					email=payload.input.to_email,
					error=?e,
					"Error processing message"
				);
				capture_anyhow(&e);
			}
		});
	}

	bail!("RabbitMQ consumer ended unexpectedly")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn throttled_delivery_is_held_for_the_window_but_never_past_the_ack_timeout() {
		// Per-day limit hit: waiting out the window would exceed the broker's
		// ack timeout, so the hold is capped.
		assert_eq!(
			throttle_requeue_delay(Duration::from_secs(23 * 3600)),
			MAX_THROTTLE_REQUEUE_DELAY
		);
		// Window just rolled over: still not an immediate requeue.
		assert_eq!(
			throttle_requeue_delay(Duration::ZERO),
			MIN_THROTTLE_REQUEUE_DELAY
		);
		// Per-second / per-minute windows are waited out exactly.
		assert_eq!(
			throttle_requeue_delay(Duration::from_millis(750)),
			Duration::from_millis(750)
		);
	}
}
