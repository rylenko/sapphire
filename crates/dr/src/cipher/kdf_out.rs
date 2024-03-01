#[derive(Clone, Eq, Hash, PartialEq, zeroize::ZeroizeOnDrop)]
#[repr(transparent)]
pub(super) struct KdfOut([u8; Self::SIZE]);

impl KdfOut {
	const AUTH_KEY_RANGE: core::ops::Range<usize> = 32..64;
	const ENCRYPTION_KEY_RANGE: core::ops::Range<usize> = 0..32;
	const INFO: &'static [u8; 20] = b"cipher-hkdf-out-info";
	const NONCE_RANGE: core::ops::Range<usize> = 64..88;
	const SALT: &'static [u8; Self::SIZE] = &[0; Self::SIZE];
	const SIZE: usize = 88;

	#[inline]
	#[must_use]
	pub(super) fn auth_key(&self) -> &[u8] {
		&self.0[Self::AUTH_KEY_RANGE]
	}

	#[inline]
	#[must_use]
	pub(super) fn encryption_key(&self) -> &[u8] {
		&self.0[Self::ENCRYPTION_KEY_RANGE]
	}

	#[inline]
	#[must_use]
	pub(super) fn nonce(&self) -> &[u8] {
		&self.0[Self::NONCE_RANGE]
	}
}

impl From<&[u8]> for KdfOut {
	#[must_use]
	fn from(key: &[u8]) -> Self {
		let mut inner = [0; Self::SIZE];
		hkdf::Hkdf::<sha2::Sha256>::new(Some(Self::SALT), key)
			.expand(Self::INFO, &mut inner)
			.expect("HKDF output must have a good length.");
		Self(inner)
	}
}
