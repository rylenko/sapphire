/// Error when encrypting plain text.
#[derive(Debug)]
#[non_exhaustive]
pub enum Encrypt {
	/// Failed to forward sending chain.
	SendChainKdf(super::chain::error::SendKdf),
	/// Failed to encode the header.
	EncodeHeader(bincode::error::EncodeError),
	/// Failed to encrypt the header.
	HeaderBytes(alloc::boxed::Box<dyn core::error::Error>),
	/// Failed to encrypt the plain data.
	Plain(alloc::boxed::Box<dyn core::error::Error>),
}

impl From<bincode::error::EncodeError> for Encrypt {
	#[inline]
	#[must_use]
	fn from(e: bincode::error::EncodeError) -> Self {
		Self::EncodeHeader(e)
	}
}

impl From<super::chain::error::SendKdf> for Encrypt {
	#[inline]
	#[must_use]
	fn from(e: super::chain::error::SendKdf) -> Self {
		Self::SendChainKdf(e)
	}
}

impl core::error::Error for Encrypt {
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::SendChainKdf(e) => Some(e),
			// TODO: `bincode`'s wait implementation of `core::error::Error`
			Self::EncodeHeader(_) => None,
			Self::HeaderBytes(e) | Self::Plain(e) => Some(e.as_ref()),
		}
	}
}

impl core::fmt::Display for Encrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::SendChainKdf(_) => write!(f, "Failed to kdf sending chain."),
			// TODO: Remove `{}` if `bincode::error::EncodeError` is valid
			// source. See `Encrypt`'s imlementation of `core::error::Error`
			// for more details
			Self::EncodeHeader(e) => {
				write!(f, "Failed to encode the header: {e}.")
			}
			Self::HeaderBytes(_) => {
				write!(f, "Failed to encrypt the header bytes.")
			}
			Self::Plain(_) => write!(f, "Failed to encrypt a plain text."),
		}
	}
}
