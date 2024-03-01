#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Decrypt {
	Auth,
}

impl core::error::Error for Decrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Auth => None,
		}
	}
}

impl core::fmt::Display for Decrypt {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Auth => write!(f, "Failed to authenticate."),
		}
	}
}
