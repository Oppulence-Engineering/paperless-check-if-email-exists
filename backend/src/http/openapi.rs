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
		crate::http::v1::jobs::retry::v1_retry_job,
		crate::http::v1::jobs::approval_checklist::v1_job_approval_checklist,
		crate::http::v1::lists::post::v1_create_list,
		crate::http::v1::lists::get_list::v1_list_lists,
		crate::http::v1::lists::get_detail::v1_get_list,
		crate::http::v1::lists::download::v1_download_list,
		crate::http::v1::lists::delete::v1_delete_list,
		crate::http::v1::pipelines::v1_create_pipeline,
		crate::http::v1::pipelines::v1_list_pipelines,
		crate::http::v1::pipelines::v1_get_pipeline,
		crate::http::v1::pipelines::v1_update_pipeline,
		crate::http::v1::pipelines::v1_delete_pipeline,
		crate::http::v1::pipelines::v1_pause_pipeline,
		crate::http::v1::pipelines::v1_resume_pipeline,
		crate::http::v1::pipelines::v1_trigger_pipeline,
		crate::http::v1::pipelines::v1_list_pipeline_runs,
		crate::http::v1::pipelines::v1_get_pipeline_run,
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
		(name = "Pipelines", description = "Scheduled list-cleaning pipeline endpoints"),
		(name = "Account", description = "Account-level endpoints"),
		(name = "Admin", description = "Administrative endpoints"),
		(name = "Admin Jobs", description = "Administrative job endpoints"),
		(name = "Tenant", description = "Tenant-scoped account settings and domain endpoints"),
		(name = "Lists", description = "List upload, cleaning, and quality endpoints"),
		(name = "Verification", description = "Historical verification lookup endpoints"),
		(name = "Events", description = "Advanced audit log endpoints"),
		(name = "Query", description = "Advanced historical query endpoints; experimental for large reporting workloads"),
		(name = "Comments", description = "Collaboration annotation endpoints; experimental"),
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

	if let Some(generated_tags) = generated.get("tags").and_then(Value::as_array) {
		let base_tags = base
			.as_object_mut()
			.expect("openapi spec root object")
			.entry("tags")
			.or_insert_with(|| Value::Array(Vec::new()))
			.as_array_mut()
			.expect("tags array");
		for tag in generated_tags {
			if !base_tags.iter().any(|existing| existing == tag) {
				base_tags.push(tag.clone());
			}
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
			if let Some(Value::Array(one_of)) = map.get("oneOf") {
				let null_index = one_of.iter().position(|entry| {
					entry.as_object().is_some_and(|obj| {
						obj.get("type") == Some(&Value::String("null".to_string()))
					})
				});
				if one_of.len() == 2 {
					if let Some(null_index) = null_index {
						let non_null_index = if null_index == 0 { 1 } else { 0 };
						let non_null_schema = one_of[non_null_index].clone();
						map.remove("oneOf");
						map.insert("nullable".to_string(), Value::Bool(true));
						map.insert("allOf".to_string(), Value::Array(vec![non_null_schema]));
					}
				}
			}

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

fn strip_unsupported_schema_keywords(value: &mut Value) {
	match value {
		Value::Object(map) => {
			if is_schema_node(map) {
				map.remove("propertyNames");
			}
			for child in map.values_mut() {
				strip_unsupported_schema_keywords(child);
			}
		}
		Value::Array(items) => {
			for child in items {
				strip_unsupported_schema_keywords(child);
			}
		}
		_ => {}
	}
}

fn is_schema_node(map: &Map<String, Value>) -> bool {
	map.contains_key("type")
		|| map.contains_key("properties")
		|| map.contains_key("$ref")
		|| map.contains_key("allOf")
		|| map.contains_key("oneOf")
		|| map.contains_key("anyOf")
		|| map.contains_key("items")
		|| map.contains_key("additionalProperties")
		|| map.contains_key("enum")
		|| map.contains_key("const")
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

fn set_schema_example(spec: &mut Value, schema_name: &str, example: Value) {
	if let Some(schema) = schemas_mut(spec)
		.get_mut(schema_name)
		.and_then(Value::as_object_mut)
	{
		schema.insert("example".to_string(), example);
	}
}

fn set_schema_property(spec: &mut Value, schema_name: &str, property_name: &str, schema: Value) {
	let Some(schema_obj) = schemas_mut(spec)
		.get_mut(schema_name)
		.and_then(Value::as_object_mut)
	else {
		return;
	};
	let properties = schema_obj
		.entry("properties".to_string())
		.or_insert_with(|| json!({}))
		.as_object_mut()
		.expect("schema properties object");
	properties.insert(property_name.to_string(), schema);
}

fn set_schema_discriminator(
	spec: &mut Value,
	schema_name: &str,
	property_name: &str,
	mapping: Value,
) {
	if let Some(schema) = schemas_mut(spec)
		.get_mut(schema_name)
		.and_then(Value::as_object_mut)
	{
		schema.insert(
			"discriminator".to_string(),
			json!({
				"propertyName": property_name,
				"mapping": mapping,
			}),
		);
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
	properties.insert(
		"bounce_risk".to_string(),
		json!({
			"description": "Additive bounce-risk assessment. May be null when enrichment is disabled or unavailable.",
			"nullable": true,
			"allOf": [{ "$ref": "#/components/schemas/BounceRiskAssessment" }]
		}),
	);
	properties.insert(
		"provider".to_string(),
		json!({
			"$ref": "#/components/schemas/Provider",
			"nullable": true
		}),
	);
	properties.insert(
		"provider_rules_applied".to_string(),
		json!({ "type": "boolean" }),
	);
	properties.insert(
		"provider_rejection_reason".to_string(),
		json!({
			"$ref": "#/components/schemas/ProviderRejectionReason",
			"nullable": true
		}),
	);
	properties.insert(
		"provider_confidence".to_string(),
		json!({
			"$ref": "#/components/schemas/ProviderConfidence",
			"nullable": true
		}),
	);
	let required = schema
		.entry("required".to_string())
		.or_insert_with(|| json!([]))
		.as_array_mut()
		.expect("CheckEmailOutput required");
	if !required.iter().any(|value| value.as_str() == Some("score")) {
		required.push(Value::String("score".to_string()));
	}
	if !required
		.iter()
		.any(|value| value.as_str() == Some("provider_rules_applied"))
	{
		required.push(Value::String("provider_rules_applied".to_string()));
	}
}

fn ensure_check_email_request_provider_flag(spec: &mut Value) {
	let schemas = schemas_mut(spec);
	let Some(schema) = schemas
		.get_mut("CheckEmailRequest")
		.and_then(Value::as_object_mut)
	else {
		return;
	};
	let properties = schema
		.entry("properties".to_string())
		.or_insert_with(|| json!({}))
		.as_object_mut()
		.expect("CheckEmailRequest properties");
	properties.insert(
		"strict_provider_rules".to_string(),
		json!({
			"type": "boolean",
			"description": "When false, skips provider-specific syntax validation even if the provider is recognized."
		}),
	);
}

fn add_phase_two_schemas(spec: &mut Value) {
	insert_schema(
		spec,
		"Provider",
		json!({
			"type": "string",
			"enum": [
				"gmail",
				"google_workspace",
				"outlook_consumer",
				"microsoft365",
				"yahoo",
				"apple_icloud",
				"proton",
				"zoho"
			]
		}),
	);
	insert_schema(
		spec,
		"ProviderConfidence",
		json!({
			"type": "string",
			"enum": ["high", "medium", "low"]
		}),
	);
	insert_schema(
		spec,
		"ProviderRejectionReason",
		json!({
			"type": "string",
			"enum": [
				"provider_local_part_too_short",
				"provider_local_part_too_long",
				"provider_invalid_character",
				"provider_consecutive_special_characters",
				"provider_invalid_start_character",
				"provider_invalid_end_character",
				"provider_plus_addressing_not_supported",
				"provider_reserved_word",
				"provider_format_violation"
			]
		}),
	);
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
				"provider_rejected",
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
				"provider_rejected",
				"provider_local_part_too_short",
				"provider_local_part_too_long",
				"provider_invalid_character",
				"provider_consecutive_special_characters",
				"provider_invalid_start_character",
				"provider_invalid_end_character",
				"provider_plus_addressing_not_supported",
				"provider_reserved_word",
				"provider_format_violation",
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
		"Freshness",
		json!({
			"type": "string",
			"enum": ["fresh", "recent", "aging", "stale", "expired"]
		}),
	);
	insert_schema(
		spec,
		"BounceRiskCategory",
		json!({
			"type": "string",
			"enum": ["safe", "low", "medium", "high", "dangerous"]
		}),
	);
	insert_schema(
		spec,
		"RiskDirection",
		json!({
			"type": "string",
			"enum": ["increases_risk", "decreases_risk"]
		}),
	);
	insert_schema(
		spec,
		"RecommendedAction",
		json!({
			"type": "string",
			"enum": ["send", "send_with_caution", "verify_manually", "do_not_send"]
		}),
	);
	insert_schema(
		spec,
		"RiskFactor",
		json!({
			"type": "object",
			"properties": {
				"signal": { "type": "string" },
				"contribution": { "type": "integer", "format": "int32" },
				"description": { "type": "string" },
				"direction": { "$ref": "#/components/schemas/RiskDirection" }
			},
			"required": ["signal", "contribution", "description", "direction"]
		}),
	);
	insert_schema(
		spec,
		"BounceRiskAssessment",
		json!({
			"type": "object",
			"properties": {
				"score": { "type": "integer", "format": "int32", "minimum": 0, "maximum": 100 },
				"category": { "$ref": "#/components/schemas/BounceRiskCategory" },
				"confidence": { "type": "number", "format": "double", "minimum": 0.35, "maximum": 1.0 },
				"action": { "$ref": "#/components/schemas/RecommendedAction" },
				"model_version": { "type": "string" },
				"scored_at": { "type": "string", "format": "date-time" },
				"risk_factors": {
					"type": "array",
					"items": { "$ref": "#/components/schemas/RiskFactor" }
				}
			},
			"required": ["score", "category", "confidence", "action", "model_version", "scored_at", "risk_factors"]
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
				"signals": { "$ref": "#/components/schemas/ScoringSignals" },
				"verified_at": { "type": "string", "format": "date-time" },
				"age_days": { "type": "integer", "format": "int64" },
				"freshness": { "$ref": "#/components/schemas/Freshness" },
				"domain_suggestion": { "type": "string", "description": "Suggested corrected email when a likely domain typo is detected" },
				"normalized_email": { "type": "string", "description": "Canonical form of the email after alias/plus-address normalization" },
				"catch_all_severity": { "type": "string", "enum": ["low", "high"], "description": "Severity tier for catch-all domains (low=free provider, high=corporate)" }
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
				"domain": { "type": "string" },
				"strategy": { "type": "string", "enum": ["parallel", "waterfall"], "default": "parallel", "description": "Search strategy: parallel (all at once) or waterfall (high-quality patterns first)" }
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
		"ConfidenceExplanation",
		json!({
			"type": "object",
			"properties": {
				"score": { "type": "integer", "format": "int32", "minimum": 0, "maximum": 100 },
				"level": { "type": "string", "enum": ["high", "medium", "low", "very_low"] },
				"factors": { "type": "array", "items": { "type": "string" } }
			},
			"required": ["score", "level", "factors"]
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
				"confidence": {
					"nullable": true,
					"allOf": [{ "$ref": "#/components/schemas/ConfidenceExplanation" }]
				},
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
				"summary": { "$ref": "#/components/schemas/ListSummary" },
				"unique_emails": { "type": "integer", "format": "int32", "nullable": true },
				"deduplicated_count": { "type": "integer", "format": "int32", "nullable": true }
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
		"RetryJobResponse",
		json!({
			"type": "object",
			"properties": {
				"job_id": { "type": "integer", "format": "int32" },
				"status": { "type": "string" },
				"tasks_retried": { "type": "integer", "format": "int64" }
			},
			"required": ["job_id", "status", "tasks_retried"]
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
	insert_schema(
		spec,
		"PipelineRunResultLocation",
		json!({
			"type": "object",
			"properties": {
				"download_url": { "type": "string" }
			},
			"required": ["download_url"]
		}),
	);
	insert_schema(
		spec,
		"PipelineRunStats",
		json!({
			"type": "object",
			"properties": {
				"trigger_reason": { "type": "string", "nullable": true },
				"delta_mode": { "type": "boolean", "nullable": true },
				"freshness_days": { "type": "integer", "format": "int32", "nullable": true },
				"changed_only_export": { "type": "boolean", "nullable": true },
				"selected_unique_emails": { "type": "integer", "format": "int32", "nullable": true },
				"skipped_unchanged": { "type": "integer", "format": "int32", "nullable": true },
				"queued_emails": { "type": "integer", "format": "int32", "nullable": true },
				"published_tasks": { "type": "integer", "format": "int32", "nullable": true },
				"completed_tasks": { "type": "integer", "format": "int32", "nullable": true },
				"valid": { "type": "integer", "format": "int32", "nullable": true },
				"invalid": { "type": "integer", "format": "int32", "nullable": true },
				"risky": { "type": "integer", "format": "int32", "nullable": true },
				"unknown": { "type": "integer", "format": "int32", "nullable": true },
				"billed_emails": { "type": "integer", "format": "int32", "nullable": true },
				"source_name": { "type": "string", "nullable": true },
				"source_filename": { "type": "string", "nullable": true },
				"source_row_count": { "type": "integer", "format": "int32", "nullable": true },
				"source_unique_emails": { "type": "integer", "format": "int32", "nullable": true }
			},
			"additionalProperties": true
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
		"/v1/jobs/{job_id}/retry",
		"post",
		"200",
		json_response("RetryJobResponse", "Retry initiated"),
	);
	insert_schema(
		spec,
		"ApprovalCategoryBreakdown",
		json!({
			"type": "object",
			"properties": {
				"valid": { "type": "integer", "format": "int64" },
				"risky": { "type": "integer", "format": "int64" },
				"unknown": { "type": "integer", "format": "int64" },
				"invalid": { "type": "integer", "format": "int64" },
				"unprocessed": { "type": "integer", "format": "int64" }
			},
			"required": ["valid", "risky", "unknown", "invalid", "unprocessed"]
		}),
	);
	insert_schema(
		spec,
		"ApprovalRiskFlags",
		json!({
			"type": "object",
			"properties": {
				"disposable_count": { "type": "integer", "format": "int64" },
				"catch_all_count": { "type": "integer", "format": "int64" },
				"role_account_count": { "type": "integer", "format": "int64" },
				"spam_trap_count": { "type": "integer", "format": "int64" },
				"suppressed_count": { "type": "integer", "format": "int64" }
			},
			"required": ["disposable_count", "catch_all_count", "role_account_count", "spam_trap_count", "suppressed_count"]
		}),
	);
	insert_schema(
		spec,
		"ApprovalChecklistResponse",
		json!({
			"type": "object",
			"properties": {
				"job_id": { "type": "integer", "format": "int32" },
				"total_records": { "type": "integer", "format": "int32" },
				"categories": { "$ref": "#/components/schemas/ApprovalCategoryBreakdown" },
				"risk_flags": { "$ref": "#/components/schemas/ApprovalRiskFlags" },
				"safe_to_send_count": { "type": "integer", "format": "int64" },
				"safe_to_send_pct": { "type": "number" },
				"recommendation": { "type": "string" },
				"ready_to_send": { "type": "boolean" }
			},
			"required": ["job_id", "total_records", "categories", "risk_flags", "safe_to_send_count", "safe_to_send_pct", "recommendation", "ready_to_send"]
		}),
	);
	set_response(
		spec,
		"/v1/jobs/{job_id}/approval",
		"get",
		"200",
		json_response("ApprovalChecklistResponse", "Pre-send approval checklist"),
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
	ensure_check_email_request_provider_flag(spec);
	patch_phase_two_paths(spec);
	set_schema_discriminator(
		spec,
		"PipelineSource",
		"type",
		json!({
			"list_snapshot": "#/components/schemas/PipelineSource_oneOf",
			"integration": "#/components/schemas/PipelineSource_oneOf_1",
			"push": "#/components/schemas/PipelineSource_oneOf_2",
			"bucket": "#/components/schemas/PipelineSource_oneOf_3",
		}),
	);
	set_schema_example(
		spec,
		"PipelineDeliveryConfig",
		json!({
			"dashboard": true,
			"max_attempts": 5,
			"retry_backoff_seconds": 300,
			"webhook": Value::Null,
		}),
	);
	set_schema_example(
		spec,
		"CreatePipelineInput",
		json!({
			"name": "Weekly Cleanup",
			"source": { "type": "list_snapshot", "list_id": 123 },
			"schedule": { "cron": "0 9 * * 1", "timezone": "UTC" },
			"verification": { "delta_mode": true, "freshness_days": 30 },
			"policy": { "missed_run_window_hours": 24 },
			"delivery": {
				"dashboard": true,
				"max_attempts": 5,
				"retry_backoff_seconds": 300,
				"webhook": Value::Null,
			},
			"status": "active",
		}),
	);
	set_schema_example(
		spec,
		"UpdatePipelineInput",
		json!({
			"name": "Weekly Cleanup",
			"source": { "type": "list_snapshot", "list_id": 123 },
			"schedule": { "cron": "0 9 * * 1", "timezone": "UTC" },
			"verification": { "delta_mode": true, "freshness_days": 30 },
			"policy": { "missed_run_window_hours": 24 },
			"delivery": {
				"dashboard": true,
				"max_attempts": 5,
				"retry_backoff_seconds": 300,
				"webhook": Value::Null,
			},
			"status": "active",
		}),
	);
	set_schema_example(
		spec,
		"PipelineView",
		json!({
			"id": 1,
			"tenant_id": "046b6c7f-0b8a-43b9-b35d-6489e6daee91",
			"name": "Weekly Cleanup",
			"source": { "type": "list_snapshot", "list_id": 123 },
			"schedule": { "cron": "0 9 * * 1", "timezone": "UTC" },
			"verification": { "delta_mode": true, "freshness_days": 30 },
			"policy": { "missed_run_window_hours": 24 },
			"delivery": {
				"dashboard": true,
				"max_attempts": 5,
				"retry_backoff_seconds": 300,
				"webhook": Value::Null,
			},
			"status": "active",
			"next_run_at": "2000-01-23T04:56:07.000+00:00",
			"last_scheduled_at": "2000-01-23T04:56:07.000+00:00",
			"last_run_id": 5,
			"created_at": "2000-01-23T04:56:07.000+00:00",
			"updated_at": "2000-01-23T04:56:07.000+00:00",
		}),
	);
	set_schema_example(
		spec,
		"PipelineRunView",
		json!({
			"id": 1,
			"pipeline_id": 2,
			"tenant_id": "046b6c7f-0b8a-43b9-b35d-6489e6daee91",
			"trigger_type": "manual",
			"status": "completed",
			"source_snapshot": { "type": "list_snapshot", "list_id": 123 },
			"stats": {
				"trigger_reason": "manual",
				"delta_mode": true,
				"freshness_days": 30,
				"source_name": "Weekly Cleanup",
				"source_filename": "seed.csv",
				"selected_unique_emails": 1,
				"billed_emails": 1
			},
			"billed_emails": 1,
			"delivery_status": "not_requested",
			"delivery_attempts": 0,
			"created_at": "2000-01-23T04:56:07.000+00:00",
			"updated_at": "2000-01-23T04:56:07.000+00:00",
			"scheduled_for": "2000-01-23T04:56:07.000+00:00",
			"started_at": "2000-01-23T04:56:07.000+00:00",
			"completed_at": "2000-01-23T04:56:07.000+00:00",
			"job_id": 5,
			"list_id": 5,
			"result_location": { "download_url": "/v1/lists/5/download" },
			"last_delivery_attempt_at": Value::Null,
			"next_delivery_attempt_at": Value::Null,
			"delivery_error": Value::Null,
			"error_code": Value::Null,
			"error_message": Value::Null,
		}),
	);
	set_schema_property(
		spec,
		"PipelineRunView",
		"source_snapshot",
		json!({
			"$ref": "#/components/schemas/PipelineSource"
		}),
	);
	set_schema_property(
		spec,
		"PipelineRunView",
		"stats",
		json!({
			"$ref": "#/components/schemas/PipelineRunStats"
		}),
	);
	set_schema_property(
		spec,
		"PipelineRunView",
		"result_location",
		json!({
			"nullable": true,
			"allOf": [
				{ "$ref": "#/components/schemas/PipelineRunResultLocation" }
			]
		}),
	);
}

pub fn build_spec() -> Result<Value, ReacherResponseError> {
	let mut spec: Value = serde_json::from_str(BASE_OPENAPI).map_err(ReacherResponseError::from)?;
	let generated_spec =
		serde_json::to_value(BackendApiDoc::openapi()).map_err(ReacherResponseError::from)?;

	merge_openapi(&mut spec, generated_spec);
	normalize_nullable_types(&mut spec);
	strip_unsupported_schema_keywords(&mut spec);
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
