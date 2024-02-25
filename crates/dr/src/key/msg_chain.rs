#[derive(Clone, Debug, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub(crate) struct MsgChain([u8; 32]);

impl MsgChain {
	#[inline]
	#[must_use]
	pub(crate) const fn as_bytes(&self) -> &[u8; 32] {
		&self.0
	}

	#[inline]
	pub(crate) fn copy_from_slice(&mut self, slice: &[u8]) {
		self.0.copy_from_slice(slice);
	}
}

impl From<[u8; 32]> for MsgChain {
	#[inline]
	#[must_use]
	fn from(raw: [u8; 32]) -> Self {
		Self(raw)
	}
}
