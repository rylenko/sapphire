#[derive(Debug)]
#[non_exhaustive]
pub enum Decrypt {
	Auth,
	Inner(chacha20poly1305::Error),
	NoHmac,
}

impl From<chacha20poly1305::Error> for Decrypt {
	#[inline]
	#[must_use]
	fn from(e: chacha20poly1305::Error) -> Self {
		Self::Inner(e)
	}
}

impl core::error::Error for Decrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			// TODO: Wait until `chacha20poly1305::Error` refuses `std` to
			// impl `Error`
			Self::Auth | Self::Inner(_) | Self::NoHmac => None,
		}
	}
}

impl core::fmt::Display for Decrypt {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Auth => write!(f, "Failed to authenticate."),
			Self::Inner(_) => write!(f, "Failed to decrypt."),
			Self::NoHmac => write!(f, "No HMAC in cipher text."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DecryptHeader {
	Decrypt(chacha20poly1305::Error),
}

impl From<chacha20poly1305::Error> for DecryptHeader {
	#[inline]
	#[must_use]
	fn from(e: chacha20poly1305::Error) -> Self {
		Self::Decrypt(e)
	}
}

impl core::error::Error for DecryptHeader {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Decrypt(_) => {
				// TODO: Wait until `chacha20poly1305::Error` refuses `std` to
				// impl `Error`
				None
			}
		}
	}
}

impl core::fmt::Display for DecryptHeader {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Decrypt(_) => write!(f, "Failed to decrypt a header."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Encrypt {
	Encrypt(chacha20poly1305::Error),
}

impl From<chacha20poly1305::Error> for Encrypt {
	#[inline]
	#[must_use]
	fn from(e: chacha20poly1305::Error) -> Self {
		Self::Encrypt(e)
	}
}

impl core::error::Error for Encrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Encrypt(_) => {
				// TODO: Wait until `chacha20poly1305::Error` refuses `std` to
				// impl `Error`
				None
			}
		}
	}
}

impl core::fmt::Display for Encrypt {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Encrypt(_) => write!(f, "Failed to encrypt."),
		}
	}
}

/// Header encryption error.
#[derive(Debug)]
#[non_exhaustive]
pub enum EncryptHeader {
	Encrypt(chacha20poly1305::Error),
}

impl From<chacha20poly1305::Error> for EncryptHeader {
	#[inline]
	#[must_use]
	fn from(e: chacha20poly1305::Error) -> Self {
		Self::Encrypt(e)
	}
}

impl core::error::Error for EncryptHeader {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Encrypt(_) => {
				// TODO: Wait until `chacha20poly1305::Error` refuses `std` to
				// impl `Error`
				None
			}
		}
	}
}

impl core::fmt::Display for EncryptHeader {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Encrypt(_) => write!(f, "Failed to encrypt a header."),
		}
	}
}

/// Header encryption error.
#[derive(Debug)]
#[non_exhaustive]
pub enum PublicKeyDecode {
	InvalidLen,
}

impl core::error::Error for PublicKeyDecode {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::InvalidLen => None,
		}
	}
}

impl core::fmt::Display for PublicKeyDecode {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::InvalidLen => write!(f, "Invalid slice length."),
		}
	}
}
