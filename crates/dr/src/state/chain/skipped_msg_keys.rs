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

	/// Inserts new entry.
	pub(super) fn insert(
		&mut self,
		header_key: P::HeaderKey,
		msg_num: super::num::Num,
		msg_key: P::MsgKey,
	) {
		let values = self.0.entry(header_key).or_default();
		values.insert(msg_num, msg_key);
	}

	/// Gets skipped message key using encrypted header.
	///
	/// It iterates through all the missing header keys, tries to decrypt the
	/// encrypted header, if it succeeds, then it gets the message key based on
	/// the message number.
	///
	/// # Errors
	///
	/// See [`PopSkippedMsgKey`].
	///
	/// [`PopSkippedMsgKey`]: super::error::PopSkippedMsgKey
	pub(super) fn pop(
		&mut self,
		encrypted_header: &[u8],
	) -> Result<Option<P::MsgKey>, super::error::PopSkippedMsgKey> {
		use alloc::borrow::ToOwned as _;

		let mut ret = None;
		let mut empty_header_key = None;

		// Iterate over elements
		for (header_key, values) in &mut self.0 {
			// Try to decrypt header with iterated header key
			let Ok(bytes) = P::decrypt_header(header_key, encrypted_header)
			else {
				continue;
			};

			// Decode decrypted header bytes
			let header: super::header::Header<P> = bincode::decode_from_slice(
				&bytes,
				bincode::config::standard(),
			)?
			.0;

			// Try to remove message number to get message key or break loop
			// because of no point in checking other keys
			let Some(msg_key) = values.remove(&header.msg_num()) else {
				break;
			};

			if values.is_empty() {
				empty_header_key = Some(header_key.to_owned());
			}
			ret = Some(msg_key);
		}

		// Remove header key with no skipped message keys
		if let Some(ref header_key) = empty_header_key {
			self.0.remove(header_key).expect("We had a reference to it.");
		}

		Ok(ret)
	}
}

// #[cfg(test)]
// mod tests {
// #[test]
// fn test_get() {
// let storage = super::SkippedMsgKeys::new();

// }
// }
