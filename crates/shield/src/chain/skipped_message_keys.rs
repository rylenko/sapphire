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

	/// Finds and removes a skipped [message key] using [encrypted header].
	///
	/// It iterates through all the missing [header key]s, tries to decrypt the
	/// [encrypted header], if it succeeds, then it gets the [message key]
	/// based on the message number.
	///
	/// [message key]: crate::key::Message
	/// [header key]: crate::key::Header
	/// [encrypted header]: crate::header::Encrypted
	#[must_use]
	pub(super) fn find_and_remove(
		&mut self,
		encrypted_header: &crate::header::Encrypted,
	) -> Option<crate::key::Message> {
		let mut ret = None;

		// This will contain the header key whose last skipped message key was
		// removed.
		let mut empty_entry = None;

		for (header_key, message_keys) in &mut self.0 {
			// Try to decrypt encrypted header using iterated header key.
			let Ok(header) = encrypted_header.decrypt(header_key) else {
				continue;
			};

			// Try to remove skipped message key using its number from the
			// header.
			ret = message_keys.remove(&header.message_num());
			// Check that there is no skipped message keys for the header key.
			if message_keys.is_empty() {
				empty_entry = Some(header_key.clone());
			}

			// The header was decrypted, so there is no point in looking at
			// other keys.
			break;
		}

		// Remove empty header key entry if last skipped message key was
		// removed.
		if let Some(ref empty_entry) = empty_entry {
			self.0
				.remove(empty_entry)
				.expect("We found it in loop iterations.");
		}
		ret
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_find_and_remove() {
		// Create test keys.
		let header_key = crate::key::Header::new([0; 32]);
		let message_key_1 = crate::key::Message::new([1; 32]);
		let message_key_2 = crate::key::Message::new([2; 32]);

		// Create a storage.
		let mut keys = super::SkippedMessageKeys::new();
		// Insert test keys to the storage.
		keys.insert(
			Clone::clone(&header_key),
			1,
			Clone::clone(&message_key_1),
		);
		keys.insert(
			Clone::clone(&header_key),
			2,
			Clone::clone(&message_key_2),
		);
		assert_eq!(keys.0.get(&header_key).unwrap().len(), 2);

		// Create the header and encrypt it.
		let encrypted_header =
			crate::header::Header::new([3; 32], 2, 0).encrypt(&header_key);

		// Test removing of second message key.
		assert_eq!(
			keys.find_and_remove(&encrypted_header),
			Some(message_key_2)
		);

		// Test that first message key still here.
		assert_eq!(keys.0.len(), 1);
		assert_eq!(keys.0.get(&header_key).unwrap().len(), 1);

		// Try to find and remove message key that not in the storage.
		assert_eq!(keys.find_and_remove(&encrypted_header), None);
	}

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
