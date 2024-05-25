/// Sending chain of Double Ratchet algorithm.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Send {
	key: Option<crate::key::Master>,
	header_key: Option<crate::key::Header>,
	next_header_key: crate::key::Header,
	next_msg_num: u32,
	prev_msgs_cnt: u32,
}

impl Send {
	/// Creates new sending chain using passed keys.
	#[inline]
	#[must_use]
	pub(crate) const fn new(
		key: Option<crate::key::Master>,
		header_key: Option<crate::key::Header>,
		next_header_key: crate::key::Header,
	) -> Self {
		Self {
			key,
			header_key,
			next_header_key,
			next_msg_num: 0,
			prev_msgs_cnt: 0,
		}
	}
}
