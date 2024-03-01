const_assert!(8 <= Tag::SIZE && Tag::SIZE <= 32);

#[derive(
	Copy,
	Clone,
	Eq,
	Debug,
	Hash,
	PartialEq,
	zerocopy::AsBytes,
	zerocopy::FromBytes,
	zerocopy::FromZeroes,
)]
#[repr(transparent)]
pub(crate) struct Tag([u8; Self::SIZE]);

impl Tag {
	const SIZE: usize = 12;
}

impl From<[u8; 32]> for Tag {
	/// Build from MAC.
	#[must_use]
	fn from(mac: [u8; 32]) -> Self {
		let mut inner = [0; Self::SIZE];
		inner.copy_from_slice(&mac[..Self::SIZE]);
		Self(inner)
	}
}
