#[derive(Debug)]
#[non_exhaustive]
pub enum Decrypt {
	Header(super::chain::error::DecryptHeader),
	NewMsg(alloc::boxed::Box<dyn core::error::Error>),
	PopSkippedMsgKey(super::chain::error::PopSkippedMsgKey),
	RecvChainKdf(super::chain::error::RecvKdf),
	SkipCurrChainMsgKeys(super::chain::error::SkipMsgKeys),
	SkipOldChainMsgKeys(super::chain::error::SkipMsgKeys),
	SkippedMsg(alloc::boxed::Box<dyn core::error::Error>),
}

impl From<super::chain::error::DecryptHeader> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: super::chain::error::DecryptHeader) -> Self {
		Self::Header(e)
	}
}

impl From<super::chain::error::PopSkippedMsgKey> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: super::chain::error::PopSkippedMsgKey) -> Self {
		Self::PopSkippedMsgKey(e)
	}
}

impl From<super::chain::error::RecvKdf> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: super::chain::error::RecvKdf) -> Self {
		Self::RecvChainKdf(e)
	}
}

impl core::error::Error for Decrypt {
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Header(ref e) => Some(e),
			Self::NewMsg(e) | Self::SkippedMsg(e) => Some(e.as_ref()),
			Self::PopSkippedMsgKey(ref e) => Some(e),
			Self::RecvChainKdf(ref e) => Some(e),
			Self::SkipCurrChainMsgKeys(ref e)
			| Self::SkipOldChainMsgKeys(ref e) => Some(e),
		}
	}
}

impl core::fmt::Display for Decrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Header(_) => {
				write!(f, "Failed to decrypt the header.")
			}
			Self::NewMsg(_) => {
				write!(f, "Failed to decrypt the new message.")
			}
			Self::PopSkippedMsgKey(_) => {
				write!(f, "Failed to pop a skipped message key.")
			}
			Self::RecvChainKdf(_) => {
				write!(f, "Failed to kdf receiving chain.")
			}
			Self::SkipCurrChainMsgKeys(_) => {
				write!(f, "Failed to skip current chain message keys.")
			}
			Self::SkipOldChainMsgKeys(_) => {
				write!(f, "Failed to skip old chain message keys.")
			}
			Self::SkippedMsg(_) => {
				write!(f, "Failed to decrypt a skipped message.")
			}
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Encrypt {
	SendChainKdf(super::chain::error::SendKdf),
	EncodeHeader(bincode::error::EncodeError),
	HeaderBytes(alloc::boxed::Box<dyn core::error::Error>),
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
			// TODO: wait `bincode`'s implementation of `core::error::Error`
			Self::EncodeHeader(_) => None,
			Self::HeaderBytes(e) | Self::Plain(e) => Some(e.as_ref()),
		}
	}
}

impl core::fmt::Display for Encrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::SendChainKdf(_) => {
				write!(f, "Failed to kdf sending chain.")
			}
			// TODO: Remove `{}` if `bincode::error::EncodeError` is valid
			// source
			Self::EncodeHeader(e) => {
				write!(f, "Failed to encode the header: {e}.")
			}
			Self::HeaderBytes(_) => {
				write!(f, "Failed to encrypt the header bytes.")
			}
			Self::Plain(_) => {
				write!(f, "Failed to encrypt a plain text.")
			}
		}
	}
}
