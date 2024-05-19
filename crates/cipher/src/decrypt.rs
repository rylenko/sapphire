/// `auth`enticates encrypted `buf`fer and `assoc`iated data and decrypts
/// encrypted `buf`fer using `key`.
///
/// In more detail, it derives three values using the `key`: an encryption key,
/// an authentication key and a nonce. Authentication occurs using an
/// authentication key, an encrypted `buf`fer and `assoc`iated data. Then
/// `buf`fer is decrypted using encryption key and a nonce.
///
/// # Errors
///
/// [`Auth`] when passed authentication `tag` is not equal to real one.
pub fn decrypt(
	key: &[u8],
	buf: &mut [u8],
	assoc: &[&[u8]],
	tag: super::auth::Tag,
) -> Result<(), super::error::Decrypt> {
	use chacha20::cipher::{KeyIvInit as _, StreamCipher as _};

	// Derive new encryption key, authentication key and nonce.
	let mut deriver = super::key::Deriver::new();
	deriver.derive(key);

	// Compare given and expected tags
	let expected_mac = super::auth::mac(deriver.auth_key(), buf, assoc);
	if tag != super::auth::Tag::from(expected_mac) {
		return Err(super::error::Decrypt::Auth);
	}

	// Decrypt buffer using derived encryption key and nonce.
	chacha20::XChaCha20::new(
		deriver.encrypt_key().into(),
		deriver.nonce().into(),
	)
	.apply_keystream(buf);
	Ok(())
}
