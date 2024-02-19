/// Storage for skipped message keys.
#[repr(transparent)]
pub(super) struct SkippedMsgKeys<P: crate::crypto::Provider>(
	// Keys are not pair of header key and message number because of reference
	// to header key in getting function
	hashbrown::HashMap<
		P::HeaderKey,
		hashbrown::HashMap<super::num::Num, P::MsgKey>,
	>,
);

impl<P> SkippedMsgKeys<P>
where
	P: crate::crypto::Provider,
{
	/// Creates empty storage.
	#[inline]
	#[must_use]
	pub(super) fn new() -> Self {
		Self(hashbrown::HashMap::new())
	}
}

impl<P> SkippedMsgKeys<P>
where
	P: crate::crypto::Provider,
{
	/// Gets message key by header key and message number.
	#[must_use]
	pub(super) fn get(
		&self,
		header_key: &P::HeaderKey,
		msg_num: super::num::Num,
	) -> Option<&P::MsgKey> {
		self.0.get(header_key)?.get(&msg_num)
	}
}

// #[cfg(test)]
// mod tests {
// #[test]
// fn test_get() {
// let storage = super::SkippedMsgKeys::new();

// }
// }
