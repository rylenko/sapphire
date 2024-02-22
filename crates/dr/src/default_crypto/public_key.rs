/// Wrapper for [`PublicKey`].
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

impl crate::code::Encode for PublicKey {
	#[inline]
	#[must_use]
	fn encode(&self) -> alloc::vec::Vec<u8> {
		self.0.as_ref().to_vec()
	}
}

impl crate::code::Decode for PublicKey {
	type Error = super::error::PublicKeyDecode;

	fn decode(key: &[u8]) -> Result<(Self, usize), Self::Error> {
		if key.len() < 32 {
			Err(Self::Error::InvalidLen)
		} else {
			let mut inner = [0; 32];
			inner.copy_from_slice(&key[..32]);
			Ok((Self::from(inner), 32))
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_encode_and_decode() {
		use crate::code::{Decode as _, Encode as _};

		let key = super::PublicKey::from([1; 32]);
		let bytes = key.encode();
		assert_eq!(bytes, [1; 32]);
		assert_eq!(super::PublicKey::decode(&bytes).unwrap(), (key, 32));
	}
}
