const HEADER_HKDF_INFO: &[u8] = b"default_header_hkdf_info";
const HEADER_HKDF_SALT: [u8; 56] = [0; 56];
const HKDF_INFO: &[u8] = b"default_hkdf_info";
const HKDF_SALT: [u8; 80] = [0; 80];
const MSG_CHAIN_KEY_MAC_BYTE: u8 = 0x2;
const MSG_KEY_MAC_BYTE: u8 = 0x1;
const KDF_ROOT_CHAIN_HKDF_INFO: &[u8] = b"default_kdf_root_hkdf_info";

/// Default crypto provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Provider;

impl crate::crypto::Provider for Provider {
	type DecryptError = super::error::Decrypt;
	type DecryptHeaderError = super::error::DecryptHeader;
	type EncryptError = super::error::Encrypt;
	type EncryptHeaderError = super::error::EncryptHeader;
	type HeaderKey = super::header_key::HeaderKey;
	type KeyPair = super::key_pair::KeyPair;
	type MsgChainKey = zeroize::Zeroizing<[u8; 32]>;
	type MsgKey = zeroize::Zeroizing<[u8; 32]>;
	type RootKey = zeroize::Zeroizing<[u8; 32]>;
	type SharedSecret = x25519_dalek::SharedSecret;

	fn decrypt(
		key: &Self::MsgKey,
		cipher: &[u8],
		auth: &[&[u8]],
	) -> Result<alloc::vec::Vec<u8>, Self::DecryptError> {
		use chacha20poly1305::{aead::Aead as _, KeyInit as _};

		// Get cipher's length without HMAC
		if cipher.len() < 32 {
			return Err(Self::DecryptError::NoHmac);
		}
		let cipher_len_without_hmac = cipher.len() - 32;

		// Get cipher key, auth key and nonce via HKDF. [..32] is
		// encryption key, [32..64] is auth key and [64..] is nonce.
		let mut hkdf_out = zeroize::Zeroizing::new([0; 88]);
		hkdf::Hkdf::<sha2::Sha256>::new(Some(&HKDF_SALT), key.as_ref())
			.expand(HKDF_INFO, hkdf_out.as_mut())
			.expect("88 is good length.");

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
			return Err(super::error::Decrypt::Auth);
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

	fn decrypt_header(
		key: &Self::HeaderKey,
		cipher: &[u8],
	) -> Result<alloc::vec::Vec<u8>, Self::DecryptHeaderError> {
		use chacha20poly1305::{aead::Aead as _, KeyInit as _};

		// Get key and nonce via HKDF. [..32] is key and [32..] is nonce.
		let mut hkdf_out = zeroize::Zeroizing::new([0; 56]);
		hkdf::Hkdf::<sha2::Sha256>::new(Some(&HEADER_HKDF_SALT), key.as_ref())
			.expand(HEADER_HKDF_INFO, hkdf_out.as_mut())
			.expect("56 is good length.");

		// Decrypt plain text with encryption key
		let plain =
			chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
				.decrypt((&hkdf_out[32..]).into(), cipher)?;
		Ok(plain)
	}

	#[inline]
	fn dh(
		pair: &Self::KeyPair,
		public: &<Self::KeyPair as crate::crypto::KeyPair>::Public,
	) -> Self::SharedSecret {
		use crate::crypto::KeyPair as _;
		pair.private().diffie_hellman(public)
	}

	fn encrypt(
		key: &Self::MsgKey,
		plain: &[u8],
		auth: &[&[u8]],
	) -> Result<alloc::vec::Vec<u8>, Self::EncryptError> {
		use chacha20poly1305::{aead::Aead as _, KeyInit as _};

		// Get cipher key, auth key and nonce via HKDF. [..32] is
		// encryption key, [32..64] is auth key and [64..] is nonce.
		let mut hkdf_out = zeroize::Zeroizing::new([0; 88]);
		hkdf::Hkdf::<sha2::Sha256>::new(Some(&HKDF_SALT), key.as_ref())
			.expand(HKDF_INFO, hkdf_out.as_mut())
			.expect("88 is good length.");

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

	fn encrypt_header(
		key: &Self::HeaderKey,
		plain: &[u8],
	) -> Result<alloc::vec::Vec<u8>, Self::EncryptHeaderError> {
		use chacha20poly1305::{aead::Aead as _, KeyInit as _};

		// Get key and nonce via HKDF. [..32] is key and [32..] is nonce.
		let mut hkdf_out = zeroize::Zeroizing::new([0; 56]);
		hkdf::Hkdf::<sha2::Sha256>::new(Some(&HEADER_HKDF_SALT), key.as_ref())
			.expand(HEADER_HKDF_INFO, hkdf_out.as_mut())
			.expect("56 is good length.");

		// Encrypt plain text with encryption key
		let cipher =
			chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
				.encrypt((&hkdf_out[32..]).into(), plain)?;
		Ok(cipher)
	}

	fn kdf_root(
		key: &Self::RootKey,
		input: &Self::SharedSecret,
	) -> (Self::RootKey, Self::MsgChainKey, Self::HeaderKey) {
		// Get output key material with new keys via HKDF
		let mut hkdf_out = zeroize::Zeroizing::new([0; 96]);
		hkdf::Hkdf::<sha2::Sha256>::new(Some(key.as_ref()), input.as_bytes())
			.expand(KDF_ROOT_CHAIN_HKDF_INFO, hkdf_out.as_mut())
			.expect("96 is good length.");

		// Split output into keys
		let mut root_key = Self::RootKey::from([0; 32]);
		root_key.copy_from_slice(&hkdf_out[..32]);
		let mut msg_chain_key = Self::MsgChainKey::from([0; 32]);
		msg_chain_key.copy_from_slice(&hkdf_out[32..64]);
		let mut header_key = Self::HeaderKey::from([0; 32]);
		header_key.copy_from_slice(&hkdf_out[64..]);

		(root_key, msg_chain_key, header_key)
	}

	fn kdf_msg_chain(
		key: &Self::MsgChainKey,
	) -> (Self::MsgChainKey, Self::MsgKey) {
		use hkdf::hmac::Mac as _;

		// Create mac with key bytes as key
		let mut msg_chain_key_mac =
			hkdf::hmac::Hmac::<sha2::Sha256>::new_from_slice(key.as_ref())
				.expect("Any size is good.");
		let mut msg_key_mac = msg_chain_key_mac.clone();

		// Update key HMACs with their bytes
		msg_chain_key_mac.update(&[MSG_CHAIN_KEY_MAC_BYTE]);
		msg_key_mac.update(&[MSG_KEY_MAC_BYTE]);

		// Finalize HMACs
		(
			Self::MsgChainKey::new(
				msg_chain_key_mac.finalize().into_bytes().into(),
			),
			Self::MsgKey::new(msg_key_mac.finalize().into_bytes().into()),
		)
	}
}
