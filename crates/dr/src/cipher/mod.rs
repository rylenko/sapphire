pub(crate) mod error;

const HKDF_INFO: &[u8] = b"cipher_hkdf_info";
const HKDF_OUT_LEN: usize = 88;
const HKDF_SALT: &[u8] = &[0; HKDF_OUT_LEN];

/// Decrypts `buff` using `key` and authenticates it using `auth`.
///
/// Decrypts everything except the last 32 bytes. The last 32 bytes are
/// occupied by MAC. Also the buffer will not be corrupted in case of errors.
pub(crate) fn decrypt(
	key: &[u8],
	buff: &mut [u8],
	auth: &[&[u8]],
) -> Result<(), error::Decrypt> {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Check buffer length
	if buff.len() <= 32 {
		return Err(error::Decrypt::SmallBuff);
	}
	let cipher_len = buff.len() - 32;

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
		mac.update(&buff[..cipher_len]);
		for a in auth {
			mac.update(a);
		}
		// Finalize MAC and convert it to bytes
		mac.finalize().into_bytes()
	};
	// Compare MACs
	if got_mac.as_slice() != &buff[cipher_len..] {
		return Err(error::Decrypt::Auth);
	}

	// Decrypt plain text using cipher key and nonce
	chacha20::XChaCha20::new(
		(&hkdf_out[..32]).into(),
		(&hkdf_out[64..]).into(),
	)
	.apply_keystream(&mut buff[..cipher_len]);
	Ok(())
}

/// Encrypts `buff` using `key` and authenticates it with `auth`.
///
/// Encrypts everything except the last 32 bytes. The last 32 bytes are
/// occupied by MAC. Also the buffer will not be corrupted in case of errors.
pub(crate) fn encrypt(
	key: &[u8],
	buff: &mut [u8],
	auth: &[&[u8]],
) -> Result<(), error::Encrypt> {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Check buffer length
	if buff.len() <= 32 {
		return Err(error::Encrypt::SmallBuff);
	}
	let cipher_len = buff.len() - 32;

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
	.apply_keystream(&mut buff[..cipher_len]);

	// Authenticate and copy MAC to buffer
	let mac = {
		use hkdf::hmac::Mac;
		// Create MAC using auth key
		let mut mac = <hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(
			&hkdf_out[32..64],
		)
		.expect("Any size is good.");
		// Update MAC with encrypted buffer and auth data
		mac.update(&buff[..cipher_len]);
		for a in auth {
			mac.update(a);
		}
		// Finalize MAC and convert it to bytes
		mac.finalize().into_bytes()
	};
	buff[cipher_len..].copy_from_slice(mac.as_slice());
	Ok(())
}

#[cfg(test)]
mod tests {
	const KEY: &[u8] = b"key to encrypt plain text";
	const BUFF: [u8; 42] = *b"1234567890--------------MAC-HERE----------";
	const CIPHER_LEN: usize = BUFF.len() - 32;
	const AUTH: &[&[u8]] = &[b"encrypted-header", b"user-auth-data"];

	#[test]
	fn test_decrypt_and_encrypt() {
		// Test too small buffer
		let mut invalid_buff = [0; 32];
		assert!(super::encrypt(KEY, &mut invalid_buff, AUTH).is_err());

		// Clone and encrypt the buffer
		let mut buff = BUFF;
		super::encrypt(KEY, &mut buff, AUTH).unwrap();

		assert_ne!(buff, BUFF);
		assert!(super::decrypt(b"another key", &mut buff, AUTH).is_err());
		assert_ne!(buff[..CIPHER_LEN], BUFF[..CIPHER_LEN]);
		assert!(
			super::decrypt(KEY, &mut buff, &[b"invalid-auth-data"]).is_err()
		);
		assert_ne!(buff[..CIPHER_LEN], BUFF[..CIPHER_LEN]);
		assert!(super::decrypt(KEY, &mut buff, AUTH).is_ok());
		assert_eq!(buff[..CIPHER_LEN], BUFF[..CIPHER_LEN]);
	}
}
