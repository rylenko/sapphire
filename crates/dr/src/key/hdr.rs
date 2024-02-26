/// [`Hash`]able wrapper for [`zeroize`] header key.
///
/// [`Hash`]: core::hash::Hash
#[derive(Clone, Debug, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub struct Hdr([u8; 32]);

impl Hdr {
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

impl From<[u8; 32]> for Hdr {
	#[inline]
	#[must_use]
	fn from(a: [u8; 32]) -> Self {
		Self(a)
	}
}
