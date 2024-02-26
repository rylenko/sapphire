pub(crate) mod error;

const AUTH_HKDF_INFO: &[u8] = b"auth_hkdf_info";
const AUTH_HKDF_OUT_SIZE: usize = 88;
const AUTH_HKDF_SALT: &[u8] = &[0; AUTH_HKDF_OUT_SIZE];
const HKDF_INFO: &[u8] = b"hkdf_info";
const HKDF_OUT_SIZE: usize = 56;
const HKDF_SALT: &[u8] = &[0; HKDF_OUT_SIZE];

/// Decrpypts bytes `cipher` using `key`.
pub(crate) fn decrypt(key: &[u8], buff: &mut [u8]) {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Get key and nonce via HKDF. [..32] is key and [32..] is nonce
	let mut hkdf_out = zeroize::Zeroizing::new([0; HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(HKDF_SALT), key)
		.expand(HKDF_INFO, hkdf_out.as_mut())
		.expect("`HKDF_OUT_SIZE` must be a good length.");

	// Decrypt plain text with encryption key
	chacha20::XChaCha20::new((&hkdf_out[..32]).into(), (&hkdf_out[32..]).into())
		.apply_keystream(&mut buff);
	Ok(())
}

/// Decrypts `buff` using `key` and checks `mac` using `auth`.
pub(crate) fn decrypt_auth(
	key: &[u8],
	buff: &mut [u8],
	auth: &[&[u8]],
	mac: &[u8; 32],
) -> Result<(), error::DecryptAuth> {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Get cipher key, auth key and nonce via HKDF. [..32] is
	// encryption key, [32..64] is auth key and [64..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; AUTH_HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(AUTH_HKDF_SALT), key)
		.expand(AUTH_HKDF_INFO, hkdf_out.as_mut())
		.expect("`AUTH_HKDF_OUT_SIZE` must be a good length.");

	// Calculate HMAC using received cipher and auth data
	let mac_got: [u8; 32] = {
		use hkdf::hmac::Mac;

		// Create HMAC using auth key
		let mut mac =
			<hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(
				&hkdf_out[32..64],
			)
			.expect("Any size is good.");

		// Update HMAC with cipher and auth data
		mac.update(buff);
		mac.update(&hkdf_out[64..]);
		for a in auth {
			mac.update(a);
		}

		// Finalize HMAC and convert it to bytes
		mac.finalize().into_bytes().into()
	};

	// Compare HMACs
	if mac_got != mac {
		return Err(error::DecryptAuth::Auth);
	}

	// Decrypt plain text using cipher key and nonce
	chacha20::XChaCha20::new((&hkdf_out[..32]).into(), (&hkdf_out[64..]).into())
		.apply_keystream(&mut buff);
	Ok(())
}

/// Encrypts `buff` using `key`.
pub(crate) fn encrypt(key: &[u8], buff: &mut [u8]) {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Get key and nonce via HKDF. [..32] is key and [32..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(HKDF_SALT), key)
		.expand(HKDF_INFO, hkdf_out.as_mut())
		.expect("`HKDF_OUT_SIZE` must be a good length.");

	// Encrypt plain text with encryption key
	chacha20::XChaCha20::new((&hkdf_out[..32]).into(), (&hkdf_out[32..]).into())
		.apply_keystream(&mut buff);
	Ok(())
}

/// Encrypts `buff` using `key` and authenticates it with `auth`.
///
/// # Return
///
/// Message authentication code.
pub(crate) fn encrypt_auth(
	key: &[u8],
	buff: &mut [u8],
	auth: &[&[u8]],
) ->[u8; 32] {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Get cipher key, auth key and nonce via HKDF. [..32] is
	// encryption key, [32..64] is auth key and [64..] is nonce.
	let mut hkdf_out = zeroize::Zeroizing::new([0; AUTH_HKDF_OUT_SIZE]);
	hkdf::Hkdf::<sha2::Sha256>::new(Some(AUTH_HKDF_SALT), key)
		.expand(AUTH_HKDF_INFO, hkdf_out.as_mut())
		.expect("`AUTH_HKDF_OUT_SIZE` must be a good length.");

	// Encrypt plain text to buffer using encryption key and nonce
	chacha20::XChaCha20::new((&hkdf_out[..32]).into(), (&hkdf_out[64..]).into())
		.apply_keystream(buff);

	// Authenticate
	let mac: [u8; 32] = {
		use hkdf::hmac::Mac;

		// Create HMAC using auth key
		let mut mac =
			<hkdf::hmac::Hmac<sha2::Sha256> as Mac>::new_from_slice(
				&hkdf_out[32..64],
			)
			.expect("Any size is good.");

		// Update HMAC with cipher, nonce and auth data
		mac.update(&buff);
		mac.update(&hkdf_out[64..]);
		for a in auth {
			mac.update(a);
		}

		// Finalize HMAC and convert it to bytes
		mac.finalize().into_bytes().into()
	};
	Ok(mac)
}

#[cfg(test)]
mod tests {
	const KEY: &[u8] = b"key to encrypt plain text";
	const PLAIN: &[u8] = b"plain test 1234567890";
	const AUTH: &[&[u8]] = &[b"encrypted-header", b"user-auth-data"];

	#[test]
	fn test_decrypt_and_encrypt() {
		let mut buff = PLAIN.to_owned();
		super::encrypt(KEY, &mut buff);
		let mut cipher = buff.clone();

		// Test decryption of cipher
		assert_ne!(buff, PLAIN);
		assert_ne!(super::decrypt(b"another key", &mut buff), PLAIN);
		assert_eq!(super::decrypt(KEY, &mut cipher), PLAIN);
	}

	#[test]
	fn test_decrypt_auth_and_encrypt_auth() {
		let mut buff = PLAIN.to_owned();
		let mac = super::encrypt_auth(KEY, &mut buff, AUTH);
		let mut cipher = buff.clone();

		assert_ne!(buff, PLAIN);
		assert_ne!(super::decrypt_auth(b"another key", &mut buff, AUTH, &mac).unwrap(), PLAIN);
		assert!(super::decrypt_auth(KEY, &mut cipher, &[
			b"encrypted-header",
			b"invalid-user-auth-data"
		], &mac)
		.is_err());
		assert_eq!(super::decrypt_auth(KEY, &mut cipher, AUTH, &mac).unwrap(), PLAIN);
	}
}
