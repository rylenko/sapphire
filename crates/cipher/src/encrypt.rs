/// Encrypts passed `buf`fer using passed `key` and authenticates encrypted
/// `buf`fer with `assoc`iated data.
///
/// In more detail, it derives three values ​​using the `key`: an
/// encryption key, an authentication key and a nonce. The `buf`fer is
/// encrypted using an encryption key and a nonce. Authentication occurs using
/// an authentication key, an encrypted `buf`fer and `assoc`iated data.
///
/// # Return
///
/// Authentication tag of encrypted `buf`fer and `assoc`iated data, which was
/// obtained using derived authentication key.
#[must_use]
pub fn encrypt(
	key: &[u8],
	buf: &mut [u8],
	assoc: &[&[u8]],
) -> super::auth::Tag {
	// Derive new encryption key, authentication key and nonce.
	let mut deriver = super::key::Deriver::new();
	deriver.derive(key);

	// Encrypt buffer using derived encryption key and nonce.
	let mut cipher: chacha20::XChaCha20 = chacha20::cipher::KeyIvInit::new(
		Into::into(deriver.encrypt_key()),
		Into::into(deriver.nonce()),
	);
	chacha20::cipher::StreamCipher::apply_keystream(&mut cipher, buf);

	// Create authentication tag of encrypted buffer and associated data using
	// derived authentication key.
	super::auth::Tag::from(super::auth::mac(deriver.auth_key(), buf, assoc))
}
