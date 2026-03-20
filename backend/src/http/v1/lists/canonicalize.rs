/// Canonicalize an email address for deduplication.
///
/// Rules:
/// 1. Trim whitespace, lowercase everything
/// 2. Gmail/Googlemail: remove dots, strip +subaddress, normalize domain to gmail.com
/// 3. All other providers: strip +subaddress only
///
/// Returns None if the email doesn't contain exactly one '@'.
pub fn canonicalize_email(email: &str) -> Option<String> {
	let email = email.trim().to_lowercase();

	// Must contain exactly one '@'
	if email.matches('@').count() != 1 {
		return None;
	}

	let (local, domain) = email.rsplit_once('@')?;

	if local.is_empty() || domain.is_empty() {
		return None;
	}

	let result = match domain {
		"gmail.com" | "googlemail.com" => {
			let local = strip_subaddress(local);
			let local: String = local.chars().filter(|c| *c != '.').collect();
			if local.is_empty() {
				return None;
			}
			format!("{}@gmail.com", local)
		}
		_ => {
			let local = strip_subaddress(local);
			if local.is_empty() {
				return None;
			}
			format!("{}@{}", local, domain)
		}
	};

	Some(result)
}

fn strip_subaddress(local: &str) -> &str {
	match local.split_once('+') {
		Some((base, _)) => base,
		None => local,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_basic_canonicalize() {
		assert_eq!(
			canonicalize_email("User@Example.COM"),
			Some("user@example.com".to_string())
		);
	}

	#[test]
	fn test_gmail_removes_dots() {
		assert_eq!(
			canonicalize_email("a.b.c@gmail.com"),
			Some("abc@gmail.com".to_string())
		);
	}

	#[test]
	fn test_gmail_strips_subaddress() {
		assert_eq!(
			canonicalize_email("user+tag@gmail.com"),
			Some("user@gmail.com".to_string())
		);
	}

	#[test]
	fn test_gmail_combined() {
		assert_eq!(
			canonicalize_email("A.B.C+tag@GoogleMail.com"),
			Some("abc@gmail.com".to_string())
		);
	}

	#[test]
	fn test_googlemail_normalized() {
		assert_eq!(
			canonicalize_email("user@googlemail.com"),
			Some("user@gmail.com".to_string())
		);
	}

	#[test]
	fn test_other_provider_strips_subaddress() {
		assert_eq!(
			canonicalize_email("user+tag@company.com"),
			Some("user@company.com".to_string())
		);
	}

	#[test]
	fn test_other_provider_keeps_dots() {
		assert_eq!(
			canonicalize_email("first.last@company.com"),
			Some("first.last@company.com".to_string())
		);
	}

	#[test]
	fn test_whitespace_trimmed() {
		assert_eq!(
			canonicalize_email("  user@example.com  "),
			Some("user@example.com".to_string())
		);
	}

	#[test]
	fn test_no_at_returns_none() {
		assert_eq!(canonicalize_email("noemail"), None);
	}

	#[test]
	fn test_empty_returns_none() {
		assert_eq!(canonicalize_email(""), None);
	}

	#[test]
	fn test_empty_local_returns_none() {
		assert_eq!(canonicalize_email("@example.com"), None);
	}

	#[test]
	fn test_empty_domain_returns_none() {
		assert_eq!(canonicalize_email("user@"), None);
	}

	#[test]
	fn test_multiple_at_returns_none() {
		assert_eq!(canonicalize_email("user@@example.com"), None);
		assert_eq!(canonicalize_email("user@host@example.com"), None);
	}

	#[test]
	fn test_gmail_dots_only_local_returns_none() {
		assert_eq!(canonicalize_email("+tag@gmail.com"), None);
	}

	#[test]
	fn test_duplicates_detected() {
		// These should all canonicalize to the same value
		let emails = vec![
			"User@Gmail.com",
			"u.s.e.r@gmail.com",
			"user+newsletter@gmail.com",
			"U.S.E.R+spam@googlemail.com",
		];
		let canonical: Vec<_> = emails
			.iter()
			.filter_map(|e| canonicalize_email(e))
			.collect();
		assert!(canonical.windows(2).all(|w| w[0] == w[1]));
	}

	#[test]
	fn test_idempotent() {
		let email = "A.B+tag@Gmail.com";
		let once = canonicalize_email(email).unwrap();
		let twice = canonicalize_email(&once).unwrap();
		assert_eq!(once, twice);
	}
}
