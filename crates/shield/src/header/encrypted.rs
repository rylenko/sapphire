/// Encrypted header decryption error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum DecryptError {
	Bytes(cipher::DecryptError),
}

impl From<cipher::DecryptError> for DecryptError {
	#[inline]
	#[must_use]
	fn from(e: cipher::DecryptError) -> Self {
		Self::Bytes(e)
	}
}

impl core::error::Error for DecryptError {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Bytes(ref e) => Some(e),
		}
	}
}

impl core::fmt::Display for DecryptError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Bytes(..) => write!(f, "Failed to decrypt header bytes."),
		}
	}
}

/// Encrypted version of the message [`Header`].
///
/// [`Header`]: super::Header
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	PartialEq,
	zerocopy::AsBytes,
	zerocopy::FromBytes,
	zerocopy::FromZeroes,
)]
#[repr(packed)]
pub(crate) struct Encrypted {
	bytes: [u8; core::mem::size_of::<super::Header>()],
	tag: cipher::Tag,
}

impl Encrypted {
	#[must_use]
	pub(super) fn encrypt(key: &[u8], header: &super::Header) -> Self {
		// Copy header's bytes to an array.
		let mut bytes = [0; core::mem::size_of::<super::Header>()];
		bytes.copy_from_slice(zerocopy::AsBytes::as_bytes(header));

		// Encrypt header bytes and get the authentication tag.
		let tag = cipher::Cipher::new(key).encrypt(&mut bytes, &[]);
		Self { bytes, tag }
	}

	pub(crate) fn decrypt(
		&self,
		key: &[u8],
	) -> Result<super::Header, DecryptError> {
		// Copy encrypted bytes to not modify the struct
		let mut bytes = self.bytes;
		// Decrypt encrypted header bytes.
		cipher::Cipher::new(key).decrypt(&mut bytes, &[], self.tag)?;

		// Deserialize decrypted bytes to the header struct.
		//
		// We can unwrap here because decrypted bytes count is equal to header
		// struct size.
		Ok(zerocopy::FromBytes::read_from(&bytes).unwrap())
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_encrypt_decrypt() -> Result<(), super::DecryptError> {
		// Test header encryption.
		let header = super::super::Header::new([5; 32], 123, 456);
		let encrypted = super::Encrypted::encrypt(b"header-key", &header);
		assert_ne!(encrypted.bytes, zerocopy::AsBytes::as_bytes(&header));

		// Test header decryption.
		let decrypted = encrypted.decrypt(b"header-key")?;
		assert_eq!(decrypted.public_key_bytes(), &[5; 32]);
		assert_eq!(decrypted.message_num(), 123);
		assert_eq!(decrypted.prev_send_chain_messages_count(), 456);
		Ok(())
	}
}
