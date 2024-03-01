/// Encrypts `buf` using `key` and authenticates it with `auth`.
#[must_use]
pub(crate) fn encrypt(
	key: &[u8],
	buf: &mut [u8],
	assoc_data: &[&[u8]],
) -> super::tag::Tag {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Get HKDF output for encryption and auth keys and nonce
	let kdf_out = super::kdf_out::KdfOut::from(key);
	// Encrypt using encryption key and nonce
	chacha20::XChaCha20::new(
		kdf_out.encryption_key().into(),
		kdf_out.nonce().into(),
	)
	.apply_keystream(buf);
	// Authenticate and build tag from MAC
	super::tag::Tag::from(super::mac::auth(
		kdf_out.auth_key(),
		buf,
		assoc_data,
	))
}
