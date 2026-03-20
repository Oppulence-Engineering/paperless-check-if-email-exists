use crate::http::ReacherResponseError;
use serde_json::{json, Map, Value};
use utoipa::OpenApi;
use warp::Filter;

use crate::http::v1::account_api_keys;
use crate::http::v1::admin::api_keys as admin_api_keys;
use crate::http::v1::admin::jobs as admin_jobs;
use crate::http::v1::admin::quota as admin_quota;
use crate::http::v1::admin::tenants as admin_tenants;
use crate::http::v1::tenant_domains;
use crate::http::v1::tenant_settings;

const BASE_OPENAPI: &str = include_str!("../../openapi.json");

#[derive(OpenApi)]
#[openapi(
	paths(
		crate::http::health::liveness::healthz,
		crate::http::health::readiness::readyz,
		crate::http::version::get::get_version,
		crate::http::openapi::openapi_spec,
		crate::http::v0::check_email::post::post_check_email,
		crate::http::v0::bulk::post::create_bulk_job,
		crate::http::v0::bulk::get::get_bulk_job_status,
		crate::http::v0::bulk::results::get_bulk_job_result,
		crate::http::v1::check_email::post::v1_check_email,
		crate::http::v1::bulk::post::v1_create_bulk_job,
		crate::http::v1::bulk::get_progress::v1_get_bulk_job_progress,
		crate::http::v1::bulk::get_results::v1_get_bulk_job_results,
		crate::http::v1::find_email::post::v1_find_email,
		crate::http::v1::find_email::get::v1_get_find_email,
		crate::http::v1::jobs::get_status::v1_get_job_status,
		crate::http::v1::jobs::cancel::v1_cancel_job,
		crate::http::v1::jobs::get_events::v1_get_job_events,
		crate::http::v1::jobs::get_results::v1_get_job_results,
		crate::http::v1::jobs::download::v1_download_job_results,
		crate::http::v1::lists::post::v1_create_list,
		crate::http::v1::lists::get_list::v1_list_lists,
		crate::http::v1::lists::get_detail::v1_get_list,
		crate::http::v1::lists::download::v1_download_list,
		crate::http::v1::lists::delete::v1_delete_list,
		crate::http::v1::reputation::check::v1_check_reputation,
		crate::http::v1::suppressions::add::v1_add_suppressions,
		crate::http::v1::suppressions::list::v1_list_suppressions,
		crate::http::v1::suppressions::check::v1_check_suppression,
		crate::http::v1::suppressions::delete::v1_delete_suppression,
		crate::http::v1::reverification::status::v1_reverification_status,
		crate::http::v1::me::v1_me,
		account_api_keys::get_api_key,
		account_api_keys::list_api_keys,
		account_api_keys::create_api_key,
		account_api_keys::update_api_key,
		account_api_keys::revoke_api_key,
		admin_tenants::create_tenant,
		admin_tenants::list_tenants,
		admin_tenants::get_tenant,
		admin_tenants::update_tenant,
		admin_tenants::delete_tenant,
		admin_quota::get_tenant_quota,
		admin_quota::reset_tenant_quota,
		admin_quota::update_tenant_quota,
		admin_api_keys::list_all_api_keys,
		admin_api_keys::create_api_key,
		admin_api_keys::list_api_keys,
		admin_api_keys::get_api_key,
		admin_api_keys::update_api_key,
		admin_api_keys::revoke_api_key,
		admin_api_keys::reactivate_api_key,
		tenant_settings::v1_get_tenant_settings,
		tenant_settings::v1_update_tenant_settings,
		tenant_settings::v1_get_tenant_webhook,
		tenant_settings::v1_update_tenant_webhook,
		tenant_settings::v1_clear_tenant_webhook,
		tenant_settings::v1_get_tenant_usage,
		tenant_domains::v1_list_tenant_domains,
		tenant_domains::v1_create_tenant_domain,
		tenant_domains::v1_get_tenant_domain,
		tenant_domains::v1_update_tenant_domain,
		tenant_domains::v1_delete_tenant_domain,
		admin_jobs::list_jobs,
		admin_jobs::get_job,
		admin_jobs::get_job_events,
		admin_jobs::get_job_results,
		admin_jobs::list_tenant_jobs,
	),
	tags(
		(name = "System", description = "System and service metadata endpoints"),
		(name = "Health", description = "Service health endpoints"),
		(name = "v0", description = "Legacy v0 API endpoints"),
		(name = "v1", description = "Primary v1 API endpoints"),
		(name = "Jobs", description = "Job lifecycle and results endpoints"),
		(name = "Account", description = "Account-level endpoints"),
		(name = "Admin", description = "Administrative endpoints"),
		(name = "Admin Jobs", description = "Administrative job endpoints"),
		(name = "Tenant", description = "Tenant-scoped account settings and domain endpoints"),
	)
)]
struct BackendApiDoc;

fn merge_openapi(base: &mut Value, generated: Value) {
	if let (Some(base_paths), Some(generated_paths)) = (
		base.get_mut("paths").and_then(Value::as_object_mut),
		generated.get("paths").and_then(Value::as_object),
	) {
		for (path, value) in generated_paths {
			base_paths.insert(path.clone(), value.clone());
		}
	}

	if let (Some(base_schemas), Some(generated_schemas)) = (
		base.get_mut("components")
			.and_then(|v| v.get_mut("schemas"))
			.and_then(Value::as_object_mut),
		generated
			.get("components")
			.and_then(|v| v.get("schemas"))
			.and_then(Value::as_object),
	) {
		for (name, value) in generated_schemas {
			base_schemas.insert(name.clone(), value.clone());
		}
	}
}

fn normalize_nullable_types(value: &mut Value) {
	match value {
		Value::Object(map) => {
			if let Some(Value::Array(type_values)) = map.get("type") {
				let mut non_null_types = type_values
					.iter()
					.filter_map(|entry| entry.as_str())
					.filter(|entry| *entry != "null");
				let first_non_null = non_null_types.next().map(str::to_string);
				let has_single_non_null = non_null_types.next().is_none();
				let has_null = type_values
					.iter()
					.any(|entry| entry.as_str() == Some("null"));
				if has_null && has_single_non_null {
					if let Some(non_null_type) = first_non_null {
						map.insert("type".to_string(), Value::String(non_null_type));
						map.insert("nullable".to_string(), Value::Bool(true));
					}
				}
			}

			for child in map.values_mut() {
				normalize_nullable_types(child);
			}
		}
		Value::Array(items) => {
			for child in items {
				normalize_nullable_types(child);
			}
		}
		_ => {}
	}
}

fn components_mut(spec: &mut Value) -> &mut Map<String, Value> {
	spec.as_object_mut()
		.expect("openapi spec root object")
		.entry("components")
		.or_insert_with(|| json!({}))
		.as_object_mut()
		.expect("components object")
}

fn schemas_mut(spec: &mut Value) -> &mut Map<String, Value> {
	components_mut(spec)
		.entry("schemas")
		.or_insert_with(|| json!({}))
		.as_object_mut()
		.expect("schemas object")
}

fn paths_mut(spec: &mut Value) -> &mut Map<String, Value> {
	spec.as_object_mut()
		.expect("openapi spec root object")
		.entry("paths")
		.or_insert_with(|| json!({}))
		.as_object_mut()
		.expect("paths object")
}

fn operation_mut<'a>(
	spec: &'a mut Value,
	path: &str,
	method: &str,
) -> Option<&'a mut Map<String, Value>> {
	paths_mut(spec)
		.get_mut(path)
		.and_then(Value::as_object_mut)
		.and_then(|path_item| path_item.get_mut(method))
		.and_then(Value::as_object_mut)
}

fn insert_schema(spec: &mut Value, name: &str, schema: Value) {
	schemas_mut(spec).insert(name.to_string(), schema);
}

fn json_response(schema_name: &str, description: &str) -> Value {
	json!({
		"description": description,
		"content": {
			"application/json": {
				"schema": {
					"$ref": format!("#/components/schemas/{schema_name}")
				}
			}
		}
	})
}

fn binary_response(description: &str, content_type: &str) -> Value {
	json!({
		"description": description,
		"content": {
			content_type: {
				"schema": {
					"type": "string",
					"format": "binary"
				}
			}
		}
	})
}

fn set_request_body(
	spec: &mut Value,
	path: &str,
	method: &str,
	content_type: &str,
	schema_name: &str,
	required: bool,
) {
	if let Some(operation) = operation_mut(spec, path, method) {
		operation.insert(
			"requestBody".to_string(),
			json!({
				"required": required,
				"content": {
					content_type: {
						"schema": {
							"$ref": format!("#/components/schemas/{schema_name}")
						}
					}
				}
			}),
		);
	}
}

fn set_response(spec: &mut Value, path: &str, method: &str, status: &str, response: Value) {
	if let Some(operation) = operation_mut(spec, path, method) {
		operation
			.entry("responses".to_string())
			.or_insert_with(|| json!({}))
			.as_object_mut()
			.expect("responses object")
			.insert(status.to_string(), response);
	}
}

fn ensure_check_email_output_scored(spec: &mut Value) {
	let schemas = schemas_mut(spec);
	let Some(schema) = schemas
		.get_mut("CheckEmailOutput")
		.and_then(Value::as_object_mut)
	else {
		return;
	};
	let properties = schema
		.entry("properties".to_string())
		.or_insert_with(|| json!({}))
		.as_object_mut()
		.expect("CheckEmailOutput properties");
	properties.insert(
		"score".to_string(),
		json!({ "$ref": "#/components/schemas/EmailScore" }),
	);
	let required = schema
		.entry("required".to_string())
		.or_insert_with(|| json!([]))
		.as_array_mut()
		.expect("CheckEmailOutput required");
	if !required.iter().any(|value| value.as_str() == Some("score")) {
		required.push(Value::String("score".to_string()));
	}
}

fn add_phase_two_schemas(spec: &mut Value) {
	insert_schema(
		spec,
		"EmailCategory",
		json!({
			"type": "string",
			"enum": ["valid", "risky", "unknown", "invalid"]
		}),
	);
	insert_schema(
		spec,
		"SubReason",
		json!({
			"type": "string",
			"enum": [
				"deliverable",
				"invalid_syntax",
				"invalid_recipient",
				"smtp_undeliverable",
				"disabled_mailbox",
				"no_mx",
				"smtp_error",
				"smtp_unreachable",
				"catch_all",
				"full_inbox",
				"disposable",
				"role_account",
				"spam_trap",
				"risky",
				"unknown"
			]
		}),
	);
	insert_schema(
		spec,
		"ReasonCode",
		json!({
			"type": "string",
			"enum": [
				"deliverable",
				"invalid_syntax",
				"invalid_recipient",
				"smtp_undeliverable",
				"disabled_mailbox",
				"no_mx",
				"smtp_error",
				"smtp_unreachable",
				"catch_all",
				"full_inbox",
				"disposable",
				"role_account",
				"spam_trap",
				"unknown_deliverability",
				"free_provider",
				"possible_typo"
			]
		}),
	);
	insert_schema(
		spec,
		"ScoringSignals",
		json!({
			"type": "object",
			"properties": {
				"valid_syntax": { "type": "boolean" },
				"reachable": { "$ref": "#/components/schemas/Reachable" },
				"has_mx_records": { "type": "boolean" },
				"smtp_error": { "type": "boolean" },
				"smtp_can_connect": { "type": "boolean" },
				"smtp_is_deliverable": { "type": "boolean" },
				"smtp_is_disabled": { "type": "boolean" },
				"smtp_is_catch_all": { "type": "boolean" },
				"smtp_has_full_inbox": { "type": "boolean" },
				"is_disposable": { "type": "boolean" },
				"is_role_account": { "type": "boolean" },
				"is_spam_trap_domain": { "type": "boolean" },
				"is_free_provider": { "type": "boolean" },
				"has_domain_suggestion": { "type": "boolean" }
			},
			"required": [
				"valid_syntax",
				"reachable",
				"has_mx_records",
				"smtp_error",
				"smtp_can_connect",
				"smtp_is_deliverable",
				"smtp_is_disabled",
				"smtp_is_catch_all",
				"smtp_has_full_inbox",
				"is_disposable",
				"is_role_account",
				"is_spam_trap_domain",
				"is_free_provider",
				"has_domain_suggestion"
			]
		}),
	);
	insert_schema(
		spec,
		"EmailScore",
		json!({
			"type": "object",
			"properties": {
				"score": { "type": "integer", "format": "int32", "minimum": 0, "maximum": 100 },
				"category": { "$ref": "#/components/schemas/EmailCategory" },
				"sub_reason": { "$ref": "#/components/schemas/SubReason" },
				"safe_to_send": { "type": "boolean" },
				"reason_codes": {
					"type": "array",
					"items": { "$ref": "#/components/schemas/ReasonCode" }
				},
				"signals": { "$ref": "#/components/schemas/ScoringSignals" }
			},
			"required": ["score", "category", "sub_reason", "safe_to_send", "reason_codes", "signals"]
		}),
	);
	insert_schema(
		spec,
		"FindEmailRequest",
		json!({
			"type": "object",
			"properties": {
				"first_name": { "type": "string" },
				"last_name": { "type": "string" },
				"domain": { "type": "string" }
			},
			"required": ["first_name", "last_name", "domain"]
		}),
	);
	insert_schema(
		spec,
		"FindEmailAcceptedResponse",
		json!({
			"type": "object",
			"properties": {
				"job_id": { "type": "integer", "format": "int32" },
				"bulk_job_id": { "type": "integer", "format": "int32" },
				"status": { "type": "string" },
				"candidates_checked": { "type": "integer", "format": "int32" }
			},
			"required": ["job_id", "bulk_job_id", "status", "candidates_checked"]
		}),
	);
	insert_schema(
		spec,
		"FinderBestMatch",
		json!({
			"type": "object",
			"properties": {
				"email": { "type": "string" },
				"score": { "type": "integer", "format": "int32" },
				"confidence": { "type": "string" },
				"pattern": { "type": "string" }
			},
			"required": ["email", "score", "confidence", "pattern"]
		}),
	);
	insert_schema(
		spec,
		"FinderCandidateResult",
		json!({
			"type": "object",
			"properties": {
				"email": { "type": "string" },
				"pattern": { "type": "string" },
				"score": { "type": "integer", "format": "int32" },
				"category": { "$ref": "#/components/schemas/EmailCategory" },
				"sub_reason": { "$ref": "#/components/schemas/SubReason" },
				"is_reachable": { "$ref": "#/components/schemas/Reachable" },
				"result": {
					"nullable": true,
					"allOf": [{ "$ref": "#/components/schemas/CheckEmailOutput" }]
				}
			},
			"required": ["email", "pattern", "score", "category", "sub_reason", "is_reachable"]
		}),
	);
	insert_schema(
		spec,
		"FindEmailStatusResponse",
		json!({
			"type": "object",
			"properties": {
				"job_id": { "type": "integer", "format": "int32" },
				"bulk_job_id": { "type": "integer", "format": "int32" },
				"status": { "type": "string" },
				"domain_has_mx": { "type": "boolean" },
				"domain_is_catch_all": { "type": "boolean" },
				"candidates_checked": { "type": "integer", "format": "int32" },
				"results": {
					"type": "array",
					"items": { "$ref": "#/components/schemas/FinderCandidateResult" }
				},
				"best_match": {
					"nullable": true,
					"allOf": [{ "$ref": "#/components/schemas/FinderBestMatch" }]
				}
			},
			"required": [
				"job_id",
				"bulk_job_id",
				"status",
				"domain_has_mx",
				"domain_is_catch_all",
				"candidates_checked",
				"results"
			]
		}),
	);
	insert_schema(
		spec,
		"ListUploadRequest",
		json!({
			"type": "object",
			"properties": {
				"file": { "type": "string", "format": "binary" },
				"name": { "type": "string", "nullable": true },
				"email_column": { "type": "string", "nullable": true }
			},
			"required": ["file"]
		}),
	);
	insert_schema(
		spec,
		"ListUploadResponse",
		json!({
			"type": "object",
			"properties": {
				"list_id": { "type": "integer", "format": "int32" },
				"job_id": { "type": "integer", "format": "int32" },
				"total_rows": { "type": "integer", "format": "int32" },
				"email_column": { "type": "string" }
			},
			"required": ["list_id", "job_id", "total_rows", "email_column"]
		}),
	);
	insert_schema(
		spec,
		"ListItem",
		json!({
			"type": "object",
			"properties": {
				"id": { "type": "integer", "format": "int32" },
				"name": { "type": "string" },
				"original_filename": { "type": "string" },
				"status": { "type": "string" },
				"total_rows": { "type": "integer", "format": "int32" },
				"email_column": { "type": "string" }
			},
			"required": ["id", "name", "original_filename", "status", "total_rows", "email_column"]
		}),
	);
	insert_schema(
		spec,
		"ListListResponse",
		json!({
			"type": "object",
			"properties": {
				"lists": { "type": "array", "items": { "$ref": "#/components/schemas/ListItem" } },
				"total": { "type": "integer", "format": "int64" }
			},
			"required": ["lists", "total"]
		}),
	);
	insert_schema(
		spec,
		"ListSummary",
		json!({
			"type": "object",
			"properties": {
				"total_valid": { "type": "integer", "format": "int64" },
				"total_risky": { "type": "integer", "format": "int64" },
				"total_unknown": { "type": "integer", "format": "int64" },
				"total_invalid": { "type": "integer", "format": "int64" },
				"total_processed": { "type": "integer", "format": "int64" }
			},
			"required": ["total_valid", "total_risky", "total_unknown", "total_invalid", "total_processed"]
		}),
	);
	insert_schema(
		spec,
		"ListDetailResponse",
		json!({
			"type": "object",
			"properties": {
				"id": { "type": "integer", "format": "int32" },
				"job_id": { "type": "integer", "format": "int32" },
				"name": { "type": "string" },
				"status": { "type": "string" },
				"total_rows": { "type": "integer", "format": "int32" },
				"email_column": { "type": "string" },
				"summary": { "$ref": "#/components/schemas/ListSummary" }
			},
			"required": ["id", "job_id", "name", "status", "total_rows", "email_column", "summary"]
		}),
	);
	insert_schema(
		spec,
		"ListDeleteResponse",
		json!({
			"type": "object",
			"properties": {
				"deleted": { "type": "boolean" }
			},
			"required": ["deleted"]
		}),
	);
	insert_schema(
		spec,
		"JobTaskResult",
		json!({
			"type": "object",
			"properties": {
				"id": { "type": "integer", "format": "int64" },
				"task_state": { "type": "string" },
				"result": {
					"nullable": true,
					"allOf": [{ "$ref": "#/components/schemas/CheckEmailOutput" }]
				},
				"error": { "type": "string", "nullable": true },
				"retry_count": { "type": "integer", "format": "int32" }
			},
			"required": ["id", "task_state", "retry_count"]
		}),
	);
	insert_schema(
		spec,
		"JobResultPageResponse",
		json!({
			"type": "object",
			"properties": {
				"results": { "type": "array", "items": { "$ref": "#/components/schemas/JobTaskResult" } },
				"next_cursor": { "type": "integer", "format": "int64", "nullable": true },
				"has_more": { "type": "boolean" }
			},
			"required": ["results", "has_more"]
		}),
	);
	insert_schema(
		spec,
		"BulkJobResultsResponse",
		json!({
			"type": "object",
			"properties": {
				"results": { "type": "array", "items": { "$ref": "#/components/schemas/CheckEmailOutput" } }
			},
			"required": ["results"]
		}),
	);
	insert_schema(
		spec,
		"ReputationCheckRequest",
		json!({
			"type": "object",
			"properties": {
				"domain": { "type": "string" },
				"force_refresh": { "type": "boolean", "default": false }
			},
			"required": ["domain"]
		}),
	);
	insert_schema(
		spec,
		"BlacklistResult",
		json!({
			"type": "object",
			"properties": {
				"provider": { "type": "string" },
				"listed": { "type": "boolean" },
				"lookup_time_ms": { "type": "integer", "format": "int64" }
			},
			"required": ["provider", "listed", "lookup_time_ms"]
		}),
	);
	insert_schema(
		spec,
		"DnsRecordResults",
		json!({
			"type": "object",
			"properties": {
				"has_spf": { "type": "boolean" },
				"spf_valid": { "type": "boolean" },
				"has_dkim": { "type": "boolean" },
				"has_dmarc": { "type": "boolean" },
				"dmarc_policy": { "type": "string", "nullable": true },
				"has_mx": { "type": "boolean" },
				"mx_records": { "type": "array", "items": { "type": "string" } }
			},
			"required": ["has_spf", "spf_valid", "has_dkim", "has_dmarc", "has_mx", "mx_records"]
		}),
	);
	insert_schema(
		spec,
		"DomainInfo",
		json!({
			"type": "object",
			"properties": {
				"domain_age_days": { "type": "integer", "format": "int64", "nullable": true },
				"registrar": { "type": "string", "nullable": true },
				"created_at": { "type": "string", "nullable": true }
			}
		}),
	);
	insert_schema(
		spec,
		"ReputationCheckResponse",
		json!({
			"type": "object",
			"properties": {
				"domain": { "type": "string" },
				"score": { "type": "integer", "format": "int32", "minimum": 0, "maximum": 100 },
				"risk_level": { "type": "string" },
				"blacklist_results": {
					"type": "array",
					"items": { "$ref": "#/components/schemas/BlacklistResult" }
				},
				"dns_records": { "$ref": "#/components/schemas/DnsRecordResults" },
				"domain_info": { "$ref": "#/components/schemas/DomainInfo" },
				"cached": { "type": "boolean" }
			},
			"required": ["domain", "score", "risk_level", "blacklist_results", "dns_records", "domain_info", "cached"]
		}),
	);
	insert_schema(
		spec,
		"SuppressionReason",
		json!({
			"type": "string",
			"enum": ["manual", "bounce", "invalid", "spam_trap", "unsubscribe", "complaint", "auto_invalid"]
		}),
	);
	insert_schema(
		spec,
		"AddSuppressionsRequest",
		json!({
			"type": "object",
			"properties": {
				"emails": { "type": "array", "items": { "type": "string" } },
				"reason": { "$ref": "#/components/schemas/SuppressionReason" },
				"source": { "type": "string", "nullable": true },
				"notes": { "type": "string", "nullable": true }
			},
			"required": ["emails"]
		}),
	);
	insert_schema(
		spec,
		"AddSuppressionsResponse",
		json!({
			"type": "object",
			"properties": {
				"added": { "type": "integer", "format": "int64" },
				"duplicates": { "type": "integer", "format": "int64" }
			},
			"required": ["added", "duplicates"]
		}),
	);
	insert_schema(
		spec,
		"SuppressionEntry",
		json!({
			"type": "object",
			"properties": {
				"id": { "type": "integer", "format": "int32" },
				"email": { "type": "string" },
				"reason": { "$ref": "#/components/schemas/SuppressionReason" },
				"source": { "type": "string", "nullable": true },
				"notes": { "type": "string", "nullable": true },
				"created_at": { "type": "string", "format": "date-time" }
			},
			"required": ["id", "email", "reason", "created_at"]
		}),
	);
	insert_schema(
		spec,
		"SuppressionListResponse",
		json!({
			"type": "object",
			"properties": {
				"entries": { "type": "array", "items": { "$ref": "#/components/schemas/SuppressionEntry" } },
				"total": { "type": "integer", "format": "int64" }
			},
			"required": ["entries", "total"]
		}),
	);
	insert_schema(
		spec,
		"SuppressionCheckResponse",
		json!({
			"type": "object",
			"properties": {
				"suppressed": { "type": "boolean" },
				"reason": { "$ref": "#/components/schemas/SuppressionReason" },
				"source": { "type": "string", "nullable": true },
				"created_at": { "type": "string", "format": "date-time", "nullable": true }
			},
			"required": ["suppressed"]
		}),
	);
	insert_schema(
		spec,
		"SuppressionDeleteResponse",
		json!({
			"type": "object",
			"properties": {
				"deleted": { "type": "boolean" }
			},
			"required": ["deleted"]
		}),
	);
	insert_schema(
		spec,
		"ReverificationStatusResponse",
		json!({
			"type": "object",
			"properties": {
				"enabled": { "type": "boolean" },
				"staleness_days": { "type": "integer", "format": "int32" },
				"batch_size": { "type": "integer", "format": "int32" },
				"last_run_at": { "type": "string", "format": "date-time", "nullable": true },
				"next_run_at": { "type": "string", "format": "date-time", "nullable": true },
				"last_job_id": { "type": "integer", "format": "int32", "nullable": true },
				"emails_requeued": { "type": "integer", "format": "int32" }
			},
			"required": ["enabled"]
		}),
	);
}

fn patch_phase_two_paths(spec: &mut Value) {
	set_request_body(
		spec,
		"/v0/check_email",
		"post",
		"application/json",
		"CheckEmailRequest",
		true,
	);
	set_request_body(
		spec,
		"/v1/check_email",
		"post",
		"application/json",
		"CheckEmailRequest",
		true,
	);
	set_response(
		spec,
		"/v0/check_email",
		"post",
		"200",
		json_response("CheckEmailOutput", "Email verification result"),
	);
	set_response(
		spec,
		"/v1/check_email",
		"post",
		"200",
		json_response("CheckEmailOutput", "Email verification result"),
	);

	set_response(
		spec,
		"/v1/bulk/{job_id}/results",
		"get",
		"200",
		json!({
			"description": "Bulk job results",
			"content": {
				"application/json": {
					"schema": { "$ref": "#/components/schemas/BulkJobResultsResponse" }
				},
				"text/csv": {
					"schema": { "type": "string", "format": "binary" }
				}
			}
		}),
	);

	set_response(
		spec,
		"/v1/jobs/{job_id}/results",
		"get",
		"200",
		json_response("JobResultPageResponse", "Job result page"),
	);
	set_response(
		spec,
		"/v1/jobs/{job_id}/download",
		"get",
		"200",
		json!({
			"description": "Job result download stream",
			"content": {
				"text/csv": { "schema": { "type": "string", "format": "binary" } },
				"application/x-ndjson": { "schema": { "type": "string", "format": "binary" } }
			}
		}),
	);

	set_request_body(
		spec,
		"/v1/find_email",
		"post",
		"application/json",
		"FindEmailRequest",
		true,
	);
	set_response(
		spec,
		"/v1/find_email",
		"post",
		"202",
		json_response("FindEmailAcceptedResponse", "Finder job accepted"),
	);
	set_response(
		spec,
		"/v1/find_email/{job_id}",
		"get",
		"200",
		json_response("FindEmailStatusResponse", "Finder job result"),
	);

	set_request_body(
		spec,
		"/v1/lists",
		"post",
		"multipart/form-data",
		"ListUploadRequest",
		true,
	);
	set_response(
		spec,
		"/v1/lists",
		"post",
		"202",
		json_response("ListUploadResponse", "List upload accepted"),
	);
	set_response(
		spec,
		"/v1/lists",
		"get",
		"200",
		json_response("ListListResponse", "List resources"),
	);
	set_response(
		spec,
		"/v1/lists/{list_id}",
		"get",
		"200",
		json_response("ListDetailResponse", "List detail"),
	);
	set_response(
		spec,
		"/v1/lists/{list_id}",
		"delete",
		"200",
		json_response("ListDeleteResponse", "List deleted"),
	);
	set_response(
		spec,
		"/v1/lists/{list_id}/download",
		"get",
		"200",
		binary_response("Cleaned list CSV download", "text/csv"),
	);

	set_request_body(
		spec,
		"/v1/reputation/check",
		"post",
		"application/json",
		"ReputationCheckRequest",
		true,
	);
	set_response(
		spec,
		"/v1/reputation/check",
		"post",
		"200",
		json_response("ReputationCheckResponse", "Reputation check response"),
	);

	set_request_body(
		spec,
		"/v1/suppressions",
		"post",
		"application/json",
		"AddSuppressionsRequest",
		true,
	);
	set_response(
		spec,
		"/v1/suppressions",
		"post",
		"200",
		json_response("AddSuppressionsResponse", "Suppression entries added"),
	);
	set_response(
		spec,
		"/v1/suppressions",
		"get",
		"200",
		json_response("SuppressionListResponse", "Suppression list"),
	);
	set_response(
		spec,
		"/v1/suppressions/check",
		"get",
		"200",
		json_response("SuppressionCheckResponse", "Suppression check result"),
	);
	set_response(
		spec,
		"/v1/suppressions/{id}",
		"delete",
		"200",
		json_response("SuppressionDeleteResponse", "Suppression entry deleted"),
	);
	set_response(
		spec,
		"/v1/reverification/status",
		"get",
		"200",
		json_response(
			"ReverificationStatusResponse",
			"Reverification schedule status",
		),
	);
}

fn augment_phase_two_openapi(spec: &mut Value) {
	add_phase_two_schemas(spec);
	ensure_check_email_output_scored(spec);
	patch_phase_two_paths(spec);
}

pub fn build_spec() -> Result<Value, ReacherResponseError> {
	let mut spec: Value = serde_json::from_str(BASE_OPENAPI).map_err(ReacherResponseError::from)?;
	let generated_spec =
		serde_json::to_value(BackendApiDoc::openapi()).map_err(ReacherResponseError::from)?;

	merge_openapi(&mut spec, generated_spec);
	normalize_nullable_types(&mut spec);
	augment_phase_two_openapi(&mut spec);
	Ok(spec)
}

/// Serve the merged OpenAPI document for all documented REST endpoints.
#[utoipa::path(
	get,
	path = "/openapi.json",
	tag = "System",
	responses(
		(status = 200, description = "Merged OpenAPI specification for all documented REST endpoints")
	)
)]
pub fn openapi_spec() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
	warp::path!("openapi.json")
		.and(warp::get())
		.and_then(|| async move {
			build_spec()
				.map(|v| warp::reply::json(&v))
				.map_err(|e| warp::reject::custom(e))
		})
}
