pub(crate) mod error;

const HKDF_INFO: &[u8] = b"cipher_hkdf_info";
const HKDF_OUT_LEN: usize = 88;
const HKDF_SALT: &[u8] = &[0; HKDF_OUT_LEN];

/// Decrypts `buf` using `key` and authenticates it using `auth`.
///
/// Decrypts everything except the last 32 bytes. The last 32 bytes are
/// occupied by MAC. Also the buffer will not be corrupted in case of errors.
pub(crate) fn decrypt(
	key: &[u8],
	buf: &mut [u8],
	auth: &[&[u8]],
) -> Result<(), error::Decrypt> {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Check buffer length
	if buf.len() <= 32 {
		return Err(error::Decrypt::SmallBuf);
	}
	let cipher_len = buf.len() - 32;

	// [..32] is encryption key, [32..64] is auth key and [64..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; HKDF_OUT_LEN]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(HKDF_SALT), key)
		.expand(HKDF_INFO, hkdf_out.as_mut())
		.expect("`HKDF_OUT_LEN` must be a good length.");

	// Calculate MAC using received cipher and auth data
	let got_mac = {
		use hkdf::hmac::Mac;

		// Create MAC using auth key
		let mut mac = <hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(
			&hkdf_out[32..64],
		)
		.expect("Any size is good.");
		// Update MAC with cipher and auth data
		mac.update(&buf[..cipher_len]);
		for a in auth {
			mac.update(a);
		}
		// Finalize MAC and convert it to bytes
		mac.finalize().into_bytes()
	};
	// Compare MACs
	if got_mac.as_slice() != &buf[cipher_len..] {
		return Err(error::Decrypt::Auth);
	}

	// Decrypt plain text using cipher key and nonce
	chacha20::XChaCha20::new(
		(&hkdf_out[..32]).into(),
		(&hkdf_out[64..]).into(),
	)
	.apply_keystream(&mut buf[..cipher_len]);
	Ok(())
}

/// Encrypts `buf` using `key` and authenticates it with `auth`.
///
/// Encrypts everything except the last 32 bytes. The last 32 bytes are
/// occupied by MAC. Also the buffer will not be corrupted in case of errors.
pub(crate) fn encrypt(
	key: &[u8],
	buf: &mut [u8],
	auth: &[&[u8]],
) -> Result<(), error::Encrypt> {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Check buffer length
	if buf.len() <= 32 {
		return Err(error::Encrypt::SmallBuf);
	}
	let cipher_len = buf.len() - 32;

	// [..32] is encryption key, [32..64] is auth key and [64..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; HKDF_OUT_LEN]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(HKDF_SALT), key)
		.expand(HKDF_INFO, hkdf_out.as_mut())
		.expect("`HKDF_OUT_LEN` must be a good length.");

	// Encrypt using encryption key and nonce
	chacha20::XChaCha20::new(
		(&hkdf_out[..32]).into(),
		(&hkdf_out[64..]).into(),
	)
	.apply_keystream(&mut buf[..cipher_len]);

	// Authenticate and copy MAC to buffer
	let mac = {
		use hkdf::hmac::Mac;
		// Create MAC using auth key
		let mut mac = <hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(
			&hkdf_out[32..64],
		)
		.expect("Any size is good.");
		// Update MAC with encrypted buffer and auth data
		mac.update(&buf[..cipher_len]);
		for a in auth {
			mac.update(a);
		}
		// Finalize MAC and convert it to bytes
		mac.finalize().into_bytes()
	};
	buf[cipher_len..].copy_from_slice(mac.as_slice());
	Ok(())
}

#[cfg(test)]
mod tests {
	const KEY: &[u8] = b"key to encrypt plain text";
	const BUF: [u8; 42] = *b"1234567890--------------MAC-HERE----------";
	const CIPHER_LEN: usize = BUF.len() - 32;
	const AUTH: &[&[u8]] = &[b"encrypted-header", b"user-auth-data"];

	#[test]
	fn test_decrypt_and_encrypt() {
		// Test too small buffer
		let mut invalid_buf = [0; 32];
		assert!(super::encrypt(KEY, &mut invalid_buf, AUTH).is_err());

		// Clone and encrypt the buffer
		let mut buf = BUF;
		super::encrypt(KEY, &mut buf, AUTH).unwrap();

		assert_ne!(buf, BUF);
		assert!(super::decrypt(b"another key", &mut buf, AUTH).is_err());
		assert_ne!(buf[..CIPHER_LEN], BUF[..CIPHER_LEN]);
		assert!(
			super::decrypt(KEY, &mut buf, &[b"invalid-auth-data"]).is_err()
		);
		assert_ne!(buf[..CIPHER_LEN], BUF[..CIPHER_LEN]);
		assert!(super::decrypt(KEY, &mut buf, AUTH).is_ok());
		assert_eq!(buf[..CIPHER_LEN], BUF[..CIPHER_LEN]);
	}
}
