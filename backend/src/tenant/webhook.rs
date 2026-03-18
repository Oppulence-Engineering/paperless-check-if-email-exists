use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// The header name for the webhook signature.
pub const WEBHOOK_SIGNATURE_HEADER: &str = "X-Reacher-Signature-256";

/// Compute an HMAC-SHA256 signature for a payload using the given secret.
/// Returns the signature as `sha256=<hex>`.
pub fn sign_payload(secret: &str, payload: &[u8]) -> String {
	let mut mac =
		HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC accepts any key length");
	mac.update(payload);
	let result = mac.finalize();
	format!("sha256={}", hex::encode(result.into_bytes()))
}

/// Verify an HMAC-SHA256 signature against a payload.
/// The `signature` should be in `sha256=<hex>` format.
pub fn verify_signature(secret: &str, payload: &[u8], signature: &str) -> bool {
	let expected = sign_payload(secret, payload);
	// Use constant-time comparison to prevent timing attacks
	constant_time_eq(expected.as_bytes(), signature.as_bytes())
}

/// Constant-time byte comparison to prevent timing attacks.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
	if a.len() != b.len() {
		return false;
	}
	let mut result = 0u8;
	for (x, y) in a.iter().zip(b.iter()) {
		result |= x ^ y;
	}
	result == 0
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sign_and_verify() {
		let secret = "my-webhook-secret";
		let payload = b"hello world";
		let sig = sign_payload(secret, payload);

		assert!(sig.starts_with("sha256="));
		assert!(verify_signature(secret, payload, &sig));
	}

	#[test]
	fn test_wrong_secret_fails() {
		let payload = b"hello world";
		let sig = sign_payload("correct-secret", payload);
		assert!(!verify_signature("wrong-secret", payload, &sig));
	}

	#[test]
	fn test_tampered_payload_fails() {
		let secret = "my-secret";
		let sig = sign_payload(secret, b"original payload");
		assert!(!verify_signature(secret, b"tampered payload", &sig));
	}

	#[test]
	fn test_deterministic() {
		let secret = "test";
		let payload = b"same input";
		let sig1 = sign_payload(secret, payload);
		let sig2 = sign_payload(secret, payload);
		assert_eq!(sig1, sig2);
	}

	#[test]
	fn test_different_payloads_different_sigs() {
		let secret = "test";
		let sig1 = sign_payload(secret, b"payload one");
		let sig2 = sign_payload(secret, b"payload two");
		assert_ne!(sig1, sig2);
	}

	#[test]
	fn test_constant_time_eq() {
		assert!(constant_time_eq(b"hello", b"hello"));
		assert!(!constant_time_eq(b"hello", b"world"));
		assert!(!constant_time_eq(b"hello", b"hell"));
	}
}
