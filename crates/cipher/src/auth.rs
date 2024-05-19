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
pub struct Tag([u8; Self::SIZE]);

impl Tag {
	/// Size of tag cutted from message authentication code. Double Ratchet
	/// specification requires it to be in [8; 32]. So there must be an
	/// assertion.
	const SIZE: usize = 12;

	// Required by the Double Ratchet specification.
	utils::const_assert!(_SIZE_ASSERT, 8 <= Self::SIZE && Self::SIZE <= 32);

	/// Builds new tag using passed `key`, `buf`fer and `assoc`iated data.
	#[must_use]
	pub(crate) fn new(key: &[u8], buf: &[u8], assoc: &[&[u8]]) -> Self {
		// Authenticate the buffer and associated data using accepted key.
		let mac = auth(key, buf, assoc);

		// Cut the tag from message authentication code.
		let mut tag_bytes = [0; Self::SIZE];
		tag_bytes.copy_from_slice(&mac[..Self::SIZE]);
		Self(tag_bytes)
	}
}

/// Authenticates `buf`fer and `assoc`iated data using `key`.
fn auth(key: &[u8], buf: &[u8], assoc: &[&[u8]]) -> [u8; 32] {
	use hmac::Mac as _;

	// Create message authentication code builder using accepted key.
	let mut mac = MacImpl::new_from_slice(key).expect("Any size is good.");

	// Update with accepted buffer.
	mac.update(buf);
	// Update with accepted associated data.
	for data in assoc {
		mac.update(data);
	}

	// Finalize message authentication code into return array.
	mac.finalize().into_bytes().into()
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_auth() {
		const MAC: [u8; 32] = [
			190, 67, 118, 3, 32, 204, 105, 154, 67, 54, 231, 226, 3, 245, 208,
			32, 62, 15, 71, 76, 142, 242, 203, 183, 115, 100, 178, 229, 224,
			119, 252, 107,
		];

		let tag = super::Tag::new(b"key", b"buf", &[b"assoc1", b"assoc2"]);
		assert_eq!(tag.0, MAC[..super::Tag::SIZE]);
	}
}
