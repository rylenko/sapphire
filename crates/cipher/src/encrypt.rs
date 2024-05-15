/// Encrypts `buf`fer using `key` and creates authentication code using
/// encrypted `buf`fer and `assoc`iated data.
///
/// Note that encryption and authentication occurs using keys derived from
/// `key`.
///
/// # Return
///
/// Authentication code of encrypted `buf`fer and `assoc`iated data using
/// derived authentication key.
#[must_use]
pub fn encrypt(key: &[u8], buf: &mut [u8], assoc: &[&[u8]]) -> [u8; 32] {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Derive new encryption key, authentication key and nonce.
	let mut deriver = super::kdf::Deriver::new();
	deriver.derive(key);

	// Encrypt buffer using derived encryption key and nonce.
	chacha20::XChaCha20::new(
		deriver.encrypt_key().into(),
		deriver.nonce().into(),
	)
	.apply_keystream(buf);

	// Create authentication code of encrypted buffer and associated data using
	// derived authentication key.
	super::mac::auth(deriver.auth_key(), buf, assoc)
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_encrypt() {
		let mut buf = [10; 32];
		let auth = super::encrypt(b"key", &mut buf, &[b"assoc1", b"assoc2"]);
		assert_eq!(buf, [
			180, 14, 21, 155, 162, 31, 103, 119, 216, 196, 122, 119, 232, 216,
			160, 103, 174, 171, 163, 194, 128, 109, 220, 19, 223, 51, 23, 22,
			58, 207, 144, 110
		]);
		assert_eq!(auth, [
			22, 192, 247, 139, 160, 169, 189, 126, 66, 185, 63, 25, 47, 100,
			200, 37, 98, 244, 240, 10, 147, 218, 226, 20, 212, 144, 51, 233,
			209, 241, 203, 230
		]);
	}
}
