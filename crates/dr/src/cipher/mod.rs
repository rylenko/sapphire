pub(crate) mod error;

const AUTH_HKDF_INFO: &[u8] = b"auth_hkdf_info";
const AUTH_HKDF_OUT_SIZE: usize = 88;
const AUTH_HKDF_SALT: &[u8] = &[0; AUTH_HKDF_OUT_SIZE];
const HKDF_INFO: &[u8] = b"hkdf_info";
const HKDF_OUT_SIZE: usize = 56;
const HKDF_SALT: &[u8] = &[0; HKDF_OUT_SIZE];

/// Decrpypts header bytes `cipher` using `key`.
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
	let cipher_len_without_hmac = cipher.len() - 32;

	// Get cipher key, auth key and nonce via HKDF. [..32] is
	// encryption key, [32..64] is auth key and [64..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; AUTH_HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(AUTH_HKDF_SALT), key)
		.expand(AUTH_HKDF_INFO, hkdf_out.as_mut())
		.expect("`AUTH_HKDF_OUT_SIZE` must be a good length.");

	// Calculate HMAC using received cipher and auth data
	let got_hmac: [u8; 32] = {
		use hkdf::hmac::Mac;

		// Create HMAC using auth key
		let mut hmac =
			<hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(
				&hkdf_out[32..64],
			)
			.expect("Any size is good.");

		// Update HMAC with cipher and auth data
		hmac.update(cipher);
		for a in auth {
			hmac.update(a);
		}

		// Finalize HMAC and convert it to bytes
		hmac.finalize().into_bytes().into()
	};

	// Compare HMACs
	if got_hmac != cipher[cipher_len_without_hmac..] {
		return Err(error::DecryptAuth::Auth);
	}

	// Decrypt plain text using cipher key and nonce
	let plain =
		chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
			.decrypt(
				(&hkdf_out[64..]).into(),
				&cipher[..cipher_len_without_hmac],
			)?;
	Ok(plain)
}

/// Encrypts header bytes `plain` using key.
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
	let mut cipher =
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
		hmac.update(&cipher);
		for a in auth {
			hmac.update(a);
		}

		// Finalize HMAC and convert it to bytes
		hmac.finalize().into_bytes().into()
	};

	// Append authenticated HMAC to end of cipher text
	cipher.extend(hmac);
	Ok(cipher)
}
