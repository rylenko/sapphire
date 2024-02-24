/// Wrapper for [`PrivateKey`].
///
/// [`PrivateKey`]: x25519_dalek::PrivateKey
#[derive(Clone)]
pub struct Private(x25519_dalek::StaticSecret);

impl Private {
	#[inline]
	#[must_use]
	pub(crate) fn random() -> Self {
		Self(x25519_dalek::StaticSecret::random())
	}

	#[inline]
	#[must_use]
	pub(crate) fn dh(
		&self,
		public: super::public::Public,
	) -> x25519_dalek::SharedSecret {
		self.0.diffie_hellman(&public.to_dalek())
	}

	#[inline]
	#[must_use]
	pub(super) fn as_dalek(&self) -> &x25519_dalek::StaticSecret {
		&self.0
	}
}
