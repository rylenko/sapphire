/// Storage of skipped message keys to get them later for decryption.
#[derive(Clone, Default, Eq, PartialEq)]
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
		Self::default()
	}

	/// Inserts new entry to the storage.
	pub(super) fn insert(
		&mut self,
		header_key: crate::key::Header,
		message_num: u32,
		message_key: crate::key::Message,
	) {
		// Insert skipped message key to the message numbers and message keys
		// mapping.
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
		let mut keys = super::SkippedMessageKeys::new();
		assert_eq!(keys.0.len(), 0);

		// Insert test keys to the storage.
		keys.insert(Clone::clone(&header_key), 1, Clone::clone(&message_key));
		keys.insert(Clone::clone(&header_key), 2, Clone::clone(&message_key));

		// Test insertions.
		assert_eq!(keys.0.len(), 1);
		let entry = keys.0.get(&header_key).unwrap();
		assert_eq!(entry.len(), 2);
		assert_eq!(entry.get(&1).unwrap(), &message_key);
		assert_eq!(entry.get(&2).unwrap(), &message_key);
	}
}
