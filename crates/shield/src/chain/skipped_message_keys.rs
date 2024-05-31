#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum InsertError {
	/// The maximum length of skipped message keys has been reached.
	MaxLen,
}

impl core::error::Error for InsertError {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::MaxLen => None,
		}
	}
}

impl core::fmt::Display for InsertError {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::MaxLen => write!(
				f,
				"The maximum length of missing message keys has been reached."
			),
		}
	}
}

/// Storage of skipped message keys to get them later for decryption.
#[derive(Clone, Eq, PartialEq)]
pub(super) struct SkippedMessageKeys {
	/// The keys are not the header key and the message number, so that the
	/// header key is not cloned for the map getting function.
	storage: hashbrown::HashMap<
		crate::key::Header,
		hashbrown::HashMap<u32, crate::key::Message>,
	>,
	/// Maximum length of the mapping with message numbers and message keys.
	max_len: usize,
}

impl SkippedMessageKeys {
	/// Creates empty storage.
	#[inline]
	#[must_use]
	pub(super) fn new(max_len: usize) -> Self {
		Self { storage: hashbrown::HashMap::new(), max_len }
	}

	/// Inserts new entry to the storage.
	pub(super) fn insert(
		&mut self,
		header_key: crate::key::Header,
		message_num: u32,
		message_key: crate::key::Message,
	) -> Result<(), InsertError> {
		// Get map with message numbers and message keys or create an empty
		// map.
		let entry = self.storage.entry(header_key).or_default();
		// Validate if we can add another skipped message key.
		if entry.len() >= self.max_len {
			return Err(InsertError::MaxLen);
		}
		// Insert skipped message key.
		entry.insert(message_num, message_key);
		Ok(())
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
		let mut keys = super::SkippedMessageKeys::new(2);
		assert_eq!(keys.storage.len(), 0);

		// Insert test keys to the storage.
		assert!(keys
			.insert(header_key.clone(), 1, message_key.clone())
			.is_ok());
		assert!(keys
			.insert(header_key.clone(), 2, message_key.clone())
			.is_ok());

		// Test insertions.
		assert_eq!(keys.storage.len(), 1);
		let entry = keys.storage.get(&header_key).unwrap();
		assert_eq!(entry.len(), 2);
		assert_eq!(entry.get(&1).unwrap(), &message_key);
		assert_eq!(entry.get(&2).unwrap(), &message_key);

		// Test maximum length validation.
		assert_eq!(
			keys.insert(header_key.clone(), 2, message_key.clone()),
			Err(super::InsertError::MaxLen)
		);
	}
}
