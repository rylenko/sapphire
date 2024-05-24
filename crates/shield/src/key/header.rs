/// Header key to encrypt or decrypt headers.
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	PartialEq,
	zeroize::ZeroizeOnDrop,
	zerocopy::AsBytes,
)]
#[repr(transparent)]
pub struct Header([u8; 32]);

impl Header {
	/// Builds the new header key using passed `bytes`.
	#[inline]
	#[must_use]
	pub const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}
}

impl From<[u8; 32]> for Header {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::new(bytes)
	}
}
