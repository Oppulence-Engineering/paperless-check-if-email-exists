//! Public destinations for connections derived from untrusted input.

use std::io;
use std::net::{IpAddr, SocketAddr};

/// Excludes private, reserved, documentation and address-translation ranges.
pub fn is_private_ip(ip: IpAddr) -> bool {
	match ip {
		IpAddr::V4(ip) => {
			let [a, b, c, _] = ip.octets();
			ip.is_private()
				|| ip.is_loopback()
				|| ip.is_link_local()
				|| a == 0 || a >= 224
				|| (a == 100 && b & 0xc0 == 64)
				|| (a == 192 && b == 0 && matches!(c, 0 | 2))
				|| (a == 192 && b == 88 && c == 99)
				|| (a == 198 && matches!(b, 18 | 19))
				|| (a == 198 && b == 51 && c == 100)
				|| (a == 203 && b == 0 && c == 113)
		}
		IpAddr::V6(ip) => {
			if let Some(v4) = ip.to_ipv4_mapped() {
				return is_private_ip(IpAddr::V4(v4));
			}
			let [a, b, ..] = ip.segments();
			// Ordinary global unicast only; transition ranges can encode private IPv4.
			a & 0xe000 != 0x2000
				|| (a == 0x2001 && (b < 0x0200 || b == 0x0db8))
				|| a == 0x2002
				|| (a == 0x3fff && b < 0x1000)
		}
	}
}

/// Resolve once; callers must connect to these addresses without resolving again.
pub async fn resolve_public_addresses(host: &str, port: u16) -> io::Result<Vec<SocketAddr>> {
	let addrs = tokio::net::lookup_host((host.trim_end_matches('.'), port))
		.await?
		.collect::<Vec<_>>();
	validate_public_addresses(&addrs)?;
	Ok(addrs)
}

fn validate_public_addresses(addrs: &[SocketAddr]) -> io::Result<()> {
	if addrs.is_empty() || addrs.iter().any(|addr| is_private_ip(addr.ip())) {
		return Err(io::Error::new(
			io::ErrorKind::PermissionDenied,
			"Destination must resolve exclusively to public addresses",
		));
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn rejects_private_reserved_and_mixed_answers() {
		for ip in [
			"0.1.2.3",
			"10.0.0.1",
			"100.64.0.1",
			"127.0.0.1",
			"169.254.169.254",
			"172.31.255.255",
			"192.168.0.1",
			"192.0.0.1",
			"192.0.2.1",
			"192.88.99.1",
			"198.19.0.1",
			"198.51.100.1",
			"203.0.113.1",
			"224.0.0.1",
			"240.0.0.1",
			"::",
			"::1",
			"::ffff:127.0.0.1",
			"64:ff9b::a00:1",
			"64:ff9b:1::1",
			"100::1",
			"2001::1",
			"2001:db8::1",
			"2002:a00:1::1",
			"3fff::1",
			"5f00::1",
			"fc00::1",
			"fe80::1",
			"fec0::1",
			"ff00::1",
		] {
			let private = SocketAddr::new(ip.parse().unwrap(), 25);
			assert!(is_private_ip(private.ip()), "{}", ip);
			assert!(validate_public_addresses(&["8.8.8.8:25".parse().unwrap(), private]).is_err());
			assert!(resolve_public_addresses(ip, 25).await.is_err(), "{}", ip);
		}
		for ip in [
			"8.8.8.8",
			"1.1.1.1",
			"2607:f8b0:400e:c00::1a",
			"::ffff:8.8.8.8",
		] {
			assert!(!is_private_ip(ip.parse().unwrap()), "{}", ip);
		}
		assert!(validate_public_addresses(&[]).is_err());
	}
}
