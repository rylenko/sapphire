/// Implementation of [`Mac`] used for authentication.
///
/// [`Mac`]: hmac::Mac
type MacImpl = hmac::Hmac<sha2::Sha256>;

/// Authentication tag, which is equal to the cutted message authentication
/// code.
#[derive(
	Clone,
	Copy,
	Eq,
	Debug,
	Hash,
	PartialEq,
	zerocopy::AsBytes,
	zerocopy::FromBytes,
	zerocopy::FromZeroes,
)]
#[repr(transparent)]
pub struct Tag([u8; Self::LEN]);

impl Tag {
	/// Length of tag bytes.
	const LEN: usize = 12;

	// Required by the Double Ratchet specification.
	utils::const_assert!(_LEN_ASSERT, 8 <= Self::LEN && Self::LEN <= 32);
}

impl From<[u8; 32]> for Tag {
	/// Cuts the tag from message authentication code.
	#[must_use]
	fn from(mac: [u8; 32]) -> Self {
		let mut bytes = [0; Self::LEN];
		bytes.copy_from_slice(&mac[..Self::LEN]);
		Self(bytes)
	}
}

/// Message authenticator to authenticate buffer and associated data using
/// authentication key.
#[derive(Clone, Debug)]
#[repr(transparent)]
pub(crate) struct Mac {
	inner: MacImpl,
}

impl Mac {
	/// Creates new message authenticator based on passed `key`.
	#[must_use]
	pub(crate) fn new(key: &[u8]) -> Self {
		Self {
			inner: hmac::Mac::new_from_slice(key).expect("Any size is good."),
		}
	}

	/// Authenticates `buf`fer and `assoc`iated data using authentication key.
	///
	/// After using this method, the authenticator returns to its initial
	/// state.
	#[must_use]
	pub(crate) fn auth(&mut self, buf: &[u8], assoc: &[&[u8]]) -> [u8; 32] {
		// Update with accepted buffer.
		hmac::Mac::update(&mut self.inner, buf);
		// Update with accepted associated data.
		for assoc_part in assoc {
			hmac::Mac::update(&mut self.inner, assoc_part);
		}

		// Drain message authentication code into return bytes.
		Into::into(hmac::Mac::finalize_reset(&mut self.inner).into_bytes())
	}
}

#[cfg(test)]
mod tests {
	const ASSOC: &[&[u8]] = &[b"assoc1", b"assoc2"];
	const BUF: &[u8] = b"buf";
	const KEY: &[u8] = b"key";
	const MAC: [u8; 32] = [
		190, 67, 118, 3, 32, 204, 105, 154, 67, 54, 231, 226, 3, 245, 208, 32,
		62, 15, 71, 76, 142, 242, 203, 183, 115, 100, 178, 229, 224, 119, 252,
		107,
	];

	#[test]
	fn test_mac() {
		let mut mac = super::Mac::new(KEY);
		// Test authenticator output and test reset after each authentication.
		assert_eq!(mac.auth(BUF, ASSOC), MAC);
		assert_eq!(mac.auth(BUF, ASSOC), MAC);
	}

	#[test]
	fn test_tag() {
		assert_eq!(
			super::Tag::from(MAC).0,
			MAC[..core::mem::size_of::<super::Tag>()]
		);
	}
}
