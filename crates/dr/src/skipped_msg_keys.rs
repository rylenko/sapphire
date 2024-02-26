/// Keys are not pair of header key and message number because of
/// reference to header key in getting function
pub(super) type Inner = hashbrown::HashMap<
	super::key::Header,
	hashbrown::HashMap<u32, super::key::Msg>,
>;

/// Storage for skipped message keys.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub(super) struct SkippedMsgKeys(Inner);

impl SkippedMsgKeys {
	/// Creates empty storage.
	#[inline]
	#[must_use]
	pub(super) fn new() -> Self {
		Self(hashbrown::HashMap::new())
	}

	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(super) fn inner(&self) -> &Inner {
		&self.0
	}

	/// Inserts new entry.
	pub(super) fn insert(
		&mut self,
		header_key: super::key::Header,
		msg_num: u32,
		msg_key: super::key::Msg,
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
		encrypted_header_buff: &mut [u8],
	) -> Result<Option<super::key::Msg>, super::error::PopSkippedMsgKey> {
		use zerocopy::FromBytes as _;

		let mut ret = None;
		// The header key will be located here, which will no longer contain
		// skipped message keys
		let mut empty_header_key = None;

		// Iterate over elements
		for (header_key, values) in &mut self.0 {
			// Try to decrypt header with iterated header key
			if super::cipher::decrypt(
				header_key.as_bytes(),
				encrypted_header_buff,
				&[],
			)
			.is_ok()
			{
				// Extract message number from header bytes
				let msg_num = super::header::Header::ref_from(
					&encrypted_header_buff[..encrypted_header_buff.len() - 32],
				)
				.ok_or(super::error::PopSkippedMsgKey::HeaderFromBytes)?
				.msg_num();

				// Try to remove message number to get message key
				if let Some(msg_key) = values.remove(&msg_num) {
					if values.is_empty() {
						empty_header_key = Some(header_key.clone());
					}
					ret = Some(msg_key);
				};

				// We managed to decrypt it, so there is no point in looking at
				// other keys
				break;
			};
		}

		// Remove header key with no skipped message keys
		if let Some(ref header_key) = empty_header_key {
			self.0.remove(header_key).expect("We had a reference to it.");
		}
		Ok(ret)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_insert() {
		// Create test data
		let header_key = crate::key::Header::from([1; 32]);
		let msg_key = crate::key::Msg::from([2; 32]);

		// Insert
		let mut a = super::SkippedMsgKeys::new();
		a.insert(header_key.clone(), 100, msg_key.clone());

		// Get
		let got_msg_key =
			a.0.get(&header_key).unwrap().get(&100).unwrap().as_bytes();
		assert_eq!(got_msg_key, &[2; 32]);
	}

	#[test]
	fn test_pop() {
		// See receiving chain tests.
	}
}
