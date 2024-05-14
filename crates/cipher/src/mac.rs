type MacImpl = hmac::Hmac<sha2::Sha256>;

/// Authenticates concatenation of buffer and associated data using passed key.
#[must_use]
pub fn auth(key: &[u8], buf: &[u8], assoc: &[&[u8]]) -> [u8; 32] {
	use hmac::Mac as _;

	// Create MAC instance using accepted key.
	let mut mac = MacImpl::new_from_slice(key).expect("Any size is good.");

	// Update MAC with accepted buffer.
	mac.update(buf);
	// Update MAC with accepted associated data.
	for data in assoc {
		mac.update(data);
	}

	// Finalize MAC into return array.
	mac.finalize().into_bytes().into()
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_auth() {
		let mac = super::auth(b"key", b"buf", &[b"assoc1", b"assoc2"]);
		assert_eq!(mac, [
			190, 67, 118, 3, 32, 204, 105, 154, 67, 54, 231, 226, 3, 245, 208,
			32, 62, 15, 71, 76, 142, 242, 203, 183, 115, 100, 178, 229, 224,
			119, 252, 107
		]);
	}
}
