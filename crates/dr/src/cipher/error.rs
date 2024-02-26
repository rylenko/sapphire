#[derive(Debug)]
#[non_exhaustive]
pub enum Decrypt {
	Auth,
	SmallBuf,
}

impl core::error::Error for Decrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Auth | Self::SmallBuf => None,
		}
	}
}

impl core::fmt::Display for Decrypt {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Auth => write!(f, "Failed to authenticate."),
			Self::SmallBuf => write!(f, "Too small bufer."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Encrypt {
	SmallBuf,
}

impl core::error::Error for Encrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::SmallBuf => None,
		}
	}
}

impl core::fmt::Display for Encrypt {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::SmallBuf => write!(f, "Too small bufer."),
		}
	}
}
