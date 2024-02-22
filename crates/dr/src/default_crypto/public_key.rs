/// Wrapper for [`PublicKey`] with bincode traits implementation.
///
/// [`PublicKey`]: x25519_dalek::PublicKey
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct PublicKey(x25519_dalek::PublicKey);

impl From<x25519_dalek::PublicKey> for PublicKey {
	#[inline]
	#[must_use]
	fn from(inner: x25519_dalek::PublicKey) -> Self {
		Self(inner)
	}
}

impl From<[u8; 32]> for PublicKey {
	#[inline]
	#[must_use]
	fn from(bytes: [u8; 32]) -> Self {
		Self::from(x25519_dalek::PublicKey::from(bytes))
	}
}

impl core::ops::Deref for PublicKey {
	type Target = x25519_dalek::PublicKey;

	#[inline]
	#[must_use]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl bincode::Encode for PublicKey {
	#[inline]
	fn encode<E>(&self, e: &mut E) -> Result<(), bincode::error::EncodeError>
	where
		E: bincode::enc::Encoder,
	{
		bincode::Encode::encode(self.0.as_bytes(), e)
	}
}

impl bincode::Decode for PublicKey {
	#[inline]
	fn decode<D>(d: &mut D) -> Result<Self, bincode::error::DecodeError>
	where
		D: bincode::de::Decoder,
	{
		Ok(From::<[u8; 32]>::from(bincode::Decode::decode(d)?))
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_encode_and_decode() {
		let key = super::PublicKey::from([1; 32]);

		// Encode
		let bytes =
			bincode::encode_to_vec(key, bincode::config::standard()).unwrap();
		assert_eq!(bytes, [1; 32]);

		// Decode
		let key_copy =
			bincode::decode_from_slice(&bytes, bincode::config::standard())
				.unwrap()
				.0;
		assert_eq!(key, key_copy);
	}
}
