/// Storage of skipped message keys to get them later for decryption.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub(super) struct SkippedMessageKeys(
	/// The keys are not the header key and the message number, so that the
	/// header key is not cloned for the map getting function.
	hashbrown::HashMap<
		crate::key::Header,
		hashbrown::HashMap<u32, crate::key::Message>,
	>,
);

impl SkippedMessageKeys {
	/// Creates empty storage.
	#[inline]
	#[must_use]
	pub(super) fn new() -> Self {
		Self(hashbrown::HashMap::new())
	}

	/// Inserts new entry to the storage.
	pub(super) fn insert(
		&mut self,
		header_key: crate::key::Header,
		message_num: u32,
		message_key: crate::key::Message,
	) {
		self.0.entry(header_key).or_default().insert(message_num, message_key);
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_insert() {
		// Create test keys.
		let header_key = crate::key::Header::new([0; 32]);
		let message_key = crate::key::Message::new([1; 32]);

		// Create a storage.
		let mut storage = super::SkippedMessageKeys::new();
		assert_eq!(storage.0.len(), 0);

		// Insert test keys to the storage.
		storage.insert(header_key.clone(), 1, message_key.clone());
		storage.insert(header_key.clone(), 2, message_key.clone());

		// Test insertions.
		assert_eq!(storage.0.len(), 1);
		let entry = storage.0.get(&header_key).unwrap();
		assert_eq!(entry.len(), 2);
		assert_eq!(entry.get(&1).unwrap(), &message_key);
		assert_eq!(entry.get(&2).unwrap(), &message_key);
	}
}
