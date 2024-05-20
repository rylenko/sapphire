#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Decrypt {
	Bytes(cipher::error::Decrypt),
}

impl From<cipher::error::Decrypt> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: cipher::error::Decrypt) -> Self {
		Self::Bytes(e)
	}
}

impl core::error::Error for Decrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Bytes(ref e) => Some(e),
		}
	}
}

impl core::fmt::Display for Decrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Bytes(..) => write!(f, "Failed to decrypt header bytes."),
		}
	}
}
