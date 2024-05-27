/*!
Decryption function and error type.
*/

/// Decryption error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Error {
	/// Accepted authentication code is not equal to real one.
	Auth,
}

impl core::error::Error for Error {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Auth => None,
		}
	}
}

impl core::fmt::Display for Error {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Auth => write!(f, "Authentication codes are not equal."),
		}
	}
}

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
) -> Result<(), Error> {
	// Derive new encryption key, authentication key and nonce.
	let mut deriver = super::key::Deriver::new();
	deriver.derive(key);

	// Compare given and expected tags
	let expected_mac = super::auth::mac(deriver.auth_key(), buf, assoc);
	if tag != super::auth::Tag::from(expected_mac) {
		return Err(Error::Auth);
	}

	// Decrypt buffer using derived encryption key and nonce.
	let mut cipher: chacha20::XChaCha20 = chacha20::cipher::KeyIvInit::new(
		Into::into(deriver.encrypt_key()),
		Into::into(deriver.nonce()),
	);
	chacha20::cipher::StreamCipher::apply_keystream(&mut cipher, buf);
	Ok(())
}
