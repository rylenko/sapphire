/// Message key to encrypt or decrypt messages.
#[derive(Clone, Debug, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub(crate) struct Message([u8; 32]);

impl Message {
	/// Builds the new message key using passed `bytes`.
	#[inline]
	#[must_use]
	pub(crate) const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}

	/// Returns key bytes.
	#[inline]
	#[must_use]
	pub(crate) const fn as_bytes(&self) -> &[u8; 32] {
		&self.0
	}
}

impl AsRef<[u8]> for Message {
	#[inline]
	#[must_use]
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl From<[u8; 32]> for Message {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::new(bytes)
	}
}
