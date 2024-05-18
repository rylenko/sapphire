/*!
The header of a new messages and its encrypted version.
*/

/// Header of the new message.
///
/// Contains the sender's public key bytes, message number and previous sending
/// chain messages count.
#[derive(
	Clone,
	Copy,
	Debug,
	Eq,
	Hash,
	PartialEq,
	zerocopy::AsBytes,
	zerocopy::FromBytes,
	zerocopy::FromZeroes,
)]
#[repr(packed)]
pub(crate) struct Header {
	public_key_bytes: [u8; 32],
	msg_num: u32,
	prev_chain_msgs_cnt: u32,
}

impl Header {
	#[inline]
	#[must_use]
	pub(crate) const fn new(
		public_key_bytes: [u8; 32],
		msg_num: u32,
		prev_chain_msgs_cnt: u32,
	) -> Self {
		Self { public_key_bytes, msg_num, prev_chain_msgs_cnt }
	}

	#[inline]
	#[must_use]
	pub(crate) const fn msg_num(&self) -> u32 {
		self.msg_num
	}

	#[inline]
	#[must_use]
	pub(crate) const fn prev_chain_msgs_cnt(&self) -> u32 {
		self.prev_chain_msgs_cnt
	}

	#[inline]
	#[must_use]
	pub(crate) const fn public_key_bytes(&self) -> [u8; 32] {
		self.public_key_bytes
	}
}
