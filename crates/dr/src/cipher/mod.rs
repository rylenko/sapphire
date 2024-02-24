pub(crate) mod error;

const AUTH_HKDF_INFO: &[u8] = b"auth_hkdf_info";
const AUTH_HKDF_OUT_SIZE: usize = 88;
const AUTH_HKDF_SALT: &[u8] = &[0; AUTH_HKDF_OUT_SIZE];
const HKDF_INFO: &[u8] = b"hkdf_info";
const HKDF_OUT_SIZE: usize = 56;
const HKDF_SALT: &[u8] = &[0; HKDF_OUT_SIZE];

/// Decrpypts bytes `cipher` using `key`.
pub(crate) fn decrypt(
	key: &[u8],
	cipher: &[u8],
) -> Result<alloc::vec::Vec<u8>, error::Decrypt> {
	use chacha20poly1305::{aead::Aead as _, KeyInit as _};

	// Get key and nonce via HKDF. [..32] is key and [32..] is nonce
	let mut hkdf_out = zeroize::Zeroizing::new([0; HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(HKDF_SALT), key)
		.expand(HKDF_INFO, hkdf_out.as_mut())
		.expect("`HKDF_OUT_SIZE` must be a good length.");

	// Decrypt plain text with encryption key
	let plain =
		chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
			.decrypt((&hkdf_out[32..]).into(), cipher)?;
	Ok(plain)
}

/// Decrypts `cipher` using `key` and authentication data `auth`.
pub(crate) fn decrypt_auth(
	key: &[u8],
	cipher: &[u8],
	auth: &[&[u8]],
) -> Result<alloc::vec::Vec<u8>, error::DecryptAuth> {
	use chacha20poly1305::{aead::Aead as _, KeyInit as _};

	// Get cipher's length without HMAC
	if cipher.len() < 32 {
		return Err(error::DecryptAuth::NoHmac);
	}

	// Split cipher into encrypted_plain and hmac
	let encrypted_plain = &cipher[..cipher.len() - 32];
	let hmac_expected = &cipher[cipher.len() - 32..];

	// Get cipher key, auth key and nonce via HKDF. [..32] is
	// encryption key, [32..64] is auth key and [64..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; AUTH_HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(AUTH_HKDF_SALT), key)
		.expand(AUTH_HKDF_INFO, hkdf_out.as_mut())
		.expect("`AUTH_HKDF_OUT_SIZE` must be a good length.");

	// Calculate HMAC using received cipher and auth data
	let hmac_got: [u8; 32] = {
		use hkdf::hmac::Mac;

		// Create HMAC using auth key
		let mut hmac =
			<hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(
				&hkdf_out[32..64],
			)
			.expect("Any size is good.");

		// Update HMAC with cipher and auth data
		hmac.update(encrypted_plain);
		for a in auth {
			hmac.update(a);
		}

		// Finalize HMAC and convert it to bytes
		hmac.finalize().into_bytes().into()
	};

	// Compare HMACs
	if hmac_got != hmac_expected {
		return Err(error::DecryptAuth::Auth);
	}

	// Decrypt plain text using cipher key and nonce
	let plain =
		chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
			.decrypt((&hkdf_out[64..]).into(), encrypted_plain)?;
	Ok(plain)
}

/// Encrypts bytes `plain` using `key`.
pub(crate) fn encrypt(
	key: &[u8],
	plain: &[u8],
) -> Result<alloc::vec::Vec<u8>, error::Encrypt> {
	use chacha20poly1305::{aead::Aead as _, KeyInit as _};

	// Get key and nonce via HKDF. [..32] is key and [32..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(HKDF_SALT), key)
		.expand(HKDF_INFO, hkdf_out.as_mut())
		.expect("`HKDF_OUT_SIZE` must be a good length.");

	// Encrypt plain text with encryption key
	let cipher =
		chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
			.encrypt((&hkdf_out[32..]).into(), plain)?;
	Ok(cipher)
}

/// Encrypts `plain` using `key` and authentication data `auth`.
pub(crate) fn encrypt_auth(
	key: &[u8],
	plain: &[u8],
	auth: &[&[u8]],
) -> Result<alloc::vec::Vec<u8>, error::EncryptAuth> {
	use chacha20poly1305::{aead::Aead as _, KeyInit as _};

	// Get cipher key, auth key and nonce via HKDF. [..32] is
	// encryption key, [32..64] is auth key and [64..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; AUTH_HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(AUTH_HKDF_SALT), key)
		.expand(AUTH_HKDF_INFO, hkdf_out.as_mut())
		.expect("`AUTH_HKDF_OUT_SIZE` must be a good length.");

	// Encrypt plain text using encryption key and nonce
	let mut encrypted_plain =
		chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
			.encrypt((&hkdf_out[64..]).into(), plain)?;

	// Authenticate
	let hmac: [u8; 32] = {
		use hkdf::hmac::Mac;

		// Create HMAC using auth key
		let mut hmac =
			<hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(
				&hkdf_out[32..64],
			)
			.expect("Any size is good.");

		// Update HMAC with cipher and auth data
		hmac.update(&encrypted_plain);
		for a in auth {
			hmac.update(a);
		}

		// Finalize HMAC and convert it to bytes
		hmac.finalize().into_bytes().into()
	};

	// Append authenticated HMAC to end of cipher text
	encrypted_plain.extend(hmac);
	Ok(encrypted_plain)
}

#[cfg(test)]
mod tests {
	const KEY: &[u8] = b"key to encrypt plain text";
	const PLAIN: &[u8] = b"plain test 1234567890";

	#[test]
	fn test_decrypt_and_encrypt() {
		let cipher = super::encrypt(KEY, PLAIN).unwrap();

		// Test decryption of cipher
		assert_ne!(cipher, PLAIN);
		assert!(super::decrypt(b"another key", &cipher).is_err());
		assert_eq!(super::decrypt(KEY, &cipher).unwrap(), PLAIN);

		// Test uniqueness of cipher
		assert_ne!(cipher, super::encrypt(KEY, b"another plain").unwrap());
		assert_ne!(cipher, super::encrypt(b"another key", PLAIN).unwrap());
	}
}
