/// Key pair based on [`x25519_dalek`].
#[derive(Clone)]
pub struct KeyPair(x25519_dalek::StaticSecret, super::public_key::PublicKey);

impl Eq for KeyPair {}

impl PartialEq for KeyPair {
	#[inline]
	#[must_use]
	fn eq(&self, other: &Self) -> bool {
		self.1 == other.1
	}
}

impl core::fmt::Debug for KeyPair {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_tuple("KeyPair").field(&"<secret>").field(&self.1).finish()
	}
}

impl crate::crypto::KeyPair for KeyPair {
	type Private = x25519_dalek::StaticSecret;
	type Public = super::public_key::PublicKey;

	fn rand() -> Self {
		let private = x25519_dalek::StaticSecret::random();
		let public = x25519_dalek::PublicKey::from(&private);
		Self(private, Self::Public::from(public))
	}

	#[inline]
	fn private(&self) -> &Self::Private {
		&self.0
	}

	#[inline]
	fn public(&self) -> &Self::Public {
		&self.1
	}
}
