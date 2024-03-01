/// Decrypts `buf` using `key` and authenticates it using `auth`.
///
/// Decrypts everything except the last 32 bytes. The last 32 bytes are
/// occupied by MAC. Also the buffer will not be corrupted in case of errors.
pub(crate) fn decrypt(
	key: &[u8],
	buf: &mut [u8],
	assoc_data: &[&[u8]],
	tag: super::tag::Tag,
) -> Result<(), super::error::Decrypt> {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// KDF with common key
	let kdf_out = super::kdf_out::KdfOut::from(key);
	// Authenticate, build tag from MAC and compare tags
	let got_mac = super::mac::auth(kdf_out.auth_key(), buf, assoc_data);
	if super::tag::Tag::from(got_mac) != tag {
		return Err(super::error::Decrypt::Auth);
	}
	// Decrypt plain text using cipher key and nonce
	chacha20::XChaCha20::new(
		kdf_out.encryption_key().into(),
		kdf_out.nonce().into(),
	)
	.apply_keystream(buf);
	Ok(())
}
