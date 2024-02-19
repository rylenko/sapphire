#[derive(Debug)]
#[non_exhaustive]
pub enum SendKdf {
	NoHeaderKey,
	NoKey,
}

impl core::error::Error for SendKdf {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::NoHeaderKey | Self::NoKey => None,
		}
	}
}

impl core::fmt::Display for SendKdf {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::NoHeaderKey => {
				write!(f, "There is no header key to forward chain.")
			}
			Self::NoKey => write!(f, "There is no base key to forward chain."),
		}
	}
}
