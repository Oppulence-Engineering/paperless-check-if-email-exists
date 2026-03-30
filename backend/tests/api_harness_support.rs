#![allow(dead_code)]

use crate::test_helpers::{
	build_test_config, insert_api_key, insert_api_key_with_status, insert_comment, insert_domain,
	insert_event, insert_finder_job, insert_finder_result, insert_job,
	insert_keys_for_existing_tenant, insert_legacy_bulk_job, insert_legacy_email_result,
	insert_list, insert_pipeline, insert_pipeline_run, insert_reputation_cache,
	insert_scored_task, insert_suppression, insert_tenant, insert_tenant_with_keys, ConfigProfile,
	TenantApiKeysFixture, ADMIN_SECRET,
};
use reacher_backend::http::openapi::build_spec;
use reacher_backend::http::routes::{all_route_specs, ApiOperationKey};
use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
use sqlx::PgPool;
use std::collections::BTreeSet;
use std::sync::Arc;
use uuid::Uuid;
use warp::http::HeaderMap;
use warp::test::{request, RequestBuilder};

const UPGRADE_TENANT_ID: &str = "11111111-1111-1111-1111-111111111111";
const UPGRADE_JOB_ID: i32 = 4101;
const UPGRADE_LIST_ID: i32 = 4201;

#[derive(Debug, Clone, Copy)]
enum AuthProfile {
	None,
	Secret,
	AdminSecret,
	AdminSecretInvalid,
	BearerFull,
	BearerBulk,
	BearerLists,
	BearerVerify,
	BearerPipelines,
}

#[derive(Debug, Clone, Copy)]
enum PathProfile {
	Literal(&'static str),
	LegacyBulkStatus,
	LegacyBulkResults,
	FinderGet,
	ListGet,
	ListQuality,
	ListDownload,
	ListDelete,
	PipelineGet,
	PipelinePatch,
	PipelineDelete,
	PipelinePause,
	PipelineResume,
	PipelineTrigger,
	PipelineRuns,
	PipelineRunGet,
	SuppressionDelete,
	CommentDelete,
	JobGet,
	JobCancel,
	JobEvents,
	JobResults,
	JobDownload,
	JobRetry,
	JobRetryCancelled,
	JobApproval,
	JobLatency,
	JobCancelCompleted,
	EmailHistory,
	DomainGet,
	DomainPatch,
	DomainDelete,
	SelfApiKeyGet,
	SelfApiKeyPatch,
	SelfApiKeyDelete,
	AdminTenantGet,
	AdminTenantPut,
	AdminTenantDelete,
	AdminQuotaGet,
	AdminQuotaPatch,
	AdminQuotaReset,
	AdminJobGet,
	AdminJobEvents,
	AdminJobResults,
	AdminTenantJobs,
	AdminTenantApiKeysCreate,
	AdminTenantApiKeysList,
	AdminTenantApiKeyGet,
	AdminTenantApiKeyPatch,
	AdminTenantApiKeyDelete,
	AdminTenantApiKeyReactivate,
	PipelineTriggerConflict,
}

#[derive(Debug, Clone, Copy)]
enum BodyProfile {
	None,
	JsonV0CheckEmail,
	JsonV0BulkCreate,
	JsonOnboard,
	JsonV1CheckEmail,
	JsonFindEmailCreate,
	JsonFindEmailInvalidStrategy,
	MultipartListUpload,
	JsonPipelineCreate,
	JsonPipelinePatch,
	JsonPipelineTrigger,
	JsonPipelineTriggerConflict,
	JsonReputationCheck,
	JsonSuppressionsAdd,
	JsonV1BulkCreate,
	JsonCommentsCreate,
	JsonCommentsEmpty,
	JsonMeSettingsPatch,
	JsonMeWebhookPatch,
	JsonMeDomainCreate,
	JsonMeDomainCreateDuplicate,
	JsonMeDomainPatch,
	JsonMeDomainPatchConflict,
	JsonMeApiKeyCreate,
	JsonMeApiKeyPatch,
	JsonEmptyObject,
	MalformedJson,
	JsonAdminTenantCreate,
	JsonAdminTenantUpdate,
	JsonAdminQuotaPatch,
	JsonAdminApiKeyCreate,
	JsonAdminApiKeyPatch,
}

#[derive(Debug, Clone, Copy)]
enum Expectation {
	Json(&'static [&'static str]),
	Error(&'static str),
	Csv,
	OpenApi,
	StatusOnly,
}

#[derive(Debug, Clone, Copy)]
pub struct HarnessCase {
	key: ApiOperationKey,
	config: ConfigProfile,
	auth: AuthProfile,
	path: PathProfile,
	body: BodyProfile,
	expected_status: u16,
	expectation: Expectation,
	upgrade_safe: bool,
}

#[derive(Debug)]
pub struct HarnessFixtures {
	tenant: TenantApiKeysFixture,
	legacy_status_job: i32,
	legacy_results_job: i32,
	job_main: i32,
	job_cancel: i32,
	job_retry: i32,
	job_cancelled: i32,
	finder_job: i32,
	list_main: i32,
	list_delete: i32,
	pipeline_main: i64,
	pipeline_pause: i64,
	pipeline_resume: i64,
	pipeline_trigger: i64,
	pipeline_delete: i64,
	pipeline_active_conflict: i64,
	pipeline_run: i64,
	suppression_id: i32,
	comment_delete_id: i64,
	self_api_key_get: Uuid,
	self_api_key_update: Uuid,
	self_api_key_delete: Uuid,
	admin_tenant_get: Uuid,
	admin_tenant_update: Uuid,
	admin_tenant_delete: Uuid,
	admin_tenant_quota: Uuid,
	admin_api_key_tenant: Uuid,
	admin_api_key_get: Uuid,
	admin_api_key_update: Uuid,
	admin_api_key_delete: Uuid,
	admin_api_key_reactivate: Uuid,
	domain_get: String,
	domain_update: String,
	domain_delete: String,
}

pub struct HarnessRuntime {
	public: Arc<reacher_backend::config::BackendConfig>,
	db_only: Arc<reacher_backend::config::BackendConfig>,
	bearer: Arc<reacher_backend::config::BackendConfig>,
	admin: Arc<reacher_backend::config::BackendConfig>,
	pseudo_worker: Arc<reacher_backend::config::BackendConfig>,
	worker_rabbit: Arc<reacher_backend::config::BackendConfig>,
	pipeline_enabled: Arc<reacher_backend::config::BackendConfig>,
}

macro_rules! case {
	($method:literal, $path:literal, $config:expr, $auth:expr, $path_profile:expr, $body:expr, $status:expr, $expectation:expr $(,)?) => {
		HarnessCase {
			key: ApiOperationKey {
				method: $method,
				path: $path,
			},
			config: $config,
			auth: $auth,
			path: $path_profile,
			body: $body,
			expected_status: $status,
			expectation: $expectation,
			upgrade_safe: false,
		}
	};
}

macro_rules! upgrade_case {
	($method:literal, $path:literal, $config:expr, $auth:expr, $path_profile:expr, $body:expr, $status:expr, $expectation:expr $(,)?) => {
		HarnessCase {
			key: ApiOperationKey {
				method: $method,
				path: $path,
			},
			config: $config,
			auth: $auth,
			path: $path_profile,
			body: $body,
			expected_status: $status,
			expectation: $expectation,
			upgrade_safe: true,
		}
	};
}

fn verification_result(
	email: &str,
	is_reachable: &str,
	is_disposable: bool,
	is_role_account: bool,
	is_catch_all: bool,
	has_full_inbox: bool,
) -> serde_json::Value {
	serde_json::json!({
		"input": email,
		"is_reachable": is_reachable,
		"misc": {
			"is_disposable": is_disposable,
			"is_role_account": is_role_account,
			"is_b2c": false,
			"is_spam_trap_domain": false
		},
		"mx": {
			"accepts_email": true,
			"records": ["mx1.example.com"]
		},
		"smtp": {
			"can_connect_smtp": true,
			"has_full_inbox": has_full_inbox,
			"is_catch_all": is_catch_all,
			"is_deliverable": is_reachable == "safe",
			"is_disabled": false
		},
		"syntax": {
			"address": email,
			"domain": email.split('@').nth(1).unwrap_or_default(),
			"is_valid_syntax": true,
			"username": email.split('@').next().unwrap_or_default()
		}
	})
}

async fn set_task_latency(pool: &PgPool, task_id: i32, millis: i32) {
	sqlx::query(
		"UPDATE v1_task_result SET started_at = completed_at - ($2::TEXT || ' milliseconds')::INTERVAL WHERE id = $1",
	)
	.bind(task_id)
	.bind(millis.to_string())
	.execute(pool)
	.await
	.expect("set_task_latency failed");
}

pub async fn seed_fixtures(pool: &PgPool) -> HarnessFixtures {
	let tenant = insert_tenant_with_keys(pool, "harness-main").await;

	let legacy_status_job = insert_legacy_bulk_job(pool, 2).await;
	insert_legacy_email_result(
		pool,
		legacy_status_job,
		verification_result(
			"legacy-safe@example.com",
			"safe",
			false,
			false,
			false,
			false,
		),
	)
	.await;

	let legacy_results_job = insert_legacy_bulk_job(pool, 2).await;
	insert_legacy_email_result(
		pool,
		legacy_results_job,
		verification_result("legacy-one@example.com", "safe", false, false, false, false),
	)
	.await;
	insert_legacy_email_result(
		pool,
		legacy_results_job,
		verification_result(
			"legacy-two@example.com",
			"invalid",
			false,
			false,
			false,
			false,
		),
	)
	.await;

	let job_main = insert_job(pool, Some(tenant.tenant_id), 3, "completed").await;
	let main_task_1 = insert_scored_task(
		pool,
		job_main,
		Some(tenant.tenant_id),
		"good@example.com",
		None,
		Some(verification_result(
			"good@example.com",
			"safe",
			false,
			false,
			false,
			false,
		)),
		"completed",
		Some(95),
		Some("valid"),
		Some("deliverable"),
		Some(true),
		Some(vec!["deliverable".to_string()]),
		Some("good@example.com"),
		false,
	)
	.await;
	let main_task_2 = insert_scored_task(
		pool,
		job_main,
		Some(tenant.tenant_id),
		"risky@example.com",
		None,
		Some(verification_result(
			"risky@example.com",
			"risky",
			true,
			false,
			true,
			false,
		)),
		"completed",
		Some(58),
		Some("risky"),
		Some("catch_all"),
		Some(false),
		Some(vec!["catch_all".to_string()]),
		Some("risky@example.com"),
		false,
	)
	.await;
	let main_task_3 = insert_scored_task(
		pool,
		job_main,
		Some(tenant.tenant_id),
		"bad@example.com",
		None,
		Some(verification_result(
			"bad@example.com",
			"invalid",
			false,
			true,
			false,
			true,
		)),
		"completed",
		Some(4),
		Some("invalid"),
		Some("full_inbox"),
		Some(false),
		Some(vec!["invalid".to_string()]),
		Some("bad@example.com"),
		false,
	)
	.await;
	set_task_latency(pool, main_task_1, 110).await;
	set_task_latency(pool, main_task_2, 420).await;
	set_task_latency(pool, main_task_3, 900).await;
	insert_event(pool, job_main, Some(main_task_1), "job.created").await;
	insert_event(pool, job_main, Some(main_task_1), "task.completed").await;
	insert_event(pool, job_main, Some(main_task_2), "job.completed").await;

	let job_cancel = insert_job(pool, Some(tenant.tenant_id), 2, "running").await;
	insert_scored_task(
		pool,
		job_cancel,
		Some(tenant.tenant_id),
		"cancel1@example.com",
		None,
		None,
		"queued",
		None,
		None,
		None,
		None,
		None,
		Some("cancel1@example.com"),
		false,
	)
	.await;
	insert_scored_task(
		pool,
		job_cancel,
		Some(tenant.tenant_id),
		"cancel2@example.com",
		None,
		None,
		"queued",
		None,
		None,
		None,
		None,
		None,
		Some("cancel2@example.com"),
		false,
	)
	.await;

	let job_retry = insert_job(pool, Some(tenant.tenant_id), 2, "failed").await;
	let retry_task_one = insert_scored_task(
		pool,
		job_retry,
		Some(tenant.tenant_id),
		"retry1@example.com",
		None,
		None,
		"failed",
		None,
		None,
		None,
		None,
		None,
		Some("retry1@example.com"),
		false,
	)
	.await;
	let retry_task_two = insert_scored_task(
		pool,
		job_retry,
		Some(tenant.tenant_id),
		"retry2@example.com",
		None,
		None,
		"failed",
		None,
		None,
		None,
		None,
		None,
		Some("retry2@example.com"),
		false,
	)
	.await;
	for (task_id, email) in [
		(retry_task_one, "retry1@example.com"),
		(retry_task_two, "retry2@example.com"),
	] {
		let payload = serde_json::to_value(reacher_backend::worker::do_work::CheckEmailTask {
			input: reacher_backend::http::CheckEmailRequest {
				to_email: email.to_string(),
				..Default::default()
			}
			.to_check_email_input(Arc::new(reacher_backend::config::BackendConfig::empty())),
			job_id: reacher_backend::worker::do_work::CheckEmailJobId::Bulk(job_retry),
			webhook: None,
			metadata: Some(reacher_backend::worker::do_work::TaskMetadata {
				tenant_id: Some(tenant.tenant_id.to_string()),
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: None,
				dedupe_key: None,
				task_db_id: Some(task_id),
			}),
		})
		.expect("serialize retry payload");
		sqlx::query("UPDATE v1_task_result SET payload = $2 WHERE id = $1")
			.bind(task_id)
			.bind(payload)
			.execute(pool)
			.await
			.expect("update retry payload");
	}

	let job_cancelled = insert_job(pool, Some(tenant.tenant_id), 1, "cancelled").await;
	insert_scored_task(
		pool,
		job_cancelled,
		Some(tenant.tenant_id),
		"cancelled@example.com",
		None,
		None,
		"cancelled",
		None,
		None,
		None,
		None,
		None,
		Some("cancelled@example.com"),
		false,
	)
	.await;

	let list_job = insert_job(pool, Some(tenant.tenant_id), 3, "completed").await;
	let original_rows = serde_json::json!({
		"0": {"email": "good@example.com", "name": "Good"},
		"1": {"email": "risky@example.com", "name": "Risky"},
		"2": {"email": "bad@example.com", "name": "Bad"}
	});
	let list_main = insert_list(
		pool,
		tenant.tenant_id,
		list_job,
		"Harness Main List",
		"completed",
		3,
		&["email", "name"],
		original_rows,
	)
	.await;
	insert_scored_task(
		pool,
		list_job,
		Some(tenant.tenant_id),
		"good@example.com",
		Some(serde_json::json!({"list_id": list_main, "row_index": 0, "email_column": "email"})),
		Some(verification_result(
			"good@example.com",
			"safe",
			false,
			false,
			false,
			false,
		)),
		"completed",
		Some(93),
		Some("valid"),
		Some("deliverable"),
		Some(true),
		Some(vec!["deliverable".to_string()]),
		Some("good@example.com"),
		false,
	)
	.await;
	insert_scored_task(
		pool,
		list_job,
		Some(tenant.tenant_id),
		"risky@example.com",
		Some(serde_json::json!({"list_id": list_main, "row_index": 1, "email_column": "email"})),
		Some(verification_result(
			"risky@example.com",
			"risky",
			true,
			false,
			true,
			false,
		)),
		"completed",
		Some(60),
		Some("risky"),
		Some("catch_all"),
		Some(false),
		Some(vec!["catch_all".to_string()]),
		Some("risky@example.com"),
		false,
	)
	.await;
	insert_scored_task(
		pool,
		list_job,
		Some(tenant.tenant_id),
		"bad@example.com",
		Some(serde_json::json!({"list_id": list_main, "row_index": 2, "email_column": "email"})),
		Some(verification_result(
			"bad@example.com",
			"invalid",
			false,
			true,
			false,
			true,
		)),
		"completed",
		Some(5),
		Some("invalid"),
		Some("full_inbox"),
		Some(false),
		Some(vec!["invalid".to_string()]),
		Some("bad@example.com"),
		false,
	)
	.await;

	let list_delete_job = insert_job(pool, Some(tenant.tenant_id), 1, "completed").await;
	let list_delete = insert_list(
		pool,
		tenant.tenant_id,
		list_delete_job,
		"Harness Delete List",
		"completed",
		1,
		&["email"],
		serde_json::json!({"0": {"email": "delete@example.com"}}),
	)
	.await;
	insert_scored_task(
		pool,
		list_delete_job,
		Some(tenant.tenant_id),
		"delete@example.com",
		Some(serde_json::json!({"list_id": list_delete, "row_index": 0, "email_column": "email"})),
		Some(verification_result(
			"delete@example.com",
			"safe",
			false,
			false,
			false,
			false,
		)),
		"completed",
		Some(90),
		Some("valid"),
		Some("deliverable"),
		Some(true),
		Some(vec!["deliverable".to_string()]),
		Some("delete@example.com"),
		false,
	)
	.await;

	let source = serde_json::json!({"type": "list_snapshot", "list_id": list_main});
	let pipeline_main = insert_pipeline(
		pool,
		tenant.tenant_id,
		"Harness Pipeline Main",
		source.clone(),
	)
	.await;
	let pipeline_pause = insert_pipeline(
		pool,
		tenant.tenant_id,
		"Harness Pipeline Pause",
		source.clone(),
	)
	.await;
	let pipeline_resume = insert_pipeline(
		pool,
		tenant.tenant_id,
		"Harness Pipeline Resume",
		source.clone(),
	)
	.await;
	sqlx::query("UPDATE v1_pipelines SET status = 'paused'::pipeline_status WHERE id = $1")
		.bind(pipeline_resume)
		.execute(pool)
		.await
		.expect("pause seed");
	let pipeline_trigger = insert_pipeline(
		pool,
		tenant.tenant_id,
		"Harness Pipeline Trigger",
		source.clone(),
	)
	.await;
	let pipeline_active_conflict = insert_pipeline(
		pool,
		tenant.tenant_id,
		"Harness Pipeline Active Conflict",
		source.clone(),
	)
	.await;
	let pipeline_delete = insert_pipeline(
		pool,
		tenant.tenant_id,
		"Harness Pipeline Delete",
		source.clone(),
	)
	.await;
	let pipeline_run = insert_pipeline_run(
		pool,
		pipeline_main,
		tenant.tenant_id,
		"completed",
		Some(job_main),
		Some(list_main),
	)
	.await;
	sqlx::query(
		r#"
		INSERT INTO v1_usage_events (
			tenant_id,
			pipeline_id,
			pipeline_run_id,
			job_id,
			source,
			reserved_emails,
			committed_emails,
			status,
			metadata
		)
		VALUES ($1, $2, $3, $4, 'pipeline', 3, 3, 'committed', '{}'::jsonb)
		"#,
	)
	.bind(tenant.tenant_id)
	.bind(pipeline_main)
	.bind(pipeline_run)
	.bind(job_main)
	.execute(pool)
	.await
	.expect("insert usage event failed");
	insert_pipeline_run(
		pool,
		pipeline_trigger,
		tenant.tenant_id,
		"running",
		Some(job_main),
		Some(list_main),
	)
	.await;
	insert_pipeline_run(
		pool,
		pipeline_active_conflict,
		tenant.tenant_id,
		"running",
		Some(job_main),
		Some(list_main),
	)
	.await;

	let suppression_id =
		insert_suppression(pool, tenant.tenant_id, "suppressed@example.com", "manual").await;
	insert_comment(
		pool,
		tenant.tenant_id,
		Some(job_main),
		None,
		"Existing comment",
	)
	.await;
	let comment_delete_id =
		insert_comment(pool, tenant.tenant_id, Some(job_main), None, "Delete me").await;

	let domain_get = "existing.example.com".to_string();
	let domain_update = "patchable.example.com".to_string();
	let domain_delete = "remove.example.com".to_string();
	insert_domain(pool, tenant.tenant_id, &domain_get, true, true).await;
	insert_domain(pool, tenant.tenant_id, &domain_update, true, false).await;
	insert_domain(pool, tenant.tenant_id, &domain_delete, true, false).await;

	let (_self_get_key, self_api_key_get) = insert_api_key(pool, tenant.tenant_id).await;
	let (_self_update_key, self_api_key_update) = insert_api_key(pool, tenant.tenant_id).await;
	let (_self_delete_key, self_api_key_delete) = insert_api_key(pool, tenant.tenant_id).await;

	let reputation_response = serde_json::json!({
		"domain": "cached.example.com",
		"score": 81,
		"risk_level": "low",
		"blacklist_results": [],
		"dns_records": {
			"has_spf": true,
			"spf_valid": true,
			"has_dkim": true,
			"has_dmarc": true,
			"dmarc_policy": "reject",
			"has_mx": true,
			"mx_records": ["mx1.cached.example.com"]
		},
		"domain_info": {
			"domain_age_days": 1200,
			"registrar": "Harness Registrar",
			"created_at": "2020-01-01T00:00:00Z"
		},
		"cached": false
	});
	insert_reputation_cache(pool, "cached.example.com", reputation_response, 81, "low").await;

	let finder_bulk_job = insert_job(pool, Some(tenant.tenant_id), 2, "completed").await;
	let finder_job = insert_finder_job(
		pool,
		tenant.tenant_id,
		finder_bulk_job,
		"completed",
		"example.com",
		2,
		Some("jane.doe@example.com"),
	)
	.await;
	let finder_task = insert_scored_task(
		pool,
		finder_bulk_job,
		Some(tenant.tenant_id),
		"jane.doe@example.com",
		None,
		Some(verification_result(
			"jane.doe@example.com",
			"safe",
			false,
			false,
			false,
			false,
		)),
		"completed",
		Some(97),
		Some("valid"),
		Some("deliverable"),
		Some(true),
		Some(vec!["deliverable".to_string()]),
		Some("jane.doe@example.com"),
		false,
	)
	.await;
	insert_finder_result(
		pool,
		finder_job,
		Some(finder_task),
		"jane.doe@example.com",
		"first.last",
		97,
		"valid",
		Some(verification_result(
			"jane.doe@example.com",
			"safe",
			false,
			false,
			false,
			false,
		)),
	)
	.await;
	insert_finder_result(
		pool,
		finder_job,
		None,
		"jdoe@example.com",
		"flast",
		72,
		"risky",
		None,
	)
	.await;

	let admin_tenant_get = insert_tenant(pool, "admin-get", Some(1000), 10).await;
	let admin_tenant_update = insert_tenant(pool, "admin-update", Some(1000), 12).await;
	let admin_tenant_delete = insert_tenant(pool, "admin-delete", Some(1000), 0).await;
	let admin_tenant_quota = insert_tenant(pool, "admin-quota", Some(500), 42).await;
	let admin_api_key_tenant = insert_tenant(pool, "admin-apikeys", Some(1000), 0).await;
	let (_admin_get_key, admin_api_key_get) = insert_api_key(pool, admin_api_key_tenant).await;
	let (_admin_update_key, admin_api_key_update) =
		insert_api_key(pool, admin_api_key_tenant).await;
	let (_admin_delete_key, admin_api_key_delete) =
		insert_api_key(pool, admin_api_key_tenant).await;
	let (_admin_revoked_key, admin_api_key_reactivate) =
		insert_api_key_with_status(pool, admin_api_key_tenant, "revoked").await;

	HarnessFixtures {
		tenant,
		legacy_status_job,
		legacy_results_job,
		job_main,
		job_cancel,
		job_retry,
		job_cancelled,
		finder_job,
		list_main,
		list_delete,
		pipeline_main,
		pipeline_pause,
		pipeline_resume,
		pipeline_trigger,
		pipeline_delete,
		pipeline_active_conflict,
		pipeline_run,
		suppression_id,
		comment_delete_id,
		self_api_key_get,
		self_api_key_update,
		self_api_key_delete,
		admin_tenant_get,
		admin_tenant_update,
		admin_tenant_delete,
		admin_tenant_quota,
		admin_api_key_tenant,
		admin_api_key_get,
		admin_api_key_update,
		admin_api_key_delete,
		admin_api_key_reactivate,
		domain_get,
		domain_update,
		domain_delete,
	}
}

pub async fn build_runtime(db_url: &str, amqp_url: &str) -> HarnessRuntime {
	HarnessRuntime {
		public: build_test_config(ConfigProfile::Public, None, None).await,
		db_only: build_test_config(ConfigProfile::DbOnly, Some(db_url), None).await,
		bearer: build_test_config(ConfigProfile::BearerTenant, Some(db_url), None).await,
		admin: build_test_config(ConfigProfile::AdminSecret, Some(db_url), None).await,
		pseudo_worker: build_test_config(ConfigProfile::PseudoWorker, Some(db_url), None).await,
		worker_rabbit: build_test_config(ConfigProfile::WorkerRabbit, Some(db_url), Some(amqp_url))
			.await,
		pipeline_enabled: build_test_config(
			ConfigProfile::PipelineEnabled,
			Some(db_url),
			Some(amqp_url),
		)
		.await,
	}
}

pub async fn seed_upgrade_fixtures(pool: &PgPool) -> HarnessFixtures {
	let tenant_id = Uuid::parse_str(UPGRADE_TENANT_ID).expect("valid upgrade tenant id");
	let tenant = insert_keys_for_existing_tenant(pool, tenant_id).await;
	let comment_delete_id =
		insert_comment(pool, tenant_id, Some(UPGRADE_JOB_ID), None, "Upgrade fixture comment").await;
	let pipeline_main = insert_pipeline(
		pool,
		tenant_id,
		"Upgrade Pipeline",
		serde_json::json!({"type": "list_snapshot", "list_id": UPGRADE_LIST_ID}),
	)
	.await;
	let pipeline_run = insert_pipeline_run(
		pool,
		pipeline_main,
		tenant_id,
		"completed",
		Some(UPGRADE_JOB_ID),
		Some(UPGRADE_LIST_ID),
	)
	.await;
	sqlx::query(
		r#"
		INSERT INTO v1_usage_events (
			tenant_id,
			pipeline_id,
			pipeline_run_id,
			job_id,
			source,
			reserved_emails,
			committed_emails,
			status,
			metadata
		)
		VALUES ($1, $2, $3, $4, 'pipeline', 3, 3, 'committed', '{}'::jsonb)
		"#,
	)
	.bind(tenant_id)
	.bind(pipeline_main)
	.bind(pipeline_run)
	.bind(UPGRADE_JOB_ID)
	.execute(pool)
	.await
	.expect("insert upgrade pipeline usage event failed");

	HarnessFixtures {
		tenant,
		legacy_status_job: 0,
		legacy_results_job: 0,
		job_main: UPGRADE_JOB_ID,
		job_cancel: 0,
		job_retry: 0,
		job_cancelled: 0,
		finder_job: 0,
		list_main: UPGRADE_LIST_ID,
		list_delete: 0,
		pipeline_main,
		pipeline_pause: 0,
		pipeline_resume: 0,
		pipeline_trigger: 0,
		pipeline_delete: 0,
		pipeline_active_conflict: 0,
		pipeline_run,
		suppression_id: 0,
		comment_delete_id,
		self_api_key_get: Uuid::nil(),
		self_api_key_update: Uuid::nil(),
		self_api_key_delete: Uuid::nil(),
		admin_tenant_get: Uuid::nil(),
		admin_tenant_update: Uuid::nil(),
		admin_tenant_delete: Uuid::nil(),
		admin_tenant_quota: Uuid::nil(),
		admin_api_key_tenant: Uuid::nil(),
		admin_api_key_get: Uuid::nil(),
		admin_api_key_update: Uuid::nil(),
		admin_api_key_delete: Uuid::nil(),
		admin_api_key_reactivate: Uuid::nil(),
		domain_get: String::new(),
		domain_update: String::new(),
		domain_delete: String::new(),
	}
}

pub fn canonical_cases() -> Vec<HarnessCase> {
	vec![
		upgrade_case!(
			"GET",
			"/healthz",
			ConfigProfile::Public,
			AuthProfile::None,
			PathProfile::Literal("/healthz"),
			BodyProfile::None,
			200,
			Expectation::Json(&["status"])
		),
		upgrade_case!(
			"GET",
			"/readyz",
			ConfigProfile::Public,
			AuthProfile::None,
			PathProfile::Literal("/readyz"),
			BodyProfile::None,
			200,
			Expectation::Json(&["status", "checks"])
		),
		case!(
			"GET",
			"/version",
			ConfigProfile::Public,
			AuthProfile::None,
			PathProfile::Literal("/version"),
			BodyProfile::None,
			200,
			Expectation::Json(&["version"])
		),
		upgrade_case!(
			"GET",
			"/openapi.json",
			ConfigProfile::Public,
			AuthProfile::None,
			PathProfile::Literal("/openapi.json"),
			BodyProfile::None,
			200,
			Expectation::OpenApi
		),
		case!(
			"POST",
			"/v0/check_email",
			ConfigProfile::DbOnly,
			AuthProfile::Secret,
			PathProfile::Literal("/v0/check_email"),
			BodyProfile::JsonV0CheckEmail,
			200,
			Expectation::Json(&["is_reachable", "score"])
		),
		case!(
			"POST",
			"/v0/bulk",
			ConfigProfile::DbOnly,
			AuthProfile::Secret,
			PathProfile::Literal("/v0/bulk"),
			BodyProfile::JsonV0BulkCreate,
			200,
			Expectation::Json(&["job_id"])
		),
		case!(
			"GET",
			"/v0/bulk/{job_id}",
			ConfigProfile::DbOnly,
			AuthProfile::Secret,
			PathProfile::LegacyBulkStatus,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "summary", "job_status"])
		),
		case!(
			"GET",
			"/v0/bulk/{job_id}/results",
			ConfigProfile::DbOnly,
			AuthProfile::Secret,
			PathProfile::LegacyBulkResults,
			BodyProfile::None,
			200,
			Expectation::Json(&["results"])
		),
		case!(
			"POST",
			"/v1/check-email-with-onboard",
			ConfigProfile::DbOnly,
			AuthProfile::None,
			PathProfile::Literal("/v1/check-email-with-onboard"),
			BodyProfile::JsonOnboard,
			201,
			Expectation::Json(&["tenant", "api_key", "verification_result"])
		),
		case!(
			"POST",
			"/v1/check_email",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/check_email"),
			BodyProfile::JsonV1CheckEmail,
			200,
			Expectation::Json(&["is_reachable", "score"])
		),
		case!(
			"POST",
			"/v1/find_email",
			ConfigProfile::WorkerRabbit,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/find_email"),
			BodyProfile::JsonFindEmailCreate,
			202,
			Expectation::Json(&["job_id", "bulk_job_id", "status"])
		),
		case!(
			"GET",
			"/v1/find_email/{job_id}",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::FinderGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "results", "best_match"])
		),
		case!(
			"POST",
			"/v1/lists",
			ConfigProfile::WorkerRabbit,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/lists"),
			BodyProfile::MultipartListUpload,
			202,
			Expectation::Json(&["list_id", "job_id", "total_rows"])
		),
		case!(
			"GET",
			"/v1/lists",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/lists"),
			BodyProfile::None,
			200,
			Expectation::Json(&["lists", "total"])
		),
		upgrade_case!(
			"GET",
			"/v1/lists/{list_id}",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::ListGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "summary"])
		),
		upgrade_case!(
			"GET",
			"/v1/lists/{list_id}/quality",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::ListQuality,
			BodyProfile::None,
			200,
			Expectation::Json(&["list_id", "quality_grade", "categories"])
		),
		case!(
			"GET",
			"/v1/lists/{list_id}/download",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::ListDownload,
			BodyProfile::None,
			200,
			Expectation::Csv
		),
		case!(
			"DELETE",
			"/v1/lists/{list_id}",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::ListDelete,
			BodyProfile::None,
			200,
			Expectation::Json(&["deleted"])
		),
		case!(
			"POST",
			"/v1/pipelines",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/pipelines"),
			BodyProfile::JsonPipelineCreate,
			201,
			Expectation::Json(&["id", "name", "source"])
		),
		upgrade_case!(
			"GET",
			"/v1/pipelines",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/pipelines"),
			BodyProfile::None,
			200,
			Expectation::Json(&["pipelines", "total"])
		),
		upgrade_case!(
			"GET",
			"/v1/pipelines/{pipeline_id}",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::PipelineGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "status", "source"])
		),
		case!(
			"PATCH",
			"/v1/pipelines/{pipeline_id}",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::PipelinePatch,
			BodyProfile::JsonPipelinePatch,
			200,
			Expectation::Json(&["id", "name", "status"])
		),
		case!(
			"DELETE",
			"/v1/pipelines/{pipeline_id}",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::PipelineDelete,
			BodyProfile::None,
			200,
			Expectation::Json(&["deleted"])
		),
		case!(
			"POST",
			"/v1/pipelines/{pipeline_id}/pause",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::PipelinePause,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "status"])
		),
		case!(
			"POST",
			"/v1/pipelines/{pipeline_id}/resume",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::PipelineResume,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "status"])
		),
		case!(
			"POST",
			"/v1/pipelines/{pipeline_id}/trigger",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::PipelineTrigger,
			BodyProfile::JsonPipelineTrigger,
			202,
			Expectation::Json(&["run_id", "status"])
		),
		upgrade_case!(
			"GET",
			"/v1/pipelines/{pipeline_id}/runs",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::PipelineRuns,
			BodyProfile::None,
			200,
			Expectation::Json(&["runs", "total"])
		),
		upgrade_case!(
			"GET",
			"/v1/pipelines/{pipeline_id}/runs/{run_id}",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerFull,
			PathProfile::PipelineRunGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "pipeline_id", "status"])
		),
		case!(
			"POST",
			"/v1/reputation/check",
			ConfigProfile::DbOnly,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/reputation/check"),
			BodyProfile::JsonReputationCheck,
			200,
			Expectation::Json(&["domain", "score", "cached"])
		),
		case!(
			"POST",
			"/v1/suppressions",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/suppressions"),
			BodyProfile::JsonSuppressionsAdd,
			200,
			Expectation::Json(&["added", "duplicates"])
		),
		case!(
			"GET",
			"/v1/suppressions/check",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/suppressions/check?email=suppressed%40example.com"),
			BodyProfile::None,
			200,
			Expectation::Json(&["suppressed"])
		),
		case!(
			"GET",
			"/v1/suppressions",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/suppressions"),
			BodyProfile::None,
			200,
			Expectation::Json(&["entries", "total"])
		),
		case!(
			"DELETE",
			"/v1/suppressions/{id}",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::SuppressionDelete,
			BodyProfile::None,
			200,
			Expectation::Json(&["deleted"])
		),
		case!(
			"POST",
			"/v1/bulk",
			ConfigProfile::WorkerRabbit,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/bulk"),
			BodyProfile::JsonV1BulkCreate,
			200,
			Expectation::Json(&["job_id"])
		),
		case!(
			"GET",
			"/v1/bulk/{job_id}",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "task_summary"])
		),
		case!(
			"GET",
			"/v1/bulk/{job_id}/results",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobResults,
			BodyProfile::None,
			200,
			Expectation::Json(&["results"])
		),
		case!(
			"GET",
			"/v1/reverification/status",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/reverification/status"),
			BodyProfile::None,
			200,
			Expectation::Json(&["enabled"])
		),
		case!(
			"GET",
			"/v1/events",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/events"),
			BodyProfile::None,
			200,
			Expectation::Json(&["events", "total"])
		),
		case!(
			"GET",
			"/v1/emails/{email}/history",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::EmailHistory,
			BodyProfile::None,
			200,
			Expectation::Json(&["email", "history", "total"])
		),
		case!(
			"GET",
			"/v1/query",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/query?job_id=1"),
			BodyProfile::None,
			200,
			Expectation::Json(&["results", "total"])
		),
		case!(
			"POST",
			"/v1/comments",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/comments"),
			BodyProfile::JsonCommentsCreate,
			201,
			Expectation::Json(&["id", "body"])
		),
		upgrade_case!(
			"GET",
			"/v1/comments",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/comments?job_id=1"),
			BodyProfile::None,
			200,
			Expectation::Json(&["comments", "total"])
		),
		case!(
			"DELETE",
			"/v1/comments/{comment_id}",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::CommentDelete,
			BodyProfile::None,
			200,
			Expectation::Json(&["deleted"])
		),
		upgrade_case!(
			"GET",
			"/v1/jobs/{job_id}",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "task_summary"])
		),
		case!(
			"POST",
			"/v1/jobs/{job_id}/cancel",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobCancel,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "tasks_cancelled", "status"])
		),
		case!(
			"GET",
			"/v1/jobs/{job_id}/events",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobEvents,
			BodyProfile::None,
			200,
			Expectation::Json(&["events", "total"])
		),
		upgrade_case!(
			"GET",
			"/v1/jobs/{job_id}/results",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobResults,
			BodyProfile::None,
			200,
			Expectation::Json(&["results"])
		),
		case!(
			"GET",
			"/v1/jobs/{job_id}/download",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobDownload,
			BodyProfile::None,
			200,
			Expectation::Csv
		),
		case!(
			"POST",
			"/v1/jobs/{job_id}/retry",
			ConfigProfile::WorkerRabbit,
			AuthProfile::BearerFull,
			PathProfile::JobRetry,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "tasks_retried", "status"])
		),
		case!(
			"GET",
			"/v1/jobs/{job_id}/approval",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobApproval,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "recommendation", "ready_to_send"])
		),
		case!(
			"GET",
			"/v1/jobs/{job_id}/latency",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobLatency,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "avg_duration_ms", "p95_duration_ms"])
		),
		upgrade_case!(
			"GET",
			"/v1/me",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me"),
			BodyProfile::None,
			200,
			Expectation::Json(&["tenant_id", "tenant_name"])
		),
		case!(
			"GET",
			"/v1/me/settings",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/settings"),
			BodyProfile::None,
			200,
			Expectation::Json(&["tenant_id", "slug"])
		),
		case!(
			"PATCH",
			"/v1/me/settings",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/settings"),
			BodyProfile::JsonMeSettingsPatch,
			200,
			Expectation::Json(&["tenant_id", "result_retention_days"])
		),
		case!(
			"GET",
			"/v1/me/webhook",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/webhook"),
			BodyProfile::None,
			200,
			Expectation::Json(&["tenant_id", "webhook_signing_secret_configured"])
		),
		case!(
			"PATCH",
			"/v1/me/webhook",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/webhook"),
			BodyProfile::JsonMeWebhookPatch,
			200,
			Expectation::Json(&["tenant_id", "default_webhook_url"])
		),
		case!(
			"DELETE",
			"/v1/me/webhook",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/webhook"),
			BodyProfile::None,
			200,
			Expectation::Json(&["tenant_id", "webhook_cleared"])
		),
		case!(
			"GET",
			"/v1/me/usage",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/usage"),
			BodyProfile::None,
			200,
			Expectation::Json(&["tenant_id", "quota_unlimited"])
		),
		case!(
			"GET",
			"/v1/me/domains",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/domains"),
			BodyProfile::None,
			200,
			Expectation::Json(&["domains"])
		),
		case!(
			"POST",
			"/v1/me/domains",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/domains"),
			BodyProfile::JsonMeDomainCreate,
			201,
			Expectation::Json(&["id", "domain"])
		),
		case!(
			"GET",
			"/v1/me/domains/{domain}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::DomainGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "domain"])
		),
		case!(
			"PATCH",
			"/v1/me/domains/{domain}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::DomainPatch,
			BodyProfile::JsonMeDomainPatch,
			200,
			Expectation::Json(&["id", "domain"])
		),
		case!(
			"DELETE",
			"/v1/me/domains/{domain}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::DomainDelete,
			BodyProfile::None,
			200,
			Expectation::Json(&["deleted"])
		),
		case!(
			"GET",
			"/v1/me/api-keys",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/api-keys"),
			BodyProfile::None,
			200,
			Expectation::Json(&["api_keys"])
		),
		case!(
			"GET",
			"/v1/me/api-keys/{key_id}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::SelfApiKeyGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "key_prefix"])
		),
		case!(
			"POST",
			"/v1/me/api-keys",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/api-keys"),
			BodyProfile::JsonMeApiKeyCreate,
			201,
			Expectation::Json(&["id", "key", "key_prefix"])
		),
		case!(
			"PATCH",
			"/v1/me/api-keys/{key_id}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::SelfApiKeyPatch,
			BodyProfile::JsonMeApiKeyPatch,
			200,
			Expectation::Json(&["id", "name"])
		),
		case!(
			"DELETE",
			"/v1/me/api-keys/{key_id}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::SelfApiKeyDelete,
			BodyProfile::None,
			200,
			Expectation::Json(&["revoked", "key_id"])
		),
		case!(
			"POST",
			"/v1/admin/tenants",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::Literal("/v1/admin/tenants"),
			BodyProfile::JsonAdminTenantCreate,
			201,
			Expectation::Json(&["id", "slug"])
		),
		case!(
			"GET",
			"/v1/admin/tenants",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::Literal("/v1/admin/tenants"),
			BodyProfile::None,
			200,
			Expectation::Json(&["tenants", "total"])
		),
		case!(
			"GET",
			"/v1/admin/tenants/{tenant_id}",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "slug"])
		),
		case!(
			"PUT",
			"/v1/admin/tenants/{tenant_id}",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantPut,
			BodyProfile::JsonAdminTenantUpdate,
			200,
			Expectation::Json(&["id", "name"])
		),
		case!(
			"DELETE",
			"/v1/admin/tenants/{tenant_id}",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantDelete,
			BodyProfile::None,
			200,
			Expectation::Json(&["deleted"])
		),
		case!(
			"GET",
			"/v1/admin/tenants/{tenant_id}/quota",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminQuotaGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["tenant_id", "remaining_quota"])
		),
		case!(
			"PATCH",
			"/v1/admin/tenants/{tenant_id}/quota",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminQuotaPatch,
			BodyProfile::JsonAdminQuotaPatch,
			200,
			Expectation::Json(&["tenant_id", "monthly_email_limit"])
		),
		case!(
			"POST",
			"/v1/admin/tenants/{tenant_id}/quota/reset",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminQuotaReset,
			BodyProfile::None,
			200,
			Expectation::Json(&["tenant_id", "used_this_period"])
		),
		case!(
			"GET",
			"/v1/admin/jobs",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::Literal("/v1/admin/jobs"),
			BodyProfile::None,
			200,
			Expectation::Json(&["jobs", "total"])
		),
		case!(
			"GET",
			"/v1/admin/jobs/{job_id}",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminJobGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["job_id", "status"])
		),
		case!(
			"GET",
			"/v1/admin/jobs/{job_id}/events",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminJobEvents,
			BodyProfile::None,
			200,
			Expectation::Json(&["events", "total"])
		),
		case!(
			"GET",
			"/v1/admin/jobs/{job_id}/results",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminJobResults,
			BodyProfile::None,
			200,
			Expectation::Json(&["results"])
		),
		case!(
			"GET",
			"/v1/admin/tenants/{tenant_id}/jobs",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantJobs,
			BodyProfile::None,
			200,
			Expectation::Json(&["jobs", "total"])
		),
		case!(
			"GET",
			"/v1/admin/api-keys",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::Literal("/v1/admin/api-keys"),
			BodyProfile::None,
			200,
			Expectation::Json(&["api_keys", "total"])
		),
		case!(
			"POST",
			"/v1/admin/tenants/{tenant_id}/api-keys",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantApiKeysCreate,
			BodyProfile::JsonAdminApiKeyCreate,
			201,
			Expectation::Json(&["id", "key", "key_prefix"])
		),
		case!(
			"GET",
			"/v1/admin/tenants/{tenant_id}/api-keys",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantApiKeysList,
			BodyProfile::None,
			200,
			Expectation::Json(&["api_keys"])
		),
		case!(
			"GET",
			"/v1/admin/tenants/{tenant_id}/api-keys/{key_id}",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantApiKeyGet,
			BodyProfile::None,
			200,
			Expectation::Json(&["id", "key_prefix"])
		),
		case!(
			"PATCH",
			"/v1/admin/tenants/{tenant_id}/api-keys/{key_id}",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantApiKeyPatch,
			BodyProfile::JsonAdminApiKeyPatch,
			200,
			Expectation::Json(&["id", "name"])
		),
		case!(
			"DELETE",
			"/v1/admin/tenants/{tenant_id}/api-keys/{key_id}",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantApiKeyDelete,
			BodyProfile::None,
			200,
			Expectation::Json(&["revoked", "key_id"])
		),
		case!(
			"POST",
			"/v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecret,
			PathProfile::AdminTenantApiKeyReactivate,
			BodyProfile::None,
			200,
			Expectation::Json(&["key_id", "reactivated"])
		),
	]
}

pub fn upgrade_safe_cases() -> Vec<HarnessCase> {
	canonical_cases()
		.into_iter()
		.filter(|case| case.upgrade_safe)
		.collect()
}

fn render_path(path: PathProfile, fixtures: &HarnessFixtures) -> String {
	match path {
		PathProfile::Literal(path) => {
			if path == "/v1/query?job_id=1" {
				format!("/v1/query?job_id={}", fixtures.job_main)
			} else if path == "/v1/comments?job_id=1" {
				format!("/v1/comments?job_id={}", fixtures.job_main)
			} else {
				path.to_string()
			}
		}
		PathProfile::LegacyBulkStatus => format!("/v0/bulk/{}", fixtures.legacy_status_job),
		PathProfile::LegacyBulkResults => {
			format!(
				"/v0/bulk/{}/results?format=json",
				fixtures.legacy_results_job
			)
		}
		PathProfile::FinderGet => format!("/v1/find_email/{}", fixtures.finder_job),
		PathProfile::ListGet => format!("/v1/lists/{}", fixtures.list_main),
		PathProfile::ListQuality => format!("/v1/lists/{}/quality", fixtures.list_main),
		PathProfile::ListDownload => format!("/v1/lists/{}/download", fixtures.list_main),
		PathProfile::ListDelete => format!("/v1/lists/{}", fixtures.list_delete),
		PathProfile::PipelineGet => format!("/v1/pipelines/{}", fixtures.pipeline_main),
		PathProfile::PipelinePatch => format!("/v1/pipelines/{}", fixtures.pipeline_main),
		PathProfile::PipelineDelete => format!("/v1/pipelines/{}", fixtures.pipeline_delete),
		PathProfile::PipelinePause => format!("/v1/pipelines/{}/pause", fixtures.pipeline_pause),
		PathProfile::PipelineResume => {
			format!("/v1/pipelines/{}/resume", fixtures.pipeline_resume)
		}
		PathProfile::PipelineTrigger => {
			format!("/v1/pipelines/{}/trigger", fixtures.pipeline_trigger)
		}
		PathProfile::PipelineRuns => format!("/v1/pipelines/{}/runs", fixtures.pipeline_main),
		PathProfile::PipelineRunGet => {
			format!(
				"/v1/pipelines/{}/runs/{}",
				fixtures.pipeline_main, fixtures.pipeline_run
			)
		}
		PathProfile::SuppressionDelete => format!("/v1/suppressions/{}", fixtures.suppression_id),
		PathProfile::CommentDelete => format!("/v1/comments/{}", fixtures.comment_delete_id),
		PathProfile::JobGet => format!("/v1/jobs/{}", fixtures.job_main),
		PathProfile::JobCancel => format!("/v1/jobs/{}/cancel", fixtures.job_cancel),
		PathProfile::JobEvents => format!("/v1/jobs/{}/events", fixtures.job_main),
		PathProfile::JobResults => format!("/v1/jobs/{}/results", fixtures.job_main),
		PathProfile::JobDownload => format!("/v1/jobs/{}/download", fixtures.job_main),
		PathProfile::JobRetry => format!("/v1/jobs/{}/retry", fixtures.job_retry),
		PathProfile::JobRetryCancelled => {
			format!("/v1/jobs/{}/retry", fixtures.job_cancelled)
		}
		PathProfile::JobApproval => format!("/v1/jobs/{}/approval", fixtures.job_main),
		PathProfile::JobLatency => format!("/v1/jobs/{}/latency", fixtures.job_main),
		PathProfile::JobCancelCompleted => {
			format!("/v1/jobs/{}/cancel", fixtures.job_main)
		}
		PathProfile::EmailHistory => "/v1/emails/good%40example.com/history".to_string(),
		PathProfile::DomainGet => format!("/v1/me/domains/{}", fixtures.domain_get),
		PathProfile::DomainPatch => format!("/v1/me/domains/{}", fixtures.domain_update),
		PathProfile::DomainDelete => format!("/v1/me/domains/{}", fixtures.domain_delete),
		PathProfile::SelfApiKeyGet => format!("/v1/me/api-keys/{}", fixtures.self_api_key_get),
		PathProfile::SelfApiKeyPatch => {
			format!("/v1/me/api-keys/{}", fixtures.self_api_key_update)
		}
		PathProfile::SelfApiKeyDelete => {
			format!("/v1/me/api-keys/{}", fixtures.self_api_key_delete)
		}
		PathProfile::AdminTenantGet => {
			format!("/v1/admin/tenants/{}", fixtures.admin_tenant_get)
		}
		PathProfile::AdminTenantPut => {
			format!("/v1/admin/tenants/{}", fixtures.admin_tenant_update)
		}
		PathProfile::AdminTenantDelete => {
			format!("/v1/admin/tenants/{}", fixtures.admin_tenant_delete)
		}
		PathProfile::AdminQuotaGet => {
			format!("/v1/admin/tenants/{}/quota", fixtures.admin_tenant_quota)
		}
		PathProfile::AdminQuotaPatch => {
			format!("/v1/admin/tenants/{}/quota", fixtures.admin_tenant_quota)
		}
		PathProfile::AdminQuotaReset => {
			format!(
				"/v1/admin/tenants/{}/quota/reset",
				fixtures.admin_tenant_quota
			)
		}
		PathProfile::AdminJobGet => format!("/v1/admin/jobs/{}", fixtures.job_main),
		PathProfile::AdminJobEvents => format!("/v1/admin/jobs/{}/events", fixtures.job_main),
		PathProfile::AdminJobResults => format!("/v1/admin/jobs/{}/results", fixtures.job_main),
		PathProfile::AdminTenantJobs => {
			format!("/v1/admin/tenants/{}/jobs", fixtures.tenant.tenant_id)
		}
		PathProfile::AdminTenantApiKeysCreate => format!(
			"/v1/admin/tenants/{}/api-keys",
			fixtures.admin_api_key_tenant
		),
		PathProfile::AdminTenantApiKeysList => format!(
			"/v1/admin/tenants/{}/api-keys",
			fixtures.admin_api_key_tenant
		),
		PathProfile::AdminTenantApiKeyGet => format!(
			"/v1/admin/tenants/{}/api-keys/{}",
			fixtures.admin_api_key_tenant, fixtures.admin_api_key_get
		),
		PathProfile::AdminTenantApiKeyPatch => format!(
			"/v1/admin/tenants/{}/api-keys/{}",
			fixtures.admin_api_key_tenant, fixtures.admin_api_key_update
		),
		PathProfile::AdminTenantApiKeyDelete => format!(
			"/v1/admin/tenants/{}/api-keys/{}",
			fixtures.admin_api_key_tenant, fixtures.admin_api_key_delete
		),
		PathProfile::AdminTenantApiKeyReactivate => format!(
			"/v1/admin/tenants/{}/api-keys/{}/reactivate",
			fixtures.admin_api_key_tenant, fixtures.admin_api_key_reactivate
		),
		PathProfile::PipelineTriggerConflict => format!(
			"/v1/pipelines/{}/trigger",
			fixtures.pipeline_active_conflict
		),
	}
}

fn apply_auth(
	builder: RequestBuilder,
	auth: AuthProfile,
	fixtures: &HarnessFixtures,
) -> RequestBuilder {
	match auth {
		AuthProfile::None => builder,
		AuthProfile::Secret => {
			builder.header(REACHER_SECRET_HEADER, crate::test_helpers::TEST_SECRET)
		}
		AuthProfile::AdminSecret => builder.header(REACHER_SECRET_HEADER, ADMIN_SECRET),
		AuthProfile::AdminSecretInvalid => builder.header(REACHER_SECRET_HEADER, "wrong-secret"),
		AuthProfile::BearerFull => builder.header(
			"Authorization",
			format!("Bearer {}", fixtures.tenant.full_access_key),
		),
		AuthProfile::BearerBulk => builder.header(
			"Authorization",
			format!("Bearer {}", fixtures.tenant.bulk_key),
		),
		AuthProfile::BearerLists => builder.header(
			"Authorization",
			format!("Bearer {}", fixtures.tenant.lists_key),
		),
		AuthProfile::BearerVerify => builder.header(
			"Authorization",
			format!("Bearer {}", fixtures.tenant.verify_key),
		),
		AuthProfile::BearerPipelines => builder.header(
			"Authorization",
			format!("Bearer {}", fixtures.tenant.pipelines_key),
		),
	}
}

fn multipart_body() -> (String, Vec<u8>) {
	let boundary = "----reacher-harness-boundary";
	let body = format!(
		"--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"contacts.csv\"\r\nContent-Type: text/csv\r\n\r\nemail,name\r\nupload@example.com,Upload User\r\n--{boundary}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\nHarness Upload\r\n--{boundary}--\r\n"
	);
	(boundary.to_string(), body.into_bytes())
}

fn apply_body(
	builder: RequestBuilder,
	body: BodyProfile,
	fixtures: &HarnessFixtures,
) -> RequestBuilder {
	match body {
		BodyProfile::None => builder,
		BodyProfile::JsonV0CheckEmail => builder.json(&serde_json::json!({
			"to_email": "sandbox@example.com",
			"sandbox": true
		})),
		BodyProfile::JsonV0BulkCreate => builder.json(&serde_json::json!({
			"input_type": "email",
			"input": ["legacy1@example.com", "legacy2@example.com"]
		})),
		BodyProfile::JsonOnboard => builder.json(&serde_json::json!({
			"email_to_verify": "not-an-email",
			"tenant_name": "Harness Onboard",
			"contact_email": "owner@harness.test",
			"slug": "harness-onboard"
		})),
		BodyProfile::JsonV1CheckEmail => builder.json(&serde_json::json!({
			"to_email": "sandbox@example.com",
			"sandbox": true
		})),
		BodyProfile::JsonFindEmailCreate => builder.json(&serde_json::json!({
			"first_name": "Jane",
			"last_name": "Doe",
			"domain": "gmail.com",
			"strategy": "parallel"
		})),
		BodyProfile::JsonFindEmailInvalidStrategy => builder.json(&serde_json::json!({
			"first_name": "Jane",
			"last_name": "Doe",
			"domain": "gmail.com",
			"strategy": "spiral"
		})),
		BodyProfile::MultipartListUpload => {
			let (boundary, body) = multipart_body();
			builder
				.header(
					"content-type",
					format!("multipart/form-data; boundary={boundary}"),
				)
				.body(body)
		}
		BodyProfile::JsonPipelineCreate => builder.json(&serde_json::json!({
			"name": "Harness Pipeline Created",
			"status": "active",
			"source": {"type": "list_snapshot", "list_id": fixtures.list_main},
			"schedule": {"cron": "0 * * * *", "timezone": "UTC"},
			"verification": {"delta_mode": false},
			"policy": {"missed_run_window_hours": 24},
			"delivery": {"dashboard": true}
		})),
		BodyProfile::JsonPipelinePatch => builder.json(&serde_json::json!({
			"name": "Harness Pipeline Renamed"
		})),
		BodyProfile::JsonPipelineTrigger => builder.json(&serde_json::json!({
			"force": true,
			"reason": "harness"
		})),
		BodyProfile::JsonPipelineTriggerConflict => builder.json(&serde_json::json!({
			"reason": "active run conflict"
		})),
		BodyProfile::JsonReputationCheck => builder.json(&serde_json::json!({
			"domain": "cached.example.com",
			"force_refresh": false
		})),
		BodyProfile::JsonSuppressionsAdd => builder.json(&serde_json::json!({
			"emails": ["new-suppression@example.com"],
			"reason": "manual"
		})),
		BodyProfile::JsonV1BulkCreate => builder.json(&serde_json::json!({
			"input": ["bulk1@example.com", "bulk2@example.com"]
		})),
		BodyProfile::JsonCommentsCreate => builder.json(&serde_json::json!({
			"job_id": fixtures.job_main,
			"body": "Harness created comment",
			"author": "Harness"
		})),
		BodyProfile::JsonCommentsEmpty => builder.json(&serde_json::json!({
			"job_id": fixtures.job_main,
			"body": "   ",
			"author": "Harness"
		})),
		BodyProfile::JsonMeSettingsPatch => builder.json(&serde_json::json!({
			"result_retention_days": 45
		})),
		BodyProfile::JsonMeWebhookPatch => builder.json(&serde_json::json!({
			"default_webhook_url": "https://example.com/webhook",
			"webhook_signing_secret": "secret"
		})),
		BodyProfile::JsonMeDomainCreate => builder.json(&serde_json::json!({
			"domain": "created.example.com",
			"is_active": true
		})),
		BodyProfile::JsonMeDomainCreateDuplicate => builder.json(&serde_json::json!({
			"domain": "existing.example.com",
			"is_active": true
		})),
		BodyProfile::JsonMeDomainPatch => builder.json(&serde_json::json!({
			"notes": "patched"
		})),
		BodyProfile::JsonMeDomainPatchConflict => builder.json(&serde_json::json!({
			"domain": fixtures.domain_get
		})),
		BodyProfile::JsonMeApiKeyCreate => builder.json(&serde_json::json!({
			"name": "Harness API Key",
			"scopes": ["verify"]
		})),
		BodyProfile::JsonMeApiKeyPatch => builder.json(&serde_json::json!({
			"name": "Updated Harness Key"
		})),
		BodyProfile::JsonEmptyObject => builder.json(&serde_json::json!({})),
		BodyProfile::MalformedJson => builder.header("content-type", "application/json").body("{"),
		BodyProfile::JsonAdminTenantCreate => builder.json(&serde_json::json!({
			"name": "Harness Admin Tenant",
			"slug": "harness-admin-created",
			"contact_email": "admin-created@example.com",
			"plan_tier": "starter"
		})),
		BodyProfile::JsonAdminTenantUpdate => builder.json(&serde_json::json!({
			"name": "Harness Updated Tenant"
		})),
		BodyProfile::JsonAdminQuotaPatch => builder.json(&serde_json::json!({
			"monthly_email_limit": 1200
		})),
		BodyProfile::JsonAdminApiKeyCreate => builder.json(&serde_json::json!({
			"name": "Admin Created Key",
			"scopes": ["verify"]
		})),
		BodyProfile::JsonAdminApiKeyPatch => builder.json(&serde_json::json!({
			"name": "Admin Updated Key"
		})),
	}
}

fn config_for_case<'a>(
	runtime: &'a HarnessRuntime,
	profile: ConfigProfile,
) -> &'a Arc<reacher_backend::config::BackendConfig> {
	match profile {
		ConfigProfile::Public => &runtime.public,
		ConfigProfile::DbOnly => &runtime.db_only,
		ConfigProfile::BearerTenant => &runtime.bearer,
		ConfigProfile::AdminSecret => &runtime.admin,
		ConfigProfile::PseudoWorker => &runtime.pseudo_worker,
		ConfigProfile::WorkerRabbit => &runtime.worker_rabbit,
		ConfigProfile::PipelineEnabled => &runtime.pipeline_enabled,
	}
}

fn assert_json_fields(value: &serde_json::Value, fields: &[&str]) {
	let object = value.as_object().expect("response must be a JSON object");
	for field in fields {
		assert!(
			object.contains_key(*field),
			"response object missing field '{}': {}",
			field,
			value
		);
	}
}

fn header_prefix(headers: &HeaderMap, name: &str, expected_prefix: &str) {
	let value = headers
		.get(name)
		.unwrap_or_else(|| panic!("missing header '{}'", name))
		.to_str()
		.expect("header to_str");
	assert!(
		value.starts_with(expected_prefix),
		"header '{}' expected prefix '{}' but got '{}'",
		name,
		expected_prefix,
		value
	);
}

pub fn inventory_keys() -> BTreeSet<(String, String)> {
	all_route_specs()
		.iter()
		.map(|spec| (spec.key.method.to_string(), spec.key.path.to_string()))
		.collect()
}

pub fn harness_keys(cases: &[HarnessCase]) -> BTreeSet<(String, String)> {
	cases
		.iter()
		.map(|case| (case.key.method.to_string(), case.key.path.to_string()))
		.collect()
}

pub fn openapi_keys() -> BTreeSet<(String, String)> {
	let spec = build_spec().expect("runtime openapi build");
	let paths = spec["paths"].as_object().expect("openapi paths object");
	let mut keys = BTreeSet::new();
	for (path, value) in paths {
		let methods = value.as_object().expect("path item object");
		for method in methods.keys() {
			keys.insert((method.to_ascii_uppercase(), path.clone()));
		}
	}
	keys
}

pub fn auth_failure_cases() -> Vec<HarnessCase> {
	vec![
		case!(
			"GET",
			"/v1/me",
			ConfigProfile::BearerTenant,
			AuthProfile::None,
			PathProfile::Literal("/v1/me"),
			BodyProfile::None,
			401,
			Expectation::Error(
				"Authentication required. Provide Authorization: Bearer <api_key> or x-reacher-secret header.",
			),
		),
		case!(
			"GET",
			"/v1/admin/tenants",
			ConfigProfile::AdminSecret,
			AuthProfile::AdminSecretInvalid,
			PathProfile::Literal("/v1/admin/tenants"),
			BodyProfile::None,
			400,
			Expectation::StatusOnly,
		),
		case!(
			"GET",
			"/v1/events",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerVerify,
			PathProfile::Literal("/v1/events"),
			BodyProfile::None,
			403,
			Expectation::Error("API key lacks required scope: bulk"),
		),
		case!(
			"GET",
			"/v1/lists/{list_id}/quality",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerBulk,
			PathProfile::ListQuality,
			BodyProfile::None,
			403,
			Expectation::Error("API key lacks required scope: lists"),
		),
	]
}

pub fn validation_failure_cases() -> Vec<HarnessCase> {
	vec![
		case!(
			"GET",
			"/v1/events?since=bad-date",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerBulk,
			PathProfile::Literal("/v1/events?since=bad-date"),
			BodyProfile::None,
			400,
			Expectation::Error("Invalid 'since' date format. Expected RFC3339."),
		),
		case!(
			"GET",
			"/v1/query?until=bad-date",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerBulk,
			PathProfile::Literal("/v1/query?until=bad-date"),
			BodyProfile::None,
			400,
			Expectation::Error("Invalid 'until' date format. Expected RFC3339."),
		),
		case!(
			"POST",
			"/v1/find_email",
			ConfigProfile::WorkerRabbit,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/find_email"),
			BodyProfile::JsonFindEmailInvalidStrategy,
			400,
			Expectation::Error("Invalid strategy 'spiral'. Expected 'parallel' or 'waterfall'."),
		),
		case!(
			"POST",
			"/v1/comments",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/comments"),
			BodyProfile::JsonCommentsEmpty,
			400,
			Expectation::Error("Comment body cannot be empty"),
		),
		case!(
			"PATCH",
			"/v1/me/api-keys/{key_id}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::SelfApiKeyPatch,
			BodyProfile::JsonEmptyObject,
			400,
			Expectation::Error("No fields to update"),
		),
		case!(
			"PATCH",
			"/v1/me/api-keys/{key_id}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::SelfApiKeyPatch,
			BodyProfile::MalformedJson,
			400,
			Expectation::StatusOnly,
		),
	]
}

pub fn missing_resource_cases() -> Vec<HarnessCase> {
	vec![
		case!(
			"GET",
			"/v1/jobs/999999",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/jobs/999999"),
			BodyProfile::None,
			404,
			Expectation::Error("Job not found"),
		),
		case!(
			"GET",
			"/v1/lists/999999/quality",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerLists,
			PathProfile::Literal("/v1/lists/999999/quality"),
			BodyProfile::None,
			404,
			Expectation::Error("List not found"),
		),
		case!(
			"GET",
			"/v1/pipelines/999999",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerPipelines,
			PathProfile::Literal("/v1/pipelines/999999"),
			BodyProfile::None,
			404,
			Expectation::Error("Pipeline not found"),
		),
		case!(
			"DELETE",
			"/v1/comments/999999",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/comments/999999"),
			BodyProfile::None,
			404,
			Expectation::Error("Comment not found"),
		),
		case!(
			"GET",
			"/v1/me/domains/missing.example.com",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/domains/missing.example.com"),
			BodyProfile::None,
			404,
			Expectation::Error("Domain not found"),
		),
		case!(
			"GET",
			"/v1/me/api-keys/00000000-0000-0000-0000-000000000000",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/api-keys/00000000-0000-0000-0000-000000000000"),
			BodyProfile::None,
			404,
			Expectation::Error("API key not found"),
		),
	]
}

pub fn conflict_cases() -> Vec<HarnessCase> {
	vec![
		case!(
			"POST",
			"/v1/jobs/{job_id}/cancel",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobCancelCompleted,
			BodyProfile::None,
			409,
			Expectation::Error("Cannot cancel job with status: completed"),
		),
		case!(
			"POST",
			"/v1/jobs/{job_id}/retry",
			ConfigProfile::PseudoWorker,
			AuthProfile::BearerFull,
			PathProfile::JobRetryCancelled,
			BodyProfile::None,
			409,
			Expectation::Error("Cannot retry a cancelled job"),
		),
		case!(
			"POST",
			"/v1/pipelines/{pipeline_id}/trigger",
			ConfigProfile::PipelineEnabled,
			AuthProfile::BearerPipelines,
			PathProfile::PipelineTriggerConflict,
			BodyProfile::JsonPipelineTriggerConflict,
			409,
			Expectation::Error("Pipeline already has an active run"),
		),
		case!(
			"POST",
			"/v1/me/domains",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::Literal("/v1/me/domains"),
			BodyProfile::JsonMeDomainCreateDuplicate,
			409,
			Expectation::Error("Domain already exists for this tenant"),
		),
		case!(
			"PATCH",
			"/v1/me/domains/{domain}",
			ConfigProfile::BearerTenant,
			AuthProfile::BearerFull,
			PathProfile::DomainPatch,
			BodyProfile::JsonMeDomainPatchConflict,
			409,
			Expectation::Error("Domain already exists for this tenant"),
		),
	]
}

pub async fn execute_cases(
	cases: &[HarnessCase],
	fixtures: &HarnessFixtures,
	runtime: &HarnessRuntime,
) {
	for case in cases {
		let config = Arc::clone(config_for_case(runtime, case.config));
		let path = render_path(case.path, fixtures);
		let route_key = format!("{} {}", case.key.method, case.key.path);
		let builder = request().method(case.key.method).path(&path);
		let builder = apply_auth(builder, case.auth, fixtures);
		let builder = apply_body(builder, case.body, fixtures);

		let response = builder.reply(&create_routes(config)).await;
		assert_eq!(
			response.status().as_u16(),
			case.expected_status,
			"unexpected status for {route_key} via path {}: {}",
			path,
			String::from_utf8_lossy(response.body())
		);

		match case.expectation {
			Expectation::Json(fields) => {
				header_prefix(response.headers(), "content-type", "application/json");
				let value: serde_json::Value =
					serde_json::from_slice(response.body()).expect("json response");
				assert_json_fields(&value, fields);
			}
			Expectation::Error(message) => {
				header_prefix(response.headers(), "content-type", "application/json");
				let value: serde_json::Value =
					serde_json::from_slice(response.body()).expect("json error response");
				assert_eq!(
					value["error"].as_str(),
					Some(message),
					"unexpected error body for {route_key}",
				);
			}
			Expectation::Csv => {
				header_prefix(response.headers(), "content-type", "text/csv");
				assert!(
					!response.body().is_empty(),
					"csv response body must not be empty for {}",
					route_key
				);
			}
			Expectation::OpenApi => {
				header_prefix(response.headers(), "content-type", "application/json");
				let value: serde_json::Value =
					serde_json::from_slice(response.body()).expect("openapi json");
				assert_json_fields(&value, &["openapi", "paths"]);
			}
			Expectation::StatusOnly => {}
		}
	}
}
