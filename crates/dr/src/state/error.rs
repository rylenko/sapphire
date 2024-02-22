#[derive(Debug)]
#[non_exhaustive]
pub enum Decrypt {
	Header(DecryptHeader),
	NewMsg(alloc::boxed::Box<dyn core::error::Error>),
	PopSkippedMsgKey(PopSkippedMsgKey),
	RecvChainKdf(RecvKdf),
	SkipCurrChainMsgKeys(SkipMsgKeys),
	SkipOldChainMsgKeys(SkipMsgKeys),
	SkippedMsg(alloc::boxed::Box<dyn core::error::Error>),
}

impl From<DecryptHeader> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: DecryptHeader) -> Self {
		Self::Header(e)
	}
}

impl From<PopSkippedMsgKey> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: PopSkippedMsgKey) -> Self {
		Self::PopSkippedMsgKey(e)
	}
}

impl From<RecvKdf> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: RecvKdf) -> Self {
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
			Self::Header(..) => {
				write!(f, "Failed to decrypt the header.")
			}
			Self::NewMsg(..) => {
				write!(f, "Failed to decrypt the new message.")
			}
			Self::PopSkippedMsgKey(..) => {
				write!(f, "Failed to pop a skipped message key.")
			}
			Self::RecvChainKdf(..) => {
				write!(f, "Failed to kdf receiving chain.")
			}
			Self::SkipCurrChainMsgKeys(..) => {
				write!(f, "Failed to skip current chain message keys.")
			}
			Self::SkipOldChainMsgKeys(..) => {
				write!(f, "Failed to skip old chain message keys.")
			}
			Self::SkippedMsg(..) => {
				write!(f, "Failed to decrypt a skipped message.")
			}
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DecryptHeader {
	Decode(bincode::error::DecodeError),
	KeysNotFit,
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
			Self::Decode(..) | Self::KeysNotFit => None,
		}
	}
}

impl core::fmt::Display for DecryptHeader {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Decode(e) => {
				write!(f, "Failed to decode the header: {e}.")
			}
			Self::KeysNotFit => {
				write!(f, "Keys do not fit..")
			}
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Encrypt {
	SendChainKdf(SendKdf),
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

impl From<SendKdf> for Encrypt {
	#[inline]
	#[must_use]
	fn from(e: SendKdf) -> Self {
		Self::SendChainKdf(e)
	}
}

impl core::error::Error for Encrypt {
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::SendChainKdf(e) => Some(e),
			// TODO: wait `bincode`'s implementation of `core::error::Error`
			Self::EncodeHeader(..) => None,
			Self::HeaderBytes(e) | Self::Plain(e) => Some(e.as_ref()),
		}
	}
}

impl core::fmt::Display for Encrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::SendChainKdf(..) => {
				write!(f, "Failed to kdf sending chain.")
			}
			// TODO: Remove `{}` if `bincode::error::EncodeError` is valid
			// source
			Self::EncodeHeader(e) => {
				write!(f, "Failed to encode the header: {e}.")
			}
			Self::HeaderBytes(..) => {
				write!(f, "Failed to encrypt the header bytes.")
			}
			Self::Plain(..) => {
				write!(f, "Failed to encrypt a plain text.")
			}
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
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			// TODO: `bincode`'s wait implementation of `core::error::Error`
			Self::DecodeHeader(..) => None,
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
			Self::Kdf(..) => {
				write!(f, "Failed to push forward receive chain.")
			}
		}
	}
}
