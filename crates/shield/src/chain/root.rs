/// Root chain of the Double Ratchet state.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub(crate) struct Root {
	key: super::key::Root,
}

impl Root {
	/// Creates new root chain based on passed [`key`].
	///
	/// [`key`]: super::key::Root
	#[inline]
	#[must_use]
	pub(crate) const fn new(key: super::key::Root) -> Self {
		Self { key }
	}
}
