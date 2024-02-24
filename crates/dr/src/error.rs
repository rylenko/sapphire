#[derive(Debug)]
#[non_exhaustive]
pub enum Decrypt {
	Header(DecryptHeader),
	NewMsg(super::cipher::error::DecryptAuth),
	PopSkippedMsgKey(PopSkippedMsgKey),
	RecvChainKdf(RecvKdf),
	SkipCurrChainMsgKeys(SkipMsgKeys),
	SkipOldChainMsgKeys(SkipMsgKeys),
	SkippedMsg(super::cipher::error::DecryptAuth),
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
			Self::NewMsg(ref e) | Self::SkippedMsg(ref e) => Some(e),
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
	Decode,
	KeysNotFit,
}

impl core::error::Error for DecryptHeader {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Decode | Self::KeysNotFit => None,
		}
	}
}

impl core::fmt::Display for DecryptHeader {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Decode => {
				write!(f, "Failed to decode the header.")
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
	HeaderBytes(super::cipher::error::Encrypt),
	Plain(super::cipher::error::EncryptAuth),
	SendChainKdf(SendKdf),
}

impl From<super::cipher::error::Encrypt> for Encrypt {
	#[inline]
	#[must_use]
	fn from(e: super::cipher::error::Encrypt) -> Self {
		Self::HeaderBytes(e)
	}
}

impl From<super::cipher::error::EncryptAuth> for Encrypt {
	#[inline]
	#[must_use]
	fn from(e: super::cipher::error::EncryptAuth) -> Self {
		Self::Plain(e)
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
			Self::HeaderBytes(ref e) => Some(e),
			Self::Plain(ref e) => Some(e),
			Self::SendChainKdf(e) => Some(e),
		}
	}
}

impl core::fmt::Display for Encrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::SendChainKdf(..) => {
				write!(f, "Failed to kdf sending chain.")
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
	DecodeHeader,
}

impl core::error::Error for PopSkippedMsgKey {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::DecodeHeader => None,
		}
	}
}

impl core::fmt::Display for PopSkippedMsgKey {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::DecodeHeader => {
				write!(f, "Failed to decode the header.")
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
