/// Message key to encrypt or decrypt messages.
#[derive(
	Clone,
	Debug,
	Eq,
	Hash,
	PartialEq,
	zerocopy::AsBytes,
	zeroize::ZeroizeOnDrop,
)]
#[repr(transparent)]
pub(crate) struct Message([u8; 32]);

impl Message {
	/// Builds the new message key using passed `bytes`.
	#[inline]
	#[must_use]
	pub(super) const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}
}

impl From<[u8; 32]> for Message {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::new(bytes)
	}
}
