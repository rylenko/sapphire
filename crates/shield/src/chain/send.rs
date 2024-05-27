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
		match self.key {
			Some(ref mut key) => Ok(key.evolve()),
			None => Err(ForwardError::NoKey),
		}
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
		assert!(chain.forward().is_ok());
		assert_ne!(chain.key, Some(crate::key::Master::new([0; 32])));

		chain.key = None;
		assert_eq!(chain.forward(), Err(super::ForwardError::NoKey));
	}
}
