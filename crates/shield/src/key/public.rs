/// User's public key.
#[derive(
	Clone,
	Copy,
	Debug,
	Eq,
	Hash,
	PartialEq,
	zerocopy::AsBytes,
	zerocopy::FromBytes,
	zerocopy::FromZeroes,
)]
#[repr(transparent)]
pub struct Public([u8; 32]);

impl Public {
	/// Builds the new public key using passed `bytes`.
	///
	/// TODO: Remove this method and build public key only from private key.
	#[inline]
	#[must_use]
	pub(crate) const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}
}

impl From<[u8; 32]> for Public {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::new(bytes)
	}
}
