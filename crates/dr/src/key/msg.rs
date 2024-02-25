#[derive(Clone, Debug, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub struct Msg([u8; 32]);

impl Msg {
	#[inline]
	#[must_use]
	pub(crate) const fn as_bytes(&self) -> &[u8; 32] {
		&self.0
	}
}

impl From<[u8; 32]> for Msg {
	#[inline]
	#[must_use]
	fn from(raw: [u8; 32]) -> Self {
		Self(raw)
	}
}
