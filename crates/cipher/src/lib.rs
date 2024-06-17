/*!
Auxiliary things for encrypting and decrypting data.
*/

#![feature(error_in_core)]
#![no_std]

mod auth;

pub use auth::Tag;

/// Key derivation function used in cipher struct to derive keys.
type KdfImpl = hkdf::Hkdf<sha2::Sha256>;

/// Cipher type to [encrypt] and [decrypt] the data.
///
/// [encrypt]: Self::encrypt
/// [decrypt]: Self::decrypt
pub struct Cipher {
	inner: chacha20::XChaCha20,
	mac: auth::Mac,
}

impl Cipher {
	const KDF_INFO: &'static [u8] = b"cipher-kdf-info";
	const KDF_SALT: &'static [u8] = &[0; 88];

	/// Creates new cipher.
	///
	/// In more detail, it derives three values using the `key`: an encryption
	/// key, an authentication key and a nonce to use in decryption.
	///
	/// # Panics
	///
	/// If the size of the KDF output array has the wrong size. This is in no
	/// way user dependent and is unlikely to ever happen.
	#[must_use]
	pub fn new(key: &[u8]) -> Self {
		// Derive new encryption key, authentication key and nonce.
		let mut keys = zeroize::Zeroizing::new([0; 88]);
		KdfImpl::new(Some(Self::KDF_SALT), key)
			.expand(Self::KDF_INFO, keys.as_mut())
			.expect("Output must have a good length.");

		// Create inner cipher based on derived encryption key and nonce.
		let inner = chacha20::cipher::KeyIvInit::new(
			Into::into(&keys[..32]),
			Into::into(&keys[64..88]),
		);
		// Create message authenticator based on the derived authentication
		// key.
		let mac = auth::Mac::new(&keys[32..64]);
		Self { inner, mac }
	}

	/// `auth`enticates encrypted `buf`fer and `assoc`iated data using derived
	/// authentication key and decrypts encrypted `buf`fer using derived
	/// decryption key and nonce.
	///
	/// # Errors
	///
	/// [`Auth`] when passed authentication `tag` is not equal to real one.
	pub fn decrypt(
		&mut self,
		buf: &mut [u8],
		assoc: &[&[u8]],
		tag: auth::Tag,
	) -> Result<(), DecryptError> {
		// Compare given and expected tags.
		if tag != Tag::from(self.mac.auth(buf, assoc)) {
			return Err(DecryptError::Auth);
		}

		// Decrypt accepted buffer using inner cipher.
		chacha20::cipher::StreamCipher::apply_keystream(&mut self.inner, buf);
		Ok(())
	}

	/// Encrypts passed `buf`fer using derived encryption key and authenticates
	/// encrypted `buf`fer with `assoc`iated data using derived authentication
	/// key.
	#[must_use]
	pub fn encrypt(&mut self, buf: &mut [u8], assoc: &[&[u8]]) -> auth::Tag {
		// Encrypt accepted buffer using inner cipher.
		chacha20::cipher::StreamCipher::apply_keystream(&mut self.inner, buf);

		// Create authentication tag of encrypted buffer and associated data
		// using derived authentication key.
		Tag::from(self.mac.auth(buf, assoc))
	}

	/// Seeks to the specified position of the cipher.
	///
	/// Necessary if data is encrypted and decrypted in the same cipher
	/// instance.
	#[inline]
	pub fn seek(&mut self, pos: usize) {
		chacha20::cipher::StreamCipherSeek::seek(&mut self.inner, pos);
	}
}

/// Decryption error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DecryptError {
	/// Accepted authentication code is not equal to real one.
	Auth,
}

impl core::error::Error for DecryptError {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Auth => None,
		}
	}
}

impl core::fmt::Display for DecryptError {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Auth => write!(f, "Authentication codes are not equal."),
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_encrypt_decrypt() {
		// Create the cipher.
		let mut cipher = super::Cipher::new(b"secret-key");

		// Test encryption.
		let mut buf = [111; 111];
		let tag = cipher.encrypt(&mut buf, &[b"a1", b"a2"]);
		assert_ne!(buf, [111; 111]);

		// To use cipher from zero.
		cipher.seek(0);

		// Test decryption with an invalid associated data.
		assert_eq!(
			cipher.decrypt(&mut buf, &[b"a1"], tag),
			Err(super::DecryptError::Auth)
		);

		// Test decryption with an invalid authentication tag.
		let mut invalid_tag = tag;
		zerocopy::AsBytes::as_bytes_mut(&mut invalid_tag)[0] ^= 1;
		assert_eq!(
			cipher.decrypt(&mut buf, &[b"a1", b"a2"], invalid_tag),
			Err(super::DecryptError::Auth)
		);

		// Test decryption with an invalid key.
		let mut invalid_cipher = super::Cipher::new(b"invalid-key");
		assert_eq!(
			invalid_cipher.decrypt(&mut buf, &[b"a1", b"a2"], tag),
			Err(super::DecryptError::Auth)
		);

		// Test decryption with the valid data.
		assert!(cipher.decrypt(&mut buf, &[b"a1", b"a2"], tag).is_ok());
		assert_eq!(buf, [111; 111]);
	}
}
