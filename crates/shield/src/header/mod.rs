/*!
The header of a new messages and its encrypted version.
*/

mod encrypted;

pub(crate) use encrypted::Encrypted;

/// Header of the new message.
///
/// Contains the sender's public key bytes, message number and previous sending
/// chain messages count.
#[derive(
	Clone,
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
	message_num: u32,
	prev_send_chain_messages_count: u32,
}

impl Header {
	/// Creates new header, which contains public key bytes, message number and
	/// previous sending chain message count.
	#[inline]
	#[must_use]
	pub(crate) const fn new(
		public_key_bytes: [u8; 32],
		message_num: u32,
		prev_send_chain_messages_count: u32,
	) -> Self {
		Self { public_key_bytes, message_num, prev_send_chain_messages_count }
	}

	/// Encrypts current header using passed `key`.
	#[inline]
	#[must_use]
	pub(crate) fn encrypt(
		&self,
		key: &crate::key::Header,
	) -> encrypted::Encrypted {
		Encrypted::encrypt(key, self)
	}

	#[inline]
	#[must_use]
	pub(crate) const fn message_num(&self) -> u32 {
		self.message_num
	}

	#[inline]
	#[must_use]
	pub(crate) const fn prev_send_chain_messages_count(&self) -> u32 {
		self.prev_send_chain_messages_count
	}

	#[inline]
	#[must_use]
	pub(crate) const fn public_key_bytes(&self) -> &[u8; 32] {
		&self.public_key_bytes
	}
}
