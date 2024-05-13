type KdfInner = hkdf::Hkdf<sha2::Sha256>;

/// Derives  passed keys into authentication keys, encryption keys and nonces.
#[derive(Clone, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub struct Deriver([u8; 88]);

impl Deriver {
	const INFO: &'static [u8] = b"cipher-kdf-info";
	const SALT: &'static [u8] = &[0; core::mem::size_of::<Self>()];

	/// Creates new KDF with zeroed derived data.
	#[inline]
	#[must_use]
	pub fn new() -> Self {
		Self([0; 88])
	}

	/// Gets derived authentication key.
	#[inline]
	#[must_use]
	pub fn auth_key(&self) -> &[u8] {
		&self.0[32..64]
	}

	/// Derives authencation key, encryption key and nonce using passed `key`.
	///
	/// Use accessors to get derived data.
	pub fn derive(&mut self, key: &[u8]) {
		KdfInner::new(Some(Self::SALT), key)
			.expand(Self::INFO, &mut self.0)
			.expect("Output must have a good length.");
	}

	/// Gets derived encryption key.
	#[inline]
	#[must_use]
	pub fn encrypt_key(&self) -> &[u8] {
		&self.0[..32]
	}

	/// Gets derived nonce.
	#[inline]
	#[must_use]
	pub fn nonce(&self) -> &[u8] {
		&self.0[64..]
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_kdf() {
		let mut deriver = super::Deriver::new();
		deriver.derive(b"kdf-key");
		assert_eq!(deriver.auth_key(), [
			233, 142, 52, 60, 203, 98, 155, 249, 13, 126, 113, 132, 40, 177,
			56, 46, 20, 155, 118, 238, 200, 191, 132, 108, 228, 169, 95, 151,
			237, 202, 95, 227
		]);
		assert_eq!(deriver.encrypt_key(), [
			239, 40, 18, 58, 53, 126, 141, 77, 189, 2, 23, 140, 130, 159, 43,
			248, 66, 188, 84, 38, 3, 55, 17, 19, 45, 34, 136, 93, 135, 51, 56,
			22
		]);
		assert_eq!(deriver.nonce(), [
			54, 8, 214, 29, 61, 95, 130, 142, 191, 67, 199, 128, 73, 216, 154,
			165, 127, 124, 231, 6, 142, 196, 131, 23
		]);
	}
}
