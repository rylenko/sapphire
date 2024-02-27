/// Keys are not pair of header key and message number because of
/// reference to header key in getting function
pub(super) type Inner = hashbrown::HashMap<
	super::key::Hdr,
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

	/// Extends from another storage.
	pub(super) fn extend(&mut self, from: Self) {
		for (hdr_key, values) in from.0 {
			self.0.entry(hdr_key).or_default().extend(values.into_iter());
		}
	}

	/// Inserts new entry.
	pub(super) fn insert(
		&mut self,
		hdr_key: super::key::Hdr,
		msg_num: u32,
		msg_key: super::key::Msg,
	) {
		let values = self.0.entry(hdr_key).or_default();
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
		encrypted_hdr_buf: &mut [u8; super::encrypted_hdr_buf::LEN],
	) -> Result<Option<super::key::Msg>, super::error::PopSkippedMsgKey> {
		use zerocopy::FromBytes as _;

		let mut ret = None;
		// The header key will be located here, which will no longer contain
		// skipped message keys
		let mut empty_hdr_key = None;

		// Iterate over elements
		for (hdr_key, values) in &mut self.0 {
			// Try to decrypt header with iterated header key
			if super::cipher::decrypt(
				hdr_key.as_bytes(),
				encrypted_hdr_buf,
				&[],
			)
			.is_ok()
			{
				// Extract message number from header bytes
				let msg_num = super::hdr::Hdr::ref_from(
					&encrypted_hdr_buf
						[..super::encrypted_hdr_buf::LEN_WITHOUT_MAC],
				)
				.ok_or(super::error::PopSkippedMsgKey::HdrFromBytes)?
				.msg_num();

				// Try to remove message number to get message key
				if let Some(msg_key) = values.remove(&msg_num) {
					if values.is_empty() {
						empty_hdr_key = Some(hdr_key.clone());
					}
					ret = Some(msg_key);
				};

				// We managed to decrypt it, so there is no point in looking at
				// other keys
				break;
			};
		}

		// Remove header key with no skipped message keys
		if let Some(ref hdr_key) = empty_hdr_key {
			self.0.remove(hdr_key).expect("We had a reference to it.");
		}
		Ok(ret)
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_extend() {
		// Create test data
		let hdr_key_1 = crate::key::Hdr::from([1; 32]);
		let msg_key_1 = crate::key::Msg::from([2; 32]);
		let hdr_key_2 = crate::key::Hdr::from([3; 32]);
		let msg_key_2 = crate::key::Msg::from([4; 32]);
		let msg_key_3 = crate::key::Msg::from([5; 32]);

		// Create first storage
		let mut a = super::SkippedMsgKeys::new();
		a.insert(hdr_key_1.clone(), 0, msg_key_1.clone());

		// Create second storage
		let mut b = super::SkippedMsgKeys::new();
		b.insert(hdr_key_1.clone(), 3, msg_key_2.clone());
		b.insert(hdr_key_2.clone(), 1, msg_key_3.clone());

		// Extend and assert
		a.extend(b);
		assert_eq!(
			a.0.get(&hdr_key_1).unwrap().get(&0).unwrap().as_bytes(),
			msg_key_1.as_bytes(),
		);
		assert_eq!(
			a.0.get(&hdr_key_1).unwrap().get(&3).unwrap().as_bytes(),
			msg_key_2.as_bytes(),
		);
		assert_eq!(
			a.0.get(&hdr_key_2).unwrap().get(&1).unwrap().as_bytes(),
			msg_key_3.as_bytes(),
		);
	}

	#[test]
	fn test_insert() {
		// Create test data
		let hdr_key = crate::key::Hdr::from([1; 32]);
		let msg_key = crate::key::Msg::from([2; 32]);

		// Insert
		let mut storage = super::SkippedMsgKeys::new();
		storage.insert(hdr_key.clone(), 100, msg_key.clone());

		// Assert
		assert_eq!(
			storage.0.get(&hdr_key).unwrap().get(&100).unwrap().as_bytes(),
			msg_key.as_bytes(),
		);
	}

	#[test]
	fn test_pop() {
		// See receiving chain tests.
	}
}
