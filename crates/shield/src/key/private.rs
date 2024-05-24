/// User's private key.
#[derive(Clone, zeroize::ZeroizeOnDrop, zerocopy::AsBytes)]
#[repr(transparent)]
pub struct Private([u8; 32]);

impl Private {
	/// Builds new private key using passed `bytes`.
	#[inline]
	#[must_use]
	pub const fn new(bytes: [u8; 32]) -> Self {
		Self(bytes)
	}

	/// Generates random private key.
	#[must_use]
	pub fn random() -> Self {
		/// Randomize third-party private key and move its bytes to this
		/// struct.
		Self::new(x25519_dalek::StaticSecret::random().to_bytes())
	}

	/// Gets [public key] using current private key.
	///
	/// [public key]: super::public::Public
	#[must_use]
	pub fn get_public(&self) -> super::public::Public {
		// Get third-party public key from private key bytes.
		let x25519_private = x25519_dalek::StaticSecret::from(self.0);
		let x25519_public = x25519_dalek::PublicKey::from(&x25519_private);

		// Move third-party public key bytes to local struct.
		super::public::Public::new(x25519_public.to_bytes())
	}
}

impl From<[u8; 32]> for Private {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::new(bytes)
	}
}

// Prohibit implementations that can easily reveal private details.
impl !core::fmt::Debug for Private {}
impl !core::fmt::Display for Private {}

#[cfg(test)]
mod tests {
	#[test]
	fn test_get_public() {
		let private = super::Private::new([1; 32]);
		let public = private.get_public();
		assert_eq!(zerocopy::AsBytes::as_bytes(&public), [
			164, 224, 146, 146, 182, 81, 194, 120, 185, 119, 44, 86, 159, 95,
			169, 187, 19, 217, 6, 180, 106, 182, 140, 157, 249, 220, 43, 68,
			9, 248, 162, 9
		]);
	}
}
