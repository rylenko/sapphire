const MSG_CHAIN_KEY_MAC_BYTE: u8 = 0x2;
const ENCRYPT_HKDF_SALT: [u8; 80] = [0; 80];
const ENCRYPT_HKDF_INFO: &[u8] = b"default_encrypt_hkdf_info";
const ENCRYPT_HEADER_BYTES_HKDF_SALT: [u8; 56] = [0; 56];
const ENCRYPT_HEADER_BYTES_HKDF_INFO: &[u8] =
	b"default_encrypt_header_bytes_hkdf_info";
const MSG_KEY_MAC_BYTE: u8 = 0x1;
const KDF_ROOT_CHAIN_HKDF_INFO: &[u8] = b"default_kdf_root_chain_hkdf_info";

/// Default crypto provider.
#[derive(Debug, Eq, PartialEq)]
pub struct Provider;

impl crate::crypto::Provider for Provider {
	type EncryptError = super::error::Encrypt;
	type EncryptHeaderBytesError = super::error::EncryptHeaderBytes;
	type HeaderKey = super::header_key::HeaderKey;
	type KeyPair = super::key_pair::KeyPair;
	type MsgChainKey = zeroize::Zeroizing<[u8; 32]>;
	type MsgKey = zeroize::Zeroizing<[u8; 32]>;
	type RootChainKey = zeroize::Zeroizing<[u8; 32]>;
	type SharedSecret = x25519_dalek::SharedSecret;

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

		// Get encryption key, auth key and nonce via HKDF. [..32] is
		// encryption key, [32..64] is auth key and [64..] is nonce.
		let mut hkdf_out = zeroize::Zeroizing::new([0; 88]);
		hkdf::Hkdf::<sha2::Sha256>::new(
			Some(&ENCRYPT_HKDF_SALT),
			key.as_ref(),
		)
		.expand(ENCRYPT_HKDF_INFO, hkdf_out.as_mut())
		.expect("88 is good length.");

		// Encrypt plain text using encryption key and nonce
		let mut cipher =
			chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
				.encrypt((&hkdf_out[64..]).into(), plain)?;

		// Get HMAC output to append to cipher
		let hmac_out: [u8; 32] = {
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
		cipher.extend(&hmac_out);

		Ok(cipher)
	}

	fn encrypt_header_bytes(
		key: &Self::HeaderKey,
		bytes: &[u8],
	) -> Result<alloc::vec::Vec<u8>, Self::EncryptHeaderBytesError> {
		use chacha20poly1305::{aead::Aead as _, KeyInit as _};

		// Get encryption key and nonce via HKDF. [..32] is
		// encryption key and [32..] is nonce.
		let mut hkdf_out = zeroize::Zeroizing::new([0; 56]);
		hkdf::Hkdf::<sha2::Sha256>::new(
			Some(&ENCRYPT_HEADER_BYTES_HKDF_SALT),
			key.as_ref(),
		)
		.expand(ENCRYPT_HEADER_BYTES_HKDF_INFO, hkdf_out.as_mut())
		.expect("56 is good length.");

		// Encrypt plain text with encryption key
		let cipher =
			chacha20poly1305::XChaCha20Poly1305::new((&hkdf_out[..32]).into())
				.encrypt((&hkdf_out[32..]).into(), bytes)?;
		Ok(cipher)
	}

	fn kdf_root_chain(
		key: &Self::RootChainKey,
		input: &Self::SharedSecret,
	) -> (Self::RootChainKey, Self::MsgChainKey, Self::HeaderKey) {
		// Get output key material with new keys via HKDF
		let mut hkdf_out = zeroize::Zeroizing::new([0; 96]);
		hkdf::Hkdf::<sha2::Sha256>::new(Some(key.as_ref()), input.as_bytes())
			.expand(KDF_ROOT_CHAIN_HKDF_INFO, hkdf_out.as_mut())
			.expect("96 is good length.");

		// Split output into keys
		let mut root_chain_key = Self::RootChainKey::from([0; 32]);
		root_chain_key.copy_from_slice(&hkdf_out[..32]);
		let mut msg_chain_key = Self::MsgChainKey::from([0; 32]);
		msg_chain_key.copy_from_slice(&hkdf_out[32..64]);
		let mut header_key = Self::HeaderKey::from([0; 32]);
		header_key.copy_from_slice(&hkdf_out[64..]);

		(root_chain_key, msg_chain_key, header_key)
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
