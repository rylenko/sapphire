/// Header of new message.
///
/// Contains the public key of sender, message number and count of previous
/// sending chain messages.
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
pub(super) struct Hdr {
	public_key: super::key::Public,
	msg_num: u32,
	prev_send_msgs_cnt: u32,
}

impl Hdr {
	#[inline]
	#[must_use]
	pub(super) const fn new(
		public_key: super::key::Public,
		msg_num: u32,
		prev_send_msgs_cnt: u32,
	) -> Self {
		Self { public_key, msg_num, prev_send_msgs_cnt }
	}

	#[inline]
	#[must_use]
	pub(super) const fn msg_num(self) -> u32 {
		self.msg_num
	}

	#[inline]
	#[must_use]
	pub(super) const fn prev_send_msgs_cnt(self) -> u32 {
		self.prev_send_msgs_cnt
	}

	#[inline]
	#[must_use]
	pub(super) const fn public_key(self) -> super::key::Public {
		self.public_key
	}
}
