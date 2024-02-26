#[derive(Debug)]
#[non_exhaustive]
pub enum Decrypt {
	Hdr(DecryptHdr),
	NewMsg(super::cipher::error::Decrypt),
	PopSkippedMsgKey(PopSkippedMsgKey),
	RecvChainKdf(RecvKdf),
	SkipCurrChainMsgKeys(SkipMsgKeys),
	SkipOldChainMsgKeys(SkipMsgKeys),
	SkippedMsg(super::cipher::error::Decrypt),
	SmallEncryptedHdrBuf,
}

impl From<DecryptHdr> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: DecryptHdr) -> Self {
		Self::Hdr(e)
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
			Self::Hdr(ref e) => Some(e),
			Self::NewMsg(ref e) | Self::SkippedMsg(ref e) => Some(e),
			Self::PopSkippedMsgKey(ref e) => Some(e),
			Self::RecvChainKdf(ref e) => Some(e),
			Self::SkipCurrChainMsgKeys(ref e)
			| Self::SkipOldChainMsgKeys(ref e) => Some(e),
			Self::SmallEncryptedHdrBuf => None,
		}
	}
}

impl core::fmt::Display for Decrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Hdr(..) => {
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
			Self::SmallEncryptedHdrBuf => {
				write!(f, "Encrypted header's buffer too small. Use `dr::create_encrypted_header_buf`.")
			}
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DecryptHdr {
	FromBytes,
	KeysNotFit,
}

impl core::error::Error for DecryptHdr {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::FromBytes | Self::KeysNotFit => None,
		}
	}
}

impl core::fmt::Display for DecryptHdr {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::FromBytes => {
				write!(f, "Failed to convert bytes to header.")
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
	Buf(super::cipher::error::Encrypt),
	Hdr(super::cipher::error::Encrypt),
	SendChainKdf(SendKdf),
	SmallEncryptedHdrBuf,
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
			Self::Buf(ref e) | Self::Hdr(ref e) => Some(e),
			Self::SendChainKdf(e) => Some(e),
			Self::SmallEncryptedHdrBuf => None,
		}
	}
}

impl core::fmt::Display for Encrypt {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Buf(..) => {
				write!(f, "Failed to encrypt a buffer.")
			}
			Self::Hdr(..) => {
				write!(f, "Failed to encrypt the header bytes.")
			}
			Self::SendChainKdf(..) => {
				write!(f, "Failed to kdf sending chain.")
			}
			Self::SmallEncryptedHdrBuf => {
				write!(f, "Encrypted header's buffer too small.")
			}
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PopSkippedMsgKey {
	HdrFromBytes,
}

impl core::error::Error for PopSkippedMsgKey {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::HdrFromBytes => None,
		}
	}
}

impl core::fmt::Display for PopSkippedMsgKey {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::HdrFromBytes => {
				write!(f, "Failed to convert bytes to header.")
			}
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum RecvKdf {
	NoKey,
}

impl core::error::Error for RecvKdf {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::NoKey => None,
		}
	}
}

impl core::fmt::Display for RecvKdf {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::NoKey => write!(f, "There is no base key to forward chain."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum SendKdf {
	NoHdrKey,
	NoKey,
}

impl core::error::Error for SendKdf {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::NoHdrKey | Self::NoKey => None,
		}
	}
}

impl core::fmt::Display for SendKdf {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::NoHdrKey => {
				write!(f, "There is no header key to forward chain.")
			}
			Self::NoKey => write!(f, "There is no base key to forward chain."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum SkipMsgKeys {
	Kdf(RecvKdf),
	NoHdrKey,
	TooMuch,
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
			Self::Kdf(ref e) => Some(e),
			Self::NoHdrKey | Self::TooMuch => None,
		}
	}
}

impl core::fmt::Display for SkipMsgKeys {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Kdf(..) => {
				write!(f, "Failed to push forward receive chain.")
			}
			Self::NoHdrKey => {
				write!(f, "No header key to set.")
			}
			Self::TooMuch => {
				write!(f, "Too much message keys to skip.")
			}
		}
	}
}
