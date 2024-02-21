#[derive(Debug)]
#[non_exhaustive]
pub enum DecryptHeader {
	Decode(bincode::error::DecodeError),
	Decrypt(alloc::boxed::Box<dyn core::error::Error>),
	NoKey,
}

impl From<bincode::error::DecodeError> for DecryptHeader {
	#[inline]
	#[must_use]
	fn from(e: bincode::error::DecodeError) -> Self {
		Self::Decode(e)
	}
}

impl core::error::Error for DecryptHeader {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			// TODO: wait `bincode`'s implementation of `core::error::Error`
			Self::Decode(_) | Self::NoKey => None,
			Self::Decrypt(e) => Some(e.as_ref()),
		}
	}
}

impl core::fmt::Display for DecryptHeader {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Decode(e) => {
				write!(f, "Failed to decode the header: {e}.")
			}
			Self::Decrypt(_) => write!(f, "Failed to decrypt the header."),
			Self::NoKey => write!(f, "No header key to decrypt."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PopSkippedMsgKey {
	DecodeHeader(bincode::error::DecodeError),
}

impl From<bincode::error::DecodeError> for PopSkippedMsgKey {
	#[inline]
	#[must_use]
	fn from(e: bincode::error::DecodeError) -> Self {
		Self::DecodeHeader(e)
	}
}

impl core::error::Error for PopSkippedMsgKey {
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			// TODO: `bincode`'s wait implementation of `core::error::Error`
			Self::DecodeHeader(_) => None,
		}
	}
}

impl core::fmt::Display for PopSkippedMsgKey {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			// TODO: Remove `{}` if `bincode::error::DecodeError` is valid
			// source
			Self::DecodeHeader(e) => {
				write!(f, "Failed to decode the header: {e}.")
			}
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RecvKdf {
	NoHeaderKey,
	NoKey,
}

impl core::error::Error for RecvKdf {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::NoHeaderKey | Self::NoKey => None,
		}
	}
}

impl core::fmt::Display for RecvKdf {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::NoHeaderKey => {
				write!(f, "There is no header key to forward chain.")
			}
			Self::NoKey => write!(f, "There is no base key to forward chain."),
		}
	}
}

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

#[derive(Debug)]
#[non_exhaustive]
pub enum SkipMsgKeys {
	TooMuch,
	Kdf(RecvKdf),
}

impl From<RecvKdf> for SkipMsgKeys {
	#[inline]
	#[must_use]
	fn from(e: RecvKdf) -> Self {
		Self::Kdf(e)
	}
}

impl core::error::Error for SkipMsgKeys {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::TooMuch => None,
			Self::Kdf(ref e) => Some(e),
		}
	}
}

impl core::fmt::Display for SkipMsgKeys {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::TooMuch => {
				write!(f, "Too much message keys to skip.")
			}
			Self::Kdf(_) => write!(f, "Failed to push forward receive chain."),
		}
	}
}
