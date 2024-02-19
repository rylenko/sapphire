/*!
Errors of default implementation.
*/

/// Error when encrypting plain data.
#[derive(Debug)]
#[non_exhaustive]
pub enum Encrypt {
	/// ChaCha20Poly1305 encryption error.
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

/// Error when encrypting header's bytes.
#[derive(Debug)]
#[non_exhaustive]
pub enum EncryptHeaderBytes {
	/// ChaCha20Poly1305 encryption error.
	Encrypt(chacha20poly1305::Error),
}

impl From<chacha20poly1305::Error> for EncryptHeaderBytes {
	#[inline]
	#[must_use]
	fn from(e: chacha20poly1305::Error) -> Self {
		Self::Encrypt(e)
	}
}

impl core::error::Error for EncryptHeaderBytes {
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

impl core::fmt::Display for EncryptHeaderBytes {
	#[inline]
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::Encrypt(_) => write!(f, "Failed to encrypt."),
		}
	}
}
