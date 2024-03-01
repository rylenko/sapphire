#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub enum Decrypt {
	Bytes(crate::cipher::error::Decrypt),
	FromBytes,
}

impl From<crate::cipher::error::Decrypt> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: crate::cipher::error::Decrypt) -> Self {
		Self::Bytes(e)
	}
}

impl core::error::Error for Decrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Bytes(ref e) => Some(e),
			Self::FromBytes => None,
		}
	}
}

impl core::fmt::Display for Decrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Bytes(..) => {
				write!(f, "Failed to decrypt the bytes.")
			}
			Self::FromBytes => {
				write!(f, "Failed to build the header from bytes.")
			}
		}
	}
}
