/// Receiving chain of Double Ratchet algorithm.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Receiving {
	master_key: Option<crate::key::Master>,
	header_key: Option<crate::key::Header>,
	next_header_key: crate::key::Header,
	next_message_num: u32,
	skipped_message_keys: super::skipped_message_keys::SkippedMessageKeys,
}

impl Receiving {
	/// Creates a new receiving chain using passed keys.
	#[inline]
	#[must_use]
	pub(crate) fn new(
		next_header_key: crate::key::Header,
		skipped_message_keys_max_len: usize,
	) -> Self {
		Self {
			master_key: None,
			header_key: None,
			next_header_key,
			next_message_num: 0,
			skipped_message_keys:
				super::skipped_message_keys::SkippedMessageKeys::new(
					skipped_message_keys_max_len,
				),
		}
	}
}
