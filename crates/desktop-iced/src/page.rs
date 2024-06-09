/// Application's pages enumeration.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub(crate) enum Page {
	Start,
	Settings,
}

impl Default for Page {
	/// Returns start page as default.
	#[inline]
	#[must_use]
	fn default() -> Self {
		Self::Start
	}
}
