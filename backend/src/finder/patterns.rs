use std::collections::HashSet;
use unicode_normalization::{char::is_combining_mark, UnicodeNormalization};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
	pub email: String,
	pub pattern: String,
}

pub fn normalize_name(input: &str) -> String {
	input
		.trim()
		.nfkd()
		.filter(|c| !is_combining_mark(*c))
		.collect::<String>()
		.to_lowercase()
		.chars()
		.filter(|c| c.is_ascii_alphanumeric())
		.collect()
}

pub fn normalize_domain(input: &str) -> String {
	input.trim().to_lowercase()
}

pub fn generate_candidates(first_name: &str, last_name: &str, domain: &str) -> Vec<Candidate> {
	let first = normalize_name(first_name);
	let last = normalize_name(last_name);
	let domain = normalize_domain(domain);

	if first.is_empty() || last.is_empty() || domain.is_empty() {
		return Vec::new();
	}

	let first_initial = first.chars().next().unwrap().to_string();
	let last_initial = last.chars().next().unwrap().to_string();
	let patterns = vec![
		("first", first.clone()),
		("last", last.clone()),
		("first.last", format!("{}.{}", first, last)),
		("first_last", format!("{}_{}", first, last)),
		("firstlast", format!("{}{}", first, last)),
		("f.last", format!("{}.{}", first_initial, last)),
		("first.l", format!("{}.{}", first, last_initial)),
		("flast", format!("{}{}", first_initial, last)),
		("firstl", format!("{}{}", first, last_initial)),
		("last.first", format!("{}.{}", last, first)),
		("lastf", format!("{}{}", last, first_initial)),
		("f_last", format!("{}_{}", first_initial, last)),
		("first-last", format!("{}-{}", first, last)),
	];

	let mut seen = HashSet::new();
	patterns
		.into_iter()
		.filter_map(|(pattern, local_part)| {
			let email = format!("{}@{}", local_part, domain);
			if seen.insert(email.clone()) {
				Some(Candidate {
					email,
					pattern: pattern.to_string(),
				})
			} else {
				None
			}
		})
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn generate_candidates_builds_expected_patterns() {
		let candidates = generate_candidates("John", "Doe", "Example.com");
		assert_eq!(candidates.len(), 13);
		assert_eq!(candidates[0].email, "john@example.com");
		assert_eq!(candidates[2].email, "john.doe@example.com");
		assert_eq!(candidates[7].email, "jdoe@example.com");
	}

	#[test]
	fn normalization_strips_accents_and_symbols() {
		let candidates = generate_candidates("Jo-hn", "D'oe", "example.com");
		assert!(candidates
			.iter()
			.any(|candidate| candidate.email == "john.doe@example.com"));

		let accent = generate_candidates("Jöhn", "Doé", "example.com");
		assert!(accent
			.iter()
			.any(|candidate| candidate.email == "john.doe@example.com"));
	}
}
