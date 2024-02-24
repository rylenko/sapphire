/// Wrapper for [`PublicKey`].
///
/// [`PublicKey`]: x25519_dalek::PublicKey
///
/// TODO: Zeroize? `ZeroizeOnDrop` does not fit because of [`Copy`].
#[derive(
	Copy,
	Clone,
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
	#[inline]
	#[must_use]
	pub(super) fn to_dalek(self) -> x25519_dalek::PublicKey {
		x25519_dalek::PublicKey::from(self.0)
	}
}

impl From<[u8; 32]> for Public {
	#[inline]
	#[must_use]
	fn from(a: [u8; 32]) -> Self {
		Self(a)
	}
}

impl From<&super::private::Private> for Public {
	#[inline]
	#[must_use]
	fn from(private: &super::private::Private) -> Self {
		let public_dalek = x25519_dalek::PublicKey::from(private.as_dalek());
		Self(public_dalek.to_bytes())
	}
}
