#[must_use]
pub(super) fn auth(
	key: &[u8],
	buf: &[u8],
	assoc_data: &[&[u8]],
) -> [u8; 32] {
	use hkdf::hmac::Mac;

	// Create MAC using auth key
	let mut mac = <hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(key)
		.expect("Any size is good.");

	// Update MAC with cipher and auth data
	mac.update(buf);
	for data in assoc_data {
		mac.update(data);
	}

	// Finalize MAC and convert it to bytes
	mac.finalize().into_bytes().into()
}
