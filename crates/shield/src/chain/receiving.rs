/// Receiving chain forward moving error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ForwardError {
	/// No [`Master] key to move chain forward.
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
				write!(f, "There is no master to move chain forward.")
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
}

impl Receiving {
	/// Creates a new receiving chain using passed keys.
	#[inline]
	#[must_use]
	pub(crate) fn new(
		next_header_key: crate::key::Header,
		skipped_message_keys_max_len: usize,
	) -> Self {
		Self {
			master_key: None,
			header_key: None,
			next_header_key,
			next_message_num: 0,
			skipped_message_keys:
				super::skipped_message_keys::SkippedMessageKeys::new(
					skipped_message_keys_max_len,
				),
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

#[cfg(test)]
mod tests {
	#[test]
	fn test_forward() {
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

		// Test moving forward without master key..
		chain.master_key = None;
		assert_eq!(chain.forward(), Err(super::ForwardError::NoMasterKey));
	}

	#[test]
	fn test_upgrade() -> Result<(), super::ForwardError> {
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
