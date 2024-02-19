/// [`Hash`]able wrapper for [`Zeroizing`] header key.
///
/// [`Hash`]: core::hash::Hash.
/// [`Zeroizing`]: zeroize::Zeroizing
#[derive(Clone, Debug, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
pub struct HeaderKey([u8; 32]);

impl From<[u8; 32]> for HeaderKey {
	#[inline]
	#[must_use]
	fn from(a: [u8; 32]) -> Self {
		Self(a)
	}
}

impl core::ops::Deref for HeaderKey {
	type Target = [u8; 32];

	#[inline]
	#[must_use]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl core::ops::DerefMut for HeaderKey {
	#[inline]
	#[must_use]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}
