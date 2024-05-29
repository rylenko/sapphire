/// Sending chain forward moving error.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum ForwardError {
	/// No [`Master`] key to move chain forward.
	///
	/// [`Master`]: crate::key::Master
	NoKey,
}

impl core::error::Error for ForwardError {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::NoKey => None,
		}
	}
}

impl core::fmt::Display for ForwardError {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::NoKey => {
				write!(f, "There is no master key to move chain forward.")
			}
		}
	}
}

/// Sending chain of Double Ratchet algorithm.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Send {
	key: Option<crate::key::Master>,
	header_key: Option<crate::key::Header>,
	next_header_key: crate::key::Header,
	next_message_num: u32,
	prev_chain_messages_count: u32,
}

impl Send {
	/// Creates new sending chain using passed keys.
	#[inline]
	#[must_use]
	pub(crate) const fn new(
		key: Option<crate::key::Master>,
		header_key: Option<crate::key::Header>,
		next_header_key: crate::key::Header,
	) -> Self {
		Self {
			key,
			header_key,
			next_header_key,
			next_message_num: 0,
			prev_chain_messages_count: 0,
		}
	}

	/// Moves sending chain forward using [master key] [evolving] if [master
	/// key] is set. So see [master key] [evolving] method for more.
	///
	/// [master key]: crate::key::Master
	/// [evolving]: crate::key::Master::evolve
	pub(crate) fn forward(
		&mut self,
	) -> Result<crate::key::Message, ForwardError> {
		// Try to retrieve current master key and evolve it to get message key.
		let message_key =
			self.key.as_mut().ok_or(ForwardError::NoKey)?.evolve();

		// Increase next message number because new message key evolved.
		self.next_message_num += 1;
		Ok(message_key)
	}

	/// Upgrades the chain with a new master [`key`] and a new next [header
	/// key]. In other words, it is as if a new chain is created with
	/// information about the previous chain.
	///
	/// The current next header key becomes the current header key, the number
	/// of messages in the chain is preserved, and the message numbering is
	/// reset to zero.
	///
	/// [`key`]: crate::key::Master
	/// [header key]: crate::key::Header
	pub(crate) fn upgrade(
		&mut self,
		key: crate::key::Master,
		next_header_key: crate::key::Header,
	) {
		self.key = Some(key);
		self.header_key = Some(core::mem::replace(
			&mut self.next_header_key,
			next_header_key,
		));
		self.prev_chain_messages_count = self.next_message_num;
		self.next_message_num = 0;
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn test_forward() {
		let mut chain = super::Send::new(
			Some(crate::key::Master::new([0; 32])),
			Some(crate::key::Header::new([1; 32])),
			crate::key::Header::new([2; 32]),
		);

		// Test success of forward moving.
		assert!(chain.forward().is_ok());
		assert_ne!(chain.key, Some(crate::key::Master::new([0; 32])));
		assert_eq!(chain.next_message_num, 1);

		// Test forward moving with no master key.
		chain.key = None;
		assert_eq!(chain.forward(), Err(super::ForwardError::NoKey));
	}

	#[test]
	fn test_upgrade() -> Result<(), super::ForwardError> {
		let mut chain = super::Send::new(
			Some(crate::key::Master::new([0; 32])),
			Some(crate::key::Header::new([1; 32])),
			crate::key::Header::new([2; 32]),
		);

		// Move chain forward.
		for _ in 0..10 {
			chain.forward()?;
		}

		// Test upgrading method.
		chain.upgrade(
			crate::key::Master::new([10; 32]),
			crate::key::Header::new([11; 32]),
		);
		assert_eq!(chain.key, Some(crate::key::Master::new([10; 32])));
		assert_eq!(chain.header_key, Some(crate::key::Header::new([2; 32])));
		assert_eq!(chain.next_header_key, crate::key::Header::new([11; 32]));
		assert_eq!(chain.next_message_num, 0);
		assert_eq!(chain.prev_chain_messages_count, 10);
		Ok(())
	}
}
