#[derive(Debug)]
#[non_exhaustive]
pub enum Decrypt {
	Inner(chacha20poly1305::Error),
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
			Self::Inner(..) => {
				// TODO: Wait until `chacha20poly1305::Error` refuses `std` to
				// impl `Error`
				None
			}
		}
	}
}

impl core::fmt::Display for Decrypt {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Inner(..) => write!(f, "Failed to decrypt."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum DecryptAuth {
	Auth,
	Inner(chacha20poly1305::Error),
	NoHmac,
}

impl From<chacha20poly1305::Error> for DecryptAuth {
	#[inline]
	#[must_use]
	fn from(e: chacha20poly1305::Error) -> Self {
		Self::Inner(e)
	}
}

impl core::error::Error for DecryptAuth {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			// TODO: Wait until `chacha20poly1305::Error` refuses `std` to
			// impl `Error`
			Self::Auth | Self::Inner(..) | Self::NoHmac => None,
		}
	}
}

impl core::fmt::Display for DecryptAuth {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Auth => write!(f, "Failed to authenticate."),
			Self::Inner(..) => write!(f, "Failed to decrypt."),
			Self::NoHmac => write!(f, "No HMAC in cipher text."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum Encrypt {
	Inner(chacha20poly1305::Error),
}

impl From<chacha20poly1305::Error> for Encrypt {
	#[inline]
	#[must_use]
	fn from(e: chacha20poly1305::Error) -> Self {
		Self::Inner(e)
	}
}

impl core::error::Error for Encrypt {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Inner(..) => {
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
			Self::Inner(..) => write!(f, "Failed to encrypt."),
		}
	}
}

#[derive(Debug)]
#[non_exhaustive]
pub enum EncryptAuth {
	Inner(chacha20poly1305::Error),
}

impl From<chacha20poly1305::Error> for EncryptAuth {
	#[inline]
	#[must_use]
	fn from(e: chacha20poly1305::Error) -> Self {
		Self::Inner(e)
	}
}

impl core::error::Error for EncryptAuth {
	#[inline]
	#[must_use]
	fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
		match self {
			Self::Inner(..) => {
				// TODO: Wait until `chacha20poly1305::Error` refuses `std` to
				// impl `Error`
				None
			}
		}
	}
}

impl core::fmt::Display for EncryptAuth {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Inner(..) => write!(f, "Failed to encrypt."),
		}
	}
}
