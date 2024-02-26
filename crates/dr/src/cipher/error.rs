#[derive(Debug)]
#[non_exhaustive]
pub enum Decrypt {
	Auth,
	SmallBuff,
}

impl core::error::Error for Decrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Auth | Self::SmallBuff => None,
		}
	}
}

impl core::fmt::Display for Decrypt {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Auth => write!(f, "Failed to authenticate."),
			Self::SmallBuff => write!(f, "Too small buffer."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Encrypt {
	SmallBuff,
}

impl core::error::Error for Encrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::SmallBuff => None,
		}
	}
}

impl core::fmt::Display for Encrypt {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::SmallBuff => write!(f, "Too small buffer."),
		}
	}
}
