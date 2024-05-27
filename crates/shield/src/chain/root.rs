/// Root chain of the Double Ratchet state.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(crate) struct Root {
	key: crate::key::Root,
}

impl Root {
	/// Creates new root chain based on passed [`key`].
	///
	/// [`key`]: super::key::Root
	#[inline]
	#[must_use]
	pub(crate) const fn new(key: crate::key::Root) -> Self {
		Self { key }
	}

	/// Moves root chain forward using [root key] [evolving]. So see [root key]
	/// [evolving] method for more.
	///
	/// [root key]: crate::key::Root
	/// [evolving]: crate::key::Root::evolve
	#[inline]
	#[must_use]
	pub(crate) fn forward(
		&mut self,
		input: &x25519_dalek::SharedSecret,
	) -> (crate::key::Master, crate::key::Header) {
		self.key.evolve(input)
	}
}
