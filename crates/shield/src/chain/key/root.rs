/// Root key on which the root chain of Double Ratchet based.
///
/// Is initially an agreed shared key.
#[derive(Clone, Debug, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub struct Root([u8; 32]);

impl Root {
	#[inline]
	#[must_use]
	pub const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}

	// pub(in crate::chain) fn evolve(
	// &mut self,
	// input: crate::key::SharedSecret,
	// ) -> (super::master::Master, super::header::Header) {}
}

impl From<[u8; 32]> for Root {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::new(bytes)
	}
}
