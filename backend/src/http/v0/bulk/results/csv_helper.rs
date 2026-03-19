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

use serde::Serialize;
use std::convert::TryFrom;

/// Wrapper for serde json value to convert
/// into a csv response
#[derive(Debug)]
pub struct CsvWrapper(pub serde_json::Value);

/// Simplified output of `CheckEmailOutput` struct
/// for csv fields.
#[derive(Debug, Serialize)]
pub struct JobResultCsvResponse {
	input: String,
	is_reachable: String,
	#[serde(rename = "misc.is_disposable")]
	misc_is_disposable: bool,
	#[serde(rename = "misc.is_role_account")]
	misc_is_role_account: bool,
	#[serde(rename = "misc.gravatar_url")]
	misc_gravatar_url: Option<String>,
	#[serde(rename = "mx.accepts_mail")]
	mx_accepts_mail: bool,
	#[serde(rename = "smtp.can_connect")]
	smtp_can_connect: bool,
	#[serde(rename = "smtp.has_full_inbox")]
	smtp_has_full_inbox: bool,
	#[serde(rename = "smtp.is_catch_all")]
	smtp_is_catch_all: bool,
	#[serde(rename = "smtp.is_deliverable")]
	smtp_is_deliverable: bool,
	#[serde(rename = "smtp.is_disabled")]
	smtp_is_disabled: bool,
	#[serde(rename = "syntax.is_valid_syntax")]
	syntax_is_valid_syntax: bool,
	#[serde(rename = "syntax.domain")]
	syntax_domain: String,
	#[serde(rename = "syntax.username")]
	syntax_username: String,
	error: Option<String>,
}

/// Convert csv wrapper to csv response
/// Performs multiple allocations for string fields
/// throw error if field is missing
impl TryFrom<CsvWrapper> for JobResultCsvResponse {
	type Error = &'static str;

	fn try_from(value: CsvWrapper) -> Result<Self, Self::Error> {
		let mut input: String = String::default();
		let mut is_reachable: String = String::default();
		let mut misc_is_disposable: bool = false;
		let mut misc_is_role_account: bool = false;
		let mut misc_gravatar_url: Option<String> = None;
		let mut mx_accepts_mail: bool = false;
		let mut smtp_can_connect: bool = false;
		let mut smtp_has_full_inbox: bool = false;
		let mut smtp_is_catch_all: bool = false;
		let mut smtp_is_deliverable: bool = false;
		let mut smtp_is_disabled: bool = false;
		let mut syntax_is_valid_syntax: bool = false;
		let mut syntax_domain: String = String::default();
		let mut syntax_username: String = String::default();
		let mut error: Option<String> = None;

		let top_level = value
			.0
			.as_object()
			.ok_or("Failed to find top level object")?;
		for (key, val) in top_level.keys().zip(top_level.values()) {
			match key.as_str() {
				"input" => input = val.as_str().ok_or("input should be a string")?.to_string(),
				"is_reachable" => {
					is_reachable = val
						.as_str()
						.ok_or("is_reachable should be a string")?
						.to_string()
				}
				"misc" => {
					let misc_obj = val.as_object().ok_or("misc field should be an object")?;
					for (key, val) in misc_obj.keys().zip(misc_obj.values()) {
						match key.as_str() {
							"error" => error = Some(val.to_string()),
							"is_disposable" => {
								misc_is_disposable =
									val.as_bool().ok_or("is_disposable should be a boolean")?
							}
							"is_role_account" => {
								misc_is_role_account =
									val.as_bool().ok_or("is_role_account should be a boolean")?
							}
							"gravatar_url" => {
								if Option::is_some(&val.as_str()) {
									misc_gravatar_url = Some(val.to_string())
								}
							}
							_ => {}
						}
					}
				}
				"mx" => {
					let mx_obj = val.as_object().ok_or("mx field should be an object")?;
					for (key, val) in mx_obj.keys().zip(mx_obj.values()) {
						match key.as_str() {
							"error" => error = Some(val.to_string()),
							"accepts_email" => {
								mx_accepts_mail =
									val.as_bool().ok_or("accepts_email should be a boolean")?
							}
							_ => {}
						}
					}
				}
				"smtp" => {
					let smtp_obj = val.as_object().ok_or("mx field should be an object")?;
					for (key, val) in smtp_obj.keys().zip(smtp_obj.values()) {
						match key.as_str() {
							"error" => error = Some(val.to_string()),
							"can_connect_smtp" => {
								smtp_can_connect = val
									.as_bool()
									.ok_or("can_connect_smtp should be a boolean")?
							}
							"has_full_inbox" => {
								smtp_has_full_inbox =
									val.as_bool().ok_or("has_full_inbox should be a boolean")?
							}
							"is_catch_all" => {
								smtp_is_catch_all =
									val.as_bool().ok_or("is_catch_all should be a boolean")?
							}
							"is_deliverable" => {
								smtp_is_deliverable =
									val.as_bool().ok_or("is_deliverable should be a boolean")?
							}
							"is_disabled" => {
								smtp_is_disabled =
									val.as_bool().ok_or("is_disabled should be a boolean")?
							}
							_ => {}
						}
					}
				}
				"syntax" => {
					let syntax_obj = val.as_object().ok_or("syntax field should be an object")?;
					for (key, val) in syntax_obj.keys().zip(syntax_obj.values()) {
						match key.as_str() {
							"error" => error = Some(val.to_string()),
							"is_valid_syntax" => {
								syntax_is_valid_syntax =
									val.as_bool().ok_or("is_valid_syntax should be a boolean")?
							}
							"username" => {
								syntax_username = val
									.as_str()
									.ok_or("username should be a string")?
									.to_string()
							}
							"domain" => {
								syntax_domain =
									val.as_str().ok_or("domain should be a string")?.to_string()
							}
							_ => {}
						}
					}
				}
				// ignore unknown fields
				_ => {}
			}
		}

		Ok(JobResultCsvResponse {
			input,
			is_reachable,
			misc_is_disposable,
			misc_is_role_account,
			misc_gravatar_url,
			mx_accepts_mail,
			smtp_can_connect,
			smtp_has_full_inbox,
			smtp_is_catch_all,
			smtp_is_deliverable,
			smtp_is_disabled,
			syntax_domain,
			syntax_is_valid_syntax,
			syntax_username,
			error,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::convert::TryInto;

	fn valid_json() -> serde_json::Value {
		serde_json::json!({
			"input": "test@example.com",
			"is_reachable": "safe",
			"misc": {"is_disposable": false, "is_role_account": true, "gravatar_url": null, "error": null},
			"mx": {"accepts_email": true, "records": [], "error": null},
			"smtp": {"can_connect_smtp": true, "has_full_inbox": false, "is_catch_all": false, "is_deliverable": true, "is_disabled": false, "error": null},
			"syntax": {"is_valid_syntax": true, "domain": "example.com", "username": "test", "error": null}
		})
	}

	#[test]
	fn test_valid_conversion() {
		let wrapper = CsvWrapper(valid_json());
		let csv: JobResultCsvResponse = wrapper.try_into().unwrap();
		assert_eq!(csv.input, "test@example.com");
		assert_eq!(csv.is_reachable, "safe");
		assert!(!csv.misc_is_disposable);
		assert!(csv.misc_is_role_account);
		assert!(csv.misc_gravatar_url.is_none());
		assert!(csv.mx_accepts_mail);
		assert!(csv.smtp_can_connect);
		assert!(!csv.smtp_has_full_inbox);
		assert!(!csv.smtp_is_catch_all);
		assert!(csv.smtp_is_deliverable);
		assert!(!csv.smtp_is_disabled);
		assert!(csv.syntax_is_valid_syntax);
		assert_eq!(csv.syntax_domain, "example.com");
		assert_eq!(csv.syntax_username, "test");
	}

	#[test]
	fn test_not_an_object() {
		let wrapper = CsvWrapper(serde_json::json!("string"));
		let result: Result<JobResultCsvResponse, _> = wrapper.try_into();
		assert!(result.is_err());
	}

	#[test]
	fn test_missing_misc() {
		let mut json = valid_json();
		json.as_object_mut().unwrap().remove("misc");
		let result: Result<JobResultCsvResponse, _> = CsvWrapper(json).try_into();
		// Should still work — misc fields just won't be set (default false)
		assert!(result.is_ok());
	}

	#[test]
	fn test_with_gravatar_url() {
		let mut json = valid_json();
		json["misc"]["gravatar_url"] = serde_json::json!("https://gravatar.com/abc");
		let csv: JobResultCsvResponse = CsvWrapper(json).try_into().unwrap();
		assert!(csv.misc_gravatar_url.is_some());
	}

	#[test]
	fn test_with_error_in_smtp() {
		let mut json = valid_json();
		json["smtp"]["error"] = serde_json::json!("connection refused");
		let csv: JobResultCsvResponse = CsvWrapper(json).try_into().unwrap();
		assert!(csv.error.is_some());
	}

	#[test]
	fn test_csv_serialization() {
		let csv: JobResultCsvResponse = CsvWrapper(valid_json()).try_into().unwrap();
		let mut wtr = csv::WriterBuilder::new()
			.has_headers(true)
			.from_writer(vec![]);
		wtr.serialize(&csv).unwrap();
		let data = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
		assert!(data.contains("test@example.com"));
		assert!(data.contains("safe"));
	}

	#[test]
	fn test_unknown_fields_ignored() {
		let mut json = valid_json();
		json.as_object_mut()
			.unwrap()
			.insert("unknown_field".into(), serde_json::json!("ignored"));
		let result: Result<JobResultCsvResponse, _> = CsvWrapper(json).try_into();
		assert!(result.is_ok());
	}
}
