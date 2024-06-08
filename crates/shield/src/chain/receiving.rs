/// Receiving chain forward moving error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ForwardError {
	/// No [`Master`] key to move chain forward.
	///
	/// [`Master`]: crate::key::Master
	NoMasterKey,
}

impl core::error::Error for ForwardError {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::NoMasterKey => None,
		}
	}
}

impl core::fmt::Display for ForwardError {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::NoMasterKey => {
				write!(f, "There is no master key to move chain forward.")
			}
		}
	}
}

/// Receiving chain of Double Ratchet algorithm.
#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Receiving {
	master_key: Option<crate::key::Master>,
	header_key: Option<crate::key::Header>,
	next_header_key: crate::key::Header,
	next_message_num: u32,
	skipped_message_keys: super::skipped_message_keys::SkippedMessageKeys,
	skip_message_keys_limit: u32,
}

impl Receiving {
	/// Creates a new receiving chain using passed keys.
	#[inline]
	#[must_use]
	pub(crate) fn new(
		next_header_key: crate::key::Header,
		skip_message_keys_limit: u32,
	) -> Self {
		Self {
			master_key: None,
			header_key: None,
			next_header_key,
			next_message_num: 0,
			skipped_message_keys:
				super::skipped_message_keys::SkippedMessageKeys::new(),
			skip_message_keys_limit,
		}
	}

	/// Moves receiving chain forward using [master key] [evolving] if
	/// [master key] is set. So see [master key] [evolving] method for more.
	///
	/// [master key]: crate::key::Master
	/// [evolving]: crate::key::Master::evolve
	pub(crate) fn forward(
		&mut self,
	) -> Result<crate::key::Message, ForwardError> {
		// Try to retrieve current master key and evolve it to get message key.
		let message_key = self
			.master_key
			.as_mut()
			.ok_or(ForwardError::NoMasterKey)?
			.evolve();

		// Increase next message number because new message key evolved;
		self.next_message_num += 1;
		Ok(message_key)
	}

	/// Returns current [header key].
	///
	/// [header key]: crate::key::Header
	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(crate) fn header_key(&self) -> Option<&crate::key::Header> {
		self.header_key.as_ref()
	}

	/// Returns current [master key].
	///
	/// [master key]: crate::key::Master
	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(crate) fn master_key(&self) -> Option<&crate::key::Master> {
		self.master_key.as_ref()
	}

	/// Returns next [header key].
	///
	/// [header key]: crate::key::Header
	#[cfg(test)]
	#[inline]
	#[must_use]
	pub(crate) fn next_header_key(&self) -> &crate::key::Header {
		&self.next_header_key
	}

	pub(crate) fn skip_message_keys(
		&mut self,
		until_num: u32,
	) -> Result<(), SkipMessageKeysError> {
		// Validate skip limit to mitigate a huge storage consumption.
		if self.next_message_num + self.skip_message_keys_limit < until_num {
			return Err(SkipMessageKeysError::Limit);
		}

		// Try to get a header key or return an error if there is no header
		// key.
		//
		// Cloning is needed because of future mutability.
		let header_key = Clone::clone(
			self.header_key
				.as_ref()
				.ok_or(SkipMessageKeysError::NoHeaderKey)?,
		);

		while self.next_message_num < until_num {
			// Move chain forward to get skipped message key.
			let message_num = self.next_message_num;
			let message_key = self.forward()?;

			// Insert a new skipped message key to the storage.
			self.skipped_message_keys.insert(
				Clone::clone(&header_key),
				message_num,
				message_key,
			);
		}
		Ok(())
	}

	/// Upgrades the chain with a new master [`key`] and a new next [header
	/// key]. In other words, it is as if a new chain is created with
	/// information about the previous chain.
	///
	/// The current next header key becomes the current header key and the
	/// message numbering is reset to zero.
	///
	/// [`key`]: crate::key::Master
	/// [header key]: crate::key::Header
	pub(crate) fn upgrade(
		&mut self,
		master_key: crate::key::Master,
		next_header_key: crate::key::Header,
	) {
		self.master_key = Some(master_key);
		self.header_key = Some(core::mem::replace(
			&mut self.next_header_key,
			next_header_key,
		));
		self.next_message_num = 0;
	}
}

/// Message keys skipping error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum SkipMessageKeysError {
	/// Forward moving error.
	Forward(ForwardError),
	/// Too many keys to skip.
	Limit,
	/// No [`Header`] key to insert to the [storage].
	///
	/// [`Header`]: crate::key::Header
	/// [storage]: super::skipped_message_keys::SkippedMessageKeys
	NoHeaderKey,
}

impl From<ForwardError> for SkipMessageKeysError {
	#[inline]
	#[must_use]
	fn from(e: ForwardError) -> Self {
		Self::Forward(e)
	}
}

impl core::error::Error for SkipMessageKeysError {
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Forward(e) => Some(e),
			Self::Limit | Self::NoHeaderKey => None,
		}
	}
}

impl core::fmt::Display for SkipMessageKeysError {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Forward(..) => {
				write!(f, "Failed to move chain forward to get a message key.")
			}
			Self::Limit => write!(f, "Too many keys to skip."),
			Self::NoHeaderKey => {
				write!(f, "There is no header key to insert to the storage.")
			}
		}
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_forward() {
		// Create the chain.
		let mut chain =
			super::Receiving::new(crate::key::Header::new([1; 32]), 0);
		chain.upgrade(
			crate::key::Master::new([0; 32]),
			crate::key::Header::new([2; 32]),
		);

		// Test success of moving forward.
		assert!(chain.forward().is_ok());
		assert_ne!(chain.master_key, Some(crate::key::Master::new([0; 32])));
		assert_eq!(chain.next_message_num, 1);

		// Test moving forward without master key.
		chain.master_key = None;
		assert_eq!(chain.forward(), Err(super::ForwardError::NoMasterKey));
	}

	#[test]
	fn test_skip_message_keys() -> Result<(), super::SkipMessageKeysError> {
		const LIMIT: u32 = 2;

		// Create the chain.
		let mut chain =
			super::Receiving::new(crate::key::Header::new([1; 32]), LIMIT);
		chain.upgrade(
			crate::key::Master::new([0; 32]),
			crate::key::Header::new([2; 32]),
		);

		// Test limit validation.
		assert!(chain.skip_message_keys(LIMIT + 1).is_err());

		// Clone the chain to test forward moving.
		let mut clone = Clone::clone(&chain);
		clone.forward()?;
		clone.forward()?;
		chain.skip_message_keys(2)?;
		assert_eq!(clone.forward()?, chain.forward()?);
		Ok(())
	}

	#[test]
	fn test_upgrade() -> Result<(), super::ForwardError> {
		// Create the chain.
		let mut chain =
			super::Receiving::new(crate::key::Header::new([1; 32]), 0);
		chain.upgrade(
			crate::key::Master::new([0; 32]),
			crate::key::Header::new([2; 32]),
		);

		// Move chain forward.
		for _ in 0..10 {
			chain.forward()?;
		}

		// Test upgrading method.
		chain.upgrade(
			crate::key::Master::new([3; 32]),
			crate::key::Header::new([4; 32]),
		);
		assert_eq!(chain.master_key, Some(crate::key::Master::new([3; 32])));
		assert_eq!(chain.header_key, Some(crate::key::Header::new([2; 32])));
		assert_eq!(chain.next_header_key, crate::key::Header::new([4; 32]));
		assert_eq!(chain.next_message_num, 0);
		Ok(())
	}
}
