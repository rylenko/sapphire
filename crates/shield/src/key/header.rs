/// Header key to encrypt or decrypt headers.
#[derive(Clone, Debug, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub struct Header([u8; 32]);

impl Header {
	/// Builds the new header key using passed `bytes`.
	#[inline]
	#[must_use]
	pub const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}

	/// Returns key bytes.
	#[inline]
	#[must_use]
	pub const fn as_bytes(&self) -> &[u8; 32] {
		&self.0
	}
}

impl AsRef<[u8]> for Header {
	#[inline]
	#[must_use]
	fn as_ref(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl From<[u8; 32]> for Header {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::new(bytes)
	}
}
