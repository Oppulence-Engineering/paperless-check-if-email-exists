//! This file implements the `POST /bulk` endpoint.

use crate::config::BackendConfig;
use crate::http::CheckEmailRequest;
use check_if_email_exists::{
	check_email, CheckEmailInput, CheckEmailInputProxy, CheckEmailOutput, Reachable, LOG_TARGET,
};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use sqlxmq::{job, CurrentJob};
use std::error::Error;
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

use super::error::BulkError;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskInput {
	// fields for CheckEmailInput
	pub to_email: String,     // Email from request to verify.
	pub smtp_ports: Vec<u16>, // Ports to try for each email, in given order. Defaults to [25].
	pub proxy: Option<CheckEmailInputProxy>,
	pub hello_name: Option<String>,
	pub from_email: Option<String>,
}

impl TaskInput {
	fn to_check_email_input(
		&self,
		config: Arc<BackendConfig>,
		port: u16,
	) -> Result<CheckEmailInput, std::io::Error> {
		let request = CheckEmailRequest {
			to_email: self.to_email.clone(),
			smtp_port: Some(port),
			proxy: self.proxy.clone(),
			hello_name: self.hello_name.clone(),
			from_email: self.from_email.clone(),
			..Default::default()
		};
		request
			.validate_routing_overrides(&config)
			.map_err(std::io::Error::other)?;
		Ok(request.to_check_email_input(config))
	}
}

/// Struct that's serialized into the sqlxmq own `payload_json` table.
#[derive(Debug, Deserialize, Serialize)]
struct TaskPayload {
	id: i32,
	input: TaskInput,
}

pub async fn submit_job(
	conn_pool: &Pool<Postgres>,
	job_id: i32,
	task_input: TaskInput,
) -> Result<Uuid, BulkError> {
	let task_payload = TaskPayload {
		id: job_id,
		input: task_input,
	};

	let uuid = email_verification_task
		.builder()
		.set_json(&task_payload)
		.map_err(|e| {
			error!(
				target: LOG_TARGET,
				"Failed to serialize task for [job={}] with [error={}]",
				job_id, e
			);

			BulkError::Json(e)
		})?
		.spawn(conn_pool)
		.await
		.map_err(|e| {
			error!(
				target: LOG_TARGET,
				"Failed to submit task for [bulk_req={}] with [error={}]",
				job_id, e
			);

			e
		})?;

	Ok(uuid)
}

/// Arguments to the `#[job]` attribute allow setting default task options.
/// This task tries to verify the given email and inserts the results
/// into the email verification db table
/// NOTE: if EMAIL_TASK_BATCH_SIZE is made greater than 1 this logic
/// will have to be changed to handle a vector outputs from `check_email`.
///
/// Small note about namings: what sqlxmq calls a "job", we call it a "task".
/// We call a "job" a user bulk request, i.e. a list of "tasks".
/// Please be careful while reading code.
#[job]
pub async fn email_verification_task(
	mut current_job: CurrentJob,
	config: Arc<BackendConfig>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
	if config.verification_backend == crate::verification::VerificationBackend::Millionverifier {
		return Err("Legacy SMTP jobs must be drained before switching to MillionVerifier.".into());
	}
	crate::smtp_egress::require_smtp_ready(&config)?;
	let task_payload: TaskPayload = current_job.json()?.ok_or("Got empty task.")?;
	let job_id = task_payload.id;

	let mut final_response: Option<CheckEmailOutput> = None;

	for port in &task_payload.input.smtp_ports {
		crate::smtp_egress::require_smtp_ready(&config)?;
		let check_email_input = task_payload
			.input
			.to_check_email_input(Arc::clone(&config), *port)?;
		debug!(
			target: LOG_TARGET,
			"Starting task [email={}] for [job={}] and [uuid={}]",
			check_email_input.to_email,
			task_payload.id,
			current_job.id(),
		);

		let to_email = check_email_input.to_email.clone();
		let response = check_email(&check_email_input).await;

		debug!(
			target: LOG_TARGET,
			email=to_email,
			job_id=task_payload.id,
			uuid=?current_job.id(),
			is_reachable=?response.is_reachable,
			"Task result received"
		);

		let is_reachable = response.is_reachable == Reachable::Unknown;
		final_response = Some(response);
		// unsuccessful validation continue iteration with next possible smtp port
		if is_reachable {
			continue;
		}
		// successful validation attempt complete job break iteration
		else {
			break;
		}
	}

	// final response can only be empty if there
	// were no validation attempts. This can can
	// never occur currently
	if let Some(response) = final_response {
		// write results and terminate iteration
		#[allow(unused_variables)]
		let rec = sqlx::query!(
			r#"
			INSERT INTO email_results (job_id, result)
			VALUES ($1, $2)
			"#,
			job_id,
			serde_json::json!(response)
		)
		// TODO: This is a simplified solution and will work when
		// the job queue and email results tables are in the same
		// database. Keeping them in separate database will require
		// some custom logic on the job registry side
		// https://github.com/Diggsey/sqlxmq/issues/4
		.fetch_optional(current_job.pool())
		.await
		.map_err(|e| {
			error!(
				target: LOG_TARGET,
				"Failed to write [email={}] result to db for [job={}] and [uuid={}] with [error={}]",
				response.input,
				job_id,
				current_job.id(),
				e
			);

			e
		})?;

		debug!(
			target: LOG_TARGET,
			"Wrote result for [email={}] for [job={}] and [uuid={}]",
			response.input,
			job_id,
			current_job.id(),
		);
	}

	current_job.complete().await?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use check_if_email_exists::smtp::verif_method::EmailProvider;
	use std::time::Duration;

	#[test]
	fn legacy_job_uses_operator_identity_route_and_deadline() {
		let mut config = BackendConfig::empty();
		config.hello_name = "verify.example.com".into();
		config.from_email = "verify@example.com".into();
		config.smtp_timeout = Some(17);
		config.proxy = Some(CheckEmailInputProxy {
			host: "proxy.example.com".into(),
			port: 1080,
			..Default::default()
		});
		let config = Arc::new(config);
		let input = TaskInput {
			to_email: "Sales@example.com".into(),
			smtp_ports: vec![25],
			proxy: None,
			hello_name: None,
			from_email: None,
		};
		let check = input.to_check_email_input(Arc::clone(&config), 25).unwrap();
		let route = check
			.verif_method
			.smtp_config(EmailProvider::EverythingElse)
			.unwrap();
		assert_eq!(check.to_email, "Sales@example.com");
		assert_eq!(route.hello_name, config.hello_name);
		assert_eq!(route.from_email, config.from_email);
		assert_eq!(route.smtp_timeout, Some(Duration::from_secs(17)));
		assert_eq!(
			check
				.verif_method
				.proxies
				.get(route.proxy.as_ref().unwrap()),
			config.proxy.as_ref()
		);
		assert!(input.to_check_email_input(config, 22).is_err());
	}
}
